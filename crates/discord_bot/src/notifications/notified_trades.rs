use sqlx::{Error, PgConnection, Postgres};

/// Returns a list of chat_ids that should be notified about this trade.
pub async fn get_to_notify(
    pool: &mut PgConnection,
    politician_id: &str,
    issuer_id: i64,
    tx_id: i64,
) -> Result<Vec<Option<i64>>, Error> {
    sqlx::query_file_scalar!(
        "src/notifications/queries/get_to_notify.sql",
        politician_id,
        issuer_id,
        tx_id,
    )
    .fetch_all(pool)
    .await
}

pub async fn record_delivery(
    pool: &mut PgConnection,
    user_id: i64,
    tx_id: i64,
) -> Result<(), Error> {
    sqlx::query_file!(
        "src/notifications/queries/record_delivery.sql",
        user_id,
        tx_id
    )
    .execute(pool)
    .await?;
    Ok(())
}
