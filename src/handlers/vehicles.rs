// AXL-SAST-001: format! interpolation into sqlx::query.
use sqlx::{PgPool, Row};

pub async fn lookup(pool: &PgPool, vin: &str) -> Result<Option<String>, sqlx::Error> {
    // AXL-SAST-001: vin is interpolated into raw SQL string.
    let sql = format!("SELECT model FROM vehicles WHERE vin = '{}'", vin);
    let row = sqlx::query(&sql).fetch_optional(pool).await?;
    Ok(row.and_then(|r| r.try_get::<String, _>(0).ok()))
}
