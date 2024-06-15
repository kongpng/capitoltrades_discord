use chrono::{DateTime, NaiveDate};
use scraper::{Html, Selector};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{Error, Query};

use super::issuer::Stats;
use super::politician::{self};
use super::trade::{ExtractableItem, TxType};
use super::{issuer, Chamber, IssuerDetail, PoliticianDetail};
use crate::types::trade::Issuer;
use crate::types::trade::Owner;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Meta {
    paging: Paging,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Paging {
    page: i64,
    size: i64,
    total_items: i64,
    total_pages: i64,
}

pub trait Extractable {
    fn extract_data(document: &Html) -> Result<Self, Error>
    where
        Self: Sized + DeserializeOwned;
}

impl<T> Extractable for PaginatedResponse<T>
where
    T: DeserializeOwned + ExtractableItem,
{
    fn extract_data(document: &Html) -> Result<Self, Error> {
        // println!("HTML: {}", document.html());

        let mut data_items: Vec<T> = Vec::new();
        for item in document.select(&T::selector()) {
            println!("Extracting item");
            match T::extract(item) {
                Ok(trade) => data_items.push(trade),
                Err(e) => println!("Error extracting trade: {:?}", e),
            }
        }

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
#[derive(Serialize, Deserialize, Debug)]
pub struct PaginatedResponse<T> {
    meta: Meta,
    pub data: Vec<T>,
}

#[derive(Serialize, Deserialize)]
pub struct Response<T> {
    pub data: T,
}
