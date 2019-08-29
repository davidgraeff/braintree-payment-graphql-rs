# Braintree Query Generator

This tool generates rust sources from GraphQL Schemas.
Generated query structs are to be consumed by `Braintree::perform(query)`
of the `braintreepayments_graphql` library.

Input: Your query input files are expected to be found in `./queries/`
with a `graphql` file extension and in the graphql format.

Output: Rust files are written to `src/queries`.

Generated queries are organised in a hierarchial module layout, starting with `queries`
and followed by the kebab-case formatted name of respective filename. If you
provide a "MyQueries.graphql" file for example, the module will be named "my_queries".

Queries / Mutations are accessible via a camel-case formatted name.
If your "MyQueries.graphql" file for example contains a query or mutation "myquery" and "another_query"
this tool will generate "Myquery" and "AnotherQuery" query structs.

To bring `AnotherQuery` into scope, you would do the following

```rust
mod queries;
use queries::my_queries::AnotherQuery::*;
```
