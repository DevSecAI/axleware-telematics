// AXL-SAST-001: format! interpolation into sqlx::query.
use sqlx::{PgPool, Row};

pub async fn lookup(pool: &PgPool, vin: &str) -> Result<Option<String>, sqlx::Error> {
    // AXL-SAST-001: vin is interpolated into raw SQL string.
    let row = sqlx::query("SELECT model FROM vehicles WHERE vin = $1")
        .bind(vin)
        .fetch_optional(pool)
        .await?;
    Ok(row.and_then(|r| r.try_get::<String, _>(0).ok()))
}