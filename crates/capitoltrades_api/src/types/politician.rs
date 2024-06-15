use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use super::meta::DataType;

pub type PoliticianID = String;

#[derive(Serialize, Deserialize, Clone)]
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

impl From<DataType> for PoliticianDetail {
    fn from(item: DataType) -> Self {
        match item {
            DataType::PoliticianDetail(politician) => politician,
            _ => panic!("Expected DataItem::PoliticianDetail"),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    pub date_last_traded: Option<NaiveDate>,

    pub count_trades: i64,

    pub count_issuers: i64,

    pub volume: i64,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Chamber {
    #[serde(rename = "house")]
    House,

    #[serde(rename = "senate")]
    Senate,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Gender {
    #[serde(rename = "female")]
    Female,

    #[serde(rename = "male")]
    Male,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Party {
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
