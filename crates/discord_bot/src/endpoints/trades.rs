use super::actions::Action;
use crate::errors::HandlerResult;
use crate::markdown::trades::trade_to_markdown;
use crate::Data;
use capitoltrades_api::types::{PaginatedResponse, Trade};
use capitoltrades_api::{Client, PoliticianQuery, Query, SortDirection, TradeQuery, TradeSortBy};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;
use poise::serenity_prelude::*;
use serenity::all::CreateButton;
/* struct CommonQuery {
    page: u8,
    sort_direction: SortDirection,
} */

fn list_keyboard(query: &TradeQuery) -> Vec<serenity::builder::CreateActionRow> {
    let buttons = vec![
        ("Sort by publication date", TradeSortBy::PublicationDate),
        ("Sort by trade date", TradeSortBy::TradeDate),
        ("Sort by reporting gap", TradeSortBy::ReportingGap),
    ];

    let mut keyboard = Vec::new();

    let mut action_row = serenity::builder::CreateActionRow::Buttons(Vec::new());
    for (label, sort_by) in buttons {
        let callback_data = format!(
            "{}:{}:{}:{}",
            Action::TradesList as u8,
            sort_by as u8,
            SortDirection::Desc as u8,
            query.common.page
        );
        let button = serenity::builder::CreateButton::new(callback_data)
            .label(format!("{} Desc", label))
            .style(ButtonStyle::Primary);
        if let serenity::builder::CreateActionRow::Buttons(buttons) = &mut action_row {
            buttons.push(button);
        }
    }
    keyboard.push(action_row);

    let mut pagination_row = serenity::builder::CreateActionRow::Buttons(Vec::new());

    if query.common.page > 1 {
        let callback_data = format!(
            "{}:{}:{}:{}",
            Action::TradesList as u8,
            query.sort_by as u8,
            query.common.sort_direction as u8,
            std::cmp::max(1, query.common.page - 1)
        );
        let prev_button = serenity::builder::CreateButton::new(callback_data)
            .label("<")
            .style(ButtonStyle::Secondary);
        if let serenity::builder::CreateActionRow::Buttons(buttons) = &mut pagination_row {
            buttons.push(prev_button);
        }
    }

    let callback_data = format!(
        "{}:{}:{}:{}",
        Action::TradesList as u8,
        query.sort_by as u8,
        query.common.sort_direction as u8,
        query.common.page + 1
    );
    let next_button = serenity::builder::CreateButton::new(callback_data)
        .label(">")
        .style(ButtonStyle::Secondary);
    if let serenity::builder::CreateActionRow::Buttons(buttons) = &mut pagination_row {
        buttons.push(next_button);
    }

    keyboard.push(pagination_row);

    keyboard
}

fn text_from_response(response: &PaginatedResponse<Trade>) -> String {
    let mut text = String::new();
    for trade in &response.data {
        text.push_str(&trade_to_markdown(&trade));
        text.push_str("\n\n");
    }
    text
}

#[poise::command(prefix_command, slash_command)]
pub async fn trades(ctx: Context<'_>) -> Result<(), Error> {
    let client = Client::new();
    let query = PoliticianQuery::default();
    let response = client.get_politicians(&query).await?;
    let text = text_from_response(&response);

    let channel_id = ctx.channel_id();
    let files = Vec::new();
    let map = serde_json::json!({
        "content": text,
        "components": list_keyboard(&query),
    });

    match ctx.http().send_message(channel_id, files, &map).await {
        Ok(_) => Ok(()),
        Err(e) => {
            tracing::error!("Failed to list trades: {}", e);
            Err(e.into())
        }
    }
}

#[poise::command(prefix_command, slash_command)]
pub async fn list_callback(
    ctx: Context<'_>,
    #[description = "Payload"] payload: String,
) -> Result<(), Error> {
    let payload: Vec<&str> = payload.split(":").collect();
    let action: Action = payload[0].parse().expect("Invalid action");
    let sort_by: TradeSortBy = payload[1].parse().expect("Invalid sort by");
    let sort_direction: SortDirection = payload[2].parse().expect("Invalid sort direction");
    let page: i64 = payload[3].parse().expect("Invalid page");

    if action == Action::TradesList {
        let client = Client::new();
        let query = TradeQuery::default()
            .with_sort_by(sort_by)
            .with_sort_direction(sort_direction)
            .with_page(page);
        let response = client.get_trades(&query).await?;
        let text = text_from_response(&response);

        let channel_id = ctx.channel_id();
        let message_id = ctx.id();
        let new_attachments = Vec::new();
        let map = serde_json::json!({
            "content": text,
            "components": list_keyboard(&query),
        });

        ctx.http()
            .edit_message(channel_id, message_id.into(), &map, new_attachments)
            .await?;
    }

    Ok(())
}
