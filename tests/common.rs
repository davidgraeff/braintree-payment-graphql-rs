use braintreepayment_graphql::{Braintree, Credentials};
use failure::*;

pub(crate) fn print_errors_if_any(errors: &Vec<graphql_client::Error>) {
    println!("there are errors:");

    for error in errors {
        println!("{:?}", error);
    }
}

#[test]
fn ping_api() -> Result<(), failure::Error> {
    use braintreepayment_graphql::queries::common::ping_test::*;
    let bt = Braintree::new(Credentials::from_file("credentials.json")?);

    let response = bt.perform(PingTest {})?;

    if response.ping != "pong" {
        bail!("Expected pong response");
    }
    Ok(())
}
