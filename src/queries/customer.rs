#![allow(non_camel_case_types)]
#[allow(unused_imports)]
use crate::queries::{Amount, CountryCodeAlpha3, CurrencyCodeAlpha, CustomFieldName, Timestamp};
pub mod customer_client_token {
    #![allow(dead_code)]
    pub const OPERATION_NAME: &'static str = "customerClientToken";
    pub const QUERY : & 'static str = "mutation customerClientToken($custID: ID!) {\n    createClientToken(input:{clientToken:{customerId:$custID}}) {\n        clientToken\n    }\n}\n\nmutation createCustomer($customer: CustomerInput!) {\n    createCustomer(input:{customer:$customer}) {\n        customer {\n            email,\n            id,\n            firstName,\n            lastName,\n            company\n        }\n    }\n}\n\nmutation updateCustomer($custID: ID!, $customer: CustomerInput!) {\n    updateCustomer(input:{customerId:$custID,customer:$customer}) {\n        customer {\n            email,\n            id,\n            firstName,\n            lastName,\n            company\n        }\n    }\n}\n\nmutation deleteCustomer($custID: ID!, $clientMutationId: String) {\n    deleteCustomer(input:{customerId:$custID, clientMutationId: $clientMutationId}) {\n        clientMutationId\n    }\n}\n\nquery getCustomer($custID: ID!) {\n    node(id:$custID) {\n        __typename\n        ... on Customer {\n            id,email,firstName,lastName,company,createdAt,\n            paymentMethods(first:0){edges{cursor,node{id,usage,details {\n                __typename\n                ... on PayPalAccountDetails {\n                    email,firstName,lastName\n                }\n            }}}}\n        }\n    }\n}\n\n" ;
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
    #[doc = "Top-level fields returned when creating a client token.\n"]
    pub struct CustomerClientTokenCreateClientToken {
        #[doc = "A Base64 encoded string used to initialize client SDKs.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "clientToken")]
        pub client_token: Option<String>,
    }
    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    pub struct ResponseData {
        #[doc = "Create a client token that can be used to initialize a client in order to tokenize payment information.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "createClientToken")]
        pub create_client_token: Option<CustomerClientTokenCreateClientToken>,
    }
    #[allow(dead_code)]
    #[derive(Serialize)]
    pub struct CustomerClientToken {
        #[serde(rename = "custID")]
        pub cust_id: ID,
    }
    impl graphql_client::GraphQLQueryCLI for CustomerClientToken {
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
pub mod create_customer {
    #![allow(dead_code)]
    pub const OPERATION_NAME: &'static str = "createCustomer";
    pub const QUERY : & 'static str = "mutation customerClientToken($custID: ID!) {\n    createClientToken(input:{clientToken:{customerId:$custID}}) {\n        clientToken\n    }\n}\n\nmutation createCustomer($customer: CustomerInput!) {\n    createCustomer(input:{customer:$customer}) {\n        customer {\n            email,\n            id,\n            firstName,\n            lastName,\n            company\n        }\n    }\n}\n\nmutation updateCustomer($custID: ID!, $customer: CustomerInput!) {\n    updateCustomer(input:{customerId:$custID,customer:$customer}) {\n        customer {\n            email,\n            id,\n            firstName,\n            lastName,\n            company\n        }\n    }\n}\n\nmutation deleteCustomer($custID: ID!, $clientMutationId: String) {\n    deleteCustomer(input:{customerId:$custID, clientMutationId: $clientMutationId}) {\n        clientMutationId\n    }\n}\n\nquery getCustomer($custID: ID!) {\n    node(id:$custID) {\n        __typename\n        ... on Customer {\n            id,email,firstName,lastName,company,createdAt,\n            paymentMethods(first:0){edges{cursor,node{id,usage,details {\n                __typename\n                ... on PayPalAccountDetails {\n                    email,firstName,lastName\n                }\n            }}}}\n        }\n    }\n}\n\n" ;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    #[doc = "A string representing a custom field value. Contains letters, numbers, and underscores.\n"]
    type CustomFieldName = super::CustomFieldName;
    #[derive(PartialEq, Serialize)]
    pub struct CustomFieldInput {
        pub name: CustomFieldName,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
    }
    impl CustomFieldInput {
        pub fn new(name: CustomFieldName) -> Self {
            Self {
                name: name,
                value: None,
            }
        }
    }
    #[derive(PartialEq, Serialize)]
    pub struct CustomerInput {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub company: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "customFields")]
        pub custom_fields: Option<Vec<CustomFieldInput>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub email: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "firstName")]
        pub first_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "lastName")]
        pub last_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "phoneNumber")]
        pub phone_number: Option<String>,
    }
    impl CustomerInput {
        pub fn new() -> Self {
            Self {
                company: None,
                custom_fields: None,
                email: None,
                first_name: None,
                last_name: None,
                phone_number: None,
            }
        }
    }
    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    #[doc = "Information about a customer and their associated payment methods and transactions.\n"]
    pub struct CreateCustomerCreateCustomerCustomer {
        #[doc = "Email address for this customer.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub email: Option<String>,
        #[doc = "Unique identifier.\n"]
        pub id: ID,
        #[doc = "Customer's first name.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "firstName")]
        pub first_name: Option<String>,
        #[doc = "Customer's last name.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "lastName")]
        pub last_name: Option<String>,
        #[doc = "Company or business name associated with this customer.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub company: Option<String>,
    }
    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    #[doc = "Top-level fields returned when creating a customer.\n"]
    pub struct CreateCustomerCreateCustomer {
        #[doc = "Information about the customer that was created. Can be used when vaulting\npayment methods or creating transactions to associate those objects.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub customer: Option<CreateCustomerCreateCustomerCustomer>,
    }
    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    pub struct ResponseData {
        #[doc = "Create a customer for storing individual customer information and/or grouping transactions and multi-use payment methods.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "createCustomer")]
        pub create_customer: Option<CreateCustomerCreateCustomer>,
    }
    #[allow(dead_code)]
    #[derive(Serialize)]
    pub struct CreateCustomer {
        pub customer: CustomerInput,
    }
    impl graphql_client::GraphQLQueryCLI for CreateCustomer {
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
pub mod update_customer {
    #![allow(dead_code)]
    pub const OPERATION_NAME: &'static str = "updateCustomer";
    pub const QUERY : & 'static str = "mutation customerClientToken($custID: ID!) {\n    createClientToken(input:{clientToken:{customerId:$custID}}) {\n        clientToken\n    }\n}\n\nmutation createCustomer($customer: CustomerInput!) {\n    createCustomer(input:{customer:$customer}) {\n        customer {\n            email,\n            id,\n            firstName,\n            lastName,\n            company\n        }\n    }\n}\n\nmutation updateCustomer($custID: ID!, $customer: CustomerInput!) {\n    updateCustomer(input:{customerId:$custID,customer:$customer}) {\n        customer {\n            email,\n            id,\n            firstName,\n            lastName,\n            company\n        }\n    }\n}\n\nmutation deleteCustomer($custID: ID!, $clientMutationId: String) {\n    deleteCustomer(input:{customerId:$custID, clientMutationId: $clientMutationId}) {\n        clientMutationId\n    }\n}\n\nquery getCustomer($custID: ID!) {\n    node(id:$custID) {\n        __typename\n        ... on Customer {\n            id,email,firstName,lastName,company,createdAt,\n            paymentMethods(first:0){edges{cursor,node{id,usage,details {\n                __typename\n                ... on PayPalAccountDetails {\n                    email,firstName,lastName\n                }\n            }}}}\n        }\n    }\n}\n\n" ;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    #[doc = "A string representing a custom field value. Contains letters, numbers, and underscores.\n"]
    type CustomFieldName = super::CustomFieldName;
    #[derive(PartialEq, Serialize)]
    pub struct CustomFieldInput {
        pub name: CustomFieldName,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
    }
    impl CustomFieldInput {
        pub fn new(name: CustomFieldName) -> Self {
            Self {
                name: name,
                value: None,
            }
        }
    }
    #[derive(PartialEq, Serialize)]
    pub struct CustomerInput {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub company: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "customFields")]
        pub custom_fields: Option<Vec<CustomFieldInput>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub email: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "firstName")]
        pub first_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "lastName")]
        pub last_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "phoneNumber")]
        pub phone_number: Option<String>,
    }
    impl CustomerInput {
        pub fn new() -> Self {
            Self {
                company: None,
                custom_fields: None,
                email: None,
                first_name: None,
                last_name: None,
                phone_number: None,
            }
        }
    }
    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    #[doc = "Information about a customer and their associated payment methods and transactions.\n"]
    pub struct UpdateCustomerUpdateCustomerCustomer {
        #[doc = "Email address for this customer.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub email: Option<String>,
        #[doc = "Unique identifier.\n"]
        pub id: ID,
        #[doc = "Customer's first name.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "firstName")]
        pub first_name: Option<String>,
        #[doc = "Customer's last name.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "lastName")]
        pub last_name: Option<String>,
        #[doc = "Company or business name associated with this customer.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub company: Option<String>,
    }
    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    #[doc = "Top-level fields returned when updating a customer.\n"]
    pub struct UpdateCustomerUpdateCustomer {
        #[doc = "Information about the customer that was updated.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub customer: Option<UpdateCustomerUpdateCustomerCustomer>,
    }
    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    pub struct ResponseData {
        #[doc = "Update a customer's information.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "updateCustomer")]
        pub update_customer: Option<UpdateCustomerUpdateCustomer>,
    }
    #[allow(dead_code)]
    #[derive(Serialize)]
    pub struct UpdateCustomer {
        #[serde(rename = "custID")]
        pub cust_id: ID,
        pub customer: CustomerInput,
    }
    impl graphql_client::GraphQLQueryCLI for UpdateCustomer {
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
pub mod delete_customer {
    #![allow(dead_code)]
    pub const OPERATION_NAME: &'static str = "deleteCustomer";
    pub const QUERY : & 'static str = "mutation customerClientToken($custID: ID!) {\n    createClientToken(input:{clientToken:{customerId:$custID}}) {\n        clientToken\n    }\n}\n\nmutation createCustomer($customer: CustomerInput!) {\n    createCustomer(input:{customer:$customer}) {\n        customer {\n            email,\n            id,\n            firstName,\n            lastName,\n            company\n        }\n    }\n}\n\nmutation updateCustomer($custID: ID!, $customer: CustomerInput!) {\n    updateCustomer(input:{customerId:$custID,customer:$customer}) {\n        customer {\n            email,\n            id,\n            firstName,\n            lastName,\n            company\n        }\n    }\n}\n\nmutation deleteCustomer($custID: ID!, $clientMutationId: String) {\n    deleteCustomer(input:{customerId:$custID, clientMutationId: $clientMutationId}) {\n        clientMutationId\n    }\n}\n\nquery getCustomer($custID: ID!) {\n    node(id:$custID) {\n        __typename\n        ... on Customer {\n            id,email,firstName,lastName,company,createdAt,\n            paymentMethods(first:0){edges{cursor,node{id,usage,details {\n                __typename\n                ... on PayPalAccountDetails {\n                    email,firstName,lastName\n                }\n            }}}}\n        }\n    }\n}\n\n" ;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    #[doc = "A string representing a custom field value. Contains letters, numbers, and underscores.\n"]
    type CustomFieldName = super::CustomFieldName;
    #[derive(PartialEq, Serialize)]
    pub struct CustomFieldInput {
        pub name: CustomFieldName,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
    }
    impl CustomFieldInput {
        pub fn new(name: CustomFieldName) -> Self {
            Self {
                name: name,
                value: None,
            }
        }
    }
    #[derive(PartialEq, Serialize)]
    pub struct CustomerInput {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub company: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "customFields")]
        pub custom_fields: Option<Vec<CustomFieldInput>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub email: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "firstName")]
        pub first_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "lastName")]
        pub last_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "phoneNumber")]
        pub phone_number: Option<String>,
    }
    impl CustomerInput {
        pub fn new() -> Self {
            Self {
                company: None,
                custom_fields: None,
                email: None,
                first_name: None,
                last_name: None,
                phone_number: None,
            }
        }
    }
    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    #[doc = "Top-level output field from deleting a customer.\n"]
    pub struct DeleteCustomerDeleteCustomer {
        #[doc = "An identifier used to reconcile requests and responses.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "clientMutationId")]
        pub client_mutation_id: Option<String>,
    }
    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    pub struct ResponseData {
        #[doc = "Delete a customer, breaking association between any of the customer's\ntransactions. Will not delete if the customer has existing payment methods.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "deleteCustomer")]
        pub delete_customer: Option<DeleteCustomerDeleteCustomer>,
    }
    #[allow(dead_code)]
    #[derive(Serialize)]
    pub struct DeleteCustomer {
        #[serde(rename = "custID")]
        pub cust_id: ID,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "clientMutationId")]
        pub client_mutation_id: Option<String>,
    }
    impl graphql_client::GraphQLQueryCLI for DeleteCustomer {
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
pub mod get_customer {
    #![allow(dead_code)]
    pub const OPERATION_NAME: &'static str = "getCustomer";
    pub const QUERY : & 'static str = "mutation customerClientToken($custID: ID!) {\n    createClientToken(input:{clientToken:{customerId:$custID}}) {\n        clientToken\n    }\n}\n\nmutation createCustomer($customer: CustomerInput!) {\n    createCustomer(input:{customer:$customer}) {\n        customer {\n            email,\n            id,\n            firstName,\n            lastName,\n            company\n        }\n    }\n}\n\nmutation updateCustomer($custID: ID!, $customer: CustomerInput!) {\n    updateCustomer(input:{customerId:$custID,customer:$customer}) {\n        customer {\n            email,\n            id,\n            firstName,\n            lastName,\n            company\n        }\n    }\n}\n\nmutation deleteCustomer($custID: ID!, $clientMutationId: String) {\n    deleteCustomer(input:{customerId:$custID, clientMutationId: $clientMutationId}) {\n        clientMutationId\n    }\n}\n\nquery getCustomer($custID: ID!) {\n    node(id:$custID) {\n        __typename\n        ... on Customer {\n            id,email,firstName,lastName,company,createdAt,\n            paymentMethods(first:0){edges{cursor,node{id,usage,details {\n                __typename\n                ... on PayPalAccountDetails {\n                    email,firstName,lastName\n                }\n            }}}}\n        }\n    }\n}\n\n" ;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    #[doc = "A string representing a custom field value. Contains letters, numbers, and underscores.\n"]
    type CustomFieldName = super::CustomFieldName;
    #[doc = "An [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601#Times) timestamp with microsecond precision.\n"]
    type Timestamp = super::Timestamp;
    #[derive(PartialEq, Serialize)]
    pub struct CustomFieldInput {
        pub name: CustomFieldName,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
    }
    impl CustomFieldInput {
        pub fn new(name: CustomFieldName) -> Self {
            Self {
                name: name,
                value: None,
            }
        }
    }
    #[derive(PartialEq, Serialize)]
    pub struct CustomerInput {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub company: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "customFields")]
        pub custom_fields: Option<Vec<CustomFieldInput>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub email: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "firstName")]
        pub first_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "lastName")]
        pub last_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "phoneNumber")]
        pub phone_number: Option<String>,
    }
    impl CustomerInput {
        pub fn new() -> Self {
            Self {
                company: None,
                custom_fields: None,
                email: None,
                first_name: None,
                last_name: None,
                phone_number: None,
            }
        }
    }
    #[derive(Debug, Eq, PartialEq)]
    #[allow(non_camel_case_types)]
    pub enum PaymentMethodUsage {
        MULTI_USE,
        SINGLE_USE,
        Other(String),
    }
    impl ::serde::Serialize for PaymentMethodUsage {
        fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
            ser.serialize_str(match *self {
                PaymentMethodUsage::MULTI_USE => "MULTI_USE",
                PaymentMethodUsage::SINGLE_USE => "SINGLE_USE",
                PaymentMethodUsage::Other(ref s) => &s,
            })
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PaymentMethodUsage {
        fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let s = <String>::deserialize(deserializer)?;
            match s.as_str() {
                "MULTI_USE" => Ok(PaymentMethodUsage::MULTI_USE),
                "SINGLE_USE" => Ok(PaymentMethodUsage::SINGLE_USE),
                _ => Ok(PaymentMethodUsage::Other(s)),
            }
        }
    }
    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    #[doc = "Details about a PayPal account.\n"]
    pub struct GetCustomerNodeOnCustomerPaymentMethodsEdgesNodeDetailsOnPayPalAccountDetails {
        #[doc = "The email address associated with the PayPal account.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub email: Option<String>,
        #[doc = "The first name on the PayPal account.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "firstName")]
        pub first_name: Option<String>,
        #[doc = "The last name on the PayPal account.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "lastName")]
        pub last_name: Option<String>,
    }
    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    #[serde(tag = "__typename")]
    pub enum GetCustomerNodeOnCustomerPaymentMethodsEdgesNodeDetails {
        PayPalAccountDetails(
            GetCustomerNodeOnCustomerPaymentMethodsEdgesNodeDetailsOnPayPalAccountDetails,
        ),
        CreditCardDetails,
        CustomActionsPaymentMethodDetails,
        SamsungPayCardDetails,
        UsBankAccountDetails,
        VenmoAccountDetails,
    }
    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    #[doc = "Top-level field representing a payment method.\n"]
    pub struct GetCustomerNodeOnCustomerPaymentMethodsEdgesNode {
        #[doc = "Unique identifier.\n"]
        pub id: ID,
        #[doc = "Whether a payment method may be used only once or used multiple times.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub usage: Option<PaymentMethodUsage>,
        #[doc = "Details about the payment method specific to the type (e.g. credit card, PayPal account).\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub details: Option<GetCustomerNodeOnCustomerPaymentMethodsEdgesNodeDetails>,
    }
    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    #[doc = "A payment method within a PaymentMethodConnection.\n"]
    pub struct GetCustomerNodeOnCustomerPaymentMethodsEdges {
        #[doc = "This payment method's location within the PaymentMethodConnection. Used for requesting additional pages.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub cursor: Option<String>,
        #[doc = "The payment method.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub node: Option<GetCustomerNodeOnCustomerPaymentMethodsEdgesNode>,
    }
    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    #[doc = "A paginated list of payment methods.\n"]
    pub struct GetCustomerNodeOnCustomerPaymentMethods {
        #[doc = "A list of payment methods.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub edges: Option<Vec<Option<GetCustomerNodeOnCustomerPaymentMethodsEdges>>>,
    }
    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    #[doc = "Information about a customer and their associated payment methods and transactions.\n"]
    pub struct GetCustomerNodeOnCustomer {
        #[doc = "Unique identifier.\n"]
        pub id: ID,
        #[doc = "Email address for this customer.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub email: Option<String>,
        #[doc = "Customer's first name.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "firstName")]
        pub first_name: Option<String>,
        #[doc = "Customer's last name.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "lastName")]
        pub last_name: Option<String>,
        #[doc = "Company or business name associated with this customer.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub company: Option<String>,
        #[doc = "Date and time at which the customer was created.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "createdAt")]
        pub created_at: Option<Timestamp>,
        #[doc = "Payment methods belonging to this customer.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "paymentMethods")]
        pub payment_methods: Option<GetCustomerNodeOnCustomerPaymentMethods>,
    }
    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    #[serde(tag = "__typename")]
    pub enum GetCustomerNodeOn {
        Customer(GetCustomerNodeOnCustomer),
        PaymentMethod,
        Verification,
        Refund,
        Transaction,
    }
    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    pub struct GetCustomerNode {
        #[serde(flatten)]
        pub on: GetCustomerNodeOn,
    }
    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    pub struct ResponseData {
        #[doc = "Fetch any object that extends the Node interface using its ID.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub node: Option<GetCustomerNode>,
    }
    #[allow(dead_code)]
    #[derive(Serialize)]
    pub struct GetCustomer {
        #[serde(rename = "custID")]
        pub cust_id: ID,
    }
    impl graphql_client::GraphQLQueryCLI for GetCustomer {
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
