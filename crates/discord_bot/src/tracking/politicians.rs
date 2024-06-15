use sqlx::{postgres, Error};

pub async fn get_tracked_politicians(
    pool: &mut postgres::PgConnection,
    user_id: i64,
) -> Result<Vec<String>, Error> {
    sqlx::query_scalar!(
        "SELECT politician_id FROM tracked_politicians WHERE user_id = $1",
        user_id
    )
    .fetch_all(pool)
    .await
}

pub async fn track_politician(
    pool: &mut postgres::PgConnection,
    user_id: i64,
    politician_id: &str,
) -> Result<(), Error> {
    sqlx::query!(
        "INSERT INTO tracked_politicians (user_id, politician_id) VALUES ($1, $2)",
        user_id,
        politician_id
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn untrack_politician(
    pool: &mut postgres::PgConnection,
    user_id: i64,
    politician_id: &str,
) -> Result<(), Error> {
    sqlx::query!(
        "DELETE FROM tracked_politicians WHERE user_id = $1 AND politician_id = $2",
        user_id,
        politician_id
    )
    .execute(pool)
    .await?;
    Ok(())
}
