use chrono::{naive, DateTime, NaiveDate, Utc};
use scraper::{ElementRef, Selector};
use serde::{Deserialize, Serialize};

use crate::Error;

use super::{
    issuer::Sector,
    politician::{Politician, PoliticianID},
    Chamber, IssuerID,
};

extern crate serde_json;

pub trait ExtractableItem {
    fn selector() -> Selector;
    fn extract(item: ElementRef) -> Result<Self, Error>
    where
        Self: Sized;
}

#[derive(Copy, Clone, Deserialize, Serialize)]
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

impl ExtractableItem for Trade {
    fn selector() -> Selector {
        if let Some(selector) = Selector::parse(".q-table").ok() {
            println!("Found selector");
            selector
        } else {
            panic!("no");
        }
    }

    fn extract(item: ElementRef) -> Result<Self, Error> {
        println!("yes");
        let tx_id: i64 = item
            .select(&Selector::parse(".q-table > thead:nth-child(1) > tr:nth-child(1)").unwrap())
            .next()
            .and_then(|link: ElementRef| link.value().attr("href"))
            .and_then(|href| href.split('/').last())
            .and_then(|id| id.parse().ok())
            .unwrap_or_default();

        let politician_id = item
            .select(&Selector::parse("h3.q-fieldset.politician-name").unwrap())
            .next()
            .map(|t| t.text().collect::<String>())
            .unwrap_or_default();

        let ticker = item
            .select(&Selector::parse("span.q-field.issuer-ticker").unwrap())
            .next()
            .map(|t| t.text().collect::<String>())
            .unwrap_or_default();

        let amount = item
            .select(&Selector::parse("span.q-field.trade-size").unwrap())
            .next()
            .map(|a| {
                a.text()
                    .collect::<String>()
                    .parse::<i64>()
                    .unwrap_or_default()
            })
            .unwrap_or_default();

        print!("{:?}  cock", tx_id);
        Ok(Trade {
            tx_id,
            value: amount,
            politician_id,
            asset_id: 0,
            issuer_id: 0,
            pub_date: DateTime::from_timestamp(0, 0).unwrap(),
            filing_date: NaiveDate::from_ymd_opt(2000, 1, 1).unwrap(),
            tx_date: NaiveDate::from_ymd_opt(2000, 1, 1).unwrap(),
            tx_type: TxType::Buy,
            tx_type_extended: None,
            has_capital_gains: false,
            owner: Owner::Child,
            chamber: Chamber::House,
            price: None,
            size: None,
            size_range_high: None,
            size_range_low: None,
            filing_id: 0,
            filing_url: 0.to_string(),
            reporting_gap: 0,
            comment: None,
            committees: vec![],
            asset: Asset::default(),
            issuer: Issuer::default(),
            politician: Politician::default(),
            labels: vec![],
        })
    }
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
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

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    pub asset_type: String,

    pub asset_ticker: Option<String>,

    pub instrument: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
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

#[derive(Serialize, Deserialize, Default, Clone, Copy, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum Owner {
    #[default]
    Child,
    Joint,
    NotDisclosed,
    #[serde(rename = "self")]
    OwnerSelf,
    Spouse,
}

#[derive(Serialize, Deserialize, Clone, Copy, Default, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum TxType {
    #[default]
    Buy,
    Sell,
    Exchange,
    Receive,
}
