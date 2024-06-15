use capitoltrades_api::types::{Chamber, Party, Politician, PoliticianDetail};
use pulldown_cmark::{html, Options, Parser};

use super::format_volume;

pub fn politician_to_markdown(politician: &Politician) -> String {
    let markdown = format!(
        "{} {} ({}, {}, {})",
        &politician.first_name,
        &politician.last_name,
        match politician.party {
            Party::Democrat => "D",
            Party::Republican => "R",
            Party::Other => "O",
        },
        match politician.chamber {
            Chamber::House => "House",
            Chamber::Senate => "Senate",
        },
        politician.state_id.to_uppercase()
    );

    let parser = Parser::new(&markdown);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

fn politician_stats_to_markdown(politician: &PoliticianDetail) -> String {
    let markdown = format!(
        "Trades: {}. Issuers: {}. Volume: {}. {}",
        politician.stats.count_trades,
        politician.stats.count_issuers,
        format_volume(politician.stats.volume),
        if let Some(d) = politician.stats.date_last_traded {
            format!("Last traded: {}.", d.to_string())
        } else {
            "No trades.".to_string()
        },
    );

    let parser = Parser::new(&markdown);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

pub fn politician_detail_to_markdown(politician: &PoliticianDetail) -> String {
    let markdown = format!(
        "{} {} \\({}, {}, {}\\)\n{}",
        &politician.first_name,
        &politician.last_name,
        match politician.party {
            Party::Democrat => "D",
            Party::Republican => "R",
            Party::Other => "O",
        },
        match politician.chamber {
            Chamber::House => "House",
            Chamber::Senate => "Senate",
        },
        politician.state_id.to_uppercase(),
        politician_stats_to_markdown(&politician)
    );

    let parser = Parser::new(&markdown);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}
