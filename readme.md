# Braintree Payment / GraphQL

<img align="right" src="./doc/logo.png" />

[![Build Status](https://travis-ci.org/davidgraeff/braintree-payment-graphql.svg)](https://travis-ci.org/davidgraeff/braintree-payment-graphql)
[![](https://meritbadge.herokuapp.com/braintree-payment-graphql)](https://crates.io/crates/braintree-payment-graphql)
[![](https://img.shields.io/badge/license-MIT-blue.svg)](http://opensource.org/licenses/MIT)

For those unfamiliar with Braintree or payments processing, [Braintree](https://www.braintreepayments.com)'s homepage is a good place to start to learn more, along with the developer documentation which provides a good overview of the available tools and API's.

This crate allows easy access to Braintree via the GraphQL interface.
It offers predefined, common queries and manages connection details.

The advantage of GraphQL is the ability to write custom, specific queries
with only those input fields that you need and only your indiviual selection of response fields.

1. Design and test your queries in the [Braintree API Explorer](https://graphql.braintreepayments.com/explorer/).
2. Create queries in your crates `queries` directory, for instance `queries/some_filename.graphql`.
3. Run the "braintree-queries" tool via `cargo run --bin braintree-queries` in that directory.
   The tool generates rust structs and methods to perform your queries, in a type safe manner.
   Have a look at the `examples/` directory.

## How to get started

The first thing you want to do is create a sandbox account.
A sandbox environment can be used to test your integration without needing to go through the full application process.
Once you've created an account, follow the instructions to retrieve your Merchant ID, Public Key, and Private Key and
store them in a `credentials.json` file. Use `credentials.json.example` as a template.
**Never commit this file!** You should add it to your `.gitignore` file now.

In your rust program, initialize the `Braintree` object first.
For example by using a credentials file.

```rust
extern crate braintreepayment_graphql;
use std::error::Error;

use braintreepayment_graphql::{Braintree, Credentials};

fn main() -> Result<(), Box<dyn Error>> {
    let bt = Braintree::new(Credentials::from_file("credentials.json")?);
}
```

### Adapt the HTTP client

You might want to adapt the http clients configuration ( [reqwest](https://docs.rs/reqwest) )
to your needs:

```rust
extern crate braintreepayment_graphql;
extern crate reqwest;

use std::error::Error;
use braintreepayment_graphql::{Braintree, Credentials};

fn main() -> Result<(), Box<dyn Error>> {
    use std::time::Duration;

    let client = reqwest::Client::builder()
        .gzip(true)
        .timeout(Duration::from_secs(10))
        .build()?;

    let bt = Braintree::with_client(Credentials::from_file("credentials.json")?, client);
}
```

### Predefined module organisation

With the `bt` object you perform GraphQL queries (comparable to HTTP GET)
and mutations (similar to POST, PUT).

The `bt.perform(query)` method aims to hide GraphQL details away though.
The `query` argument refers to a *Query* name. For example "CreateCustomer".

This crate comes with predefined queries / mutations for customer management
and one-time / recurring transactions. Find the GraphQL files in `queries/` and
get an overview of available queries, parameters and return types.

The queries are organised in a hierarchial module layout, starting with `queries`.
Customer related queries live in the submodule `customer`, transactions in the
submodule `transactions`.

Query structs reside in modules with the kebab-case formatted name of the respective
query (ie, a query with the name `CreateCustomer` lives in a corresponding module with
the name `create_customer`).

For example to bring `CreateCustomer` into scope, you would do the following

```rust
use braintreepayment_graphql::queries::customer::create_customer::*;
```

### Create, Update, Delete Customers as well as create client Tokens

All customer related operations are shown below with an example each.

```rust
#[allow(unused_imports)]
use braintreepayment_graphql::{Braintree};
#[allow(unused_imports)]
use failure::*;

fn create_customer(bt: &Braintree) -> Result<String, failure::Error> {
    // You usually want to bring the module with the query struct and input types into scope first.
    use braintreepayment_graphql::queries::customer::create_customer::*;

    // Perform the query with bt.perform. You may create the variables structure ahead of time, or
    // just in place like here. 
    let customer = bt
        .perform(CreateCustomer {
            customer: CustomerInput {
                first_name: Some("first".to_owned()),
                last_name: Some("last".to_owned()),
                email: Some("test@abc.de".to_owned()),
                ..CustomerInput::new()
            },
        })? // Unwrap the response. Most of the time the interesting value is nested inside multiple strucs.
        .create_customer.and_then(|r| r.customer).ok_or(err_msg("customer"))?;

    println!("Received customer {:?}", customer);
    Ok(customer.id)
}
```

The `bt.perform` method performs a synchronous network operation and returns with a `Result`.
Network failures as well as GraphQL query problems, invalid requests but also legitim errors
result in a returned Error.
Proper error handling is shown further down on an invalid charge_payment request.

```rust
fn update_customer(bt: &Braintree, customer_id: &str) -> Result<(), failure::Error> {
    use braintreepayment_graphql::queries::customer::update_customer::*;

    let _ = bt.perform(UpdateCustomer {
        cust_id: customer_id.to_owned(),
        customer: CustomerInput {
            first_name: Some("new".to_owned()),
            last_name: Some("name".to_owned()),
            email: Some("changed@abc.de".to_owned()),
            ..CustomerInput::new()
        },
    })?;
    Ok(())
}
```

While mutations (create, update, delete) have a reasonable response layout, that cannot be said
about queries, like the next one to retrieve a customer by ID.

Braintree implements a pageable API that works the same for all types of retrieval queries
including those 1-result simple queries. This results in a quite nested response layout, which
is handled by the `unwrap_query` macro. Wrap the `bt.perform(...)?` call followed by an arrow
and the target type, in this case `GetCustomerNodeOn::Customer`.

```rust
fn get_customer(bt: &Braintree, customer_id: &str) -> Result<(), failure::Error> {
    use braintreepayment_graphql::queries::customer::get_customer::*;

    let customer = unwrap_query!(bt
        .perform(
            GetCustomer {
                cust_id: customer_id.to_owned(),
            },
        )? => GetCustomerNodeOn::Customer)?;

    assert_eq!(customer.first_name, Some("new".to_owned()));
    assert_eq!(customer.last_name, Some("name".to_owned()));
    assert_eq!(customer.email, Some("changed@abc.de".to_owned()));
    Ok(())
}
```

A client token is necessary for the Web UI to initialize with a customer context.

```rust
fn customer_client_token(bt: &Braintree, customer_id:&str) -> Result<(), failure::Error> {
    use braintreepayment_graphql::queries::customer::customer_client_token::*;

    let client_token = bt
        .perform(CustomerClientToken {
            cust_id: customer_id.to_owned(),
        })?
        .create_client_token
        .and_then(|f| f.client_token)
        .ok_or(err_msg("Token"))?;

    println!("{}", client_token);
    Ok(())
}
```

All Braintree GraphQL mutations allow to optionally hand a *mutation_id*.
Such an ID is an identifier used to reconcile requests and responses.
Some operations like `DeleteCustomer` do not return anything
else but the *mutation_id*.

```rust
fn delete_customer(bt: &Braintree, customer_id:&str) -> Result<(), failure::Error> {
    use braintreepayment_graphql::queries::customer::delete_customer::*;

    let delete_mut_id_orig = mutation_id();

    let delete_mut_id = bt
        .perform(DeleteCustomer {
            cust_id: customer_id.to_owned(),
            client_mutation_id: Some(delete_mut_id_orig.to_owned()),
        })?
        .delete_customer
        .and_then(|f| f.client_mutation_id)
        .ok_or(err_msg("Token"))?;

    assert_eq!(delete_mut_id_orig, delete_mut_id);
    Ok(())
}
```

There are a few more, rare bits related to customers, not covered by this crates' queries.
Check the [Braintree API Explorer](https://graphql.braintreepayments.com/explorer/).

### Transactions

If decimal accuracy is required, like in the finance sector, a proper
decimal number representation is necessary.
This library uses `rust_decimal`.
Create a decimal number via the macro `dec!`, ie `dec!(12.12)` or
convert a string representation or an integer. Use floating point numbers at your own risk!

Once you have this, you are able to create your first transaction.
Find transaction related queries in this section.

#### Charge single-use payment method

```rust

```

#### Vault payment

```rust

```

#### Charge vaulted payment method

```rust

```

#### Charge vaulted payment method

```rust

```

#### Remove vaulted payment method

```rust

```

#### List vaulted payment methods of a customer

```rust

```

#### List transactions of a customer

```rust

```


### Error Handling

Transactions (and other operations) may fail.
If the error is related to Braintree (in contrast to network errors),
the `braintree_error` method will return a structure with the following information:

* `message` The human-readable error message. This value is not intended to be parsed and may change at any time.
* `path` A "path" vector with the GraphQL query or mutation causing the error. For example ["ChargePaymentMethod","Input","paymentMethodId"]
* `error_class` A string that represents the error class. Can be any of
  AUTHENTICATION, AUTHORIZATION, INTERNAL, UNSUPPORTED_CLIENT, NOT_FOUND, NOT_IMPLEMENTED, RESOURCE_LIMIT, SERVICE_AVAILABILITY, VALIDATION

A quote from the Braintree website:

>> Unsuccessful transactions are a normal part of transaction processing and should not be considered exceptional. If an error occurs, you will either not receive a transaction object on the payload, or you will receive only a partial object.

```rust
fn payment_charge_fail() -> Result<(), failure::Error> {
    use braintreepayment_graphql::queries::transactions::charge_payment_method::*;
    use rust_decimal_macros::*;

    let bt = Braintree::new(Credentials::from_file("credentials.json")?);

    let payment_id = "invalid_id";

    let response = bt.perform(ChargePaymentMethod {
        payment_method_id: payment_id.to_owned(),
        transaction: TransactionInput {
            order_id: Some("demo_id".to_owned()),
            purchase_order_number: Some("demo_id".to_owned()),
            ..TransactionInput::new(dec!(12.12))
        },
        client_mutation_id: None,
    });

    // We have used an invalid payment method id. We expect a VALIDATION error.
    let error = braintree_error(response.err().as_ref());
    if !error.is_some() {
        bail!("Expected error");
    }
    let error = error.unwrap();
    assert_eq!(error.error_class, "VALIDATION".to_owned());
    assert_eq!(error.path, vec!["chargePaymentMethod".to_owned(),"input".to_owned(), "paymentMethodId".to_owned()]);
    assert_eq!(error.message, "Unknown or expired single-use payment method.".to_owned());
    Ok(())
}
```

## Testing

This crate contains full integration tests, including creating a customer, enumerate customers, delete customers,
create transactions, check transactions, charge payments, vault payments.
A valid "credentials.json" file with the "Sandbox" environment must be present.

Start test runs with `cargo test` as usual.

## Disclaimer and Limitations

Note that this is an unofficial library, provided as-is, with no support whatsoever by Braintree.

API Stability: Nested responses might be automatically collapsed at some point with a few more feature changes to
the underlying graphql-client. A convience API for paging might be added.

This crate uses a modified branch of the graphql-client crate until 0.9 is released.

MIT licensed. Pull requests are welcome.

Cheers,
David Graeff
