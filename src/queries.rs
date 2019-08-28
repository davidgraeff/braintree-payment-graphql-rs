use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[rustfmt::skip]
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum CurrencyCodeAlpha {
    AED,
    AMD,
    AOA,
    ARS,
    AUD,
    AWG,
    AZN,
    BAM,
    BBD,
    BDT,
    BGN,
    BIF,
    BMD,
    BND,
    BOB,
    BRL,
    BSD,
    BWP,
    BYN,
    BZD,
    CAD,
    CHF,
    CLP,
    CNY,
    COP,
    CRC,
    CVE,
    CZK,
    DJF,
    DKK,
    DOP,
    DZD,
    EGP,
    ETB,
    EUR,
    FJD,
    FKP,
    GBP,
    GEL,
    GHS,
    GIP,
    GMD,
    GNF,
    GTQ,
    GYD,
    HKD,
    HNL,
    HRK,
    HTG,
    HUF,
    IDR,
    ILS,
    INR,
    ISK,
    JMD,
    JPY,
    KES,
    KGS,
    KHR,
    KMF,
    KRW,
    KYD,
    KZT,
    LAK,
    LBP,
    LKR,
    LRD,
    LSL,
    LTL,
    MAD,
    MDL,
    MKD,
    MNT,
    MOP,
    MUR,
    MVR,
    MWK,
    MXN,
    MYR,
    MZN,
    NAD,
    NGN,
    NIO,
    NOK,
    NPR,
    NZD,
    PAB,
    PEN,
    PGK,
    PHP,
    PKR,
    PLN,
    PYG,
    QAR,
    RON,
    RSD,
    RUB,
    RWF,
    SAR,
    SBD,
    SCR,
    SEK,
    SGD,
    SHP,
    SLL,
    SOS,
    SRD,
    STD,
    SVC,
    SYP,
    SZL,
    THB,
    TJS,
    TOP,
    TRY,
    TTD,
    TWD,
    TZS,
    UAH,
    UGX,
    USD,
    UYU,
    UZS,
    VES,
    VND,
    VUV,
    WST,
    XAF,
    XCD,
    XOF,
    XPF,
    YER,
    ZAR,
    ZMK,
    ZWD,
}

#[rustfmt::skip]
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum CountryCodeAlpha3 {
    AFG,
    ALA,
    ALB,
    DZA,
    ASM,
    AND,
    AGO,
    AIA,
    ATA,
    ATG,
    ARG,
    ARM,
    ABW,
    AUS,
    AUT,
    AZE,
    BHS,
    BHR,
    BGD,
    BRB,
    BLR,
    BEL,
    BLZ,
    BEN,
    BMU,
    BTN,
    BOL,
    BES,
    BIH,
    BWA,
    BVT,
    BRA,
    IOT,
    BRN,
    BGR,
    BFA,
    BDI,
    KHM,
    CMR,
    CAN,
    CPV,
    CYM,
    CAF,
    TCD,
    CHL,
    CHN,
    CXR,
    CCK,
    COL,
    COM,
    COG,
    COD,
    COK,
    CRI,
    CIV,
    HRV,
    CUB,
    CUW,
    CYP,
    CZE,
    DNK,
    DJI,
    DMA,
    DOM,
    ECU,
    EGY,
    SLV,
    GNQ,
    ERI,
    EST,
    ETH,
    FLK,
    FRO,
    FJI,
    FIN,
    FRA,
    GUF,
    PYF,
    ATF,
    GAB,
    GMB,
    GEO,
    DEU,
    GHA,
    GIB,
    GRC,
    GRL,
    GRD,
    GLP,
    GUM,
    GTM,
    GGY,
    GIN,
    GNB,
    GUY,
    HTI,
    HMD,
    HND,
    HKG,
    HUN,
    ISL,
    IND,
    IDN,
    IRN,
    IRQ,
    IRL,
    IMN,
    ISR,
    ITA,
    JAM,
    JPN,
    JEY,
    JOR,
    KAZ,
    KEN,
    KIR,
    PRK,
    KOR,
    KWT,
    KGZ,
    LAO,
    LVA,
    LBN,
    LSO,
    LBR,
    LBY,
    LIE,
    LTU,
    LUX,
    MAC,
    MKD,
    MDG,
    MWI,
    MYS,
    MDV,
    MLI,
    MLT,
    MHL,
    MTQ,
    MRT,
    MUS,
    MYT,
    MEX,
    FSM,
    MDA,
    MCO,
    MNG,
    MNE,
    MSR,
    MAR,
    MOZ,
    MMR,
    NAM,
    NRU,
    NPL,
    NLD,
    NCL,
    NZL,
    NIC,
    NER,
    NGA,
    NIU,
    NFK,
    MNP,
    NOR,
    OMN,
    PAK,
    PLW,
    PSE,
    PAN,
    PNG,
    PRY,
    PER,
    PHL,
    PCN,
    POL,
    PRT,
    PRI,
    QAT,
    REU,
    ROU,
    RUS,
    RWA,
    BLM,
    SHN,
    KNA,
    LCA,
    MAF,
    SPM,
    VCT,
    WSM,
    SMR,
    STP,
    SAU,
    SEN,
    SRB,
    SYC,
    SLE,
    SGP,
    SXM,
    SVK,
    SVN,
    SLB,
    SOM,
    ZAF,
    SGS,
    SSD,
    ESP,
    LKA,
    SDN,
    SUR,
    SJM,
    SWZ,
    SWE,
    CHE,
    SYR,
    TWN,
    TJK,
    TZA,
    THA,
    TLS,
    TGO,
    TKL,
    TON,
    TTO,
    TUN,
    TUR,
    TKM,
    TCA,
    TUV,
    UGA,
    UKR,
    ARE,
    GBR,
    UMI,
    USA,
    URY,
    UZB,
    VUT,
    VAT,
    VEN,
    VNM,
    VGB,
    VIR,
    WLF,
    ESH,
    YEM,
    ZMB,
    ZWE,
}

pub type Timestamp = chrono::DateTime<chrono::Utc>;
pub type Amount = Decimal;
pub type CustomFieldName = String;

pub mod common;
pub mod customer;
pub mod transactions;

pub mod customer_helpers {
    use crate::queries::customer::get_customer::*;

    pub fn unwrap_customer(r:ResponseData) -> Option<GetCustomerNodeOnCustomer> {
        r.node.and_then(|f|
            match f.on {
                GetCustomerNodeOn::Customer(c) => Some(c),
                _ => None
            }
        )
    }
}

pub mod transaction_helpers {
    use failure::*;
    use crate::Braintree;
    use crate::queries::transactions::*;

    use charge_payment_method::ChargePaymentMethodChargePaymentMethodTransaction;
    use get_transaction::GetTransactionSearchTransactionsEdgesNode;
    use search_transaction::SearchTransactionSearchTransactionsEdgesNode;
    use vault_payment::VaultPaymentVaultPaymentMethodPaymentMethod;
    use verify_payment_method::VerifyPaymentMethodVerifyPaymentMethodVerification;

    pub fn vault(
        bt: &Braintree,
        payment_method_id: &str,
    ) -> Result<VaultPaymentVaultPaymentMethodPaymentMethod, failure::Error> {
        use crate::queries::transactions::vault_payment::*;

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

    pub fn verify(
        bt: &Braintree,
        payment_method_id: &str,
    ) -> Result<VerifyPaymentMethodVerifyPaymentMethodVerification, failure::Error> {
        use crate::queries::transactions::verify_payment_method::*;

        let r = bt
            .perform(VerifyPaymentMethod {
                input: VerifyPaymentMethodInput {
                    ..VerifyPaymentMethodInput::new(payment_method_id.to_owned())
                },
            })?
            .verify_payment_method
            .and_then(|f| f.verification)
            .ok_or(err_msg("Expected the deleted transaction"))?;

        Ok(r)
    }

    pub fn delete_transaction(bt: &Braintree, payment_method_id: &str) -> Result<(), failure::Error> {
        use crate::queries::transactions::delete_vaulted_payment::*;

        let _ = bt.perform(DeleteVaultedPayment {
            input: DeletePaymentMethodFromVaultInput {
                ..DeletePaymentMethodFromVaultInput::new(payment_method_id.to_owned())
            },
        })?;

        Ok(())
    }

    #[derive(Copy, Clone, Eq, PartialEq, Debug, Fail)]
    pub enum BrainTreeErrorKind {
        #[fail(display = "Too many results for the unique transaction ID")]
        TooManyResults,
    }


    pub fn unwrap_get_result(r: crate::queries::transactions::get_transaction::ResponseData) -> Result<Option<GetTransactionSearchTransactionsEdgesNode>, failure::Error> {
        let mut r = r.search
            .and_then(|f| f.transactions)
            .and_then(|f| f.edges)
            .ok_or(err_msg("Expected a paginated search result"))?;

        match r.len() {
            0 => {
                return Ok(None);
            }
            1 => {
                return Ok(r.pop().and_then(|f| f.and_then(|f| f.node)));
            }
            _ => {
                Err(BrainTreeErrorKind::TooManyResults)?
            }
        }
    }

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

    pub fn unwrap_search_result(r: crate::queries::transactions::search_transaction::ResponseData) -> Result<Vec<SearchTransactionSearchTransactionsEdgesNode>, failure::Error> {
        let r = r.search
            .and_then(|f| f.transactions)
            .and_then(|f| f.edges)
            .ok_or(err_msg("Expected a paginated search result"))?;

        let mut result: Vec<SearchTransactionSearchTransactionsEdgesNode> = Vec::new();

        r.into_iter().for_each(|f| {
            if let Some(g) = f {
                if let Some(h) = g.node {
                    result.push(h);
                }
            }
        });

        return Ok(result);
    }

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

    pub fn payment(
        bt: &Braintree,
        payment_method_id: &str,
        amount: rust_decimal::Decimal,
        recurring: bool,
        order_id: Option<String>,
    ) -> Result<ChargePaymentMethodChargePaymentMethodTransaction, failure::Error> {
        use crate::queries::transactions::charge_payment_method::*;

        let recurring = match recurring {
            true => Some(RecurringType::FIRST),
            false => None,
        };

        let response = bt.perform(ChargePaymentMethod {
            payment_method_id: payment_method_id.to_owned(),
            transaction: TransactionInput {
                order_id,
                purchase_order_number: Some("demo_id".to_owned()),
                recurring,
                ..TransactionInput::new(amount)
            },
            client_mutation_id: None,
        })?.charge_payment_method
            .and_then(|f| f.transaction)
            .ok_or(err_msg("Expected a payment result"))?;

        Ok(response)
    }
}