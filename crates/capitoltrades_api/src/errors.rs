#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Request failed")]
    RequestFailed,
    #[error("Parseing error")]
    ParsingError,
    #[error("Failed to extract data from html")]
    DataExtractionFailed,
}
