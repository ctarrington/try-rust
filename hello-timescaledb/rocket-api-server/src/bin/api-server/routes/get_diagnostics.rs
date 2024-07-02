use crate::RocketApiDatabase;
use rocket::get;
use rocket::serde::json::Json;
use rocket_api_server::Diagnostics;
use rocket_db_pools::Connection;

#[get("/get_diagnostics")]
pub async fn get_diagnostics(
    mut db: Connection<RocketApiDatabase>,
) -> Result<Json<Diagnostics>, rocket::response::Debug<anyhow::Error>> {
    let measurement_count = sqlx::query!("SELECT COUNT(*) FROM measurements")
        .fetch_one(&mut **db)
        .await
        .map_err(|e| rocket::response::Debug(anyhow::Error::from(e)))?
        .count
        .unwrap_or(0) as usize;

    let object_count = sqlx::query!("SELECT COUNT(DISTINCT object_uuid) FROM measurements")
        .fetch_one(&mut **db)
        .await
        .map_err(|e| rocket::response::Debug(anyhow::Error::from(e)))?
        .count
        .unwrap_or(0) as usize;

    let database_size_bytes = sqlx::query!("SELECT pg_database_size(current_database())")
        .fetch_one(&mut **db)
        .await
        .map_err(|e| rocket::response::Debug(anyhow::Error::from(e)))?
        .pg_database_size
        .unwrap_or(0) as usize;

    let database_size_gigabytes = database_size_bytes as f64 / 1024.0 / 1024.0 / 1024.0;
    let average_measurement_size_bytes = if measurement_count > 0 {
        database_size_bytes as f64 / measurement_count as f64
    } else {
        0.0
    };

    Ok(Json(Diagnostics {
        measured_at: chrono::Utc::now().naive_utc(),
        measurement_count,
        object_count,
        database_size_gigabytes,
        average_measurement_size_bytes,
    }))
}
