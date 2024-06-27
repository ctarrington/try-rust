use rocket::futures::TryStreamExt;
use rocket::serde::json::Json;
use rocket::{launch, post, routes};

use rocket_db_pools::{Connection, Database};

use rocket_api_server::Measurement;

#[derive(Database)]
#[database("rocket_api_database")]
struct RocketApiDatabase(sqlx::PgPool);

#[post("/measurement", data = "<measurement>")]
async fn insert_measurement(
    mut db: Connection<RocketApiDatabase>,
    measurement: Json<Measurement>,
) -> Result<(), rocket::response::Debug<anyhow::Error>> {
    let object_uuid = sqlx::types::Uuid::parse_str(&measurement.object_uuid.to_string())
        .map_err(|e| anyhow::Error::from(e))?;
    let sensor_uuid = sqlx::types::Uuid::parse_str(&measurement.sensor_uuid.to_string())
        .map_err(|e| anyhow::Error::from(e))?;

    let measurement_id: sqlx::types::Uuid = sqlx::query!(
            "INSERT INTO measurements (measured_at, object_uuid, sensor_uuid, latitude, longitude, object_length) VALUES ($1, $2, $3, $4, $5, $6) RETURNING measurement_id, recorded_at", measurement.measured_at, object_uuid, sensor_uuid, measurement.latitude, measurement.longitude, measurement.object_length
        )
        .fetch(&mut **db)
        .try_collect::<Vec<_>>()
        .await.map_err(|e| rocket::response::Debug(anyhow::Error::from(e)))?
        .first()
        .expect("returning result is empty")
        .measurement_id;

    println!("Inserted measurement with id: {}", measurement_id);
    Ok(())
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(RocketApiDatabase::init())
        .mount("/api", routes![insert_measurement])
}
