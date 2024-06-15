use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

use super::{
    issuer::Sector,
    meta::DataType,
    politician::{Politician, PoliticianID},
    Chamber, IssuerID,
};

extern crate serde_json;

#[derive(Copy, Clone)]
pub enum TradeSize {
    Less1K = 1,
    From1Kto15K = 2,
    From15Kto50K = 3,
    From50Kto100K = 4,
    From100Kto250K = 5,
    From250Kto500K = 6,
    From500Kto1M = 7,
    From1Mto5M = 8,
    From5Mto25M = 9,
    From25Mto50M = 10,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Trade {
    #[serde(rename = "_txId")]
    pub tx_id: i64,

    #[serde(rename = "_politicianId")]
    pub politician_id: PoliticianID,

    #[serde(rename = "_assetId")]
    pub asset_id: i64,

    #[serde(rename = "_issuerId")]
    pub issuer_id: IssuerID,

    pub pub_date: DateTime<Utc>,

    pub filing_date: NaiveDate,

    pub tx_date: NaiveDate,

    pub tx_type: TxType,

    pub tx_type_extended: Option<serde_json::Value>,

    pub has_capital_gains: bool,

    pub owner: Owner,

    pub chamber: Chamber,

    pub price: Option<f64>,

    pub size: Option<i64>,

    pub size_range_high: Option<i64>,

    pub size_range_low: Option<i64>,

    pub value: i64,

    pub filing_id: i64,

    #[serde(rename = "filingURL")]
    pub filing_url: String,

    pub reporting_gap: i64,

    pub comment: Option<String>,

    pub committees: Vec<String>,

    pub asset: Asset,

    pub issuer: Issuer,

    pub politician: Politician,

    pub labels: Vec<String>,
}

impl From<DataType> for Trade {
    fn from(item: DataType) -> Self {
        match item {
            DataType::Trade(trade) => trade,
            _ => panic!("Expected DataItem::Trade"),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    pub asset_type: String,

    pub asset_ticker: Option<String>,

    pub instrument: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Issuer {
    #[serde(rename = "_stateId")]
    pub state_id: Option<String>,

    #[serde(rename = "c2iq")]
    pub c2_iq: Option<String>,

    pub country: Option<String>,

    pub issuer_name: String,

    pub issuer_ticker: Option<String>,

    pub sector: Option<Sector>,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "kebab-case")]
pub enum Owner {
    Child,
    Joint,
    NotDisclosed,
    #[serde(rename = "self")]
    OwnerSelf,
    Spouse,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "kebab-case")]
pub enum TxType {
    Buy,
    Sell,
    Exchange,
    Receive,
}
