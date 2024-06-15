use std::default;

use chrono::NaiveDate;
use scraper::{ElementRef, Selector};
use serde::{Deserialize, Serialize};

use crate::Error;

use super::trade::ExtractableItem;

pub type PoliticianID = String;

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Politician {
    #[serde(rename = "_stateId")]
    pub state_id: String,

    pub chamber: Chamber,

    pub dob: String,

    pub first_name: String,

    pub gender: Gender,

    pub last_name: String,

    pub nickname: Option<String>,

    pub party: Party,
}

impl ExtractableItem for PoliticianDetail {
    fn selector() -> Selector {
        Selector::parse("article.issuer-item").unwrap()
    }

    fn extract(item: ElementRef) -> Result<Self, Error> {
        Ok(PoliticianDetail {
            politician_id: "".to_string(),
            state_id: "".to_string(),
            party: Party::Democrat,
            party_other: None,
            district: None,
            first_name: "".to_string(),
            last_name: "".to_string(),
            nickname: None,
            middle_name: None,
            full_name: "".to_string(),
            dob: "".to_string(),
            gender: Gender::Male,
            social_facebook: None,
            social_twitter: None,
            social_youtube: None,
            website: None,
            chamber: Chamber::House,
            committees: vec![],
            stats: Stats::default(),
        })
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PoliticianDetail {
    #[serde(rename = "_politicianId")]
    pub politician_id: PoliticianID,

    #[serde(rename = "_stateId")]
    pub state_id: String,

    pub party: Party,

    pub party_other: Option<serde_json::Value>,

    pub district: Option<String>,

    pub first_name: String,

    pub last_name: String,

    pub nickname: Option<String>,

    pub middle_name: Option<String>,

    pub full_name: String,

    pub dob: String,

    pub gender: Gender,

    pub social_facebook: Option<String>,

    pub social_twitter: Option<String>,

    pub social_youtube: Option<String>,

    pub website: Option<String>,

    pub chamber: Chamber,

    pub committees: Vec<String>,

    pub stats: Stats,
}
impl Into<Politician> for PoliticianDetail {
    fn into(self) -> Politician {
        Politician {
            state_id: self.state_id,
            chamber: self.chamber,
            dob: self.dob,
            first_name: self.first_name,
            gender: self.gender,
            last_name: self.last_name,
            nickname: self.nickname,
            party: self.party,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    pub date_last_traded: Option<NaiveDate>,

    pub count_trades: i64,

    pub count_issuers: i64,

    pub volume: i64,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub enum Chamber {
    #[default]
    #[serde(rename = "house")]
    House,

    #[serde(rename = "senate")]
    Senate,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub enum Gender {
    #[default]
    #[serde(rename = "female")]
    Female,

    #[serde(rename = "male")]
    Male,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub enum Party {
    #[default]
    #[serde(rename = "democrat")]
    Democrat,

    #[serde(rename = "republican")]
    Republican,

    #[serde(rename = "other")]
    Other,
}

impl From<&str> for Party {
    fn from(s: &str) -> Self {
        match s {
            "democrat" => Party::Democrat,
            "republican" => Party::Republican,
            _ => Party::Other,
        }
    }
}
impl std::fmt::Display for Party {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Party::Democrat => "democrat",
                Party::Republican => "republican",
                Party::Other => "other",
            }
        )?;
        Ok(())
    }
}
