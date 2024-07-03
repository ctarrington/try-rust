use crate::RocketApiDatabase;
use rocket::futures::TryStreamExt;
use rocket::post;
use rocket::serde::json::Json;
use rocket_api_server::{convert_to_sqlx_uuid, Measurement};
use rocket_db_pools::Connection;

#[post("/measurement", data = "<measurement>")]
pub async fn insert_measurement(
    mut db: Connection<RocketApiDatabase>,
    measurement: Json<Measurement>,
) -> Result<(), rocket::response::Debug<anyhow::Error>> {
    let object_uuid =
        convert_to_sqlx_uuid(&measurement.object_uuid).map_err(|e| anyhow::Error::from(e))?;
    let sensor_uuid =
        convert_to_sqlx_uuid(&measurement.sensor_uuid).map_err(|e| anyhow::Error::from(e))?;

    sqlx::query!(
            "INSERT INTO measurements (measured_at, object_uuid, sensor_uuid, latitude, longitude, object_length) VALUES ($1, $2, $3, $4, $5, $6) RETURNING measurement_uuid, recorded_at", measurement.measured_at, object_uuid, sensor_uuid, measurement.latitude, measurement.longitude, measurement.object_length
        )
        .fetch(&mut **db)
        .try_collect::<Vec<_>>()
        .await.map_err(|e| rocket::response::Debug(anyhow::Error::from(e)))?
        .first()
        .expect("returning result is empty")
        .measurement_uuid;

    Ok(())
}
