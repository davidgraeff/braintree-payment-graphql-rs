#![allow(non_camel_case_types)]
#[allow(unused_imports)]
use crate::queries::{Amount, CountryCodeAlpha3, CurrencyCodeAlpha, CustomFieldName, Timestamp};
pub mod charge_payment_method {
    #![allow(dead_code)]
    pub const OPERATION_NAME: &'static str = "chargePaymentMethod";
    pub const QUERY : & 'static str = "mutation chargePaymentMethod($paymentMethodId: ID!, $transaction:TransactionInput!, $clientMutationId: String) {\n    chargePaymentMethod(input: {\n        paymentMethodId:$paymentMethodId, transaction:$transaction, clientMutationId:$clientMutationId\n    }) {\n        transaction {\n            id,\n            createdAt,\n            amount {\n                value,\n                currencyIsoCode\n            },\n            orderId,\n            customer {\n                id\n            },\n            status\n        }\n    }\n}\n\nmutation vaultPayment($vault_payment_input: VaultPaymentMethodInput!) {\n    vaultPaymentMethod(input: $vault_payment_input) {\n        paymentMethod {\n            id\n            usage\n            customer {\n                id\n            }\n            details {\n                __typename\n                ... on CreditCardDetails {\n                    cardholderName\n                }\n                ... on PayPalAccountDetails {\n                    email\n                }\n                ... on VenmoAccountDetails {\n                    username\n                }\n                ... on UsBankAccountDetails {\n                    accountholderName\n                }\n            }\n        }\n        verification {\n            status\n        }\n    }\n}\n\nmutation deleteVaultedPayment($input: DeletePaymentMethodFromVaultInput!) {\n    deletePaymentMethodFromVault(input:$input) {\n        clientMutationId\n    }\n}\n\nmutation verifyPaymentMethod($input: VerifyPaymentMethodInput!) {\n    verifyPaymentMethod(input:$input) {\n        verification {\n            id,\n            amount{value},\n            status,\n            riskData{decision},\n            processorResponse{message,cvvResponse,avsPostalCodeResponse,avsStreetAddressResponse}\n        }\n    }\n}\n\nquery searchTransaction($input: TransactionSearchInput!) {\n    search {\n        transactions(input:$input) {\n            edges {\n                node {\n                    id,amount{value,currencyIsoCode},customer {email,id},\n                    paymentMethod{\n                        details {\n                            __typename\n                            ... on CreditCardDetails {\n                                cardholderName\n                            }\n                            ... on PayPalAccountDetails {\n                                email\n                            }\n                            ... on VenmoAccountDetails {\n                                username\n                            }\n                            ... on UsBankAccountDetails {\n                                accountholderName\n                            }\n                        }\n                    },orderId,status,createdAt\n                }\n            }\n        }\n    }\n}\n\nquery getTransaction($transactionID: String!) {\n    search {\n        transactions(input:{id: {is: $transactionID}}) {\n            edges {\n                node {\n                    id,amount{value,currencyIsoCode},customer {email,id},\n                    paymentMethod{\n                        details {\n                            __typename\n                            ... on CreditCardDetails {\n                                cardholderName\n                            }\n                            ... on PayPalAccountDetails {\n                                email\n                            }\n                            ... on VenmoAccountDetails {\n                                username\n                            }\n                            ... on UsBankAccountDetails {\n                                accountholderName\n                            }\n                        }\n                    },orderId,status,createdAt\n                }\n            }\n        }\n    }\n}\n" ;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    #[doc = "A monetary amount, either a whole number or a number with exactly two or three decimal places.\n"]
    type Amount = super::Amount;
    #[doc = "An [ISO 3166-1 alpha-3](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-3)\ncountry code. Braintree only accepts [specific alpha-3 values](https://developers.braintreepayments.com/reference/general/countries#list-of-countries).\n"]
    type CountryCodeAlpha3 = super::CountryCodeAlpha3;
    #[doc = "An [ISO 4217 alpha](https://en.wikipedia.org/wiki/ISO_4217) currency code.\nBraintree only accepts [specific alpha\nvalues](https://developers.braintreepayments.com/reference/general/currencies).\n"]
    type CurrencyCodeAlpha = super::CurrencyCodeAlpha;
    #[doc = "A string representing a custom field value. Contains letters, numbers, and underscores.\n"]
    type CustomFieldName = super::CustomFieldName;
    #[doc = "An [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601#Times) timestamp with microsecond precision.\n"]
    type Timestamp = super::Timestamp;
    #[derive(PartialEq, Serialize)]
    pub struct AddressInput {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub company: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "countryCode")]
        pub country_code: Option<CountryCodeAlpha3>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "countryCodeAlpha2")]
        pub country_code_alpha2: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "countryCodeAlpha3")]
        pub country_code_alpha3: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "countryCodeNumeric")]
        pub country_code_numeric: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "countryName")]
        pub country_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "extendedAddress")]
        pub extended_address: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "firstName")]
        pub first_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "lastName")]
        pub last_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub locality: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "postalCode")]
        pub postal_code: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub region: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "streetAddress")]
        pub street_address: Option<String>,
    }
    impl AddressInput {
        pub fn new() -> Self {
            Self {
                company: None,
                country_code: None,
                country_code_alpha2: None,
                country_code_alpha3: None,
                country_code_numeric: None,
                country_name: None,
                extended_address: None,
                first_name: None,
                last_name: None,
                locality: None,
                postal_code: None,
                region: None,
                street_address: None,
            }
        }
    }
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
    pub struct RiskDataInput {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "customerBrowser")]
        pub customer_browser: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "customerIp")]
        pub customer_ip: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "deviceData")]
        pub device_data: Option<String>,
    }
    impl RiskDataInput {
        pub fn new() -> Self {
            Self {
                customer_browser: None,
                customer_ip: None,
                device_data: None,
            }
        }
    }
    #[derive(PartialEq, Serialize)]
    pub struct TransactionDescriptorInput {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub phone: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub url: Option<String>,
    }
    impl TransactionDescriptorInput {
        pub fn new() -> Self {
            Self {
                name: None,
                phone: None,
                url: None,
            }
        }
    }
    #[derive(PartialEq, Serialize)]
    pub struct TransactionInput {
        pub amount: Amount,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub channel: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "customFields")]
        pub custom_fields: Option<Vec<CustomFieldInput>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "customerId")]
        pub customer_id: Option<ID>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub descriptor: Option<TransactionDescriptorInput>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "discountAmount")]
        pub discount_amount: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "lineItems")]
        pub line_items: Option<Vec<TransactionLineItemInput>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "merchantAccountId")]
        pub merchant_account_id: Option<ID>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "orderId")]
        pub order_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "purchaseOrderNumber")]
        pub purchase_order_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recurring: Option<RecurringType>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "riskData")]
        pub risk_data: Option<RiskDataInput>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub shipping: Option<TransactionShippingInput>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tax: Option<TransactionTaxInput>,
    }
    impl TransactionInput {
        pub fn new(amount: Amount) -> Self {
            Self {
                amount: amount,
                channel: None,
                custom_fields: None,
                customer_id: None,
                descriptor: None,
                discount_amount: None,
                line_items: None,
                merchant_account_id: None,
                order_id: None,
                purchase_order_number: None,
                recurring: None,
                risk_data: None,
                shipping: None,
                tax: None,
            }
        }
    }
    #[derive(PartialEq, Serialize)]
    pub struct TransactionLineItemInput {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "commodityCode")]
        pub commodity_code: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "discountAmount")]
        pub discount_amount: Option<String>,
        pub kind: TransactionLineItemType,
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "productCode")]
        pub product_code: Option<String>,
        pub quantity: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "taxAmount")]
        pub tax_amount: Option<String>,
        #[serde(rename = "totalAmount")]
        pub total_amount: String,
        #[serde(rename = "unitAmount")]
        pub unit_amount: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "unitOfMeasure")]
        pub unit_of_measure: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "unitTaxAmount")]
        pub unit_tax_amount: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub url: Option<String>,
    }
    impl TransactionLineItemInput {
        pub fn new(
            kind: TransactionLineItemType,
            name: String,
            quantity: String,
            total_amount: String,
            unit_amount: String,
        ) -> Self {
            Self {
                commodity_code: None,
                description: None,
                discount_amount: None,
                kind: kind,
                name: name,
                product_code: None,
                quantity: quantity,
                tax_amount: None,
                total_amount: total_amount,
                unit_amount: unit_amount,
                unit_of_measure: None,
                unit_tax_amount: None,
                url: None,
            }
        }
    }
    #[derive(PartialEq, Serialize)]
    pub struct TransactionShippingInput {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "shippingAddress")]
        pub shipping_address: Option<AddressInput>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "shippingAmount")]
        pub shipping_amount: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "shipsFromPostalCode")]
        pub ships_from_postal_code: Option<String>,
    }
    impl TransactionShippingInput {
        pub fn new() -> Self {
            Self {
                shipping_address: None,
                shipping_amount: None,
                ships_from_postal_code: None,
            }
        }
    }
    #[derive(PartialEq, Serialize)]
    pub struct TransactionTaxInput {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "taxAmount")]
        pub tax_amount: Option<Amount>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "taxExempt")]
        pub tax_exempt: Option<Boolean>,
    }
    impl TransactionTaxInput {
        pub fn new() -> Self {
            Self {
                tax_amount: None,
                tax_exempt: None,
            }
        }
    }
    #[derive(Debug, Eq, PartialEq)]
    #[allow(non_camel_case_types)]
    pub enum RecurringType {
        FIRST,
        SUBSEQUENT,
        UNSCHEDULED,
        Other(String),
    }
    impl ::serde::Serialize for RecurringType {
        fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
            ser.serialize_str(match *self {
                RecurringType::FIRST => "FIRST",
                RecurringType::SUBSEQUENT => "SUBSEQUENT",
                RecurringType::UNSCHEDULED => "UNSCHEDULED",
                RecurringType::Other(ref s) => &s,
            })
        }
    }
    impl<'de> ::serde::Deserialize<'de> for RecurringType {
        fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let s = <String>::deserialize(deserializer)?;
            match s.as_str() {
                "FIRST" => Ok(RecurringType::FIRST),
                "SUBSEQUENT" => Ok(RecurringType::SUBSEQUENT),
                "UNSCHEDULED" => Ok(RecurringType::UNSCHEDULED),
                _ => Ok(RecurringType::Other(s)),
            }
        }
    }
    #[derive(Debug, Eq, PartialEq)]
    #[allow(non_camel_case_types)]
    pub enum TransactionLineItemType {
        CREDIT,
        DEBIT,
        Other(String),
    }
    impl ::serde::Serialize for TransactionLineItemType {
        fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
            ser.serialize_str(match *self {
                TransactionLineItemType::CREDIT => "CREDIT",
                TransactionLineItemType::DEBIT => "DEBIT",
                TransactionLineItemType::Other(ref s) => &s,
            })
        }
    }
    impl<'de> ::serde::Deserialize<'de> for TransactionLineItemType {
        fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let s = <String>::deserialize(deserializer)?;
            match s.as_str() {
                "CREDIT" => Ok(TransactionLineItemType::CREDIT),
                "DEBIT" => Ok(TransactionLineItemType::DEBIT),
                _ => Ok(TransactionLineItemType::Other(s)),
            }
        }
    }
    #[derive(Debug, Eq, PartialEq)]
    #[allow(non_camel_case_types)]
    pub enum TransactionStatus {
        #[doc = "The transaction spent too much time in the `AUTHORIZED` status and was marked\nas expired. Expiration [time frames](https://developers.braintreepayments.com/reference/general/statuses#authorization-expired)\ndiffer by card type, transaction type, and, in some cases, merchant category.\n"]
        AUTHORIZATION_EXPIRED,
        #[doc = "The processor authorized the transaction, putting your customer's funds on\nhold. Your customer may see a pending charge on his or her account. However,\nbefore the customer is actually charged and before you receive the funds, you\nmust use the `captureTransaction` mutation. If you do not want to capture the\ntransaction, you should use the `reverseTransaction` mutation to avoid a\nmisuse of authorization fee.\n"]
        AUTHORIZED,
        #[doc = "If a transaction remains in a status of `AUTHORIZING`, [contact Braintree\nSupport for assistance](https://help.braintreepayments.com).\n"]
        AUTHORIZING,
        #[doc = "An error occurred when sending the transaction to the downstream processor.\nSee the transaction's `statusHistory` for the exact error.\n"]
        FAILED,
        #[doc = "The transaction was [rejected](https://articles.braintreepayments.com/control-panel/transactions/gateway-rejections)\nbased on one or more settings or rules in your Braintree gateway. See the\ntransaction's `statusHistory` to determine which resulted in the decline.\n"]
        GATEWAY_REJECTED,
        #[doc = "The processor declined the transaction while attempting to authorize it. See\nthe transaction's `statusHistory` to determine what reason the processor gave\nfor the decline.\n"]
        PROCESSOR_DECLINED,
        #[doc = "The transaction has been settled, meaning your customer has been charged and\nthe process of disbursing the funds to your bank account will begin.\n"]
        SETTLED,
        SETTLEMENT_CONFIRMED,
        #[doc = "The processor declined the transaction while attempting to capture it. See the\ntransaction's `statusHistory` to detemine why it was not settled. This status\nis rare, and only certain types of transactions can be affected.\n"]
        SETTLEMENT_DECLINED,
        #[doc = "The transaction has not yet fully settled. This status is rare, and will\ngenerally resolve to a status of `SETTLED`. Only certain types of transactions\ncan be affected.\n"]
        SETTLEMENT_PENDING,
        #[doc = "The transaction is in the process of being settled. This is a transitory state, and will resolve to a status of `SETTLED`.\n"]
        SETTLING,
        #[doc = "The transaction has been successfully captured, and will be included in the\nnext settlement batch, at which time it will become settled.\n"]
        SUBMITTED_FOR_SETTLEMENT,
        #[doc = "The transaction was voided, meaning it is no longer authorized, your\ncustomer's funds are no longer on hold, and you cannot use the\n`captureTransaction` mutation on this transaction.\n"]
        VOIDED,
        Other(String),
    }
    impl ::serde::Serialize for TransactionStatus {
        fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
            ser.serialize_str(match *self {
                TransactionStatus::AUTHORIZATION_EXPIRED => "AUTHORIZATION_EXPIRED",
                TransactionStatus::AUTHORIZED => "AUTHORIZED",
                TransactionStatus::AUTHORIZING => "AUTHORIZING",
                TransactionStatus::FAILED => "FAILED",
                TransactionStatus::GATEWAY_REJECTED => "GATEWAY_REJECTED",
                TransactionStatus::PROCESSOR_DECLINED => "PROCESSOR_DECLINED",
                TransactionStatus::SETTLED => "SETTLED",
                TransactionStatus::SETTLEMENT_CONFIRMED => "SETTLEMENT_CONFIRMED",
                TransactionStatus::SETTLEMENT_DECLINED => "SETTLEMENT_DECLINED",
                TransactionStatus::SETTLEMENT_PENDING => "SETTLEMENT_PENDING",
                TransactionStatus::SETTLING => "SETTLING",
                TransactionStatus::SUBMITTED_FOR_SETTLEMENT => "SUBMITTED_FOR_SETTLEMENT",
                TransactionStatus::VOIDED => "VOIDED",
                TransactionStatus::Other(ref s) => &s,
            })
        }
    }
    impl<'de> ::serde::Deserialize<'de> for TransactionStatus {
        fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let s = <String>::deserialize(deserializer)?;
            match s.as_str() {
                "AUTHORIZATION_EXPIRED" => Ok(TransactionStatus::AUTHORIZATION_EXPIRED),
                "AUTHORIZED" => Ok(TransactionStatus::AUTHORIZED),
                "AUTHORIZING" => Ok(TransactionStatus::AUTHORIZING),
                "FAILED" => Ok(TransactionStatus::FAILED),
                "GATEWAY_REJECTED" => Ok(TransactionStatus::GATEWAY_REJECTED),
                "PROCESSOR_DECLINED" => Ok(TransactionStatus::PROCESSOR_DECLINED),
                "SETTLED" => Ok(TransactionStatus::SETTLED),
                "SETTLEMENT_CONFIRMED" => Ok(TransactionStatus::SETTLEMENT_CONFIRMED),
                "SETTLEMENT_DECLINED" => Ok(TransactionStatus::SETTLEMENT_DECLINED),
                "SETTLEMENT_PENDING" => Ok(TransactionStatus::SETTLEMENT_PENDING),
                "SETTLING" => Ok(TransactionStatus::SETTLING),
                "SUBMITTED_FOR_SETTLEMENT" => Ok(TransactionStatus::SUBMITTED_FOR_SETTLEMENT),
                "VOIDED" => Ok(TransactionStatus::VOIDED),
                _ => Ok(TransactionStatus::Other(s)),
            }
        }
    }
    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    #[doc = "A monetary amount with currency.\n"]
    pub struct ChargePaymentMethodChargePaymentMethodTransactionAmount {
        #[doc = "The amount of money, either a whole number or a number with exactly 2 or 3 decimal places.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub value: Option<Amount>,
        #[doc = "The ISO code for the money's currency.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "currencyIsoCode")]
        pub currency_iso_code: Option<CurrencyCodeAlpha>,
    }
    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    #[doc = "Information about a customer and their associated payment methods and transactions.\n"]
    pub struct ChargePaymentMethodChargePaymentMethodTransactionCustomer {
        #[doc = "Unique identifier.\n"]
        pub id: ID,
    }
    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    #[doc = "A charge on a payment method.\n"]
    pub struct ChargePaymentMethodChargePaymentMethodTransaction {
        #[doc = "Unique identifier.\n"]
        pub id: ID,
        #[doc = "Time at which the transaction was created.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "createdAt")]
        pub created_at: Option<Timestamp>,
        #[doc = "The amount charged in this transaction. For transactions that are partially\ncaptured, this amount will be the cummulative amount captured on this transaction.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub amount: Option<ChargePaymentMethodChargePaymentMethodTransactionAmount>,
        #[doc = "The order ID for this transaction. For PayPal transactions, the PayPal Invoice ID.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "orderId")]
        pub order_id: Option<String>,
        #[doc = "Customer associated with the transaction, if applicable.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub customer: Option<ChargePaymentMethodChargePaymentMethodTransactionCustomer>,
        #[doc = "The current status of this transaction.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub status: Option<TransactionStatus>,
    }
    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    #[doc = "Top-level output field from creating a transaction.\n"]
    pub struct ChargePaymentMethodChargePaymentMethod {
        #[doc = "The transaction representing the charge on the payment method.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub transaction: Option<ChargePaymentMethodChargePaymentMethodTransaction>,
    }
    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    pub struct ResponseData {
        #[doc = "Charge any payment method and return a payload that includes details of the resulting transaction.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "chargePaymentMethod")]
        pub charge_payment_method: Option<ChargePaymentMethodChargePaymentMethod>,
    }
    #[allow(dead_code)]
    #[derive(Serialize)]
    pub struct ChargePaymentMethod {
        #[serde(rename = "paymentMethodId")]
        pub payment_method_id: ID,
        pub transaction: TransactionInput,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "clientMutationId")]
        pub client_mutation_id: Option<String>,
    }
    impl crate::GraphQLQueryCLI for ChargePaymentMethod {
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
pub mod vault_payment {
    #![allow(dead_code)]
    pub const OPERATION_NAME: &'static str = "vaultPayment";
    pub const QUERY : & 'static str = "mutation chargePaymentMethod($paymentMethodId: ID!, $transaction:TransactionInput!, $clientMutationId: String) {\n    chargePaymentMethod(input: {\n        paymentMethodId:$paymentMethodId, transaction:$transaction, clientMutationId:$clientMutationId\n    }) {\n        transaction {\n            id,\n            createdAt,\n            amount {\n                value,\n                currencyIsoCode\n            },\n            orderId,\n            customer {\n                id\n            },\n            status\n        }\n    }\n}\n\nmutation vaultPayment($vault_payment_input: VaultPaymentMethodInput!) {\n    vaultPaymentMethod(input: $vault_payment_input) {\n        paymentMethod {\n            id\n            usage\n            customer {\n                id\n            }\n            details {\n                __typename\n                ... on CreditCardDetails {\n                    cardholderName\n                }\n                ... on PayPalAccountDetails {\n                    email\n                }\n                ... on VenmoAccountDetails {\n                    username\n                }\n                ... on UsBankAccountDetails {\n                    accountholderName\n                }\n            }\n        }\n        verification {\n            status\n        }\n    }\n}\n\nmutation deleteVaultedPayment($input: DeletePaymentMethodFromVaultInput!) {\n    deletePaymentMethodFromVault(input:$input) {\n        clientMutationId\n    }\n}\n\nmutation verifyPaymentMethod($input: VerifyPaymentMethodInput!) {\n    verifyPaymentMethod(input:$input) {\n        verification {\n            id,\n            amount{value},\n            status,\n            riskData{decision},\n            processorResponse{message,cvvResponse,avsPostalCodeResponse,avsStreetAddressResponse}\n        }\n    }\n}\n\nquery searchTransaction($input: TransactionSearchInput!) {\n    search {\n        transactions(input:$input) {\n            edges {\n                node {\n                    id,amount{value,currencyIsoCode},customer {email,id},\n                    paymentMethod{\n                        details {\n                            __typename\n                            ... on CreditCardDetails {\n                                cardholderName\n                            }\n                            ... on PayPalAccountDetails {\n                                email\n                            }\n                            ... on VenmoAccountDetails {\n                                username\n                            }\n                            ... on UsBankAccountDetails {\n                                accountholderName\n                            }\n                        }\n                    },orderId,status,createdAt\n                }\n            }\n        }\n    }\n}\n\nquery getTransaction($transactionID: String!) {\n    search {\n        transactions(input:{id: {is: $transactionID}}) {\n            edges {\n                node {\n                    id,amount{value,currencyIsoCode},customer {email,id},\n                    paymentMethod{\n                        details {\n                            __typename\n                            ... on CreditCardDetails {\n                                cardholderName\n                            }\n                            ... on PayPalAccountDetails {\n                                email\n                            }\n                            ... on VenmoAccountDetails {\n                                username\n                            }\n                            ... on UsBankAccountDetails {\n                                accountholderName\n                            }\n                        }\n                    },orderId,status,createdAt\n                }\n            }\n        }\n    }\n}\n" ;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    #[doc = "A monetary amount, either a whole number or a number with exactly two or three decimal places.\n"]
    type Amount = super::Amount;
    #[doc = "An [ISO 3166-1 alpha-3](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-3)\ncountry code. Braintree only accepts [specific alpha-3 values](https://developers.braintreepayments.com/reference/general/countries#list-of-countries).\n"]
    type CountryCodeAlpha3 = super::CountryCodeAlpha3;
    #[doc = "An [ISO 4217 alpha](https://en.wikipedia.org/wiki/ISO_4217) currency code.\nBraintree only accepts [specific alpha\nvalues](https://developers.braintreepayments.com/reference/general/currencies).\n"]
    type CurrencyCodeAlpha = super::CurrencyCodeAlpha;
    #[doc = "A string representing a custom field value. Contains letters, numbers, and underscores.\n"]
    type CustomFieldName = super::CustomFieldName;
    #[doc = "An [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601#Times) timestamp with microsecond precision.\n"]
    type Timestamp = super::Timestamp;
    #[derive(PartialEq, Serialize)]
    pub struct AddressInput {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub company: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "countryCode")]
        pub country_code: Option<CountryCodeAlpha3>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "countryCodeAlpha2")]
        pub country_code_alpha2: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "countryCodeAlpha3")]
        pub country_code_alpha3: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "countryCodeNumeric")]
        pub country_code_numeric: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "countryName")]
        pub country_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "extendedAddress")]
        pub extended_address: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "firstName")]
        pub first_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "lastName")]
        pub last_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub locality: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "postalCode")]
        pub postal_code: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub region: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "streetAddress")]
        pub street_address: Option<String>,
    }
    impl AddressInput {
        pub fn new() -> Self {
            Self {
                company: None,
                country_code: None,
                country_code_alpha2: None,
                country_code_alpha3: None,
                country_code_numeric: None,
                country_name: None,
                extended_address: None,
                first_name: None,
                last_name: None,
                locality: None,
                postal_code: None,
                region: None,
                street_address: None,
            }
        }
    }
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
    pub struct RiskDataInput {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "customerBrowser")]
        pub customer_browser: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "customerIp")]
        pub customer_ip: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "deviceData")]
        pub device_data: Option<String>,
    }
    impl RiskDataInput {
        pub fn new() -> Self {
            Self {
                customer_browser: None,
                customer_ip: None,
                device_data: None,
            }
        }
    }
    #[derive(PartialEq, Serialize)]
    pub struct TransactionDescriptorInput {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub phone: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub url: Option<String>,
    }
    impl TransactionDescriptorInput {
        pub fn new() -> Self {
            Self {
                name: None,
                phone: None,
                url: None,
            }
        }
    }
    #[derive(PartialEq, Serialize)]
    pub struct TransactionInput {
        pub amount: Amount,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub channel: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "customFields")]
        pub custom_fields: Option<Vec<CustomFieldInput>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "customerId")]
        pub customer_id: Option<ID>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub descriptor: Option<TransactionDescriptorInput>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "discountAmount")]
        pub discount_amount: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "lineItems")]
        pub line_items: Option<Vec<TransactionLineItemInput>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "merchantAccountId")]
        pub merchant_account_id: Option<ID>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "orderId")]
        pub order_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "purchaseOrderNumber")]
        pub purchase_order_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recurring: Option<RecurringType>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "riskData")]
        pub risk_data: Option<RiskDataInput>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub shipping: Option<TransactionShippingInput>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tax: Option<TransactionTaxInput>,
    }
    impl TransactionInput {
        pub fn new(amount: Amount) -> Self {
            Self {
                amount: amount,
                channel: None,
                custom_fields: None,
                customer_id: None,
                descriptor: None,
                discount_amount: None,
                line_items: None,
                merchant_account_id: None,
                order_id: None,
                purchase_order_number: None,
                recurring: None,
                risk_data: None,
                shipping: None,
                tax: None,
            }
        }
    }
    #[derive(PartialEq, Serialize)]
    pub struct TransactionLineItemInput {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "commodityCode")]
        pub commodity_code: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "discountAmount")]
        pub discount_amount: Option<String>,
        pub kind: TransactionLineItemType,
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "productCode")]
        pub product_code: Option<String>,
        pub quantity: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "taxAmount")]
        pub tax_amount: Option<String>,
        #[serde(rename = "totalAmount")]
        pub total_amount: String,
        #[serde(rename = "unitAmount")]
        pub unit_amount: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "unitOfMeasure")]
        pub unit_of_measure: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "unitTaxAmount")]
        pub unit_tax_amount: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub url: Option<String>,
    }
    impl TransactionLineItemInput {
        pub fn new(
            kind: TransactionLineItemType,
            name: String,
            quantity: String,
            total_amount: String,
            unit_amount: String,
        ) -> Self {
            Self {
                commodity_code: None,
                description: None,
                discount_amount: None,
                kind: kind,
                name: name,
                product_code: None,
                quantity: quantity,
                tax_amount: None,
                total_amount: total_amount,
                unit_amount: unit_amount,
                unit_of_measure: None,
                unit_tax_amount: None,
                url: None,
            }
        }
    }
    #[derive(PartialEq, Serialize)]
    pub struct TransactionShippingInput {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "shippingAddress")]
        pub shipping_address: Option<AddressInput>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "shippingAmount")]
        pub shipping_amount: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "shipsFromPostalCode")]
        pub ships_from_postal_code: Option<String>,
    }
    impl TransactionShippingInput {
        pub fn new() -> Self {
            Self {
                shipping_address: None,
                shipping_amount: None,
                ships_from_postal_code: None,
            }
        }
    }
    #[derive(PartialEq, Serialize)]
    pub struct TransactionTaxInput {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "taxAmount")]
        pub tax_amount: Option<Amount>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "taxExempt")]
        pub tax_exempt: Option<Boolean>,
    }
    impl TransactionTaxInput {
        pub fn new() -> Self {
            Self {
                tax_amount: None,
                tax_exempt: None,
            }
        }
    }
    #[derive(PartialEq, Serialize)]
    pub struct VaultPaymentMethodInput {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "clientMutationId")]
        pub client_mutation_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "customerId")]
        pub customer_id: Option<ID>,
        #[serde(rename = "paymentMethodId")]
        pub payment_method_id: ID,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "verificationMerchantAccountId")]
        pub verification_merchant_account_id: Option<ID>,
    }
    impl VaultPaymentMethodInput {
        pub fn new(payment_method_id: ID) -> Self {
            Self {
                client_mutation_id: None,
                customer_id: None,
                payment_method_id: payment_method_id,
                verification_merchant_account_id: None,
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
    #[derive(Debug, Eq, PartialEq)]
    #[allow(non_camel_case_types)]
    pub enum RecurringType {
        FIRST,
        SUBSEQUENT,
        UNSCHEDULED,
        Other(String),
    }
    impl ::serde::Serialize for RecurringType {
        fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
            ser.serialize_str(match *self {
                RecurringType::FIRST => "FIRST",
                RecurringType::SUBSEQUENT => "SUBSEQUENT",
                RecurringType::UNSCHEDULED => "UNSCHEDULED",
                RecurringType::Other(ref s) => &s,
            })
        }
    }
    impl<'de> ::serde::Deserialize<'de> for RecurringType {
        fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let s = <String>::deserialize(deserializer)?;
            match s.as_str() {
                "FIRST" => Ok(RecurringType::FIRST),
                "SUBSEQUENT" => Ok(RecurringType::SUBSEQUENT),
                "UNSCHEDULED" => Ok(RecurringType::UNSCHEDULED),
                _ => Ok(RecurringType::Other(s)),
            }
        }
    }
    #[derive(Debug, Eq, PartialEq)]
    #[allow(non_camel_case_types)]
    pub enum TransactionLineItemType {
        CREDIT,
        DEBIT,
        Other(String),
    }
    impl ::serde::Serialize for TransactionLineItemType {
        fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
            ser.serialize_str(match *self {
                TransactionLineItemType::CREDIT => "CREDIT",
                TransactionLineItemType::DEBIT => "DEBIT",
                TransactionLineItemType::Other(ref s) => &s,
            })
        }
    }
    impl<'de> ::serde::Deserialize<'de> for TransactionLineItemType {
        fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let s = <String>::deserialize(deserializer)?;
            match s.as_str() {
                "CREDIT" => Ok(TransactionLineItemType::CREDIT),
                "DEBIT" => Ok(TransactionLineItemType::DEBIT),
                _ => Ok(TransactionLineItemType::Other(s)),
            }
        }
    }
    #[derive(Debug, Eq, PartialEq)]
    #[allow(non_camel_case_types)]
    pub enum TransactionStatus {
        #[doc = "The transaction spent too much time in the `AUTHORIZED` status and was marked\nas expired. Expiration [time frames](https://developers.braintreepayments.com/reference/general/statuses#authorization-expired)\ndiffer by card type, transaction type, and, in some cases, merchant category.\n"]
        AUTHORIZATION_EXPIRED,
        #[doc = "The processor authorized the transaction, putting your customer's funds on\nhold. Your customer may see a pending charge on his or her account. However,\nbefore the customer is actually charged and before you receive the funds, you\nmust use the `captureTransaction` mutation. If you do not want to capture the\ntransaction, you should use the `reverseTransaction` mutation to avoid a\nmisuse of authorization fee.\n"]
        AUTHORIZED,
        #[doc = "If a transaction remains in a status of `AUTHORIZING`, [contact Braintree\nSupport for assistance](https://help.braintreepayments.com).\n"]
        AUTHORIZING,
        #[doc = "An error occurred when sending the transaction to the downstream processor.\nSee the transaction's `statusHistory` for the exact error.\n"]
        FAILED,
        #[doc = "The transaction was [rejected](https://articles.braintreepayments.com/control-panel/transactions/gateway-rejections)\nbased on one or more settings or rules in your Braintree gateway. See the\ntransaction's `statusHistory` to determine which resulted in the decline.\n"]
        GATEWAY_REJECTED,
        #[doc = "The processor declined the transaction while attempting to authorize it. See\nthe transaction's `statusHistory` to determine what reason the processor gave\nfor the decline.\n"]
        PROCESSOR_DECLINED,
        #[doc = "The transaction has been settled, meaning your customer has been charged and\nthe process of disbursing the funds to your bank account will begin.\n"]
        SETTLED,
        SETTLEMENT_CONFIRMED,
        #[doc = "The processor declined the transaction while attempting to capture it. See the\ntransaction's `statusHistory` to detemine why it was not settled. This status\nis rare, and only certain types of transactions can be affected.\n"]
        SETTLEMENT_DECLINED,
        #[doc = "The transaction has not yet fully settled. This status is rare, and will\ngenerally resolve to a status of `SETTLED`. Only certain types of transactions\ncan be affected.\n"]
        SETTLEMENT_PENDING,
        #[doc = "The transaction is in the process of being settled. This is a transitory state, and will resolve to a status of `SETTLED`.\n"]
        SETTLING,
        #[doc = "The transaction has been successfully captured, and will be included in the\nnext settlement batch, at which time it will become settled.\n"]
        SUBMITTED_FOR_SETTLEMENT,
        #[doc = "The transaction was voided, meaning it is no longer authorized, your\ncustomer's funds are no longer on hold, and you cannot use the\n`captureTransaction` mutation on this transaction.\n"]
        VOIDED,
        Other(String),
    }
    impl ::serde::Serialize for TransactionStatus {
        fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
            ser.serialize_str(match *self {
                TransactionStatus::AUTHORIZATION_EXPIRED => "AUTHORIZATION_EXPIRED",
                TransactionStatus::AUTHORIZED => "AUTHORIZED",
                TransactionStatus::AUTHORIZING => "AUTHORIZING",
                TransactionStatus::FAILED => "FAILED",
                TransactionStatus::GATEWAY_REJECTED => "GATEWAY_REJECTED",
                TransactionStatus::PROCESSOR_DECLINED => "PROCESSOR_DECLINED",
                TransactionStatus::SETTLED => "SETTLED",
                TransactionStatus::SETTLEMENT_CONFIRMED => "SETTLEMENT_CONFIRMED",
                TransactionStatus::SETTLEMENT_DECLINED => "SETTLEMENT_DECLINED",
                TransactionStatus::SETTLEMENT_PENDING => "SETTLEMENT_PENDING",
                TransactionStatus::SETTLING => "SETTLING",
                TransactionStatus::SUBMITTED_FOR_SETTLEMENT => "SUBMITTED_FOR_SETTLEMENT",
                TransactionStatus::VOIDED => "VOIDED",
                TransactionStatus::Other(ref s) => &s,
            })
        }
    }
    impl<'de> ::serde::Deserialize<'de> for TransactionStatus {
        fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let s = <String>::deserialize(deserializer)?;
            match s.as_str() {
                "AUTHORIZATION_EXPIRED" => Ok(TransactionStatus::AUTHORIZATION_EXPIRED),
                "AUTHORIZED" => Ok(TransactionStatus::AUTHORIZED),
                "AUTHORIZING" => Ok(TransactionStatus::AUTHORIZING),
                "FAILED" => Ok(TransactionStatus::FAILED),
                "GATEWAY_REJECTED" => Ok(TransactionStatus::GATEWAY_REJECTED),
                "PROCESSOR_DECLINED" => Ok(TransactionStatus::PROCESSOR_DECLINED),
                "SETTLED" => Ok(TransactionStatus::SETTLED),
                "SETTLEMENT_CONFIRMED" => Ok(TransactionStatus::SETTLEMENT_CONFIRMED),
                "SETTLEMENT_DECLINED" => Ok(TransactionStatus::SETTLEMENT_DECLINED),
                "SETTLEMENT_PENDING" => Ok(TransactionStatus::SETTLEMENT_PENDING),
                "SETTLING" => Ok(TransactionStatus::SETTLING),
                "SUBMITTED_FOR_SETTLEMENT" => Ok(TransactionStatus::SUBMITTED_FOR_SETTLEMENT),
                "VOIDED" => Ok(TransactionStatus::VOIDED),
                _ => Ok(TransactionStatus::Other(s)),
            }
        }
    }
    #[derive(Debug, Eq, PartialEq)]
    #[allow(non_camel_case_types)]
    pub enum VerificationStatus {
        #[doc = "Indicates the verification was unsuccessful because of an issue communicating with the processor.\n"]
        FAILED,
        #[doc = "Indicates that the verification was unsuccessful because the payment method\nfailed one or more fraud checks. In this case, the `gatewayRejectionReason`\nwill indicate which fraud check failed.\n"]
        GATEWAY_REJECTED,
        #[doc = "Indicates that the verification is pending.\n"]
        PENDING,
        #[doc = "Indicates that the verification was unsuccessful based on the response from the processor.\n"]
        PROCESSOR_DECLINED,
        #[doc = "Indicates that the verification was successful.\n"]
        VERIFIED,
        #[doc = "Indicates that the verification is in the process of verifying.\n"]
        VERIFYING,
        Other(String),
    }
    impl ::serde::Serialize for VerificationStatus {
        fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
            ser.serialize_str(match *self {
                VerificationStatus::FAILED => "FAILED",
                VerificationStatus::GATEWAY_REJECTED => "GATEWAY_REJECTED",
                VerificationStatus::PENDING => "PENDING",
                VerificationStatus::PROCESSOR_DECLINED => "PROCESSOR_DECLINED",
                VerificationStatus::VERIFIED => "VERIFIED",
                VerificationStatus::VERIFYING => "VERIFYING",
                VerificationStatus::Other(ref s) => &s,
            })
        }
    }
    impl<'de> ::serde::Deserialize<'de> for VerificationStatus {
        fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let s = <String>::deserialize(deserializer)?;
            match s.as_str() {
                "FAILED" => Ok(VerificationStatus::FAILED),
                "GATEWAY_REJECTED" => Ok(VerificationStatus::GATEWAY_REJECTED),
                "PENDING" => Ok(VerificationStatus::PENDING),
                "PROCESSOR_DECLINED" => Ok(VerificationStatus::PROCESSOR_DECLINED),
                "VERIFIED" => Ok(VerificationStatus::VERIFIED),
                "VERIFYING" => Ok(VerificationStatus::VERIFYING),
                _ => Ok(VerificationStatus::Other(s)),
            }
        }
    }
    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    #[doc = "Information about a customer and their associated payment methods and transactions.\n"]
    pub struct VaultPaymentVaultPaymentMethodPaymentMethodCustomer {
        #[doc = "Unique identifier.\n"]
        pub id: ID,
    }
    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    #[doc = "Details about a credit card.\n"]
    pub struct VaultPaymentVaultPaymentMethodPaymentMethodDetailsOnCreditCardDetails {
        #[doc = "The cardholder's name.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "cardholderName")]
        pub cardholder_name: Option<String>,
    }
    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    #[doc = "Details about a PayPal account.\n"]
    pub struct VaultPaymentVaultPaymentMethodPaymentMethodDetailsOnPayPalAccountDetails {
        #[doc = "The email address associated with the PayPal account.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub email: Option<String>,
    }
    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    #[doc = "Details about a U.S. bank account.\n"]
    pub struct VaultPaymentVaultPaymentMethodPaymentMethodDetailsOnUsBankAccountDetails {
        #[doc = "The name of the accountholder. This is either the business name for a\nbusiness account, or the owner's full name for an individual account.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "accountholderName")]
        pub accountholder_name: Option<String>,
    }
    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    #[doc = "Details about a Venmo Account.\n"]
    pub struct VaultPaymentVaultPaymentMethodPaymentMethodDetailsOnVenmoAccountDetails {
        #[doc = "The Venmo username, as chosen by the user.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub username: Option<String>,
    }
    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    #[serde(tag = "__typename")]
    pub enum VaultPaymentVaultPaymentMethodPaymentMethodDetails {
        CreditCardDetails(VaultPaymentVaultPaymentMethodPaymentMethodDetailsOnCreditCardDetails),
        PayPalAccountDetails(
            VaultPaymentVaultPaymentMethodPaymentMethodDetailsOnPayPalAccountDetails,
        ),
        UsBankAccountDetails(
            VaultPaymentVaultPaymentMethodPaymentMethodDetailsOnUsBankAccountDetails,
        ),
        VenmoAccountDetails(
            VaultPaymentVaultPaymentMethodPaymentMethodDetailsOnVenmoAccountDetails,
        ),
        CustomActionsPaymentMethodDetails,
        SamsungPayCardDetails,
    }
    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    #[doc = "Top-level field representing a payment method.\n"]
    pub struct VaultPaymentVaultPaymentMethodPaymentMethod {
        #[doc = "Unique identifier.\n"]
        pub id: ID,
        #[doc = "Whether a payment method may be used only once or used multiple times.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub usage: Option<PaymentMethodUsage>,
        #[doc = "The customer that the payment method belongs to.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub customer: Option<VaultPaymentVaultPaymentMethodPaymentMethodCustomer>,
        #[doc = "Details about the payment method specific to the type (e.g. credit card, PayPal account).\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub details: Option<VaultPaymentVaultPaymentMethodPaymentMethodDetails>,
    }
    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    #[doc = "A verification reporting whether the payment method has passed your fraud rules\nand the issuer has ensured it is associated with a valid account.\n"]
    pub struct VaultPaymentVaultPaymentMethodVerification {
        #[doc = "The current status of this verification, indicating whether the verification\nwas successful. Braintree recommends only vaulting payment methods that are\nsuccessfully verified.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub status: Option<VerificationStatus>,
    }
    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    #[doc = "Top-level output field from vaulting a payment method.\n"]
    pub struct VaultPaymentVaultPaymentMethod {
        #[doc = "A payment method that has been stored in a merchant's vault and can be reused.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "paymentMethod")]
        pub payment_method: Option<VaultPaymentVaultPaymentMethodPaymentMethod>,
        #[doc = "The verification that was run on the payment method prior to vaulting.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub verification: Option<VaultPaymentVaultPaymentMethodVerification>,
    }
    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    pub struct ResponseData {
        #[doc = "Vault payment information from a single-use payment method and return a\npayload that includes a new multi-use payment method.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "vaultPaymentMethod")]
        pub vault_payment_method: Option<VaultPaymentVaultPaymentMethod>,
    }
    #[allow(dead_code)]
    #[derive(Serialize)]
    pub struct VaultPayment {
        pub vault_payment_input: VaultPaymentMethodInput,
    }
    impl crate::GraphQLQueryCLI for VaultPayment {
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
pub mod delete_vaulted_payment {
    #![allow(dead_code)]
    pub const OPERATION_NAME: &'static str = "deleteVaultedPayment";
    pub const QUERY : & 'static str = "mutation chargePaymentMethod($paymentMethodId: ID!, $transaction:TransactionInput!, $clientMutationId: String) {\n    chargePaymentMethod(input: {\n        paymentMethodId:$paymentMethodId, transaction:$transaction, clientMutationId:$clientMutationId\n    }) {\n        transaction {\n            id,\n            createdAt,\n            amount {\n                value,\n                currencyIsoCode\n            },\n            orderId,\n            customer {\n                id\n            },\n            status\n        }\n    }\n}\n\nmutation vaultPayment($vault_payment_input: VaultPaymentMethodInput!) {\n    vaultPaymentMethod(input: $vault_payment_input) {\n        paymentMethod {\n            id\n            usage\n            customer {\n                id\n            }\n            details {\n                __typename\n                ... on CreditCardDetails {\n                    cardholderName\n                }\n                ... on PayPalAccountDetails {\n                    email\n                }\n                ... on VenmoAccountDetails {\n                    username\n                }\n                ... on UsBankAccountDetails {\n                    accountholderName\n                }\n            }\n        }\n        verification {\n            status\n        }\n    }\n}\n\nmutation deleteVaultedPayment($input: DeletePaymentMethodFromVaultInput!) {\n    deletePaymentMethodFromVault(input:$input) {\n        clientMutationId\n    }\n}\n\nmutation verifyPaymentMethod($input: VerifyPaymentMethodInput!) {\n    verifyPaymentMethod(input:$input) {\n        verification {\n            id,\n            amount{value},\n            status,\n            riskData{decision},\n            processorResponse{message,cvvResponse,avsPostalCodeResponse,avsStreetAddressResponse}\n        }\n    }\n}\n\nquery searchTransaction($input: TransactionSearchInput!) {\n    search {\n        transactions(input:$input) {\n            edges {\n                node {\n                    id,amount{value,currencyIsoCode},customer {email,id},\n                    paymentMethod{\n                        details {\n                            __typename\n                            ... on CreditCardDetails {\n                                cardholderName\n                            }\n                            ... on PayPalAccountDetails {\n                                email\n                            }\n                            ... on VenmoAccountDetails {\n                                username\n                            }\n                            ... on UsBankAccountDetails {\n                                accountholderName\n                            }\n                        }\n                    },orderId,status,createdAt\n                }\n            }\n        }\n    }\n}\n\nquery getTransaction($transactionID: String!) {\n    search {\n        transactions(input:{id: {is: $transactionID}}) {\n            edges {\n                node {\n                    id,amount{value,currencyIsoCode},customer {email,id},\n                    paymentMethod{\n                        details {\n                            __typename\n                            ... on CreditCardDetails {\n                                cardholderName\n                            }\n                            ... on PayPalAccountDetails {\n                                email\n                            }\n                            ... on VenmoAccountDetails {\n                                username\n                            }\n                            ... on UsBankAccountDetails {\n                                accountholderName\n                            }\n                        }\n                    },orderId,status,createdAt\n                }\n            }\n        }\n    }\n}\n" ;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    #[doc = "A monetary amount, either a whole number or a number with exactly two or three decimal places.\n"]
    type Amount = super::Amount;
    #[doc = "An [ISO 3166-1 alpha-3](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-3)\ncountry code. Braintree only accepts [specific alpha-3 values](https://developers.braintreepayments.com/reference/general/countries#list-of-countries).\n"]
    type CountryCodeAlpha3 = super::CountryCodeAlpha3;
    #[doc = "An [ISO 4217 alpha](https://en.wikipedia.org/wiki/ISO_4217) currency code.\nBraintree only accepts [specific alpha\nvalues](https://developers.braintreepayments.com/reference/general/currencies).\n"]
    type CurrencyCodeAlpha = super::CurrencyCodeAlpha;
    #[doc = "A string representing a custom field value. Contains letters, numbers, and underscores.\n"]
    type CustomFieldName = super::CustomFieldName;
    #[doc = "An [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601#Times) timestamp with microsecond precision.\n"]
    type Timestamp = super::Timestamp;
    #[derive(PartialEq, Serialize)]
    pub struct AddressInput {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub company: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "countryCode")]
        pub country_code: Option<CountryCodeAlpha3>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "countryCodeAlpha2")]
        pub country_code_alpha2: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "countryCodeAlpha3")]
        pub country_code_alpha3: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "countryCodeNumeric")]
        pub country_code_numeric: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "countryName")]
        pub country_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "extendedAddress")]
        pub extended_address: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "firstName")]
        pub first_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "lastName")]
        pub last_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub locality: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "postalCode")]
        pub postal_code: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub region: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "streetAddress")]
        pub street_address: Option<String>,
    }
    impl AddressInput {
        pub fn new() -> Self {
            Self {
                company: None,
                country_code: None,
                country_code_alpha2: None,
                country_code_alpha3: None,
                country_code_numeric: None,
                country_name: None,
                extended_address: None,
                first_name: None,
                last_name: None,
                locality: None,
                postal_code: None,
                region: None,
                street_address: None,
            }
        }
    }
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
    pub struct DeletePaymentMethodFromVaultInput {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "clientMutationId")]
        pub client_mutation_id: Option<String>,
        #[serde(rename = "paymentMethodId")]
        pub payment_method_id: ID,
    }
    impl DeletePaymentMethodFromVaultInput {
        pub fn new(payment_method_id: ID) -> Self {
            Self {
                client_mutation_id: None,
                payment_method_id: payment_method_id,
            }
        }
    }
    #[derive(PartialEq, Serialize)]
    pub struct RiskDataInput {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "customerBrowser")]
        pub customer_browser: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "customerIp")]
        pub customer_ip: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "deviceData")]
        pub device_data: Option<String>,
    }
    impl RiskDataInput {
        pub fn new() -> Self {
            Self {
                customer_browser: None,
                customer_ip: None,
                device_data: None,
            }
        }
    }
    #[derive(PartialEq, Serialize)]
    pub struct TransactionDescriptorInput {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub phone: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub url: Option<String>,
    }
    impl TransactionDescriptorInput {
        pub fn new() -> Self {
            Self {
                name: None,
                phone: None,
                url: None,
            }
        }
    }
    #[derive(PartialEq, Serialize)]
    pub struct TransactionInput {
        pub amount: Amount,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub channel: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "customFields")]
        pub custom_fields: Option<Vec<CustomFieldInput>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "customerId")]
        pub customer_id: Option<ID>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub descriptor: Option<TransactionDescriptorInput>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "discountAmount")]
        pub discount_amount: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "lineItems")]
        pub line_items: Option<Vec<TransactionLineItemInput>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "merchantAccountId")]
        pub merchant_account_id: Option<ID>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "orderId")]
        pub order_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "purchaseOrderNumber")]
        pub purchase_order_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recurring: Option<RecurringType>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "riskData")]
        pub risk_data: Option<RiskDataInput>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub shipping: Option<TransactionShippingInput>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tax: Option<TransactionTaxInput>,
    }
    impl TransactionInput {
        pub fn new(amount: Amount) -> Self {
            Self {
                amount: amount,
                channel: None,
                custom_fields: None,
                customer_id: None,
                descriptor: None,
                discount_amount: None,
                line_items: None,
                merchant_account_id: None,
                order_id: None,
                purchase_order_number: None,
                recurring: None,
                risk_data: None,
                shipping: None,
                tax: None,
            }
        }
    }
    #[derive(PartialEq, Serialize)]
    pub struct TransactionLineItemInput {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "commodityCode")]
        pub commodity_code: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "discountAmount")]
        pub discount_amount: Option<String>,
        pub kind: TransactionLineItemType,
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "productCode")]
        pub product_code: Option<String>,
        pub quantity: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "taxAmount")]
        pub tax_amount: Option<String>,
        #[serde(rename = "totalAmount")]
        pub total_amount: String,
        #[serde(rename = "unitAmount")]
        pub unit_amount: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "unitOfMeasure")]
        pub unit_of_measure: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "unitTaxAmount")]
        pub unit_tax_amount: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub url: Option<String>,
    }
    impl TransactionLineItemInput {
        pub fn new(
            kind: TransactionLineItemType,
            name: String,
            quantity: String,
            total_amount: String,
            unit_amount: String,
        ) -> Self {
            Self {
                commodity_code: None,
                description: None,
                discount_amount: None,
                kind: kind,
                name: name,
                product_code: None,
                quantity: quantity,
                tax_amount: None,
                total_amount: total_amount,
                unit_amount: unit_amount,
                unit_of_measure: None,
                unit_tax_amount: None,
                url: None,
            }
        }
    }
    #[derive(PartialEq, Serialize)]
    pub struct TransactionShippingInput {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "shippingAddress")]
        pub shipping_address: Option<AddressInput>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "shippingAmount")]
        pub shipping_amount: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "shipsFromPostalCode")]
        pub ships_from_postal_code: Option<String>,
    }
    impl TransactionShippingInput {
        pub fn new() -> Self {
            Self {
                shipping_address: None,
                shipping_amount: None,
                ships_from_postal_code: None,
            }
        }
    }
    #[derive(PartialEq, Serialize)]
    pub struct TransactionTaxInput {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "taxAmount")]
        pub tax_amount: Option<Amount>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "taxExempt")]
        pub tax_exempt: Option<Boolean>,
    }
    impl TransactionTaxInput {
        pub fn new() -> Self {
            Self {
                tax_amount: None,
                tax_exempt: None,
            }
        }
    }
    #[derive(PartialEq, Serialize)]
    pub struct VaultPaymentMethodInput {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "clientMutationId")]
        pub client_mutation_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "customerId")]
        pub customer_id: Option<ID>,
        #[serde(rename = "paymentMethodId")]
        pub payment_method_id: ID,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "verificationMerchantAccountId")]
        pub verification_merchant_account_id: Option<ID>,
    }
    impl VaultPaymentMethodInput {
        pub fn new(payment_method_id: ID) -> Self {
            Self {
                client_mutation_id: None,
                customer_id: None,
                payment_method_id: payment_method_id,
                verification_merchant_account_id: None,
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
    #[derive(Debug, Eq, PartialEq)]
    #[allow(non_camel_case_types)]
    pub enum RecurringType {
        FIRST,
        SUBSEQUENT,
        UNSCHEDULED,
        Other(String),
    }
    impl ::serde::Serialize for RecurringType {
        fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
            ser.serialize_str(match *self {
                RecurringType::FIRST => "FIRST",
                RecurringType::SUBSEQUENT => "SUBSEQUENT",
                RecurringType::UNSCHEDULED => "UNSCHEDULED",
                RecurringType::Other(ref s) => &s,
            })
        }
    }
    impl<'de> ::serde::Deserialize<'de> for RecurringType {
        fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let s = <String>::deserialize(deserializer)?;
            match s.as_str() {
                "FIRST" => Ok(RecurringType::FIRST),
                "SUBSEQUENT" => Ok(RecurringType::SUBSEQUENT),
                "UNSCHEDULED" => Ok(RecurringType::UNSCHEDULED),
                _ => Ok(RecurringType::Other(s)),
            }
        }
    }
    #[derive(Debug, Eq, PartialEq)]
    #[allow(non_camel_case_types)]
    pub enum TransactionLineItemType {
        CREDIT,
        DEBIT,
        Other(String),
    }
    impl ::serde::Serialize for TransactionLineItemType {
        fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
            ser.serialize_str(match *self {
                TransactionLineItemType::CREDIT => "CREDIT",
                TransactionLineItemType::DEBIT => "DEBIT",
                TransactionLineItemType::Other(ref s) => &s,
            })
        }
    }
    impl<'de> ::serde::Deserialize<'de> for TransactionLineItemType {
        fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let s = <String>::deserialize(deserializer)?;
            match s.as_str() {
                "CREDIT" => Ok(TransactionLineItemType::CREDIT),
                "DEBIT" => Ok(TransactionLineItemType::DEBIT),
                _ => Ok(TransactionLineItemType::Other(s)),
            }
        }
    }
    #[derive(Debug, Eq, PartialEq)]
    #[allow(non_camel_case_types)]
    pub enum TransactionStatus {
        #[doc = "The transaction spent too much time in the `AUTHORIZED` status and was marked\nas expired. Expiration [time frames](https://developers.braintreepayments.com/reference/general/statuses#authorization-expired)\ndiffer by card type, transaction type, and, in some cases, merchant category.\n"]
        AUTHORIZATION_EXPIRED,
        #[doc = "The processor authorized the transaction, putting your customer's funds on\nhold. Your customer may see a pending charge on his or her account. However,\nbefore the customer is actually charged and before you receive the funds, you\nmust use the `captureTransaction` mutation. If you do not want to capture the\ntransaction, you should use the `reverseTransaction` mutation to avoid a\nmisuse of authorization fee.\n"]
        AUTHORIZED,
        #[doc = "If a transaction remains in a status of `AUTHORIZING`, [contact Braintree\nSupport for assistance](https://help.braintreepayments.com).\n"]
        AUTHORIZING,
        #[doc = "An error occurred when sending the transaction to the downstream processor.\nSee the transaction's `statusHistory` for the exact error.\n"]
        FAILED,
        #[doc = "The transaction was [rejected](https://articles.braintreepayments.com/control-panel/transactions/gateway-rejections)\nbased on one or more settings or rules in your Braintree gateway. See the\ntransaction's `statusHistory` to determine which resulted in the decline.\n"]
        GATEWAY_REJECTED,
        #[doc = "The processor declined the transaction while attempting to authorize it. See\nthe transaction's `statusHistory` to determine what reason the processor gave\nfor the decline.\n"]
        PROCESSOR_DECLINED,
        #[doc = "The transaction has been settled, meaning your customer has been charged and\nthe process of disbursing the funds to your bank account will begin.\n"]
        SETTLED,
        SETTLEMENT_CONFIRMED,
        #[doc = "The processor declined the transaction while attempting to capture it. See the\ntransaction's `statusHistory` to detemine why it was not settled. This status\nis rare, and only certain types of transactions can be affected.\n"]
        SETTLEMENT_DECLINED,
        #[doc = "The transaction has not yet fully settled. This status is rare, and will\ngenerally resolve to a status of `SETTLED`. Only certain types of transactions\ncan be affected.\n"]
        SETTLEMENT_PENDING,
        #[doc = "The transaction is in the process of being settled. This is a transitory state, and will resolve to a status of `SETTLED`.\n"]
        SETTLING,
        #[doc = "The transaction has been successfully captured, and will be included in the\nnext settlement batch, at which time it will become settled.\n"]
        SUBMITTED_FOR_SETTLEMENT,
        #[doc = "The transaction was voided, meaning it is no longer authorized, your\ncustomer's funds are no longer on hold, and you cannot use the\n`captureTransaction` mutation on this transaction.\n"]
        VOIDED,
        Other(String),
    }
    impl ::serde::Serialize for TransactionStatus {
        fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
            ser.serialize_str(match *self {
                TransactionStatus::AUTHORIZATION_EXPIRED => "AUTHORIZATION_EXPIRED",
                TransactionStatus::AUTHORIZED => "AUTHORIZED",
                TransactionStatus::AUTHORIZING => "AUTHORIZING",
                TransactionStatus::FAILED => "FAILED",
                TransactionStatus::GATEWAY_REJECTED => "GATEWAY_REJECTED",
                TransactionStatus::PROCESSOR_DECLINED => "PROCESSOR_DECLINED",
                TransactionStatus::SETTLED => "SETTLED",
                TransactionStatus::SETTLEMENT_CONFIRMED => "SETTLEMENT_CONFIRMED",
                TransactionStatus::SETTLEMENT_DECLINED => "SETTLEMENT_DECLINED",
                TransactionStatus::SETTLEMENT_PENDING => "SETTLEMENT_PENDING",
                TransactionStatus::SETTLING => "SETTLING",
                TransactionStatus::SUBMITTED_FOR_SETTLEMENT => "SUBMITTED_FOR_SETTLEMENT",
                TransactionStatus::VOIDED => "VOIDED",
                TransactionStatus::Other(ref s) => &s,
            })
        }
    }
    impl<'de> ::serde::Deserialize<'de> for TransactionStatus {
        fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let s = <String>::deserialize(deserializer)?;
            match s.as_str() {
                "AUTHORIZATION_EXPIRED" => Ok(TransactionStatus::AUTHORIZATION_EXPIRED),
                "AUTHORIZED" => Ok(TransactionStatus::AUTHORIZED),
                "AUTHORIZING" => Ok(TransactionStatus::AUTHORIZING),
                "FAILED" => Ok(TransactionStatus::FAILED),
                "GATEWAY_REJECTED" => Ok(TransactionStatus::GATEWAY_REJECTED),
                "PROCESSOR_DECLINED" => Ok(TransactionStatus::PROCESSOR_DECLINED),
                "SETTLED" => Ok(TransactionStatus::SETTLED),
                "SETTLEMENT_CONFIRMED" => Ok(TransactionStatus::SETTLEMENT_CONFIRMED),
                "SETTLEMENT_DECLINED" => Ok(TransactionStatus::SETTLEMENT_DECLINED),
                "SETTLEMENT_PENDING" => Ok(TransactionStatus::SETTLEMENT_PENDING),
                "SETTLING" => Ok(TransactionStatus::SETTLING),
                "SUBMITTED_FOR_SETTLEMENT" => Ok(TransactionStatus::SUBMITTED_FOR_SETTLEMENT),
                "VOIDED" => Ok(TransactionStatus::VOIDED),
                _ => Ok(TransactionStatus::Other(s)),
            }
        }
    }
    #[derive(Debug, Eq, PartialEq)]
    #[allow(non_camel_case_types)]
    pub enum VerificationStatus {
        #[doc = "Indicates the verification was unsuccessful because of an issue communicating with the processor.\n"]
        FAILED,
        #[doc = "Indicates that the verification was unsuccessful because the payment method\nfailed one or more fraud checks. In this case, the `gatewayRejectionReason`\nwill indicate which fraud check failed.\n"]
        GATEWAY_REJECTED,
        #[doc = "Indicates that the verification is pending.\n"]
        PENDING,
        #[doc = "Indicates that the verification was unsuccessful based on the response from the processor.\n"]
        PROCESSOR_DECLINED,
        #[doc = "Indicates that the verification was successful.\n"]
        VERIFIED,
        #[doc = "Indicates that the verification is in the process of verifying.\n"]
        VERIFYING,
        Other(String),
    }
    impl ::serde::Serialize for VerificationStatus {
        fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
            ser.serialize_str(match *self {
                VerificationStatus::FAILED => "FAILED",
                VerificationStatus::GATEWAY_REJECTED => "GATEWAY_REJECTED",
                VerificationStatus::PENDING => "PENDING",
                VerificationStatus::PROCESSOR_DECLINED => "PROCESSOR_DECLINED",
                VerificationStatus::VERIFIED => "VERIFIED",
                VerificationStatus::VERIFYING => "VERIFYING",
                VerificationStatus::Other(ref s) => &s,
            })
        }
    }
    impl<'de> ::serde::Deserialize<'de> for VerificationStatus {
        fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let s = <String>::deserialize(deserializer)?;
            match s.as_str() {
                "FAILED" => Ok(VerificationStatus::FAILED),
                "GATEWAY_REJECTED" => Ok(VerificationStatus::GATEWAY_REJECTED),
                "PENDING" => Ok(VerificationStatus::PENDING),
                "PROCESSOR_DECLINED" => Ok(VerificationStatus::PROCESSOR_DECLINED),
                "VERIFIED" => Ok(VerificationStatus::VERIFIED),
                "VERIFYING" => Ok(VerificationStatus::VERIFYING),
                _ => Ok(VerificationStatus::Other(s)),
            }
        }
    }
    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    #[doc = "Top-level output field from deleting a multi-use payment method.\n"]
    pub struct DeleteVaultedPaymentDeletePaymentMethodFromVault {
        #[doc = "An identifier used to reconcile requests and responses.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "clientMutationId")]
        pub client_mutation_id: Option<String>,
    }
    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    pub struct ResponseData {
        #[doc = "Delete a multi-use payment method from the vault.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "deletePaymentMethodFromVault")]
        pub delete_payment_method_from_vault:
            Option<DeleteVaultedPaymentDeletePaymentMethodFromVault>,
    }
    #[allow(dead_code)]
    #[derive(Serialize)]
    pub struct DeleteVaultedPayment {
        pub input: DeletePaymentMethodFromVaultInput,
    }
    impl crate::GraphQLQueryCLI for DeleteVaultedPayment {
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
pub mod verify_payment_method {
    #![allow(dead_code)]
    pub const OPERATION_NAME: &'static str = "verifyPaymentMethod";
    pub const QUERY : & 'static str = "mutation chargePaymentMethod($paymentMethodId: ID!, $transaction:TransactionInput!, $clientMutationId: String) {\n    chargePaymentMethod(input: {\n        paymentMethodId:$paymentMethodId, transaction:$transaction, clientMutationId:$clientMutationId\n    }) {\n        transaction {\n            id,\n            createdAt,\n            amount {\n                value,\n                currencyIsoCode\n            },\n            orderId,\n            customer {\n                id\n            },\n            status\n        }\n    }\n}\n\nmutation vaultPayment($vault_payment_input: VaultPaymentMethodInput!) {\n    vaultPaymentMethod(input: $vault_payment_input) {\n        paymentMethod {\n            id\n            usage\n            customer {\n                id\n            }\n            details {\n                __typename\n                ... on CreditCardDetails {\n                    cardholderName\n                }\n                ... on PayPalAccountDetails {\n                    email\n                }\n                ... on VenmoAccountDetails {\n                    username\n                }\n                ... on UsBankAccountDetails {\n                    accountholderName\n                }\n            }\n        }\n        verification {\n            status\n        }\n    }\n}\n\nmutation deleteVaultedPayment($input: DeletePaymentMethodFromVaultInput!) {\n    deletePaymentMethodFromVault(input:$input) {\n        clientMutationId\n    }\n}\n\nmutation verifyPaymentMethod($input: VerifyPaymentMethodInput!) {\n    verifyPaymentMethod(input:$input) {\n        verification {\n            id,\n            amount{value},\n            status,\n            riskData{decision},\n            processorResponse{message,cvvResponse,avsPostalCodeResponse,avsStreetAddressResponse}\n        }\n    }\n}\n\nquery searchTransaction($input: TransactionSearchInput!) {\n    search {\n        transactions(input:$input) {\n            edges {\n                node {\n                    id,amount{value,currencyIsoCode},customer {email,id},\n                    paymentMethod{\n                        details {\n                            __typename\n                            ... on CreditCardDetails {\n                                cardholderName\n                            }\n                            ... on PayPalAccountDetails {\n                                email\n                            }\n                            ... on VenmoAccountDetails {\n                                username\n                            }\n                            ... on UsBankAccountDetails {\n                                accountholderName\n                            }\n                        }\n                    },orderId,status,createdAt\n                }\n            }\n        }\n    }\n}\n\nquery getTransaction($transactionID: String!) {\n    search {\n        transactions(input:{id: {is: $transactionID}}) {\n            edges {\n                node {\n                    id,amount{value,currencyIsoCode},customer {email,id},\n                    paymentMethod{\n                        details {\n                            __typename\n                            ... on CreditCardDetails {\n                                cardholderName\n                            }\n                            ... on PayPalAccountDetails {\n                                email\n                            }\n                            ... on VenmoAccountDetails {\n                                username\n                            }\n                            ... on UsBankAccountDetails {\n                                accountholderName\n                            }\n                        }\n                    },orderId,status,createdAt\n                }\n            }\n        }\n    }\n}\n" ;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    #[doc = "A monetary amount, either a whole number or a number with exactly two or three decimal places.\n"]
    type Amount = super::Amount;
    #[doc = "An [ISO 3166-1 alpha-3](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-3)\ncountry code. Braintree only accepts [specific alpha-3 values](https://developers.braintreepayments.com/reference/general/countries#list-of-countries).\n"]
    type CountryCodeAlpha3 = super::CountryCodeAlpha3;
    #[doc = "An [ISO 4217 alpha](https://en.wikipedia.org/wiki/ISO_4217) currency code.\nBraintree only accepts [specific alpha\nvalues](https://developers.braintreepayments.com/reference/general/currencies).\n"]
    type CurrencyCodeAlpha = super::CurrencyCodeAlpha;
    #[doc = "A string representing a custom field value. Contains letters, numbers, and underscores.\n"]
    type CustomFieldName = super::CustomFieldName;
    #[doc = "An [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601#Times) timestamp with microsecond precision.\n"]
    type Timestamp = super::Timestamp;
    #[derive(PartialEq, Serialize)]
    pub struct AddressInput {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub company: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "countryCode")]
        pub country_code: Option<CountryCodeAlpha3>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "countryCodeAlpha2")]
        pub country_code_alpha2: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "countryCodeAlpha3")]
        pub country_code_alpha3: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "countryCodeNumeric")]
        pub country_code_numeric: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "countryName")]
        pub country_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "extendedAddress")]
        pub extended_address: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "firstName")]
        pub first_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "lastName")]
        pub last_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub locality: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "postalCode")]
        pub postal_code: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub region: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "streetAddress")]
        pub street_address: Option<String>,
    }
    impl AddressInput {
        pub fn new() -> Self {
            Self {
                company: None,
                country_code: None,
                country_code_alpha2: None,
                country_code_alpha3: None,
                country_code_numeric: None,
                country_name: None,
                extended_address: None,
                first_name: None,
                last_name: None,
                locality: None,
                postal_code: None,
                region: None,
                street_address: None,
            }
        }
    }
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
    pub struct DeletePaymentMethodFromVaultInput {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "clientMutationId")]
        pub client_mutation_id: Option<String>,
        #[serde(rename = "paymentMethodId")]
        pub payment_method_id: ID,
    }
    impl DeletePaymentMethodFromVaultInput {
        pub fn new(payment_method_id: ID) -> Self {
            Self {
                client_mutation_id: None,
                payment_method_id: payment_method_id,
            }
        }
    }
    #[derive(PartialEq, Serialize)]
    pub struct RiskDataInput {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "customerBrowser")]
        pub customer_browser: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "customerIp")]
        pub customer_ip: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "deviceData")]
        pub device_data: Option<String>,
    }
    impl RiskDataInput {
        pub fn new() -> Self {
            Self {
                customer_browser: None,
                customer_ip: None,
                device_data: None,
            }
        }
    }
    #[derive(PartialEq, Serialize)]
    pub struct TransactionDescriptorInput {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub phone: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub url: Option<String>,
    }
    impl TransactionDescriptorInput {
        pub fn new() -> Self {
            Self {
                name: None,
                phone: None,
                url: None,
            }
        }
    }
    #[derive(PartialEq, Serialize)]
    pub struct TransactionInput {
        pub amount: Amount,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub channel: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "customFields")]
        pub custom_fields: Option<Vec<CustomFieldInput>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "customerId")]
        pub customer_id: Option<ID>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub descriptor: Option<TransactionDescriptorInput>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "discountAmount")]
        pub discount_amount: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "lineItems")]
        pub line_items: Option<Vec<TransactionLineItemInput>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "merchantAccountId")]
        pub merchant_account_id: Option<ID>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "orderId")]
        pub order_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "purchaseOrderNumber")]
        pub purchase_order_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recurring: Option<RecurringType>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "riskData")]
        pub risk_data: Option<RiskDataInput>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub shipping: Option<TransactionShippingInput>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tax: Option<TransactionTaxInput>,
    }
    impl TransactionInput {
        pub fn new(amount: Amount) -> Self {
            Self {
                amount: amount,
                channel: None,
                custom_fields: None,
                customer_id: None,
                descriptor: None,
                discount_amount: None,
                line_items: None,
                merchant_account_id: None,
                order_id: None,
                purchase_order_number: None,
                recurring: None,
                risk_data: None,
                shipping: None,
                tax: None,
            }
        }
    }
    #[derive(PartialEq, Serialize)]
    pub struct TransactionLineItemInput {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "commodityCode")]
        pub commodity_code: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "discountAmount")]
        pub discount_amount: Option<String>,
        pub kind: TransactionLineItemType,
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "productCode")]
        pub product_code: Option<String>,
        pub quantity: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "taxAmount")]
        pub tax_amount: Option<String>,
        #[serde(rename = "totalAmount")]
        pub total_amount: String,
        #[serde(rename = "unitAmount")]
        pub unit_amount: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "unitOfMeasure")]
        pub unit_of_measure: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "unitTaxAmount")]
        pub unit_tax_amount: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub url: Option<String>,
    }
    impl TransactionLineItemInput {
        pub fn new(
            kind: TransactionLineItemType,
            name: String,
            quantity: String,
            total_amount: String,
            unit_amount: String,
        ) -> Self {
            Self {
                commodity_code: None,
                description: None,
                discount_amount: None,
                kind: kind,
                name: name,
                product_code: None,
                quantity: quantity,
                tax_amount: None,
                total_amount: total_amount,
                unit_amount: unit_amount,
                unit_of_measure: None,
                unit_tax_amount: None,
                url: None,
            }
        }
    }
    #[derive(PartialEq, Serialize)]
    pub struct TransactionShippingInput {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "shippingAddress")]
        pub shipping_address: Option<AddressInput>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "shippingAmount")]
        pub shipping_amount: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "shipsFromPostalCode")]
        pub ships_from_postal_code: Option<String>,
    }
    impl TransactionShippingInput {
        pub fn new() -> Self {
            Self {
                shipping_address: None,
                shipping_amount: None,
                ships_from_postal_code: None,
            }
        }
    }
    #[derive(PartialEq, Serialize)]
    pub struct TransactionTaxInput {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "taxAmount")]
        pub tax_amount: Option<Amount>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "taxExempt")]
        pub tax_exempt: Option<Boolean>,
    }
    impl TransactionTaxInput {
        pub fn new() -> Self {
            Self {
                tax_amount: None,
                tax_exempt: None,
            }
        }
    }
    #[derive(PartialEq, Serialize)]
    pub struct VaultPaymentMethodInput {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "clientMutationId")]
        pub client_mutation_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "customerId")]
        pub customer_id: Option<ID>,
        #[serde(rename = "paymentMethodId")]
        pub payment_method_id: ID,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "verificationMerchantAccountId")]
        pub verification_merchant_account_id: Option<ID>,
    }
    impl VaultPaymentMethodInput {
        pub fn new(payment_method_id: ID) -> Self {
            Self {
                client_mutation_id: None,
                customer_id: None,
                payment_method_id: payment_method_id,
                verification_merchant_account_id: None,
            }
        }
    }
    #[derive(PartialEq, Serialize)]
    pub struct VerifyPaymentMethodInput {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "clientMutationId")]
        pub client_mutation_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "merchantAccountId")]
        pub merchant_account_id: Option<ID>,
        #[serde(rename = "paymentMethodId")]
        pub payment_method_id: ID,
    }
    impl VerifyPaymentMethodInput {
        pub fn new(payment_method_id: ID) -> Self {
            Self {
                client_mutation_id: None,
                merchant_account_id: None,
                payment_method_id: payment_method_id,
            }
        }
    }
    #[derive(Debug, Eq, PartialEq)]
    #[allow(non_camel_case_types)]
    pub enum AvsCvvResponseCode {
        BYPASS,
        DOES_NOT_MATCH,
        ISSUER_DOES_NOT_PARTICIPATE,
        MATCHES,
        NOT_APPLICABLE,
        NOT_PROVIDED,
        NOT_VERIFIED,
        SYSTEM_ERROR,
        Other(String),
    }
    impl ::serde::Serialize for AvsCvvResponseCode {
        fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
            ser.serialize_str(match *self {
                AvsCvvResponseCode::BYPASS => "BYPASS",
                AvsCvvResponseCode::DOES_NOT_MATCH => "DOES_NOT_MATCH",
                AvsCvvResponseCode::ISSUER_DOES_NOT_PARTICIPATE => "ISSUER_DOES_NOT_PARTICIPATE",
                AvsCvvResponseCode::MATCHES => "MATCHES",
                AvsCvvResponseCode::NOT_APPLICABLE => "NOT_APPLICABLE",
                AvsCvvResponseCode::NOT_PROVIDED => "NOT_PROVIDED",
                AvsCvvResponseCode::NOT_VERIFIED => "NOT_VERIFIED",
                AvsCvvResponseCode::SYSTEM_ERROR => "SYSTEM_ERROR",
                AvsCvvResponseCode::Other(ref s) => &s,
            })
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AvsCvvResponseCode {
        fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let s = <String>::deserialize(deserializer)?;
            match s.as_str() {
                "BYPASS" => Ok(AvsCvvResponseCode::BYPASS),
                "DOES_NOT_MATCH" => Ok(AvsCvvResponseCode::DOES_NOT_MATCH),
                "ISSUER_DOES_NOT_PARTICIPATE" => {
                    Ok(AvsCvvResponseCode::ISSUER_DOES_NOT_PARTICIPATE)
                }
                "MATCHES" => Ok(AvsCvvResponseCode::MATCHES),
                "NOT_APPLICABLE" => Ok(AvsCvvResponseCode::NOT_APPLICABLE),
                "NOT_PROVIDED" => Ok(AvsCvvResponseCode::NOT_PROVIDED),
                "NOT_VERIFIED" => Ok(AvsCvvResponseCode::NOT_VERIFIED),
                "SYSTEM_ERROR" => Ok(AvsCvvResponseCode::SYSTEM_ERROR),
                _ => Ok(AvsCvvResponseCode::Other(s)),
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
    #[derive(Debug, Eq, PartialEq)]
    #[allow(non_camel_case_types)]
    pub enum RecurringType {
        FIRST,
        SUBSEQUENT,
        UNSCHEDULED,
        Other(String),
    }
    impl ::serde::Serialize for RecurringType {
        fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
            ser.serialize_str(match *self {
                RecurringType::FIRST => "FIRST",
                RecurringType::SUBSEQUENT => "SUBSEQUENT",
                RecurringType::UNSCHEDULED => "UNSCHEDULED",
                RecurringType::Other(ref s) => &s,
            })
        }
    }
    impl<'de> ::serde::Deserialize<'de> for RecurringType {
        fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let s = <String>::deserialize(deserializer)?;
            match s.as_str() {
                "FIRST" => Ok(RecurringType::FIRST),
                "SUBSEQUENT" => Ok(RecurringType::SUBSEQUENT),
                "UNSCHEDULED" => Ok(RecurringType::UNSCHEDULED),
                _ => Ok(RecurringType::Other(s)),
            }
        }
    }
    #[derive(Debug, Eq, PartialEq)]
    #[allow(non_camel_case_types)]
    pub enum RiskDecision {
        APPROVE,
        DECLINE,
        NOT_EVALUATED,
        REVIEW,
        Other(String),
    }
    impl ::serde::Serialize for RiskDecision {
        fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
            ser.serialize_str(match *self {
                RiskDecision::APPROVE => "APPROVE",
                RiskDecision::DECLINE => "DECLINE",
                RiskDecision::NOT_EVALUATED => "NOT_EVALUATED",
                RiskDecision::REVIEW => "REVIEW",
                RiskDecision::Other(ref s) => &s,
            })
        }
    }
    impl<'de> ::serde::Deserialize<'de> for RiskDecision {
        fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let s = <String>::deserialize(deserializer)?;
            match s.as_str() {
                "APPROVE" => Ok(RiskDecision::APPROVE),
                "DECLINE" => Ok(RiskDecision::DECLINE),
                "NOT_EVALUATED" => Ok(RiskDecision::NOT_EVALUATED),
                "REVIEW" => Ok(RiskDecision::REVIEW),
                _ => Ok(RiskDecision::Other(s)),
            }
        }
    }
    #[derive(Debug, Eq, PartialEq)]
    #[allow(non_camel_case_types)]
    pub enum TransactionLineItemType {
        CREDIT,
        DEBIT,
        Other(String),
    }
    impl ::serde::Serialize for TransactionLineItemType {
        fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
            ser.serialize_str(match *self {
                TransactionLineItemType::CREDIT => "CREDIT",
                TransactionLineItemType::DEBIT => "DEBIT",
                TransactionLineItemType::Other(ref s) => &s,
            })
        }
    }
    impl<'de> ::serde::Deserialize<'de> for TransactionLineItemType {
        fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let s = <String>::deserialize(deserializer)?;
            match s.as_str() {
                "CREDIT" => Ok(TransactionLineItemType::CREDIT),
                "DEBIT" => Ok(TransactionLineItemType::DEBIT),
                _ => Ok(TransactionLineItemType::Other(s)),
            }
        }
    }
    #[derive(Debug, Eq, PartialEq)]
    #[allow(non_camel_case_types)]
    pub enum TransactionStatus {
        #[doc = "The transaction spent too much time in the `AUTHORIZED` status and was marked\nas expired. Expiration [time frames](https://developers.braintreepayments.com/reference/general/statuses#authorization-expired)\ndiffer by card type, transaction type, and, in some cases, merchant category.\n"]
        AUTHORIZATION_EXPIRED,
        #[doc = "The processor authorized the transaction, putting your customer's funds on\nhold. Your customer may see a pending charge on his or her account. However,\nbefore the customer is actually charged and before you receive the funds, you\nmust use the `captureTransaction` mutation. If you do not want to capture the\ntransaction, you should use the `reverseTransaction` mutation to avoid a\nmisuse of authorization fee.\n"]
        AUTHORIZED,
        #[doc = "If a transaction remains in a status of `AUTHORIZING`, [contact Braintree\nSupport for assistance](https://help.braintreepayments.com).\n"]
        AUTHORIZING,
        #[doc = "An error occurred when sending the transaction to the downstream processor.\nSee the transaction's `statusHistory` for the exact error.\n"]
        FAILED,
        #[doc = "The transaction was [rejected](https://articles.braintreepayments.com/control-panel/transactions/gateway-rejections)\nbased on one or more settings or rules in your Braintree gateway. See the\ntransaction's `statusHistory` to determine which resulted in the decline.\n"]
        GATEWAY_REJECTED,
        #[doc = "The processor declined the transaction while attempting to authorize it. See\nthe transaction's `statusHistory` to determine what reason the processor gave\nfor the decline.\n"]
        PROCESSOR_DECLINED,
        #[doc = "The transaction has been settled, meaning your customer has been charged and\nthe process of disbursing the funds to your bank account will begin.\n"]
        SETTLED,
        SETTLEMENT_CONFIRMED,
        #[doc = "The processor declined the transaction while attempting to capture it. See the\ntransaction's `statusHistory` to detemine why it was not settled. This status\nis rare, and only certain types of transactions can be affected.\n"]
        SETTLEMENT_DECLINED,
        #[doc = "The transaction has not yet fully settled. This status is rare, and will\ngenerally resolve to a status of `SETTLED`. Only certain types of transactions\ncan be affected.\n"]
        SETTLEMENT_PENDING,
        #[doc = "The transaction is in the process of being settled. This is a transitory state, and will resolve to a status of `SETTLED`.\n"]
        SETTLING,
        #[doc = "The transaction has been successfully captured, and will be included in the\nnext settlement batch, at which time it will become settled.\n"]
        SUBMITTED_FOR_SETTLEMENT,
        #[doc = "The transaction was voided, meaning it is no longer authorized, your\ncustomer's funds are no longer on hold, and you cannot use the\n`captureTransaction` mutation on this transaction.\n"]
        VOIDED,
        Other(String),
    }
    impl ::serde::Serialize for TransactionStatus {
        fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
            ser.serialize_str(match *self {
                TransactionStatus::AUTHORIZATION_EXPIRED => "AUTHORIZATION_EXPIRED",
                TransactionStatus::AUTHORIZED => "AUTHORIZED",
                TransactionStatus::AUTHORIZING => "AUTHORIZING",
                TransactionStatus::FAILED => "FAILED",
                TransactionStatus::GATEWAY_REJECTED => "GATEWAY_REJECTED",
                TransactionStatus::PROCESSOR_DECLINED => "PROCESSOR_DECLINED",
                TransactionStatus::SETTLED => "SETTLED",
                TransactionStatus::SETTLEMENT_CONFIRMED => "SETTLEMENT_CONFIRMED",
                TransactionStatus::SETTLEMENT_DECLINED => "SETTLEMENT_DECLINED",
                TransactionStatus::SETTLEMENT_PENDING => "SETTLEMENT_PENDING",
                TransactionStatus::SETTLING => "SETTLING",
                TransactionStatus::SUBMITTED_FOR_SETTLEMENT => "SUBMITTED_FOR_SETTLEMENT",
                TransactionStatus::VOIDED => "VOIDED",
                TransactionStatus::Other(ref s) => &s,
            })
        }
    }
    impl<'de> ::serde::Deserialize<'de> for TransactionStatus {
        fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let s = <String>::deserialize(deserializer)?;
            match s.as_str() {
                "AUTHORIZATION_EXPIRED" => Ok(TransactionStatus::AUTHORIZATION_EXPIRED),
                "AUTHORIZED" => Ok(TransactionStatus::AUTHORIZED),
                "AUTHORIZING" => Ok(TransactionStatus::AUTHORIZING),
                "FAILED" => Ok(TransactionStatus::FAILED),
                "GATEWAY_REJECTED" => Ok(TransactionStatus::GATEWAY_REJECTED),
                "PROCESSOR_DECLINED" => Ok(TransactionStatus::PROCESSOR_DECLINED),
                "SETTLED" => Ok(TransactionStatus::SETTLED),
                "SETTLEMENT_CONFIRMED" => Ok(TransactionStatus::SETTLEMENT_CONFIRMED),
                "SETTLEMENT_DECLINED" => Ok(TransactionStatus::SETTLEMENT_DECLINED),
                "SETTLEMENT_PENDING" => Ok(TransactionStatus::SETTLEMENT_PENDING),
                "SETTLING" => Ok(TransactionStatus::SETTLING),
                "SUBMITTED_FOR_SETTLEMENT" => Ok(TransactionStatus::SUBMITTED_FOR_SETTLEMENT),
                "VOIDED" => Ok(TransactionStatus::VOIDED),
                _ => Ok(TransactionStatus::Other(s)),
            }
        }
    }
    #[derive(Debug, Eq, PartialEq)]
    #[allow(non_camel_case_types)]
    pub enum VerificationStatus {
        #[doc = "Indicates the verification was unsuccessful because of an issue communicating with the processor.\n"]
        FAILED,
        #[doc = "Indicates that the verification was unsuccessful because the payment method\nfailed one or more fraud checks. In this case, the `gatewayRejectionReason`\nwill indicate which fraud check failed.\n"]
        GATEWAY_REJECTED,
        #[doc = "Indicates that the verification is pending.\n"]
        PENDING,
        #[doc = "Indicates that the verification was unsuccessful based on the response from the processor.\n"]
        PROCESSOR_DECLINED,
        #[doc = "Indicates that the verification was successful.\n"]
        VERIFIED,
        #[doc = "Indicates that the verification is in the process of verifying.\n"]
        VERIFYING,
        Other(String),
    }
    impl ::serde::Serialize for VerificationStatus {
        fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
            ser.serialize_str(match *self {
                VerificationStatus::FAILED => "FAILED",
                VerificationStatus::GATEWAY_REJECTED => "GATEWAY_REJECTED",
                VerificationStatus::PENDING => "PENDING",
                VerificationStatus::PROCESSOR_DECLINED => "PROCESSOR_DECLINED",
                VerificationStatus::VERIFIED => "VERIFIED",
                VerificationStatus::VERIFYING => "VERIFYING",
                VerificationStatus::Other(ref s) => &s,
            })
        }
    }
    impl<'de> ::serde::Deserialize<'de> for VerificationStatus {
        fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let s = <String>::deserialize(deserializer)?;
            match s.as_str() {
                "FAILED" => Ok(VerificationStatus::FAILED),
                "GATEWAY_REJECTED" => Ok(VerificationStatus::GATEWAY_REJECTED),
                "PENDING" => Ok(VerificationStatus::PENDING),
                "PROCESSOR_DECLINED" => Ok(VerificationStatus::PROCESSOR_DECLINED),
                "VERIFIED" => Ok(VerificationStatus::VERIFIED),
                "VERIFYING" => Ok(VerificationStatus::VERIFYING),
                _ => Ok(VerificationStatus::Other(s)),
            }
        }
    }
    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    #[doc = "A monetary amount with currency.\n"]
    pub struct VerifyPaymentMethodVerifyPaymentMethodVerificationAmount {
        #[doc = "The amount of money, either a whole number or a number with exactly 2 or 3 decimal places.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub value: Option<Amount>,
    }
    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    #[doc = "Data from advanced risk evaluations.\n"]
    pub struct VerifyPaymentMethodVerifyPaymentMethodVerificationRiskData {
        #[doc = "The risk decision on whether the transaction should be permitted.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub decision: Option<RiskDecision>,
    }
    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    #[doc = "Detailed response information from the processor.\n"]
    pub struct VerifyPaymentMethodVerifyPaymentMethodVerificationProcessorResponse {
        #[doc = "The text explanation of the processor response code.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub message: Option<String>,
        #[doc = "The processing bank's response to the provided CVV.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "cvvResponse")]
        pub cvv_response: Option<AvsCvvResponseCode>,
        #[doc = "The processing bank's response to the provided billing postal or zip code.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "avsPostalCodeResponse")]
        pub avs_postal_code_response: Option<AvsCvvResponseCode>,
        #[doc = "The processing bank's response to the provided billing street address.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "avsStreetAddressResponse")]
        pub avs_street_address_response: Option<AvsCvvResponseCode>,
    }
    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    #[doc = "A verification reporting whether the payment method has passed your fraud rules\nand the issuer has ensured it is associated with a valid account.\n"]
    pub struct VerifyPaymentMethodVerifyPaymentMethodVerification {
        #[doc = "Unique identifier.\n"]
        pub id: ID,
        #[doc = "For a credit card, the amount used when performing the verification.\n"]
        #[deprecated(
            note = "Depending on the type of payment method being verified, some verifications do not have an amount. On a credit card verification, use `paymentMethodVerificationDetails.amount` instead."
        )]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub amount: Option<VerifyPaymentMethodVerifyPaymentMethodVerificationAmount>,
        #[doc = "The current status of this verification, indicating whether the verification\nwas successful. Braintree recommends only vaulting payment methods that are\nsuccessfully verified.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub status: Option<VerificationStatus>,
        #[doc = "Risk data evaluated for this verification.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "riskData")]
        pub risk_data: Option<VerifyPaymentMethodVerifyPaymentMethodVerificationRiskData>,
        #[doc = "Detailed response information from the processor. Will not be present if the\nverification was rejected prior to contacting the processor.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "processorResponse")]
        pub processor_response:
            Option<VerifyPaymentMethodVerifyPaymentMethodVerificationProcessorResponse>,
    }
    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    #[doc = "Top-level output field from verifying a payment method.\n"]
    pub struct VerifyPaymentMethodVerifyPaymentMethod {
        #[doc = "The verification that was run on the payment method.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub verification: Option<VerifyPaymentMethodVerifyPaymentMethodVerification>,
    }
    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    pub struct ResponseData {
        #[doc = "Run a verification on a multi-use payment method.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "verifyPaymentMethod")]
        pub verify_payment_method: Option<VerifyPaymentMethodVerifyPaymentMethod>,
    }
    #[allow(dead_code)]
    #[derive(Serialize)]
    pub struct VerifyPaymentMethod {
        pub input: VerifyPaymentMethodInput,
    }
    impl crate::GraphQLQueryCLI for VerifyPaymentMethod {
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
pub mod search_transaction {
    #![allow(dead_code)]
    pub const OPERATION_NAME: &'static str = "searchTransaction";
    pub const QUERY : & 'static str = "mutation chargePaymentMethod($paymentMethodId: ID!, $transaction:TransactionInput!, $clientMutationId: String) {\n    chargePaymentMethod(input: {\n        paymentMethodId:$paymentMethodId, transaction:$transaction, clientMutationId:$clientMutationId\n    }) {\n        transaction {\n            id,\n            createdAt,\n            amount {\n                value,\n                currencyIsoCode\n            },\n            orderId,\n            customer {\n                id\n            },\n            status\n        }\n    }\n}\n\nmutation vaultPayment($vault_payment_input: VaultPaymentMethodInput!) {\n    vaultPaymentMethod(input: $vault_payment_input) {\n        paymentMethod {\n            id\n            usage\n            customer {\n                id\n            }\n            details {\n                __typename\n                ... on CreditCardDetails {\n                    cardholderName\n                }\n                ... on PayPalAccountDetails {\n                    email\n                }\n                ... on VenmoAccountDetails {\n                    username\n                }\n                ... on UsBankAccountDetails {\n                    accountholderName\n                }\n            }\n        }\n        verification {\n            status\n        }\n    }\n}\n\nmutation deleteVaultedPayment($input: DeletePaymentMethodFromVaultInput!) {\n    deletePaymentMethodFromVault(input:$input) {\n        clientMutationId\n    }\n}\n\nmutation verifyPaymentMethod($input: VerifyPaymentMethodInput!) {\n    verifyPaymentMethod(input:$input) {\n        verification {\n            id,\n            amount{value},\n            status,\n            riskData{decision},\n            processorResponse{message,cvvResponse,avsPostalCodeResponse,avsStreetAddressResponse}\n        }\n    }\n}\n\nquery searchTransaction($input: TransactionSearchInput!) {\n    search {\n        transactions(input:$input) {\n            edges {\n                node {\n                    id,amount{value,currencyIsoCode},customer {email,id},\n                    paymentMethod{\n                        details {\n                            __typename\n                            ... on CreditCardDetails {\n                                cardholderName\n                            }\n                            ... on PayPalAccountDetails {\n                                email\n                            }\n                            ... on VenmoAccountDetails {\n                                username\n                            }\n                            ... on UsBankAccountDetails {\n                                accountholderName\n                            }\n                        }\n                    },orderId,status,createdAt\n                }\n            }\n        }\n    }\n}\n\nquery getTransaction($transactionID: String!) {\n    search {\n        transactions(input:{id: {is: $transactionID}}) {\n            edges {\n                node {\n                    id,amount{value,currencyIsoCode},customer {email,id},\n                    paymentMethod{\n                        details {\n                            __typename\n                            ... on CreditCardDetails {\n                                cardholderName\n                            }\n                            ... on PayPalAccountDetails {\n                                email\n                            }\n                            ... on VenmoAccountDetails {\n                                username\n                            }\n                            ... on UsBankAccountDetails {\n                                accountholderName\n                            }\n                        }\n                    },orderId,status,createdAt\n                }\n            }\n        }\n    }\n}\n" ;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    #[doc = "A monetary amount, either a whole number or a number with exactly two or three decimal places.\n"]
    type Amount = super::Amount;
    #[doc = "An [ISO 3166-1 alpha-3](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-3)\ncountry code. Braintree only accepts [specific alpha-3 values](https://developers.braintreepayments.com/reference/general/countries#list-of-countries).\n"]
    type CountryCodeAlpha3 = super::CountryCodeAlpha3;
    #[doc = "An [ISO 4217 alpha](https://en.wikipedia.org/wiki/ISO_4217) currency code.\nBraintree only accepts [specific alpha\nvalues](https://developers.braintreepayments.com/reference/general/currencies).\n"]
    type CurrencyCodeAlpha = super::CurrencyCodeAlpha;
    #[doc = "A string representing a custom field value. Contains letters, numbers, and underscores.\n"]
    type CustomFieldName = super::CustomFieldName;
    #[doc = "An [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601#Times) timestamp with microsecond precision.\n"]
    type Timestamp = super::Timestamp;
    #[derive(PartialEq, Serialize)]
    pub struct AddressInput {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub company: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "countryCode")]
        pub country_code: Option<CountryCodeAlpha3>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "countryCodeAlpha2")]
        pub country_code_alpha2: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "countryCodeAlpha3")]
        pub country_code_alpha3: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "countryCodeNumeric")]
        pub country_code_numeric: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "countryName")]
        pub country_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "extendedAddress")]
        pub extended_address: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "firstName")]
        pub first_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "lastName")]
        pub last_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub locality: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "postalCode")]
        pub postal_code: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub region: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "streetAddress")]
        pub street_address: Option<String>,
    }
    impl AddressInput {
        pub fn new() -> Self {
            Self {
                company: None,
                country_code: None,
                country_code_alpha2: None,
                country_code_alpha3: None,
                country_code_numeric: None,
                country_name: None,
                extended_address: None,
                first_name: None,
                last_name: None,
                locality: None,
                postal_code: None,
                region: None,
                street_address: None,
            }
        }
    }
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
    pub struct DeletePaymentMethodFromVaultInput {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "clientMutationId")]
        pub client_mutation_id: Option<String>,
        #[serde(rename = "paymentMethodId")]
        pub payment_method_id: ID,
    }
    impl DeletePaymentMethodFromVaultInput {
        pub fn new(payment_method_id: ID) -> Self {
            Self {
                client_mutation_id: None,
                payment_method_id: payment_method_id,
            }
        }
    }
    #[derive(PartialEq, Serialize)]
    pub struct MonetaryAmountSearchInput {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "currencyIsoCode")]
        pub currency_iso_code: Option<SearchTextInput>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub value: Option<SearchRangeInput>,
    }
    impl MonetaryAmountSearchInput {
        pub fn new() -> Self {
            Self {
                currency_iso_code: None,
                value: None,
            }
        }
    }
    #[derive(PartialEq, Serialize)]
    pub struct RiskDataInput {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "customerBrowser")]
        pub customer_browser: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "customerIp")]
        pub customer_ip: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "deviceData")]
        pub device_data: Option<String>,
    }
    impl RiskDataInput {
        pub fn new() -> Self {
            Self {
                customer_browser: None,
                customer_ip: None,
                device_data: None,
            }
        }
    }
    #[derive(PartialEq, Serialize)]
    pub struct SearchCustomerInput {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<SearchValueInput>,
    }
    impl SearchCustomerInput {
        pub fn new() -> Self {
            Self { id: None }
        }
    }
    #[derive(PartialEq, Serialize)]
    pub struct SearchPaymentMethodSnapshotTypeInput {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "in")]
        pub in_: Option<Vec<PaymentMethodSnapshotSearchType>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub is: Option<PaymentMethodSnapshotSearchType>,
    }
    impl SearchPaymentMethodSnapshotTypeInput {
        pub fn new() -> Self {
            Self {
                in_: None,
                is: None,
            }
        }
    }
    #[derive(PartialEq, Serialize)]
    pub struct SearchRangeInput {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "greaterThanOrEqualTo")]
        pub greater_than_or_equal_to: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub is: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "lessThanOrEqualTo")]
        pub less_than_or_equal_to: Option<String>,
    }
    impl SearchRangeInput {
        pub fn new() -> Self {
            Self {
                greater_than_or_equal_to: None,
                is: None,
                less_than_or_equal_to: None,
            }
        }
    }
    #[derive(PartialEq, Serialize)]
    pub struct SearchTextInput {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub contains: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "endsWith")]
        pub ends_with: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub is: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "isNot")]
        pub is_not: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "startsWith")]
        pub starts_with: Option<String>,
    }
    impl SearchTextInput {
        pub fn new() -> Self {
            Self {
                contains: None,
                ends_with: None,
                is: None,
                is_not: None,
                starts_with: None,
            }
        }
    }
    #[derive(PartialEq, Serialize)]
    pub struct SearchTimestampInput {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "greaterThanOrEqualTo")]
        pub greater_than_or_equal_to: Option<Timestamp>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "lessThanOrEqualTo")]
        pub less_than_or_equal_to: Option<Timestamp>,
    }
    impl SearchTimestampInput {
        pub fn new() -> Self {
            Self {
                greater_than_or_equal_to: None,
                less_than_or_equal_to: None,
            }
        }
    }
    #[derive(PartialEq, Serialize)]
    pub struct SearchTransactionStatusInput {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "in")]
        pub in_: Option<Vec<TransactionStatus>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub is: Option<TransactionStatus>,
    }
    impl SearchTransactionStatusInput {
        pub fn new() -> Self {
            Self {
                in_: None,
                is: None,
            }
        }
    }
    #[derive(PartialEq, Serialize)]
    pub struct SearchTransactionStatusTransitionInput {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "failedAt")]
        pub failed_at: Option<SearchTimestampInput>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "settledAt")]
        pub settled_at: Option<SearchTimestampInput>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "submittedForSettlementAt")]
        pub submitted_for_settlement_at: Option<SearchTimestampInput>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "voidedAt")]
        pub voided_at: Option<SearchTimestampInput>,
    }
    impl SearchTransactionStatusTransitionInput {
        pub fn new() -> Self {
            Self {
                failed_at: None,
                settled_at: None,
                submitted_for_settlement_at: None,
                voided_at: None,
            }
        }
    }
    #[derive(PartialEq, Serialize)]
    pub struct SearchValueInput {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "in")]
        pub in_: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub is: Option<String>,
    }
    impl SearchValueInput {
        pub fn new() -> Self {
            Self {
                in_: None,
                is: None,
            }
        }
    }
    #[derive(PartialEq, Serialize)]
    pub struct TransactionDescriptorInput {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub phone: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub url: Option<String>,
    }
    impl TransactionDescriptorInput {
        pub fn new() -> Self {
            Self {
                name: None,
                phone: None,
                url: None,
            }
        }
    }
    #[derive(PartialEq, Serialize)]
    pub struct TransactionInput {
        pub amount: Amount,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub channel: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "customFields")]
        pub custom_fields: Option<Vec<CustomFieldInput>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "customerId")]
        pub customer_id: Option<ID>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub descriptor: Option<TransactionDescriptorInput>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "discountAmount")]
        pub discount_amount: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "lineItems")]
        pub line_items: Option<Vec<TransactionLineItemInput>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "merchantAccountId")]
        pub merchant_account_id: Option<ID>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "orderId")]
        pub order_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "purchaseOrderNumber")]
        pub purchase_order_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recurring: Option<RecurringType>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "riskData")]
        pub risk_data: Option<RiskDataInput>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub shipping: Option<TransactionShippingInput>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tax: Option<TransactionTaxInput>,
    }
    impl TransactionInput {
        pub fn new(amount: Amount) -> Self {
            Self {
                amount: amount,
                channel: None,
                custom_fields: None,
                customer_id: None,
                descriptor: None,
                discount_amount: None,
                line_items: None,
                merchant_account_id: None,
                order_id: None,
                purchase_order_number: None,
                recurring: None,
                risk_data: None,
                shipping: None,
                tax: None,
            }
        }
    }
    #[derive(PartialEq, Serialize)]
    pub struct TransactionLineItemInput {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "commodityCode")]
        pub commodity_code: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "discountAmount")]
        pub discount_amount: Option<String>,
        pub kind: TransactionLineItemType,
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "productCode")]
        pub product_code: Option<String>,
        pub quantity: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "taxAmount")]
        pub tax_amount: Option<String>,
        #[serde(rename = "totalAmount")]
        pub total_amount: String,
        #[serde(rename = "unitAmount")]
        pub unit_amount: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "unitOfMeasure")]
        pub unit_of_measure: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "unitTaxAmount")]
        pub unit_tax_amount: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub url: Option<String>,
    }
    impl TransactionLineItemInput {
        pub fn new(
            kind: TransactionLineItemType,
            name: String,
            quantity: String,
            total_amount: String,
            unit_amount: String,
        ) -> Self {
            Self {
                commodity_code: None,
                description: None,
                discount_amount: None,
                kind: kind,
                name: name,
                product_code: None,
                quantity: quantity,
                tax_amount: None,
                total_amount: total_amount,
                unit_amount: unit_amount,
                unit_of_measure: None,
                unit_tax_amount: None,
                url: None,
            }
        }
    }
    #[derive(PartialEq, Serialize)]
    pub struct TransactionSearchInput {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub amount: Option<MonetaryAmountSearchInput>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "createdAt")]
        pub created_at: Option<SearchTimestampInput>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub customer: Option<SearchCustomerInput>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<SearchValueInput>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "orderId")]
        pub order_id: Option<SearchTextInput>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "paymentMethodSnapshotType")]
        pub payment_method_snapshot_type: Option<SearchPaymentMethodSnapshotTypeInput>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub status: Option<SearchTransactionStatusInput>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "statusTransition")]
        pub status_transition: Option<SearchTransactionStatusTransitionInput>,
    }
    impl TransactionSearchInput {
        pub fn new() -> Self {
            Self {
                amount: None,
                created_at: None,
                customer: None,
                id: None,
                order_id: None,
                payment_method_snapshot_type: None,
                status: None,
                status_transition: None,
            }
        }
    }
    #[derive(PartialEq, Serialize)]
    pub struct TransactionShippingInput {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "shippingAddress")]
        pub shipping_address: Option<AddressInput>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "shippingAmount")]
        pub shipping_amount: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "shipsFromPostalCode")]
        pub ships_from_postal_code: Option<String>,
    }
    impl TransactionShippingInput {
        pub fn new() -> Self {
            Self {
                shipping_address: None,
                shipping_amount: None,
                ships_from_postal_code: None,
            }
        }
    }
    #[derive(PartialEq, Serialize)]
    pub struct TransactionTaxInput {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "taxAmount")]
        pub tax_amount: Option<Amount>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "taxExempt")]
        pub tax_exempt: Option<Boolean>,
    }
    impl TransactionTaxInput {
        pub fn new() -> Self {
            Self {
                tax_amount: None,
                tax_exempt: None,
            }
        }
    }
    #[derive(PartialEq, Serialize)]
    pub struct VaultPaymentMethodInput {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "clientMutationId")]
        pub client_mutation_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "customerId")]
        pub customer_id: Option<ID>,
        #[serde(rename = "paymentMethodId")]
        pub payment_method_id: ID,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "verificationMerchantAccountId")]
        pub verification_merchant_account_id: Option<ID>,
    }
    impl VaultPaymentMethodInput {
        pub fn new(payment_method_id: ID) -> Self {
            Self {
                client_mutation_id: None,
                customer_id: None,
                payment_method_id: payment_method_id,
                verification_merchant_account_id: None,
            }
        }
    }
    #[derive(PartialEq, Serialize)]
    pub struct VerifyPaymentMethodInput {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "clientMutationId")]
        pub client_mutation_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "merchantAccountId")]
        pub merchant_account_id: Option<ID>,
        #[serde(rename = "paymentMethodId")]
        pub payment_method_id: ID,
    }
    impl VerifyPaymentMethodInput {
        pub fn new(payment_method_id: ID) -> Self {
            Self {
                client_mutation_id: None,
                merchant_account_id: None,
                payment_method_id: payment_method_id,
            }
        }
    }
    #[derive(Debug, Eq, PartialEq)]
    #[allow(non_camel_case_types)]
    pub enum AvsCvvResponseCode {
        BYPASS,
        DOES_NOT_MATCH,
        ISSUER_DOES_NOT_PARTICIPATE,
        MATCHES,
        NOT_APPLICABLE,
        NOT_PROVIDED,
        NOT_VERIFIED,
        SYSTEM_ERROR,
        Other(String),
    }
    impl ::serde::Serialize for AvsCvvResponseCode {
        fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
            ser.serialize_str(match *self {
                AvsCvvResponseCode::BYPASS => "BYPASS",
                AvsCvvResponseCode::DOES_NOT_MATCH => "DOES_NOT_MATCH",
                AvsCvvResponseCode::ISSUER_DOES_NOT_PARTICIPATE => "ISSUER_DOES_NOT_PARTICIPATE",
                AvsCvvResponseCode::MATCHES => "MATCHES",
                AvsCvvResponseCode::NOT_APPLICABLE => "NOT_APPLICABLE",
                AvsCvvResponseCode::NOT_PROVIDED => "NOT_PROVIDED",
                AvsCvvResponseCode::NOT_VERIFIED => "NOT_VERIFIED",
                AvsCvvResponseCode::SYSTEM_ERROR => "SYSTEM_ERROR",
                AvsCvvResponseCode::Other(ref s) => &s,
            })
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AvsCvvResponseCode {
        fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let s = <String>::deserialize(deserializer)?;
            match s.as_str() {
                "BYPASS" => Ok(AvsCvvResponseCode::BYPASS),
                "DOES_NOT_MATCH" => Ok(AvsCvvResponseCode::DOES_NOT_MATCH),
                "ISSUER_DOES_NOT_PARTICIPATE" => {
                    Ok(AvsCvvResponseCode::ISSUER_DOES_NOT_PARTICIPATE)
                }
                "MATCHES" => Ok(AvsCvvResponseCode::MATCHES),
                "NOT_APPLICABLE" => Ok(AvsCvvResponseCode::NOT_APPLICABLE),
                "NOT_PROVIDED" => Ok(AvsCvvResponseCode::NOT_PROVIDED),
                "NOT_VERIFIED" => Ok(AvsCvvResponseCode::NOT_VERIFIED),
                "SYSTEM_ERROR" => Ok(AvsCvvResponseCode::SYSTEM_ERROR),
                _ => Ok(AvsCvvResponseCode::Other(s)),
            }
        }
    }
    #[derive(Debug, Eq, PartialEq)]
    #[allow(non_camel_case_types)]
    pub enum PaymentMethodSnapshotSearchType {
        CREDIT_CARD,
        CREDIT_CARD_VIA_APPLE_PAY,
        CREDIT_CARD_VIA_GOOGLE_PAY,
        CREDIT_CARD_VIA_MASTERPASS,
        CREDIT_CARD_VIA_SAMSUNG_PAY,
        CREDIT_CARD_VIA_VISA_CHECKOUT,
        PAYPAL,
        US_BANK_ACCOUNT,
        VENMO_ACCOUNT,
        Other(String),
    }
    impl ::serde::Serialize for PaymentMethodSnapshotSearchType {
        fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
            ser.serialize_str(match *self {
                PaymentMethodSnapshotSearchType::CREDIT_CARD => "CREDIT_CARD",
                PaymentMethodSnapshotSearchType::CREDIT_CARD_VIA_APPLE_PAY => {
                    "CREDIT_CARD_VIA_APPLE_PAY"
                }
                PaymentMethodSnapshotSearchType::CREDIT_CARD_VIA_GOOGLE_PAY => {
                    "CREDIT_CARD_VIA_GOOGLE_PAY"
                }
                PaymentMethodSnapshotSearchType::CREDIT_CARD_VIA_MASTERPASS => {
                    "CREDIT_CARD_VIA_MASTERPASS"
                }
                PaymentMethodSnapshotSearchType::CREDIT_CARD_VIA_SAMSUNG_PAY => {
                    "CREDIT_CARD_VIA_SAMSUNG_PAY"
                }
                PaymentMethodSnapshotSearchType::CREDIT_CARD_VIA_VISA_CHECKOUT => {
                    "CREDIT_CARD_VIA_VISA_CHECKOUT"
                }
                PaymentMethodSnapshotSearchType::PAYPAL => "PAYPAL",
                PaymentMethodSnapshotSearchType::US_BANK_ACCOUNT => "US_BANK_ACCOUNT",
                PaymentMethodSnapshotSearchType::VENMO_ACCOUNT => "VENMO_ACCOUNT",
                PaymentMethodSnapshotSearchType::Other(ref s) => &s,
            })
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PaymentMethodSnapshotSearchType {
        fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let s = <String>::deserialize(deserializer)?;
            match s.as_str() {
                "CREDIT_CARD" => Ok(PaymentMethodSnapshotSearchType::CREDIT_CARD),
                "CREDIT_CARD_VIA_APPLE_PAY" => {
                    Ok(PaymentMethodSnapshotSearchType::CREDIT_CARD_VIA_APPLE_PAY)
                }
                "CREDIT_CARD_VIA_GOOGLE_PAY" => {
                    Ok(PaymentMethodSnapshotSearchType::CREDIT_CARD_VIA_GOOGLE_PAY)
                }
                "CREDIT_CARD_VIA_MASTERPASS" => {
                    Ok(PaymentMethodSnapshotSearchType::CREDIT_CARD_VIA_MASTERPASS)
                }
                "CREDIT_CARD_VIA_SAMSUNG_PAY" => {
                    Ok(PaymentMethodSnapshotSearchType::CREDIT_CARD_VIA_SAMSUNG_PAY)
                }
                "CREDIT_CARD_VIA_VISA_CHECKOUT" => {
                    Ok(PaymentMethodSnapshotSearchType::CREDIT_CARD_VIA_VISA_CHECKOUT)
                }
                "PAYPAL" => Ok(PaymentMethodSnapshotSearchType::PAYPAL),
                "US_BANK_ACCOUNT" => Ok(PaymentMethodSnapshotSearchType::US_BANK_ACCOUNT),
                "VENMO_ACCOUNT" => Ok(PaymentMethodSnapshotSearchType::VENMO_ACCOUNT),
                _ => Ok(PaymentMethodSnapshotSearchType::Other(s)),
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
    #[derive(Debug, Eq, PartialEq)]
    #[allow(non_camel_case_types)]
    pub enum RecurringType {
        FIRST,
        SUBSEQUENT,
        UNSCHEDULED,
        Other(String),
    }
    impl ::serde::Serialize for RecurringType {
        fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
            ser.serialize_str(match *self {
                RecurringType::FIRST => "FIRST",
                RecurringType::SUBSEQUENT => "SUBSEQUENT",
                RecurringType::UNSCHEDULED => "UNSCHEDULED",
                RecurringType::Other(ref s) => &s,
            })
        }
    }
    impl<'de> ::serde::Deserialize<'de> for RecurringType {
        fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let s = <String>::deserialize(deserializer)?;
            match s.as_str() {
                "FIRST" => Ok(RecurringType::FIRST),
                "SUBSEQUENT" => Ok(RecurringType::SUBSEQUENT),
                "UNSCHEDULED" => Ok(RecurringType::UNSCHEDULED),
                _ => Ok(RecurringType::Other(s)),
            }
        }
    }
    #[derive(Debug, Eq, PartialEq)]
    #[allow(non_camel_case_types)]
    pub enum RiskDecision {
        APPROVE,
        DECLINE,
        NOT_EVALUATED,
        REVIEW,
        Other(String),
    }
    impl ::serde::Serialize for RiskDecision {
        fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
            ser.serialize_str(match *self {
                RiskDecision::APPROVE => "APPROVE",
                RiskDecision::DECLINE => "DECLINE",
                RiskDecision::NOT_EVALUATED => "NOT_EVALUATED",
                RiskDecision::REVIEW => "REVIEW",
                RiskDecision::Other(ref s) => &s,
            })
        }
    }
    impl<'de> ::serde::Deserialize<'de> for RiskDecision {
        fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let s = <String>::deserialize(deserializer)?;
            match s.as_str() {
                "APPROVE" => Ok(RiskDecision::APPROVE),
                "DECLINE" => Ok(RiskDecision::DECLINE),
                "NOT_EVALUATED" => Ok(RiskDecision::NOT_EVALUATED),
                "REVIEW" => Ok(RiskDecision::REVIEW),
                _ => Ok(RiskDecision::Other(s)),
            }
        }
    }
    #[derive(Debug, Eq, PartialEq)]
    #[allow(non_camel_case_types)]
    pub enum TransactionLineItemType {
        CREDIT,
        DEBIT,
        Other(String),
    }
    impl ::serde::Serialize for TransactionLineItemType {
        fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
            ser.serialize_str(match *self {
                TransactionLineItemType::CREDIT => "CREDIT",
                TransactionLineItemType::DEBIT => "DEBIT",
                TransactionLineItemType::Other(ref s) => &s,
            })
        }
    }
    impl<'de> ::serde::Deserialize<'de> for TransactionLineItemType {
        fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let s = <String>::deserialize(deserializer)?;
            match s.as_str() {
                "CREDIT" => Ok(TransactionLineItemType::CREDIT),
                "DEBIT" => Ok(TransactionLineItemType::DEBIT),
                _ => Ok(TransactionLineItemType::Other(s)),
            }
        }
    }
    #[derive(Debug, Eq, PartialEq)]
    #[allow(non_camel_case_types)]
    pub enum TransactionStatus {
        #[doc = "The transaction spent too much time in the `AUTHORIZED` status and was marked\nas expired. Expiration [time frames](https://developers.braintreepayments.com/reference/general/statuses#authorization-expired)\ndiffer by card type, transaction type, and, in some cases, merchant category.\n"]
        AUTHORIZATION_EXPIRED,
        #[doc = "The processor authorized the transaction, putting your customer's funds on\nhold. Your customer may see a pending charge on his or her account. However,\nbefore the customer is actually charged and before you receive the funds, you\nmust use the `captureTransaction` mutation. If you do not want to capture the\ntransaction, you should use the `reverseTransaction` mutation to avoid a\nmisuse of authorization fee.\n"]
        AUTHORIZED,
        #[doc = "If a transaction remains in a status of `AUTHORIZING`, [contact Braintree\nSupport for assistance](https://help.braintreepayments.com).\n"]
        AUTHORIZING,
        #[doc = "An error occurred when sending the transaction to the downstream processor.\nSee the transaction's `statusHistory` for the exact error.\n"]
        FAILED,
        #[doc = "The transaction was [rejected](https://articles.braintreepayments.com/control-panel/transactions/gateway-rejections)\nbased on one or more settings or rules in your Braintree gateway. See the\ntransaction's `statusHistory` to determine which resulted in the decline.\n"]
        GATEWAY_REJECTED,
        #[doc = "The processor declined the transaction while attempting to authorize it. See\nthe transaction's `statusHistory` to determine what reason the processor gave\nfor the decline.\n"]
        PROCESSOR_DECLINED,
        #[doc = "The transaction has been settled, meaning your customer has been charged and\nthe process of disbursing the funds to your bank account will begin.\n"]
        SETTLED,
        SETTLEMENT_CONFIRMED,
        #[doc = "The processor declined the transaction while attempting to capture it. See the\ntransaction's `statusHistory` to detemine why it was not settled. This status\nis rare, and only certain types of transactions can be affected.\n"]
        SETTLEMENT_DECLINED,
        #[doc = "The transaction has not yet fully settled. This status is rare, and will\ngenerally resolve to a status of `SETTLED`. Only certain types of transactions\ncan be affected.\n"]
        SETTLEMENT_PENDING,
        #[doc = "The transaction is in the process of being settled. This is a transitory state, and will resolve to a status of `SETTLED`.\n"]
        SETTLING,
        #[doc = "The transaction has been successfully captured, and will be included in the\nnext settlement batch, at which time it will become settled.\n"]
        SUBMITTED_FOR_SETTLEMENT,
        #[doc = "The transaction was voided, meaning it is no longer authorized, your\ncustomer's funds are no longer on hold, and you cannot use the\n`captureTransaction` mutation on this transaction.\n"]
        VOIDED,
        Other(String),
    }
    impl ::serde::Serialize for TransactionStatus {
        fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
            ser.serialize_str(match *self {
                TransactionStatus::AUTHORIZATION_EXPIRED => "AUTHORIZATION_EXPIRED",
                TransactionStatus::AUTHORIZED => "AUTHORIZED",
                TransactionStatus::AUTHORIZING => "AUTHORIZING",
                TransactionStatus::FAILED => "FAILED",
                TransactionStatus::GATEWAY_REJECTED => "GATEWAY_REJECTED",
                TransactionStatus::PROCESSOR_DECLINED => "PROCESSOR_DECLINED",
                TransactionStatus::SETTLED => "SETTLED",
                TransactionStatus::SETTLEMENT_CONFIRMED => "SETTLEMENT_CONFIRMED",
                TransactionStatus::SETTLEMENT_DECLINED => "SETTLEMENT_DECLINED",
                TransactionStatus::SETTLEMENT_PENDING => "SETTLEMENT_PENDING",
                TransactionStatus::SETTLING => "SETTLING",
                TransactionStatus::SUBMITTED_FOR_SETTLEMENT => "SUBMITTED_FOR_SETTLEMENT",
                TransactionStatus::VOIDED => "VOIDED",
                TransactionStatus::Other(ref s) => &s,
            })
        }
    }
    impl<'de> ::serde::Deserialize<'de> for TransactionStatus {
        fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let s = <String>::deserialize(deserializer)?;
            match s.as_str() {
                "AUTHORIZATION_EXPIRED" => Ok(TransactionStatus::AUTHORIZATION_EXPIRED),
                "AUTHORIZED" => Ok(TransactionStatus::AUTHORIZED),
                "AUTHORIZING" => Ok(TransactionStatus::AUTHORIZING),
                "FAILED" => Ok(TransactionStatus::FAILED),
                "GATEWAY_REJECTED" => Ok(TransactionStatus::GATEWAY_REJECTED),
                "PROCESSOR_DECLINED" => Ok(TransactionStatus::PROCESSOR_DECLINED),
                "SETTLED" => Ok(TransactionStatus::SETTLED),
                "SETTLEMENT_CONFIRMED" => Ok(TransactionStatus::SETTLEMENT_CONFIRMED),
                "SETTLEMENT_DECLINED" => Ok(TransactionStatus::SETTLEMENT_DECLINED),
                "SETTLEMENT_PENDING" => Ok(TransactionStatus::SETTLEMENT_PENDING),
                "SETTLING" => Ok(TransactionStatus::SETTLING),
                "SUBMITTED_FOR_SETTLEMENT" => Ok(TransactionStatus::SUBMITTED_FOR_SETTLEMENT),
                "VOIDED" => Ok(TransactionStatus::VOIDED),
                _ => Ok(TransactionStatus::Other(s)),
            }
        }
    }
    #[derive(Debug, Eq, PartialEq)]
    #[allow(non_camel_case_types)]
    pub enum VerificationStatus {
        #[doc = "Indicates the verification was unsuccessful because of an issue communicating with the processor.\n"]
        FAILED,
        #[doc = "Indicates that the verification was unsuccessful because the payment method\nfailed one or more fraud checks. In this case, the `gatewayRejectionReason`\nwill indicate which fraud check failed.\n"]
        GATEWAY_REJECTED,
        #[doc = "Indicates that the verification is pending.\n"]
        PENDING,
        #[doc = "Indicates that the verification was unsuccessful based on the response from the processor.\n"]
        PROCESSOR_DECLINED,
        #[doc = "Indicates that the verification was successful.\n"]
        VERIFIED,
        #[doc = "Indicates that the verification is in the process of verifying.\n"]
        VERIFYING,
        Other(String),
    }
    impl ::serde::Serialize for VerificationStatus {
        fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
            ser.serialize_str(match *self {
                VerificationStatus::FAILED => "FAILED",
                VerificationStatus::GATEWAY_REJECTED => "GATEWAY_REJECTED",
                VerificationStatus::PENDING => "PENDING",
                VerificationStatus::PROCESSOR_DECLINED => "PROCESSOR_DECLINED",
                VerificationStatus::VERIFIED => "VERIFIED",
                VerificationStatus::VERIFYING => "VERIFYING",
                VerificationStatus::Other(ref s) => &s,
            })
        }
    }
    impl<'de> ::serde::Deserialize<'de> for VerificationStatus {
        fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let s = <String>::deserialize(deserializer)?;
            match s.as_str() {
                "FAILED" => Ok(VerificationStatus::FAILED),
                "GATEWAY_REJECTED" => Ok(VerificationStatus::GATEWAY_REJECTED),
                "PENDING" => Ok(VerificationStatus::PENDING),
                "PROCESSOR_DECLINED" => Ok(VerificationStatus::PROCESSOR_DECLINED),
                "VERIFIED" => Ok(VerificationStatus::VERIFIED),
                "VERIFYING" => Ok(VerificationStatus::VERIFYING),
                _ => Ok(VerificationStatus::Other(s)),
            }
        }
    }
    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    #[doc = "A monetary amount with currency.\n"]
    pub struct SearchTransactionSearchTransactionsEdgesNodeAmount {
        #[doc = "The amount of money, either a whole number or a number with exactly 2 or 3 decimal places.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub value: Option<Amount>,
        #[doc = "The ISO code for the money's currency.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "currencyIsoCode")]
        pub currency_iso_code: Option<CurrencyCodeAlpha>,
    }
    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    #[doc = "Information about a customer and their associated payment methods and transactions.\n"]
    pub struct SearchTransactionSearchTransactionsEdgesNodeCustomer {
        #[doc = "Email address for this customer.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub email: Option<String>,
        #[doc = "Unique identifier.\n"]
        pub id: ID,
    }
    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    #[doc = "Details about a credit card.\n"]
    pub struct SearchTransactionSearchTransactionsEdgesNodePaymentMethodDetailsOnCreditCardDetails {
        #[doc = "The cardholder's name.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "cardholderName")]
        pub cardholder_name: Option<String>,
    }
    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    #[doc = "Details about a PayPal account.\n"]
    pub struct SearchTransactionSearchTransactionsEdgesNodePaymentMethodDetailsOnPayPalAccountDetails {
        #[doc = "The email address associated with the PayPal account.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub email: Option<String>,
    }
    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    #[doc = "Details about a U.S. bank account.\n"]
    pub struct SearchTransactionSearchTransactionsEdgesNodePaymentMethodDetailsOnUsBankAccountDetails {
        #[doc = "The name of the accountholder. This is either the business name for a\nbusiness account, or the owner's full name for an individual account.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "accountholderName")]
        pub accountholder_name: Option<String>,
    }
    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    #[doc = "Details about a Venmo Account.\n"]
    pub struct SearchTransactionSearchTransactionsEdgesNodePaymentMethodDetailsOnVenmoAccountDetails {
        #[doc = "The Venmo username, as chosen by the user.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub username: Option<String>,
    }
    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    #[serde(tag = "__typename")]
    pub enum SearchTransactionSearchTransactionsEdgesNodePaymentMethodDetails {
        CreditCardDetails(
            SearchTransactionSearchTransactionsEdgesNodePaymentMethodDetailsOnCreditCardDetails,
        ),
        PayPalAccountDetails(
            SearchTransactionSearchTransactionsEdgesNodePaymentMethodDetailsOnPayPalAccountDetails,
        ),
        UsBankAccountDetails(
            SearchTransactionSearchTransactionsEdgesNodePaymentMethodDetailsOnUsBankAccountDetails,
        ),
        VenmoAccountDetails(
            SearchTransactionSearchTransactionsEdgesNodePaymentMethodDetailsOnVenmoAccountDetails,
        ),
        CustomActionsPaymentMethodDetails,
        SamsungPayCardDetails,
    }
    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    #[doc = "Top-level field representing a payment method.\n"]
    pub struct SearchTransactionSearchTransactionsEdgesNodePaymentMethod {
        #[doc = "Details about the payment method specific to the type (e.g. credit card, PayPal account).\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub details: Option<SearchTransactionSearchTransactionsEdgesNodePaymentMethodDetails>,
    }
    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    #[doc = "A charge on a payment method.\n"]
    pub struct SearchTransactionSearchTransactionsEdgesNode {
        #[doc = "Unique identifier.\n"]
        pub id: ID,
        #[doc = "The amount charged in this transaction. For transactions that are partially\ncaptured, this amount will be the cummulative amount captured on this transaction.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub amount: Option<SearchTransactionSearchTransactionsEdgesNodeAmount>,
        #[doc = "Customer associated with the transaction, if applicable.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub customer: Option<SearchTransactionSearchTransactionsEdgesNodeCustomer>,
        #[doc = "The multi-use payment method associated with the transaction. Only present if\na multi-use payment method was used to create the transaction and it has not\nbeen deleted. The details of this PaymentMethod may have changed since the\ntransaction was created, details used for the transaction can be found in the\n`paymentMethodSnapshot` field.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "paymentMethod")]
        pub payment_method: Option<SearchTransactionSearchTransactionsEdgesNodePaymentMethod>,
        #[doc = "The order ID for this transaction. For PayPal transactions, the PayPal Invoice ID.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "orderId")]
        pub order_id: Option<String>,
        #[doc = "The current status of this transaction.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub status: Option<TransactionStatus>,
        #[doc = "Time at which the transaction was created.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "createdAt")]
        pub created_at: Option<Timestamp>,
    }
    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    #[doc = "A transaction within a TransactionConnection.\n"]
    pub struct SearchTransactionSearchTransactionsEdges {
        #[doc = "The transaction.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub node: Option<SearchTransactionSearchTransactionsEdgesNode>,
    }
    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    #[doc = "A paginated list of transactions.\n"]
    pub struct SearchTransactionSearchTransactions {
        #[doc = "A list of transactions.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub edges: Option<Vec<Option<SearchTransactionSearchTransactionsEdges>>>,
    }
    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    #[doc = "Top-level fields returned for a search query.\n"]
    pub struct SearchTransactionSearch {
        #[doc = "A paginated list of transactions that match the TransactionSearchInput.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub transactions: Option<SearchTransactionSearchTransactions>,
    }
    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    pub struct ResponseData {
        #[doc = "The available searches.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub search: Option<SearchTransactionSearch>,
    }
    #[allow(dead_code)]
    #[derive(Serialize)]
    pub struct SearchTransaction {
        pub input: TransactionSearchInput,
    }
    impl crate::GraphQLQueryCLI for SearchTransaction {
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
pub mod get_transaction {
    #![allow(dead_code)]
    pub const OPERATION_NAME: &'static str = "getTransaction";
    pub const QUERY : & 'static str = "mutation chargePaymentMethod($paymentMethodId: ID!, $transaction:TransactionInput!, $clientMutationId: String) {\n    chargePaymentMethod(input: {\n        paymentMethodId:$paymentMethodId, transaction:$transaction, clientMutationId:$clientMutationId\n    }) {\n        transaction {\n            id,\n            createdAt,\n            amount {\n                value,\n                currencyIsoCode\n            },\n            orderId,\n            customer {\n                id\n            },\n            status\n        }\n    }\n}\n\nmutation vaultPayment($vault_payment_input: VaultPaymentMethodInput!) {\n    vaultPaymentMethod(input: $vault_payment_input) {\n        paymentMethod {\n            id\n            usage\n            customer {\n                id\n            }\n            details {\n                __typename\n                ... on CreditCardDetails {\n                    cardholderName\n                }\n                ... on PayPalAccountDetails {\n                    email\n                }\n                ... on VenmoAccountDetails {\n                    username\n                }\n                ... on UsBankAccountDetails {\n                    accountholderName\n                }\n            }\n        }\n        verification {\n            status\n        }\n    }\n}\n\nmutation deleteVaultedPayment($input: DeletePaymentMethodFromVaultInput!) {\n    deletePaymentMethodFromVault(input:$input) {\n        clientMutationId\n    }\n}\n\nmutation verifyPaymentMethod($input: VerifyPaymentMethodInput!) {\n    verifyPaymentMethod(input:$input) {\n        verification {\n            id,\n            amount{value},\n            status,\n            riskData{decision},\n            processorResponse{message,cvvResponse,avsPostalCodeResponse,avsStreetAddressResponse}\n        }\n    }\n}\n\nquery searchTransaction($input: TransactionSearchInput!) {\n    search {\n        transactions(input:$input) {\n            edges {\n                node {\n                    id,amount{value,currencyIsoCode},customer {email,id},\n                    paymentMethod{\n                        details {\n                            __typename\n                            ... on CreditCardDetails {\n                                cardholderName\n                            }\n                            ... on PayPalAccountDetails {\n                                email\n                            }\n                            ... on VenmoAccountDetails {\n                                username\n                            }\n                            ... on UsBankAccountDetails {\n                                accountholderName\n                            }\n                        }\n                    },orderId,status,createdAt\n                }\n            }\n        }\n    }\n}\n\nquery getTransaction($transactionID: String!) {\n    search {\n        transactions(input:{id: {is: $transactionID}}) {\n            edges {\n                node {\n                    id,amount{value,currencyIsoCode},customer {email,id},\n                    paymentMethod{\n                        details {\n                            __typename\n                            ... on CreditCardDetails {\n                                cardholderName\n                            }\n                            ... on PayPalAccountDetails {\n                                email\n                            }\n                            ... on VenmoAccountDetails {\n                                username\n                            }\n                            ... on UsBankAccountDetails {\n                                accountholderName\n                            }\n                        }\n                    },orderId,status,createdAt\n                }\n            }\n        }\n    }\n}\n" ;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    #[doc = "A monetary amount, either a whole number or a number with exactly two or three decimal places.\n"]
    type Amount = super::Amount;
    #[doc = "An [ISO 3166-1 alpha-3](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-3)\ncountry code. Braintree only accepts [specific alpha-3 values](https://developers.braintreepayments.com/reference/general/countries#list-of-countries).\n"]
    type CountryCodeAlpha3 = super::CountryCodeAlpha3;
    #[doc = "An [ISO 4217 alpha](https://en.wikipedia.org/wiki/ISO_4217) currency code.\nBraintree only accepts [specific alpha\nvalues](https://developers.braintreepayments.com/reference/general/currencies).\n"]
    type CurrencyCodeAlpha = super::CurrencyCodeAlpha;
    #[doc = "A string representing a custom field value. Contains letters, numbers, and underscores.\n"]
    type CustomFieldName = super::CustomFieldName;
    #[doc = "An [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601#Times) timestamp with microsecond precision.\n"]
    type Timestamp = super::Timestamp;
    #[derive(PartialEq, Serialize)]
    pub struct AddressInput {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub company: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "countryCode")]
        pub country_code: Option<CountryCodeAlpha3>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "countryCodeAlpha2")]
        pub country_code_alpha2: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "countryCodeAlpha3")]
        pub country_code_alpha3: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "countryCodeNumeric")]
        pub country_code_numeric: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "countryName")]
        pub country_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "extendedAddress")]
        pub extended_address: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "firstName")]
        pub first_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "lastName")]
        pub last_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub locality: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "postalCode")]
        pub postal_code: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub region: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "streetAddress")]
        pub street_address: Option<String>,
    }
    impl AddressInput {
        pub fn new() -> Self {
            Self {
                company: None,
                country_code: None,
                country_code_alpha2: None,
                country_code_alpha3: None,
                country_code_numeric: None,
                country_name: None,
                extended_address: None,
                first_name: None,
                last_name: None,
                locality: None,
                postal_code: None,
                region: None,
                street_address: None,
            }
        }
    }
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
    pub struct DeletePaymentMethodFromVaultInput {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "clientMutationId")]
        pub client_mutation_id: Option<String>,
        #[serde(rename = "paymentMethodId")]
        pub payment_method_id: ID,
    }
    impl DeletePaymentMethodFromVaultInput {
        pub fn new(payment_method_id: ID) -> Self {
            Self {
                client_mutation_id: None,
                payment_method_id: payment_method_id,
            }
        }
    }
    #[derive(PartialEq, Serialize)]
    pub struct MonetaryAmountSearchInput {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "currencyIsoCode")]
        pub currency_iso_code: Option<SearchTextInput>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub value: Option<SearchRangeInput>,
    }
    impl MonetaryAmountSearchInput {
        pub fn new() -> Self {
            Self {
                currency_iso_code: None,
                value: None,
            }
        }
    }
    #[derive(PartialEq, Serialize)]
    pub struct RiskDataInput {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "customerBrowser")]
        pub customer_browser: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "customerIp")]
        pub customer_ip: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "deviceData")]
        pub device_data: Option<String>,
    }
    impl RiskDataInput {
        pub fn new() -> Self {
            Self {
                customer_browser: None,
                customer_ip: None,
                device_data: None,
            }
        }
    }
    #[derive(PartialEq, Serialize)]
    pub struct SearchCustomerInput {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<SearchValueInput>,
    }
    impl SearchCustomerInput {
        pub fn new() -> Self {
            Self { id: None }
        }
    }
    #[derive(PartialEq, Serialize)]
    pub struct SearchPaymentMethodSnapshotTypeInput {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "in")]
        pub in_: Option<Vec<PaymentMethodSnapshotSearchType>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub is: Option<PaymentMethodSnapshotSearchType>,
    }
    impl SearchPaymentMethodSnapshotTypeInput {
        pub fn new() -> Self {
            Self {
                in_: None,
                is: None,
            }
        }
    }
    #[derive(PartialEq, Serialize)]
    pub struct SearchRangeInput {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "greaterThanOrEqualTo")]
        pub greater_than_or_equal_to: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub is: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "lessThanOrEqualTo")]
        pub less_than_or_equal_to: Option<String>,
    }
    impl SearchRangeInput {
        pub fn new() -> Self {
            Self {
                greater_than_or_equal_to: None,
                is: None,
                less_than_or_equal_to: None,
            }
        }
    }
    #[derive(PartialEq, Serialize)]
    pub struct SearchTextInput {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub contains: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "endsWith")]
        pub ends_with: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub is: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "isNot")]
        pub is_not: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "startsWith")]
        pub starts_with: Option<String>,
    }
    impl SearchTextInput {
        pub fn new() -> Self {
            Self {
                contains: None,
                ends_with: None,
                is: None,
                is_not: None,
                starts_with: None,
            }
        }
    }
    #[derive(PartialEq, Serialize)]
    pub struct SearchTimestampInput {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "greaterThanOrEqualTo")]
        pub greater_than_or_equal_to: Option<Timestamp>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "lessThanOrEqualTo")]
        pub less_than_or_equal_to: Option<Timestamp>,
    }
    impl SearchTimestampInput {
        pub fn new() -> Self {
            Self {
                greater_than_or_equal_to: None,
                less_than_or_equal_to: None,
            }
        }
    }
    #[derive(PartialEq, Serialize)]
    pub struct SearchTransactionStatusInput {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "in")]
        pub in_: Option<Vec<TransactionStatus>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub is: Option<TransactionStatus>,
    }
    impl SearchTransactionStatusInput {
        pub fn new() -> Self {
            Self {
                in_: None,
                is: None,
            }
        }
    }
    #[derive(PartialEq, Serialize)]
    pub struct SearchTransactionStatusTransitionInput {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "failedAt")]
        pub failed_at: Option<SearchTimestampInput>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "settledAt")]
        pub settled_at: Option<SearchTimestampInput>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "submittedForSettlementAt")]
        pub submitted_for_settlement_at: Option<SearchTimestampInput>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "voidedAt")]
        pub voided_at: Option<SearchTimestampInput>,
    }
    impl SearchTransactionStatusTransitionInput {
        pub fn new() -> Self {
            Self {
                failed_at: None,
                settled_at: None,
                submitted_for_settlement_at: None,
                voided_at: None,
            }
        }
    }
    #[derive(PartialEq, Serialize)]
    pub struct SearchValueInput {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "in")]
        pub in_: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub is: Option<String>,
    }
    impl SearchValueInput {
        pub fn new() -> Self {
            Self {
                in_: None,
                is: None,
            }
        }
    }
    #[derive(PartialEq, Serialize)]
    pub struct TransactionDescriptorInput {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub phone: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub url: Option<String>,
    }
    impl TransactionDescriptorInput {
        pub fn new() -> Self {
            Self {
                name: None,
                phone: None,
                url: None,
            }
        }
    }
    #[derive(PartialEq, Serialize)]
    pub struct TransactionInput {
        pub amount: Amount,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub channel: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "customFields")]
        pub custom_fields: Option<Vec<CustomFieldInput>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "customerId")]
        pub customer_id: Option<ID>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub descriptor: Option<TransactionDescriptorInput>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "discountAmount")]
        pub discount_amount: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "lineItems")]
        pub line_items: Option<Vec<TransactionLineItemInput>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "merchantAccountId")]
        pub merchant_account_id: Option<ID>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "orderId")]
        pub order_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "purchaseOrderNumber")]
        pub purchase_order_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub recurring: Option<RecurringType>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "riskData")]
        pub risk_data: Option<RiskDataInput>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub shipping: Option<TransactionShippingInput>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tax: Option<TransactionTaxInput>,
    }
    impl TransactionInput {
        pub fn new(amount: Amount) -> Self {
            Self {
                amount: amount,
                channel: None,
                custom_fields: None,
                customer_id: None,
                descriptor: None,
                discount_amount: None,
                line_items: None,
                merchant_account_id: None,
                order_id: None,
                purchase_order_number: None,
                recurring: None,
                risk_data: None,
                shipping: None,
                tax: None,
            }
        }
    }
    #[derive(PartialEq, Serialize)]
    pub struct TransactionLineItemInput {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "commodityCode")]
        pub commodity_code: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "discountAmount")]
        pub discount_amount: Option<String>,
        pub kind: TransactionLineItemType,
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "productCode")]
        pub product_code: Option<String>,
        pub quantity: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "taxAmount")]
        pub tax_amount: Option<String>,
        #[serde(rename = "totalAmount")]
        pub total_amount: String,
        #[serde(rename = "unitAmount")]
        pub unit_amount: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "unitOfMeasure")]
        pub unit_of_measure: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "unitTaxAmount")]
        pub unit_tax_amount: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub url: Option<String>,
    }
    impl TransactionLineItemInput {
        pub fn new(
            kind: TransactionLineItemType,
            name: String,
            quantity: String,
            total_amount: String,
            unit_amount: String,
        ) -> Self {
            Self {
                commodity_code: None,
                description: None,
                discount_amount: None,
                kind: kind,
                name: name,
                product_code: None,
                quantity: quantity,
                tax_amount: None,
                total_amount: total_amount,
                unit_amount: unit_amount,
                unit_of_measure: None,
                unit_tax_amount: None,
                url: None,
            }
        }
    }
    #[derive(PartialEq, Serialize)]
    pub struct TransactionSearchInput {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub amount: Option<MonetaryAmountSearchInput>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "createdAt")]
        pub created_at: Option<SearchTimestampInput>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub customer: Option<SearchCustomerInput>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<SearchValueInput>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "orderId")]
        pub order_id: Option<SearchTextInput>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "paymentMethodSnapshotType")]
        pub payment_method_snapshot_type: Option<SearchPaymentMethodSnapshotTypeInput>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub status: Option<SearchTransactionStatusInput>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "statusTransition")]
        pub status_transition: Option<SearchTransactionStatusTransitionInput>,
    }
    impl TransactionSearchInput {
        pub fn new() -> Self {
            Self {
                amount: None,
                created_at: None,
                customer: None,
                id: None,
                order_id: None,
                payment_method_snapshot_type: None,
                status: None,
                status_transition: None,
            }
        }
    }
    #[derive(PartialEq, Serialize)]
    pub struct TransactionShippingInput {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "shippingAddress")]
        pub shipping_address: Option<AddressInput>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "shippingAmount")]
        pub shipping_amount: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "shipsFromPostalCode")]
        pub ships_from_postal_code: Option<String>,
    }
    impl TransactionShippingInput {
        pub fn new() -> Self {
            Self {
                shipping_address: None,
                shipping_amount: None,
                ships_from_postal_code: None,
            }
        }
    }
    #[derive(PartialEq, Serialize)]
    pub struct TransactionTaxInput {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "taxAmount")]
        pub tax_amount: Option<Amount>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "taxExempt")]
        pub tax_exempt: Option<Boolean>,
    }
    impl TransactionTaxInput {
        pub fn new() -> Self {
            Self {
                tax_amount: None,
                tax_exempt: None,
            }
        }
    }
    #[derive(PartialEq, Serialize)]
    pub struct VaultPaymentMethodInput {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "clientMutationId")]
        pub client_mutation_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "customerId")]
        pub customer_id: Option<ID>,
        #[serde(rename = "paymentMethodId")]
        pub payment_method_id: ID,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "verificationMerchantAccountId")]
        pub verification_merchant_account_id: Option<ID>,
    }
    impl VaultPaymentMethodInput {
        pub fn new(payment_method_id: ID) -> Self {
            Self {
                client_mutation_id: None,
                customer_id: None,
                payment_method_id: payment_method_id,
                verification_merchant_account_id: None,
            }
        }
    }
    #[derive(PartialEq, Serialize)]
    pub struct VerifyPaymentMethodInput {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "clientMutationId")]
        pub client_mutation_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "merchantAccountId")]
        pub merchant_account_id: Option<ID>,
        #[serde(rename = "paymentMethodId")]
        pub payment_method_id: ID,
    }
    impl VerifyPaymentMethodInput {
        pub fn new(payment_method_id: ID) -> Self {
            Self {
                client_mutation_id: None,
                merchant_account_id: None,
                payment_method_id: payment_method_id,
            }
        }
    }
    #[derive(Debug, Eq, PartialEq)]
    #[allow(non_camel_case_types)]
    pub enum AvsCvvResponseCode {
        BYPASS,
        DOES_NOT_MATCH,
        ISSUER_DOES_NOT_PARTICIPATE,
        MATCHES,
        NOT_APPLICABLE,
        NOT_PROVIDED,
        NOT_VERIFIED,
        SYSTEM_ERROR,
        Other(String),
    }
    impl ::serde::Serialize for AvsCvvResponseCode {
        fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
            ser.serialize_str(match *self {
                AvsCvvResponseCode::BYPASS => "BYPASS",
                AvsCvvResponseCode::DOES_NOT_MATCH => "DOES_NOT_MATCH",
                AvsCvvResponseCode::ISSUER_DOES_NOT_PARTICIPATE => "ISSUER_DOES_NOT_PARTICIPATE",
                AvsCvvResponseCode::MATCHES => "MATCHES",
                AvsCvvResponseCode::NOT_APPLICABLE => "NOT_APPLICABLE",
                AvsCvvResponseCode::NOT_PROVIDED => "NOT_PROVIDED",
                AvsCvvResponseCode::NOT_VERIFIED => "NOT_VERIFIED",
                AvsCvvResponseCode::SYSTEM_ERROR => "SYSTEM_ERROR",
                AvsCvvResponseCode::Other(ref s) => &s,
            })
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AvsCvvResponseCode {
        fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let s = <String>::deserialize(deserializer)?;
            match s.as_str() {
                "BYPASS" => Ok(AvsCvvResponseCode::BYPASS),
                "DOES_NOT_MATCH" => Ok(AvsCvvResponseCode::DOES_NOT_MATCH),
                "ISSUER_DOES_NOT_PARTICIPATE" => {
                    Ok(AvsCvvResponseCode::ISSUER_DOES_NOT_PARTICIPATE)
                }
                "MATCHES" => Ok(AvsCvvResponseCode::MATCHES),
                "NOT_APPLICABLE" => Ok(AvsCvvResponseCode::NOT_APPLICABLE),
                "NOT_PROVIDED" => Ok(AvsCvvResponseCode::NOT_PROVIDED),
                "NOT_VERIFIED" => Ok(AvsCvvResponseCode::NOT_VERIFIED),
                "SYSTEM_ERROR" => Ok(AvsCvvResponseCode::SYSTEM_ERROR),
                _ => Ok(AvsCvvResponseCode::Other(s)),
            }
        }
    }
    #[derive(Debug, Eq, PartialEq)]
    #[allow(non_camel_case_types)]
    pub enum PaymentMethodSnapshotSearchType {
        CREDIT_CARD,
        CREDIT_CARD_VIA_APPLE_PAY,
        CREDIT_CARD_VIA_GOOGLE_PAY,
        CREDIT_CARD_VIA_MASTERPASS,
        CREDIT_CARD_VIA_SAMSUNG_PAY,
        CREDIT_CARD_VIA_VISA_CHECKOUT,
        PAYPAL,
        US_BANK_ACCOUNT,
        VENMO_ACCOUNT,
        Other(String),
    }
    impl ::serde::Serialize for PaymentMethodSnapshotSearchType {
        fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
            ser.serialize_str(match *self {
                PaymentMethodSnapshotSearchType::CREDIT_CARD => "CREDIT_CARD",
                PaymentMethodSnapshotSearchType::CREDIT_CARD_VIA_APPLE_PAY => {
                    "CREDIT_CARD_VIA_APPLE_PAY"
                }
                PaymentMethodSnapshotSearchType::CREDIT_CARD_VIA_GOOGLE_PAY => {
                    "CREDIT_CARD_VIA_GOOGLE_PAY"
                }
                PaymentMethodSnapshotSearchType::CREDIT_CARD_VIA_MASTERPASS => {
                    "CREDIT_CARD_VIA_MASTERPASS"
                }
                PaymentMethodSnapshotSearchType::CREDIT_CARD_VIA_SAMSUNG_PAY => {
                    "CREDIT_CARD_VIA_SAMSUNG_PAY"
                }
                PaymentMethodSnapshotSearchType::CREDIT_CARD_VIA_VISA_CHECKOUT => {
                    "CREDIT_CARD_VIA_VISA_CHECKOUT"
                }
                PaymentMethodSnapshotSearchType::PAYPAL => "PAYPAL",
                PaymentMethodSnapshotSearchType::US_BANK_ACCOUNT => "US_BANK_ACCOUNT",
                PaymentMethodSnapshotSearchType::VENMO_ACCOUNT => "VENMO_ACCOUNT",
                PaymentMethodSnapshotSearchType::Other(ref s) => &s,
            })
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PaymentMethodSnapshotSearchType {
        fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let s = <String>::deserialize(deserializer)?;
            match s.as_str() {
                "CREDIT_CARD" => Ok(PaymentMethodSnapshotSearchType::CREDIT_CARD),
                "CREDIT_CARD_VIA_APPLE_PAY" => {
                    Ok(PaymentMethodSnapshotSearchType::CREDIT_CARD_VIA_APPLE_PAY)
                }
                "CREDIT_CARD_VIA_GOOGLE_PAY" => {
                    Ok(PaymentMethodSnapshotSearchType::CREDIT_CARD_VIA_GOOGLE_PAY)
                }
                "CREDIT_CARD_VIA_MASTERPASS" => {
                    Ok(PaymentMethodSnapshotSearchType::CREDIT_CARD_VIA_MASTERPASS)
                }
                "CREDIT_CARD_VIA_SAMSUNG_PAY" => {
                    Ok(PaymentMethodSnapshotSearchType::CREDIT_CARD_VIA_SAMSUNG_PAY)
                }
                "CREDIT_CARD_VIA_VISA_CHECKOUT" => {
                    Ok(PaymentMethodSnapshotSearchType::CREDIT_CARD_VIA_VISA_CHECKOUT)
                }
                "PAYPAL" => Ok(PaymentMethodSnapshotSearchType::PAYPAL),
                "US_BANK_ACCOUNT" => Ok(PaymentMethodSnapshotSearchType::US_BANK_ACCOUNT),
                "VENMO_ACCOUNT" => Ok(PaymentMethodSnapshotSearchType::VENMO_ACCOUNT),
                _ => Ok(PaymentMethodSnapshotSearchType::Other(s)),
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
    #[derive(Debug, Eq, PartialEq)]
    #[allow(non_camel_case_types)]
    pub enum RecurringType {
        FIRST,
        SUBSEQUENT,
        UNSCHEDULED,
        Other(String),
    }
    impl ::serde::Serialize for RecurringType {
        fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
            ser.serialize_str(match *self {
                RecurringType::FIRST => "FIRST",
                RecurringType::SUBSEQUENT => "SUBSEQUENT",
                RecurringType::UNSCHEDULED => "UNSCHEDULED",
                RecurringType::Other(ref s) => &s,
            })
        }
    }
    impl<'de> ::serde::Deserialize<'de> for RecurringType {
        fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let s = <String>::deserialize(deserializer)?;
            match s.as_str() {
                "FIRST" => Ok(RecurringType::FIRST),
                "SUBSEQUENT" => Ok(RecurringType::SUBSEQUENT),
                "UNSCHEDULED" => Ok(RecurringType::UNSCHEDULED),
                _ => Ok(RecurringType::Other(s)),
            }
        }
    }
    #[derive(Debug, Eq, PartialEq)]
    #[allow(non_camel_case_types)]
    pub enum RiskDecision {
        APPROVE,
        DECLINE,
        NOT_EVALUATED,
        REVIEW,
        Other(String),
    }
    impl ::serde::Serialize for RiskDecision {
        fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
            ser.serialize_str(match *self {
                RiskDecision::APPROVE => "APPROVE",
                RiskDecision::DECLINE => "DECLINE",
                RiskDecision::NOT_EVALUATED => "NOT_EVALUATED",
                RiskDecision::REVIEW => "REVIEW",
                RiskDecision::Other(ref s) => &s,
            })
        }
    }
    impl<'de> ::serde::Deserialize<'de> for RiskDecision {
        fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let s = <String>::deserialize(deserializer)?;
            match s.as_str() {
                "APPROVE" => Ok(RiskDecision::APPROVE),
                "DECLINE" => Ok(RiskDecision::DECLINE),
                "NOT_EVALUATED" => Ok(RiskDecision::NOT_EVALUATED),
                "REVIEW" => Ok(RiskDecision::REVIEW),
                _ => Ok(RiskDecision::Other(s)),
            }
        }
    }
    #[derive(Debug, Eq, PartialEq)]
    #[allow(non_camel_case_types)]
    pub enum TransactionLineItemType {
        CREDIT,
        DEBIT,
        Other(String),
    }
    impl ::serde::Serialize for TransactionLineItemType {
        fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
            ser.serialize_str(match *self {
                TransactionLineItemType::CREDIT => "CREDIT",
                TransactionLineItemType::DEBIT => "DEBIT",
                TransactionLineItemType::Other(ref s) => &s,
            })
        }
    }
    impl<'de> ::serde::Deserialize<'de> for TransactionLineItemType {
        fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let s = <String>::deserialize(deserializer)?;
            match s.as_str() {
                "CREDIT" => Ok(TransactionLineItemType::CREDIT),
                "DEBIT" => Ok(TransactionLineItemType::DEBIT),
                _ => Ok(TransactionLineItemType::Other(s)),
            }
        }
    }
    #[derive(Debug, Eq, PartialEq)]
    #[allow(non_camel_case_types)]
    pub enum TransactionStatus {
        #[doc = "The transaction spent too much time in the `AUTHORIZED` status and was marked\nas expired. Expiration [time frames](https://developers.braintreepayments.com/reference/general/statuses#authorization-expired)\ndiffer by card type, transaction type, and, in some cases, merchant category.\n"]
        AUTHORIZATION_EXPIRED,
        #[doc = "The processor authorized the transaction, putting your customer's funds on\nhold. Your customer may see a pending charge on his or her account. However,\nbefore the customer is actually charged and before you receive the funds, you\nmust use the `captureTransaction` mutation. If you do not want to capture the\ntransaction, you should use the `reverseTransaction` mutation to avoid a\nmisuse of authorization fee.\n"]
        AUTHORIZED,
        #[doc = "If a transaction remains in a status of `AUTHORIZING`, [contact Braintree\nSupport for assistance](https://help.braintreepayments.com).\n"]
        AUTHORIZING,
        #[doc = "An error occurred when sending the transaction to the downstream processor.\nSee the transaction's `statusHistory` for the exact error.\n"]
        FAILED,
        #[doc = "The transaction was [rejected](https://articles.braintreepayments.com/control-panel/transactions/gateway-rejections)\nbased on one or more settings or rules in your Braintree gateway. See the\ntransaction's `statusHistory` to determine which resulted in the decline.\n"]
        GATEWAY_REJECTED,
        #[doc = "The processor declined the transaction while attempting to authorize it. See\nthe transaction's `statusHistory` to determine what reason the processor gave\nfor the decline.\n"]
        PROCESSOR_DECLINED,
        #[doc = "The transaction has been settled, meaning your customer has been charged and\nthe process of disbursing the funds to your bank account will begin.\n"]
        SETTLED,
        SETTLEMENT_CONFIRMED,
        #[doc = "The processor declined the transaction while attempting to capture it. See the\ntransaction's `statusHistory` to detemine why it was not settled. This status\nis rare, and only certain types of transactions can be affected.\n"]
        SETTLEMENT_DECLINED,
        #[doc = "The transaction has not yet fully settled. This status is rare, and will\ngenerally resolve to a status of `SETTLED`. Only certain types of transactions\ncan be affected.\n"]
        SETTLEMENT_PENDING,
        #[doc = "The transaction is in the process of being settled. This is a transitory state, and will resolve to a status of `SETTLED`.\n"]
        SETTLING,
        #[doc = "The transaction has been successfully captured, and will be included in the\nnext settlement batch, at which time it will become settled.\n"]
        SUBMITTED_FOR_SETTLEMENT,
        #[doc = "The transaction was voided, meaning it is no longer authorized, your\ncustomer's funds are no longer on hold, and you cannot use the\n`captureTransaction` mutation on this transaction.\n"]
        VOIDED,
        Other(String),
    }
    impl ::serde::Serialize for TransactionStatus {
        fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
            ser.serialize_str(match *self {
                TransactionStatus::AUTHORIZATION_EXPIRED => "AUTHORIZATION_EXPIRED",
                TransactionStatus::AUTHORIZED => "AUTHORIZED",
                TransactionStatus::AUTHORIZING => "AUTHORIZING",
                TransactionStatus::FAILED => "FAILED",
                TransactionStatus::GATEWAY_REJECTED => "GATEWAY_REJECTED",
                TransactionStatus::PROCESSOR_DECLINED => "PROCESSOR_DECLINED",
                TransactionStatus::SETTLED => "SETTLED",
                TransactionStatus::SETTLEMENT_CONFIRMED => "SETTLEMENT_CONFIRMED",
                TransactionStatus::SETTLEMENT_DECLINED => "SETTLEMENT_DECLINED",
                TransactionStatus::SETTLEMENT_PENDING => "SETTLEMENT_PENDING",
                TransactionStatus::SETTLING => "SETTLING",
                TransactionStatus::SUBMITTED_FOR_SETTLEMENT => "SUBMITTED_FOR_SETTLEMENT",
                TransactionStatus::VOIDED => "VOIDED",
                TransactionStatus::Other(ref s) => &s,
            })
        }
    }
    impl<'de> ::serde::Deserialize<'de> for TransactionStatus {
        fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let s = <String>::deserialize(deserializer)?;
            match s.as_str() {
                "AUTHORIZATION_EXPIRED" => Ok(TransactionStatus::AUTHORIZATION_EXPIRED),
                "AUTHORIZED" => Ok(TransactionStatus::AUTHORIZED),
                "AUTHORIZING" => Ok(TransactionStatus::AUTHORIZING),
                "FAILED" => Ok(TransactionStatus::FAILED),
                "GATEWAY_REJECTED" => Ok(TransactionStatus::GATEWAY_REJECTED),
                "PROCESSOR_DECLINED" => Ok(TransactionStatus::PROCESSOR_DECLINED),
                "SETTLED" => Ok(TransactionStatus::SETTLED),
                "SETTLEMENT_CONFIRMED" => Ok(TransactionStatus::SETTLEMENT_CONFIRMED),
                "SETTLEMENT_DECLINED" => Ok(TransactionStatus::SETTLEMENT_DECLINED),
                "SETTLEMENT_PENDING" => Ok(TransactionStatus::SETTLEMENT_PENDING),
                "SETTLING" => Ok(TransactionStatus::SETTLING),
                "SUBMITTED_FOR_SETTLEMENT" => Ok(TransactionStatus::SUBMITTED_FOR_SETTLEMENT),
                "VOIDED" => Ok(TransactionStatus::VOIDED),
                _ => Ok(TransactionStatus::Other(s)),
            }
        }
    }
    #[derive(Debug, Eq, PartialEq)]
    #[allow(non_camel_case_types)]
    pub enum VerificationStatus {
        #[doc = "Indicates the verification was unsuccessful because of an issue communicating with the processor.\n"]
        FAILED,
        #[doc = "Indicates that the verification was unsuccessful because the payment method\nfailed one or more fraud checks. In this case, the `gatewayRejectionReason`\nwill indicate which fraud check failed.\n"]
        GATEWAY_REJECTED,
        #[doc = "Indicates that the verification is pending.\n"]
        PENDING,
        #[doc = "Indicates that the verification was unsuccessful based on the response from the processor.\n"]
        PROCESSOR_DECLINED,
        #[doc = "Indicates that the verification was successful.\n"]
        VERIFIED,
        #[doc = "Indicates that the verification is in the process of verifying.\n"]
        VERIFYING,
        Other(String),
    }
    impl ::serde::Serialize for VerificationStatus {
        fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
            ser.serialize_str(match *self {
                VerificationStatus::FAILED => "FAILED",
                VerificationStatus::GATEWAY_REJECTED => "GATEWAY_REJECTED",
                VerificationStatus::PENDING => "PENDING",
                VerificationStatus::PROCESSOR_DECLINED => "PROCESSOR_DECLINED",
                VerificationStatus::VERIFIED => "VERIFIED",
                VerificationStatus::VERIFYING => "VERIFYING",
                VerificationStatus::Other(ref s) => &s,
            })
        }
    }
    impl<'de> ::serde::Deserialize<'de> for VerificationStatus {
        fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let s = <String>::deserialize(deserializer)?;
            match s.as_str() {
                "FAILED" => Ok(VerificationStatus::FAILED),
                "GATEWAY_REJECTED" => Ok(VerificationStatus::GATEWAY_REJECTED),
                "PENDING" => Ok(VerificationStatus::PENDING),
                "PROCESSOR_DECLINED" => Ok(VerificationStatus::PROCESSOR_DECLINED),
                "VERIFIED" => Ok(VerificationStatus::VERIFIED),
                "VERIFYING" => Ok(VerificationStatus::VERIFYING),
                _ => Ok(VerificationStatus::Other(s)),
            }
        }
    }
    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    #[doc = "A monetary amount with currency.\n"]
    pub struct GetTransactionSearchTransactionsEdgesNodeAmount {
        #[doc = "The amount of money, either a whole number or a number with exactly 2 or 3 decimal places.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub value: Option<Amount>,
        #[doc = "The ISO code for the money's currency.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "currencyIsoCode")]
        pub currency_iso_code: Option<CurrencyCodeAlpha>,
    }
    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    #[doc = "Information about a customer and their associated payment methods and transactions.\n"]
    pub struct GetTransactionSearchTransactionsEdgesNodeCustomer {
        #[doc = "Email address for this customer.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub email: Option<String>,
        #[doc = "Unique identifier.\n"]
        pub id: ID,
    }
    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    #[doc = "Details about a credit card.\n"]
    pub struct GetTransactionSearchTransactionsEdgesNodePaymentMethodDetailsOnCreditCardDetails {
        #[doc = "The cardholder's name.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "cardholderName")]
        pub cardholder_name: Option<String>,
    }
    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    #[doc = "Details about a PayPal account.\n"]
    pub struct GetTransactionSearchTransactionsEdgesNodePaymentMethodDetailsOnPayPalAccountDetails {
        #[doc = "The email address associated with the PayPal account.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub email: Option<String>,
    }
    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    #[doc = "Details about a U.S. bank account.\n"]
    pub struct GetTransactionSearchTransactionsEdgesNodePaymentMethodDetailsOnUsBankAccountDetails {
        #[doc = "The name of the accountholder. This is either the business name for a\nbusiness account, or the owner's full name for an individual account.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "accountholderName")]
        pub accountholder_name: Option<String>,
    }
    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    #[doc = "Details about a Venmo Account.\n"]
    pub struct GetTransactionSearchTransactionsEdgesNodePaymentMethodDetailsOnVenmoAccountDetails {
        #[doc = "The Venmo username, as chosen by the user.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub username: Option<String>,
    }
    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    #[serde(tag = "__typename")]
    pub enum GetTransactionSearchTransactionsEdgesNodePaymentMethodDetails {
        CreditCardDetails(
            GetTransactionSearchTransactionsEdgesNodePaymentMethodDetailsOnCreditCardDetails,
        ),
        PayPalAccountDetails(
            GetTransactionSearchTransactionsEdgesNodePaymentMethodDetailsOnPayPalAccountDetails,
        ),
        UsBankAccountDetails(
            GetTransactionSearchTransactionsEdgesNodePaymentMethodDetailsOnUsBankAccountDetails,
        ),
        VenmoAccountDetails(
            GetTransactionSearchTransactionsEdgesNodePaymentMethodDetailsOnVenmoAccountDetails,
        ),
        CustomActionsPaymentMethodDetails,
        SamsungPayCardDetails,
    }
    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    #[doc = "Top-level field representing a payment method.\n"]
    pub struct GetTransactionSearchTransactionsEdgesNodePaymentMethod {
        #[doc = "Details about the payment method specific to the type (e.g. credit card, PayPal account).\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub details: Option<GetTransactionSearchTransactionsEdgesNodePaymentMethodDetails>,
    }
    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    #[doc = "A charge on a payment method.\n"]
    pub struct GetTransactionSearchTransactionsEdgesNode {
        #[doc = "Unique identifier.\n"]
        pub id: ID,
        #[doc = "The amount charged in this transaction. For transactions that are partially\ncaptured, this amount will be the cummulative amount captured on this transaction.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub amount: Option<GetTransactionSearchTransactionsEdgesNodeAmount>,
        #[doc = "Customer associated with the transaction, if applicable.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub customer: Option<GetTransactionSearchTransactionsEdgesNodeCustomer>,
        #[doc = "The multi-use payment method associated with the transaction. Only present if\na multi-use payment method was used to create the transaction and it has not\nbeen deleted. The details of this PaymentMethod may have changed since the\ntransaction was created, details used for the transaction can be found in the\n`paymentMethodSnapshot` field.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "paymentMethod")]
        pub payment_method: Option<GetTransactionSearchTransactionsEdgesNodePaymentMethod>,
        #[doc = "The order ID for this transaction. For PayPal transactions, the PayPal Invoice ID.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "orderId")]
        pub order_id: Option<String>,
        #[doc = "The current status of this transaction.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub status: Option<TransactionStatus>,
        #[doc = "Time at which the transaction was created.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "createdAt")]
        pub created_at: Option<Timestamp>,
    }
    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    #[doc = "A transaction within a TransactionConnection.\n"]
    pub struct GetTransactionSearchTransactionsEdges {
        #[doc = "The transaction.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub node: Option<GetTransactionSearchTransactionsEdgesNode>,
    }
    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    #[doc = "A paginated list of transactions.\n"]
    pub struct GetTransactionSearchTransactions {
        #[doc = "A list of transactions.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub edges: Option<Vec<Option<GetTransactionSearchTransactionsEdges>>>,
    }
    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    #[doc = "Top-level fields returned for a search query.\n"]
    pub struct GetTransactionSearch {
        #[doc = "A paginated list of transactions that match the TransactionSearchInput.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub transactions: Option<GetTransactionSearchTransactions>,
    }
    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    pub struct ResponseData {
        #[doc = "The available searches.\n"]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub search: Option<GetTransactionSearch>,
    }
    #[allow(dead_code)]
    #[derive(Serialize)]
    pub struct GetTransaction {
        #[serde(rename = "transactionID")]
        pub transaction_id: String,
    }
    impl crate::GraphQLQueryCLI for GetTransaction {
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
