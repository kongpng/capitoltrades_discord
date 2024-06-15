use std::clone;

use chrono::NaiveDate;
use scraper::{ElementRef, Html, Selector};
use serde::{Deserialize, Serialize};

use crate::Error;

use super::{meta::Extractable, trade::ExtractableItem, Response};

extern crate serde_json;

pub type IssuerID = i64;

impl ExtractableItem for IssuerDetail {
    fn selector() -> Selector {
        Selector::parse("article.issuer-item").unwrap()
    }

    fn extract(item: ElementRef) -> Result<Self, Error> {
        let id = item
            .value()
            .attr("data-issuer-id")
            .unwrap_or_default()
            .parse()
            .unwrap_or_default();
        let name = item
            .select(&Selector::parse("h2.issuer-name").unwrap())
            .next()
            .map(|n| n.text().collect())
            .unwrap_or_default();
        let ticker = item
            .select(&Selector::parse("span.issuer-ticker").unwrap())
            .next()
            .map(|t| t.text().collect())
            .unwrap_or_default();

        Ok(IssuerDetail {
            issuer_id: id,
            state_id: None,
            c2_iq: None,
            country: None,
            issuer_name: name,
            issuer_ticker: Some(ticker),
            performance: None,
            sector: None,
            stats: Stats {
                count_trades: 0,
                volume: 0,
                count_politicians: 0,
                date_last_traded: NaiveDate::from_ymd_opt(2000, 1, 1).unwrap(),
            },
        })
    }
}

impl Extractable for Response<IssuerDetail> {
    fn extract_data(document: &Html) -> Result<Self, Error> {
        let issuer_detail =
            IssuerDetail::extract(document.select(&IssuerDetail::selector()).next().unwrap())?;

        Ok(Response {
            data: issuer_detail,
        })
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IssuerDetail {
    #[serde(rename = "_issuerId")]
    pub issuer_id: IssuerID,

    #[serde(rename = "_stateId")]
    pub state_id: Option<String>,

    #[serde(rename = "c2iq")]
    pub c2_iq: Option<String>,

    pub country: Option<String>,

    pub issuer_name: String,

    pub issuer_ticker: Option<String>,

    pub performance: Option<Performance>,

    pub sector: Option<Sector>,

    pub stats: Stats,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Performance {
    pub eod_prices: Vec<Vec<EodPrice>>,

    pub mcap: i64,

    pub trailing1: f64,

    pub trailing1_change: f64,

    pub trailing7: f64,

    pub trailing7_change: f64,

    pub trailing30: f64,

    pub trailing30_change: f64,

    pub trailing90: f64,

    pub trailing90_change: f64,

    pub trailing365: f64,

    pub trailing365_change: f64,

    pub wtd: f64,

    pub wtd_change: f64,

    pub mtd: f64,

    pub mtd_change: f64,

    pub qtd: f64,

    pub qtd_change: f64,

    pub ytd: f64,

    pub ytd_change: f64,
}

impl Performance {
    pub fn last_price(&self) -> Option<f64> {
        EodPrice::last_price_from_vec(&self.eod_prices)
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Stats {
    #[serde(rename = "countTrades")]
    pub count_trades: i64,

    #[serde(rename = "countPoliticians")]
    pub count_politicians: i64,

    #[serde(rename = "volume")]
    pub volume: i64,

    #[serde(rename = "dateLastTraded")]
    pub date_last_traded: NaiveDate,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum EodPrice {
    Double(f64),

    NaiveDate(NaiveDate),
}
impl EodPrice {
    pub fn last_price_from_vec(v: &[Vec<EodPrice>]) -> Option<f64> {
        if v.is_empty() {
            return None;
        }
        for item in v.get(0).unwrap() {
            match item {
                EodPrice::Double(price) => return Some(*price),
                _ => continue,
            }
        }
        None
    }
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum MarketCap {
    /// >200B
    Mega = 1,
    /// 10B-200B
    Large = 2,
    /// 2B-10B
    Mid = 3,
    /// 300M-2B
    Small = 4,
    /// 50M-300M
    Micro = 5,
    /// <50M
    Nano = 6,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum Sector {
    CommunicationServices,
    ConsumerDiscretionary,
    ConsumerStaples,
    Energy,
    Financials,
    HealthCare,
    Industrials,
    InformationTechnology,
    Materials,
    RealEstate,
    Utilities,
    Other,
}
impl std::fmt::Display for Sector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Sector::CommunicationServices => "communication-services",
                Sector::ConsumerDiscretionary => "consumer-discretionary",
                Sector::ConsumerStaples => "consumer-staples",
                Sector::Energy => "energy",
                Sector::Financials => "financials",
                Sector::HealthCare => "health-care",
                Sector::Industrials => "industrials",
                Sector::InformationTechnology => "information-technology",
                Sector::Materials => "materials",
                Sector::RealEstate => "real-estate",
                Sector::Utilities => "utilities",
                Sector::Other => "other",
            }
        )?;
        Ok(())
    }
}
