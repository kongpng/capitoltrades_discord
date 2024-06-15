use capitoltrades_api::types::IssuerDetail;
use sqlx::{postgres, Error};

pub async fn get_tracked_issuers(
    pool: &mut postgres::PgConnection,
    user_id: i64,
) -> Result<Vec<i64>, Error> {
    sqlx::query_scalar!(
        r#"SELECT issuer_id FROM tracked_issuers WHERE user_id = $1"#,
        user_id
    )
    .fetch_all(pool)
    .await
}

pub async fn track_issuer(
    pool: &mut postgres::PgConnection,
    user_id: i64,
    issuer: &IssuerDetail,
) -> Result<(), Error> {
    let price = match &issuer.performance {
        Some(performance) => performance.last_price(),
        None => None,
    };
    sqlx::query!(
        "INSERT INTO tracked_issuers (user_id, issuer_id, last_price) VALUES ($1, $2, $3)",
        user_id,
        issuer.issuer_id as i64,
        price,
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn untrack_issuer(
    pool: &mut postgres::PgConnection,
    user_id: i64,
    issuer_id: i64,
) -> Result<(), Error> {
    sqlx::query!(
        "DELETE FROM tracked_issuers WHERE user_id = $1 AND issuer_id = $2",
        user_id,
        issuer_id
    )
    .execute(pool)
    .await?;
    Ok(())
}
