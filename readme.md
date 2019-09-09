# Braintree Payment / GraphQL

<img align="right" src="./doc/logo.png" />

[![Build Status](https://github.com/davidgraeff/braintree-payment-graphql-rs/workflows/Integration/badge.svg)](https://github.com/davidgraeff/braintree-payment-graphql-rs/actions)
[![](https://meritbadge.herokuapp.com/braintreepayment_graphql)](https://crates.io/crates/braintreepayment_graphql)
[![](https://img.shields.io/badge/license-MIT-blue.svg)](http://opensource.org/licenses/MIT)

For those unfamiliar with Braintree or payments processing, [Braintree](https://www.braintreepayments.com)'s homepage is a good place to start to learn more, along with the developer documentation which provides a good overview of the available tools and API's.

This crate allows easy access to Braintree via the GraphQL interface.
It offers predefined, common queries and manages connection details.

The advantage of GraphQL is the ability to write custom, specific queries
with only those input fields that you need and your individual selection of response fields.

1. Design and test your queries in the [Braintree API Explorer](https://graphql.braintreepayments.com/explorer/).
2. Store those graphql queries in your crates `queries` directory, for instance `queries/some_filename.graphql`.
3. Run the "braintree-queries" tool via `cargo run --bin braintree-queries` in that directory.
   The tool generates rust structs and methods to perform your queries in a type safe manner.
   Have a look at the `examples/` directory.

## Cargo features

* rustls-tls: Use rustls instead of native-tls (openssl on Linux).
  If you want to compile this crate with [MUSL](https://doc.rust-lang.org/edition-guide/rust-2018/platform-and-target-support/musl-support-for-fully-static-binaries.html),
  this is what you want. Don't forget to disable the default features with --no-default-features.

## How to get started

The first thing you want to do is create a sandbox account.
A sandbox environment can be used to test your integration without needing to go through the full application process.
Once you've created an account, follow the instructions to retrieve your Merchant ID, Public Key, and Private Key and
store them in a `credentials.json` file. Use `credentials.json.example` as a template.
**Never commit this file!** You should add it to your `.gitignore` file now.

In your rust program, initialize the `Braintree` object via those credentials.

```rust
use std::error::Error;

use braintreepayment_graphql::{Braintree, Credentials};
use serde_json::from_str;

fn main() -> Result<(), Box<dyn Error>> {
    let bt = Braintree::new(Credentials::from_file("credentials.json")?);
    // OR: Avoid the IO access on start and embed the file
    let bt = Braintree::new(from_str(include_str!("credentials.json"))?);
}
```

You might want to adapt the http clients configuration ( [reqwest](https://docs.rs/reqwest) )
to your needs via the [`Braintree::with_client`] constructor.

### Module organisation

With the `bt` object you perform GraphQL queries (comparable to HTTP GET)
and mutations (similar to POST, PUT).

The `bt.perform(query)` method aims to hide GraphQL details away.
The `query` argument refers to a *Query* name. For example "CreateCustomer".

This crate comes with predefined queries / mutations for customer management
and one-time / recurring transactions. Find the GraphQL files in `queries/` and
get an overview of available queries, parameters and return types.

The queries are organised in a hierarchical module layout, starting with `queries`.
Customer related queries live in the submodule `customer`, transactions in the
submodule `transactions`.

Query structs reside in modules with the kebab-case formatted name of the respective
query (ie, a query with the name `CreateCustomer` lives in a corresponding module with
the name `create_customer`).

For example to bring `CreateCustomer` into scope, you would do the following

```rust
use braintreepayment_graphql::queries::customer::create_customer::*;
```

### Create, Update, Delete Customers as well as create client tokens

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
Network failures, invalid and bad requests but also legit errors (like "Gateway denied")
result in a returned Error.
Proper error handling is shown further down on an invalid `charge_payment` request.

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

Braintree implements a pageable API which results in some deeply nested response structures.
Use `unwrap_customer` to extract the customer object.

```rust
fn get_customer(bt: &Braintree, customer_id: &str) -> Result<(), failure::Error> {
    use braintreepayment_graphql::queries::{customer::get_customer::*, customer_helpers::unwrap_customer};

    let customer = bt
        .perform(
            GetCustomer {
                cust_id: customer_id.to_owned(),
            },
        )?;
    let customer = unwrap_customer(customer).ok_or(err_msg("No customer found with the given ID"))?;

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
        .ok_or(err_msg("No token found in the response"))?;

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
fn payment(
    bt: &Braintree,
    payment_method_id: &str,
    amount: rust_decimal::Decimal,
    order_id:Option<String>
) -> Result<ChargePaymentMethodChargePaymentMethodTransaction, failure::Error> {
    use braintreepayment_graphql::queries::transactions::charge_payment_method::*;

    let response = bt.perform(ChargePaymentMethod {
        payment_method_id: payment_method_id.to_owned(),
        transaction: TransactionInput {
            order_id,
            purchase_order_number: Some("demo_id".to_owned()),
            ..TransactionInput::new(amount)
        },
        client_mutation_id: None,
    })?.charge_payment_method
       .and_then(|f| f.transaction)
       .ok_or(err_msg("Expected a payment result"))?;

   Ok(response)
}
```

#### Vault payment

A vaulted payment response contains a new payment method ID which can be used the same like a single-use payment method ID.

```rust
fn vault(
    bt: &Braintree,
    payment_method_id: &str,
) -> Result<VaultPaymentVaultPaymentMethodPaymentMethod, failure::Error> {
    use braintreepayment_graphql::queries::transactions::vault_payment::*;

    let r = bt
        .perform(VaultPayment {
            vault_payment_input: VaultPaymentMethodInput {
                ..VaultPaymentMethodInput::new(payment_method_id.to_owned())
            },
        })?
        .vault_payment_method
        .and_then(|f| f.payment_method)
        .ok_or(err_msg("Expected a vault result"))?;
    Ok(r)
}
```

#### Remove vaulted payment method

```rust
fn delete_transaction(bt: &Braintree, payment_method_id: &str) -> Result<(), failure::Error> {
    use braintreepayment_graphql::queries::transactions::delete_vaulted_payment::*;

    let _ = bt.perform(DeleteVaultedPayment {
        input: DeletePaymentMethodFromVaultInput {
            ..DeletePaymentMethodFromVaultInput::new(payment_method_id.to_owned())
        },
    })?;

    Ok(())
}
```


#### Search for a transaction

```rust
use braintreepayment_graphql::queries::transaction_helper::unwrap_search_result;

pub fn search_transaction(
    bt: &Braintree,
    order_id: &str,
) -> Result<Vec<SearchTransactionSearchTransactionsEdgesNode>, failure::Error> {
    use crate::queries::transactions::search_transaction::*;

    let r = bt
        .perform(SearchTransaction {
            input: TransactionSearchInput {
                order_id: Some(SearchTextInput {
                    is: Some(order_id.to_owned()),
                    ..SearchTextInput::new()
                }),
                ..TransactionSearchInput::new()
            },
        })?;

    unwrap_search_result(r)
}
```


#### Get transaction by ID

```rust
use braintreepayment_graphql::queries::transaction_helper::unwrap_get_result;

pub fn get_transaction(
    bt: &Braintree,
    transaction_id: &str,
) -> Result<Option<GetTransactionSearchTransactionsEdgesNode>, failure::Error> {
    use crate::queries::transactions::get_transaction::*;

    let r = bt
        .perform(GetTransaction {
            transaction_id: transaction_id.to_owned(),
        })?;

    Ok(unwrap_get_result(r)?)
}
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

## Disclaimer and Limitations

Note that this is an unofficial library, provided as-is, with no support whatsoever by Braintree.
The generator tool uses a modified branch of the graphql-client crate until 0.9 is released.

MIT licensed. Pull requests are welcome.

Cheers,
David Graeff
