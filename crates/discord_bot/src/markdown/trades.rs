use crate::markdown::politicians::politician_to_markdown;
use capitoltrades_api::types::{Asset, Trade};
use pulldown_cmark::{html, Parser};

fn asset_to_markdown(asset: &Asset) -> String {
    let markdown = match asset.asset_ticker {
        Some(ref asset_ticker) => format!("*{}* \\({}\\)", asset_ticker, asset.asset_type,),
        None => asset.asset_type.clone(),
    };

    let parser = Parser::new(&markdown);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

pub fn trade_to_markdown(trade: &Trade) -> String {
    let markdown = format!(
        "{} sold {} units of {} {} on {}\\.\nPub date: {}\\. Reporting gap: {} days\\. [Filing link]({})\\.",
        politician_to_markdown(&trade.politician),
        match trade.size {
            Some(size) => size.to_string(),
            None => "an unknown number of".to_string(),
        },
        asset_to_markdown(&trade.asset),
        match trade.price {
            Some(price) => format!("at a price of *${}*", price),
            None => "".to_string(),
        },
        trade.tx_date.to_string(),
        trade.pub_date.date_naive().to_string(),
        trade.reporting_gap,
        trade.filing_url,
    );
    println!("woops");
    let parser = Parser::new(&markdown);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}
