extern crate chrono;
extern crate doc_comment;
extern crate failure;
extern crate rand;
#[cfg(feature = "rustfmt")]
extern crate rustfmt_nightly;
extern crate serde;
extern crate serde_json;
extern crate syn;

use failure::*;
use graphql_client::Response;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;

pub mod generate;
pub mod queries;

#[doc(hidden)]
#[derive(Fail, Debug)]
#[fail(display = "The request was not successful")]
pub struct BraintreeError {
    errors: Vec<graphql_client::Error>,
}

/// The Braintree API can operate in a sandboxed environment.
/// Set the environment via the [Credentials].
#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub enum Environment {
    Sandbox,
    Production,
}

#[doc(hidden)]
impl Environment {
    fn base_url(&self) -> &str {
        match *self {
            Environment::Sandbox => "https://payments.sandbox.braintree-api.com/graphql",
            Environment::Production => "https://payments.braintree-api.com/graphql",
        }
    }
}

/// Your Braintree access including your merchant ID, your public and private key.
#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub struct Credentials {
    environment: Environment,
    #[allow(dead_code)]
    merchant_id: String,
    public_key: String,
    private_key: String,
}

#[doc(hidden)]
impl Credentials {
    pub fn from_file(credential_file: &str) -> Result<Credentials, failure::Error> {
        let mut f = File::open(credential_file)?;
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer)?;
        Ok(serde_json::from_slice(buffer.as_slice())?)
    }
}

/// The braintree credentials and http client state
///
/// Call the `perform` method on an object of this type
/// to perform queries or mutations.
pub struct Braintree {
    pub credentials: Credentials,
    merchant_url: String,
    user_agent: String,
    pub client: reqwest::Client,
}

/// Use the [braintree_error] method on a failed [Braintree::perform] call and get this struct back
/// with an English error message, a deterministic path based on the GraphQL schema and an error class.
#[derive(Debug)]
pub struct BraintreeErrorResult {
    pub message: String,
    pub path: Vec<String>,
    pub error_class: String,
}

/// All paginated query responses hide the actual content deeply nested in multiple wrapper structs
/// and enums. This macro unwraps the data.
#[macro_export]
macro_rules! unwrap_query {
    ($var:expr => $variant:path) => {
        match $var.node.ok_or(err_msg("Node"))?.on {
            $variant(v) => Ok(v),
            _ => Err(err_msg("Entry not found!")),
        }
    };
}

/// Return a mutation ID. The Braintree API allows to send a generated, random ID with each mutation
/// which will be part of the response. This allows to match a specific mutation with a response.
pub fn mutation_id() -> String {
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};

    thread_rng().sample_iter(&Alphanumeric).take(30).collect()
}

impl Braintree {
    /// Create a new Braintree instance with credentials
    pub fn new(credentials: Credentials) -> Braintree {
        Self::with_client(credentials, reqwest::Client::new())
    }

    /// Create a new Braintree instance with credentials and a custom client
    pub fn with_client(credentials: Credentials, client: reqwest::Client) -> Braintree {
        Braintree {
            merchant_url: format!(
                "{}/merchants/{}/",
                credentials.environment.base_url(),
                credentials.merchant_id
            ),
            user_agent: format!("Braintree Rust {}", env!("CARGO_PKG_VERSION")),
            credentials,
            client,
        }
    }

    /// You usually do not want to call this directly, but [perform] instead.
    ///
    /// This method will synchronously send the given query to Braintree and receives the response.
    /// It returns the plain text response non-deserialized.
    pub fn perform_graphql_response<QBody>(
        &self,
        query: graphql_client::QueryBody<QBody>,
    ) -> Result<String, failure::Error>
    where
        QBody: serde::ser::Serialize,
    {
        use reqwest::header::USER_AGENT;

        let mut resp = self
            .client
            .post(self.credentials.environment.base_url())
            .basic_auth(&self.credentials.public_key, Some(&self.credentials.private_key))
            .header(USER_AGENT, &self.user_agent)
            .header("Braintree-Version", "2019-09-01")
            .json(&query)
            .send()?;
        Ok(resp.text()?)
    }

    /// This method will synchronously send the given query to Braintree and receives the response.
    /// It returns a data response structure.
    pub fn perform<QUERY>(
        &self,
        variables: QUERY,
    ) -> std::result::Result<<QUERY as graphql_client::GraphQLQueryCLI>::ResponseData, failure::Error>
    where
        QUERY: graphql_client::GraphQLQueryCLI,
    {
        let response_body: Response<_> =
            serde_json::from_str(&self.perform_graphql_response(variables.into_query_body())?)?;
        if let Some(errors) = &response_body.errors {
            #[cfg(test)]
            print_errors_if_any(errors);
            if errors.len() > 0 {
                return Err(BraintreeError { errors: errors.clone() }.into());
            }
        }
        if let Some(data) = response_body.data {
            return Ok(data);
        }
        Err(format_err!("No data"))
    }
}

/// Use this method on a failed [Braintree::perform] call and get a [BraintreeErrorResult] struct back
/// with an English error message, a deterministic path based on the GraphQL schema and an error class.
///
/// None is returned if either the input is None or the returned error is not a Braintree error
/// or if the Braintree error does not follow the documented error specification (fields are missing).
pub fn braintree_error(response: Option<&Error>) -> Option<BraintreeErrorResult> {
    if response.is_none() {
        return None;
    }
    let response = response.unwrap();
    let err: Option<&BraintreeError> = response.as_fail().downcast_ref::<BraintreeError>();
    if err.is_none() {
        return None;
    }
    let err = err.unwrap();

    if err.errors.is_empty() {
        return None;
    }

    let first_err = match err.errors.get(0) {
        Some(first_err) => first_err,
        None => return None,
    };

    let message = first_err.message.clone();
    let path = first_err
        .path
        .as_ref()
        .and_then(|f| Some(f.clone()))
        .unwrap_or_default();
    let mut path: Vec<String> = path.iter().map(|f| format!("{}", f)).collect();

    let error_data = first_err.extensions.as_ref().unwrap();
    if let Some(input_path) = error_data.get("inputPath") {
        let input_path = input_path.as_array().and_then(|v| Some(v.clone())).unwrap_or_default();
        let mut input_path: Vec<String> = input_path
            .iter()
            .map(|f| f.as_str().unwrap_or_default().to_owned())
            .collect();
        path.append(&mut input_path);
    }

    if let Some(error_class) = error_data.get("errorClass") {
        let error_class = error_class
            .as_str()
            .and_then(|v| Some(v.to_owned()))
            .unwrap_or_default();
        return Some(BraintreeErrorResult {
            message,
            path,
            error_class,
        });
    } else {
        return None;
    }
}

#[cfg(test)]
pub fn print_errors_if_any(errors: &Vec<graphql_client::Error>) {
    println!("there are errors:");

    for error in errors {
        println!("{:?}", error);
    }
}
