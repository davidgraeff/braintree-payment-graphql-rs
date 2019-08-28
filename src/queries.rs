use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[rustfmt::skip]
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum CurrencyCodeAlpha {
    AED, AMD, AOA, ARS, AUD, AWG, AZN, BAM, BBD, BDT, BGN, BIF, BMD, BND, BOB, BRL, BSD, BWP,
    BYN, BZD, CAD, CHF, CLP, CNY, COP, CRC, CVE, CZK, DJF, DKK, DOP, DZD, EGP, ETB, EUR, FJD,
    FKP, GBP, GEL, GHS, GIP, GMD, GNF, GTQ, GYD, HKD, HNL, HRK, HTG, HUF, IDR, ILS, INR, ISK,
    JMD, JPY, KES, KGS, KHR, KMF, KRW, KYD, KZT, LAK, LBP, LKR, LRD, LSL, LTL, MAD, MDL, MKD,
    MNT, MOP, MUR, MVR, MWK, MXN, MYR, MZN, NAD, NGN, NIO, NOK, NPR, NZD, PAB, PEN, PGK, PHP,
    PKR, PLN, PYG, QAR, RON, RSD, RUB, RWF, SAR, SBD, SCR, SEK, SGD, SHP, SLL, SOS, SRD, STD,
    SVC, SYP, SZL, THB, TJS, TOP, TRY, TTD, TWD, TZS, UAH, UGX, USD, UYU, UZS, VES, VND, VUV,
    WST, XAF, XCD, XOF, XPF, YER, ZAR, ZMK, ZWD,
}

#[rustfmt::skip]
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum CountryCodeAlpha3 {
    AFG, ALA, ALB, DZA, ASM, AND, AGO, AIA, ATA, ATG, ARG, ARM, ABW, AUS, AUT, AZE, BHS, BHR,
    BGD, BRB, BLR, BEL, BLZ, BEN, BMU, BTN, BOL, BES, BIH, BWA, BVT, BRA, IOT, BRN, BGR, BFA,
    BDI, KHM, CMR, CAN, CPV, CYM, CAF, TCD, CHL, CHN, CXR, CCK, COL, COM, COG, COD, COK, CRI,
    CIV, HRV, CUB, CUW, CYP, CZE, DNK, DJI, DMA, DOM, ECU, EGY, SLV, GNQ, ERI, EST, ETH, FLK,
    FRO, FJI, FIN, FRA, GUF, PYF, ATF, GAB, GMB, GEO, DEU, GHA, GIB, GRC, GRL, GRD, GLP, GUM,
    GTM, GGY, GIN, GNB, GUY, HTI, HMD, HND, HKG, HUN, ISL, IND, IDN, IRN, IRQ, IRL, IMN, ISR,
    ITA, JAM, JPN, JEY, JOR, KAZ, KEN, KIR, PRK, KOR, KWT, KGZ, LAO, LVA, LBN, LSO, LBR, LBY,
    LIE, LTU, LUX, MAC, MKD, MDG, MWI, MYS, MDV, MLI, MLT, MHL, MTQ, MRT, MUS, MYT, MEX, FSM,
    MDA, MCO, MNG, MNE, MSR, MAR, MOZ, MMR, NAM, NRU, NPL, NLD, NCL, NZL, NIC, NER, NGA, NIU,
    NFK, MNP, NOR, OMN, PAK, PLW, PSE, PAN, PNG, PRY, PER, PHL, PCN, POL, PRT, PRI, QAT, REU,
    ROU, RUS, RWA, BLM, SHN, KNA, LCA, MAF, SPM, VCT, WSM, SMR, STP, SAU, SEN, SRB, SYC, SLE,
    SGP, SXM, SVK, SVN, SLB, SOM, ZAF, SGS, SSD, ESP, LKA, SDN, SUR, SJM, SWZ, SWE, CHE, SYR,
    TWN, TJK, TZA, THA, TLS, TGO, TKL, TON, TTO, TUN, TUR, TKM, TCA, TUV, UGA, UKR, ARE, GBR,
    UMI, USA, URY, UZB, VUT, VAT, VEN, VNM, VGB, VIR, WLF, ESH, YEM, ZMB, ZWE,
}

pub type Timestamp = chrono::DateTime<chrono::Utc>;
pub type Amount = Decimal;
pub type CustomFieldName = String;

pub mod common;
pub mod customer;
pub mod transactions;
