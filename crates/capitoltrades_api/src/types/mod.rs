pub mod meta;
use scraper::{ElementRef, Selector};

use crate::Error;

pub use self::meta::{Meta, PaginatedResponse, Paging, Response};

pub mod issuer;
pub use self::issuer::{EodPrice, IssuerDetail, IssuerID, MarketCap, Performance, Sector};

pub mod trade;

mod politician;
pub use self::politician::{Chamber, Gender, Party, Politician, PoliticianDetail, PoliticianID};
