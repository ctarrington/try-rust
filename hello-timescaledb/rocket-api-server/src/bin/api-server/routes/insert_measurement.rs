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
        convert_to_sqlx_uuid(&measurement.object_uuid).map_err(anyhow::Error::from)?;
    let sensor_uuid =
        convert_to_sqlx_uuid(&measurement.sensor_uuid).map_err(anyhow::Error::from)?;

    sqlx::query!(
            "INSERT INTO measurements (measured_at, object_uuid, sensor_uuid, latitude, longitude, altitude, x_position, y_position, z_position, x_velocity, y_velocity, z_velocity, object_length, object_width, object_height, flavor, toppings, color, texture) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19) RETURNING measurement_uuid, recorded_at", measurement.measured_at, object_uuid, sensor_uuid, measurement.latitude, measurement.longitude, measurement.altitude, measurement.x_position, measurement.y_position, measurement.z_position, measurement.x_velocity, measurement.y_velocity, measurement.z_velocity, measurement.object_length, measurement.object_width, measurement.object_height, measurement.flavor, measurement.toppings, measurement.color, measurement.texture
        )
        .fetch(&mut **db)
        .try_collect::<Vec<_>>()
        .await.map_err(|e| rocket::response::Debug(anyhow::Error::from(e)))?
        .first()
        .expect("returning result is empty");

    Ok(())
}
