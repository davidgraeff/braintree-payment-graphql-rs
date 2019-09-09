## For maintainers

Update the Braintree schema file with `cd braintree-queries-generator && ./update_schema.sh`.
Run `cargo run -p braintree-queries-generator` in this directory to update the generated query/mutation files.

Build the documentation locally with `cargo +nightly doc --features external_doc`

## Testing

This crate contains full integration tests, including creating a customer, enumerate customers, delete customers,
create transactions, check transactions, charge payments, vault payments.
A valid "credentials.json" file with the "Sandbox" environment must be present.

Start test runs with `cargo test` as usual.
