use scraper::{Html, Selector};
use serde::de::DeserializeOwned;
use url::Url;

use crate::{
    query::{IssuerQuery, PoliticianQuery, Query, TradeQuery},
    types::{
        meta::Extractable, trade::Trade, IssuerDetail, PaginatedResponse, PoliticianDetail,
        Response,
    },
    user_agent::get_user_agent,
    Error,
};

pub struct Client {
    base_api_url: &'static str,
}

impl Client {
    pub fn new() -> Self {
        Self {
            base_api_url: "https://capitoltrades.com",
        }
    }

    fn get_url(&self, path: &str, query: Option<&impl Query>) -> Url {
        let mut url = Url::parse(format!("{}{}", &self.base_api_url, path).as_str()).unwrap();
        match query {
            Some(query) => query.add_to_url(&mut url),
            None => url,
        }
    }

    async fn fetch_html<Q>(&self, path: &str, query: Option<&Q>) -> Result<String, Error>
    where
        Q: Query,
    {
        let url = self.get_url(path, query);
        let client = reqwest::Client::builder()
            .user_agent(get_user_agent())
            .build()
            .map_err(|e| {
                tracing::error!("Failed to build HTTP client: {}", e);
                Error::RequestFailed
            })?;
        let html = client
            .get(url)
            .header("content-type", "application/json")
            .header("origin", "https://www.capitoltrades.com")
            .header("referer", "https://www.capitoltrades.com")
            .header("accept", "*/*")
            .header("accept-language", "en-US,en;q=0.9")
            .header("sec-fetch-dest", "empty")
            .header("sec-fetch-mode", "cors")
            .header("sec-fetch-site", "same-site")
            .send()
            .await
            .map_err(|e| {
                tracing::error!("Failed to get resource: {}", e);
                Error::RequestFailed
            })?
            .text()
            .await
            .map_err(|e| {
                tracing::error!("Failed to parse response: {}", e);
                Error::RequestFailed
            })?;

        Ok(html)
    }

    async fn get<T, Q>(&self, path: &str, query: Option<&Q>) -> Result<T, Error>
    where
        T: DeserializeOwned + Extractable,
        Q: Query,
    {
        let html = self.fetch_html(path, query).await?;
        let document = Html::parse_document(&html);
        T::extract_data(&document).map_err(|e| {
            tracing::error!("Failed to extract data: {}", e);
            Error::DataExtractionFailed
        })
    }

    pub async fn get_trades(&self, query: &TradeQuery) -> Result<PaginatedResponse<Trade>, Error> {
        let test = self
            .get::<PaginatedResponse<Trade>, TradeQuery>("/trades", Some(query))
            .await;
        println!("{:?}", test);
        test
    }

    pub async fn get_politicians(
        &self,
        query: &PoliticianQuery,
    ) -> Result<PaginatedResponse<PoliticianDetail>, Error> {
        self.get::<PaginatedResponse<PoliticianDetail>, PoliticianQuery>(
            "/politicians",
            Some(query),
        )
        .await
    }

    pub async fn get_issuer(&self, issuer_id: i64) -> Result<Response<IssuerDetail>, Error> {
        self.get::<Response<IssuerDetail>, IssuerQuery>(
            format!("/issuers/{}", issuer_id).as_str(),
            None,
        )
        .await
    }

    pub async fn get_issuers(
        &self,
        query: &IssuerQuery,
    ) -> Result<PaginatedResponse<IssuerDetail>, Error> {
        self.get::<PaginatedResponse<IssuerDetail>, IssuerQuery>("/issuers", Some(query))
            .await
    }
}
