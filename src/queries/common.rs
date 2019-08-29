#![allow(non_camel_case_types)]
#[allow(unused_imports)]
use crate::queries::{Amount, CountryCodeAlpha3, CurrencyCodeAlpha, CustomFieldName, Timestamp};
pub mod ping_test {
    #![allow(dead_code)]
    pub const OPERATION_NAME: &'static str = "pingTest";
    pub const QUERY: &'static str = "query pingTest {ping}\n";
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    pub struct ResponseData {
        #[doc = "Returns the literal string 'pong'.\n"]
        pub ping: String,
    }
    #[allow(dead_code)]
    #[derive(Serialize)]
    pub struct PingTest {}
    impl crate::GraphQLQueryCLI for PingTest {
        type ResponseData = ResponseData;
        fn into_query_body(self) -> ::graphql_client::QueryBody<Self> {
            graphql_client::QueryBody {
                variables: self,
                query: QUERY,
                operation_name: OPERATION_NAME,
            }
        }
    }
}
