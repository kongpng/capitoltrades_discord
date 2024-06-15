use chrono::{DateTime, NaiveDate};
use scraper::{Html, Selector};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::Error;

use super::issuer::Stats;
use super::politician::{self};
use super::trade::TxType;
use super::{issuer, Chamber, IssuerDetail, PoliticianDetail, Trade};
use crate::types::trade::Issuer;
use crate::types::trade::Owner;
enum ExtractedData {
    Trade(Vec<Trade>),
    PoliticianDetail(Vec<PoliticianDetail>),
    IssuerDetail(Vec<IssuerDetail>),
}

pub enum DataType {
    Trade(Trade),
    PoliticianDetail(PoliticianDetail),
    IssuerDetail(IssuerDetail),
}
#[derive(Serialize, Deserialize, Default)]
pub struct Meta {
    paging: Paging,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Paging {
    page: i64,
    size: i64,
    total_items: i64,
    total_pages: i64,
}

pub trait Extractable {
    fn extract_data(document: &Html, datatype: DataType) -> Result<Self, Error>
    where
        Self: Sized + DeserializeOwned;
}

impl<T> Extractable for PaginatedResponse<T>
where
    T: DeserializeOwned,
{
    fn extract_data(document: &Html, data_type: DataType) -> Result<Self, Error> {
        let data_items: Vec<T> = match data_type {
            DataType::Trade(trade) => {
                let trade_selector = Selector::parse("article.trade-item").unwrap();
                document
                    .select(&trade_selector)
                    .map(|trade_item| {
                        let id = trade_item
                            .value()
                            .attr("data-trade-id")
                            .unwrap_or_default()
                            .to_string();
                        let ticker = trade_item
                            .select(&Selector::parse("span.trade-ticker").unwrap())
                            .next()
                            .map(|t| t.text().collect::<Vec<_>>().join(" "))
                            .unwrap();

                        let amount = trade_item
                            .select(&Selector::parse("span.trade-amount").unwrap())
                            .next()
                            .map(|a| {
                                a.text()
                                    .collect::<String>()
                                    .parse::<i64>()
                                    .unwrap_or_default()
                            })
                            .unwrap_or_default();

                        DataType::Trade(Trade {
                            tx_id: id.parse().unwrap_or_default(),
                            politician_id: trade.politician_id.clone(),
                            asset_id: trade.asset_id,
                            issuer_id: trade.issuer_id,
                            pub_date: trade.pub_date,
                            filing_date: trade.filing_date,
                            tx_date: trade.tx_date,
                            tx_type: trade.tx_type,
                            tx_type_extended: trade.tx_type_extended.clone(),
                            has_capital_gains: trade.has_capital_gains,
                            owner: trade.owner,
                            chamber: trade.chamber.clone(),
                            price: trade.price,
                            size: trade.size,
                            size_range_high: trade.size_range_high,
                            size_range_low: trade.size_range_low,
                            value: amount,
                            filing_id: trade.filing_id,
                            filing_url: trade.filing_url.clone(),
                            reporting_gap: trade.reporting_gap,
                            comment: trade.comment.clone(),
                            committees: trade.committees.clone(),
                            asset: trade.asset.clone(),
                            issuer: trade.issuer.clone(),
                            politician: trade.politician.clone(),
                            labels: trade.labels.clone(),
                        });
                        serde_json::from_value(serde_json::to_value(trade.clone()).unwrap())
                            .unwrap()
                    })
                    .collect()
            }
            DataType::PoliticianDetail(politician) => {
                let politician_selector = Selector::parse("article.politician-item").unwrap();
                document
                    .select(&politician_selector)
                    .map(|politician_item| {
                        let id = politician_item
                            .value()
                            .attr("data-politician-id")
                            .unwrap_or_default()
                            .to_string();
                        let name = politician_item
                            .select(&Selector::parse("h2.politician-name").unwrap())
                            .next()
                            .map(|n| n.text().collect())
                            .unwrap_or_default();
                        let party = politician_item
                            .select(&Selector::parse("span.politician-party").unwrap())
                            .next()
                            .map(|p| p.text().collect::<String>())
                            .unwrap_or_default();
                        // ... extract other politician fields

                        DataType::PoliticianDetail(PoliticianDetail {
                            politician_id: id,
                            state_id: politician.state_id.clone(),
                            party: super::Party::from(party.as_str()),
                            party_other: politician.party_other.clone(),
                            district: politician.district.clone(),
                            first_name: name,
                            last_name: politician.last_name.clone(),
                            nickname: politician.nickname.clone(),
                            middle_name: politician.middle_name.clone(),
                            full_name: politician.full_name.clone(),
                            dob: politician.dob.clone(),
                            gender: politician.gender.clone(),
                            social_facebook: politician.social_facebook.clone(),
                            social_twitter: politician.social_twitter.clone(),
                            social_youtube: politician.social_youtube.clone(),
                            website: politician.website.clone(),
                            chamber: politician.chamber.clone(),
                            committees: politician.committees.clone(),
                            stats: politician.stats.clone(),
                        });
                        serde_json::from_value(serde_json::to_value(politician.clone()).unwrap())
                            .unwrap()
                    })
                    .collect()
            }
            DataType::IssuerDetail(issuer) => {
                // Extraction logic for IssuerDetail
                let issuer_selector = Selector::parse("article.issuer-item").unwrap();
                document
                    .select(&issuer_selector)
                    .map(|issuer_item| {
                        // Extract the relevant data from each issuer item using `scraper` selectors
                        let id = issuer_item
                            .value()
                            .attr("data-issuer-id")
                            .unwrap_or_default()
                            .parse()
                            .unwrap_or_default();
                        let name = issuer_item
                            .select(&Selector::parse("h2.issuer-name").unwrap())
                            .next()
                            .map(|n| n.text().collect())
                            .unwrap_or_default();
                        let ticker = issuer_item
                            .select(&Selector::parse("span.issuer-ticker").unwrap())
                            .next()
                            .map(|t| t.text().collect())
                            .unwrap_or_default();
                        // ... extract other issuer fields

                        let issuer_detail = IssuerDetail {
                            issuer_id: id,
                            state_id: issuer.state_id.clone(),
                            c2_iq: issuer.c2_iq.clone(),
                            country: issuer.country.clone(),
                            issuer_name: name,
                            issuer_ticker: Some(ticker),
                            performance: issuer.performance.clone(),
                            sector: issuer.sector,
                            stats: Stats {
                                count_trades: 0,
                                volume: 0,
                                count_politicians: 0,
                                date_last_traded: NaiveDate::from_ymd_opt(2000, 1, 1).unwrap(),
                            },
                        };

                        serde_json::from_value(serde_json::to_value(&issuer_detail).unwrap())
                            .unwrap()
                    })
                    .collect()
            }
        };

        let meta_selector = Selector::parse("div.pagination-meta").unwrap();
        let meta_json = document
            .select(&meta_selector)
            .next()
            .map(|meta| meta.text().collect::<String>())
            .unwrap_or_default();
        let meta: super::Meta = serde_json::from_str(&meta_json).unwrap_or_default();

        Ok(PaginatedResponse {
            meta,
            data: data_items,
        })
    }
}
#[derive(Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    meta: Meta,
    pub data: Vec<T>,
}

#[derive(Serialize, Deserialize)]
pub struct Response<T> {
    pub data: T,
}
