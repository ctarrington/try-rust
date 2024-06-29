use rocket::futures::TryStreamExt;
use rocket::serde::json::Json;
use rocket::{get, launch, post, routes};

use rocket_db_pools::{Connection, Database};
use uuid::Uuid;

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

    let measurement_uuid: sqlx::types::Uuid = sqlx::query!(
            "INSERT INTO measurements (measured_at, object_uuid, sensor_uuid, latitude, longitude, object_length) VALUES ($1, $2, $3, $4, $5, $6) RETURNING measurement_uuid, recorded_at", measurement.measured_at, object_uuid, sensor_uuid, measurement.latitude, measurement.longitude, measurement.object_length
        )
        .fetch(&mut **db)
        .try_collect::<Vec<_>>()
        .await.map_err(|e| rocket::response::Debug(anyhow::Error::from(e)))?
        .first()
        .expect("returning result is empty")
        .measurement_uuid;

    println!("Inserted measurement with id: {}", measurement_uuid);
    Ok(())
}

#[get("/find_measurements?<start>&<end>")]
async fn find_measurements(
    mut db: Connection<RocketApiDatabase>,
    start: &str,
    end: &str,
) -> Result<Json<Vec<Measurement>>, rocket::response::Debug<anyhow::Error>> {
    dbg!(start, end);
    let start = chrono::NaiveDateTime::parse_from_str(&start, "%Y-%m-%dT%H:%M:%S")
        .map_err(|e| anyhow::Error::from(e))?;

    let end = chrono::NaiveDateTime::parse_from_str(&end, "%Y-%m-%dT%H:%M:%S")
        .map_err(|e| anyhow::Error::from(e))?;

    let query_result = sqlx::query!(
        "SELECT * FROM measurements m WHERE m.measured_at >= $1 AND m.measured_at < $2 ORDER BY m.object_uuid, m.measured_at DESC",
        start,
        end
    )
    .fetch_all(&mut **db)
    .await
    .map_err(|e| rocket::response::Debug(anyhow::Error::from(e)))?;

    let mut measurements: Vec<Measurement> = vec![];
    for (_record_id, record) in query_result.iter().enumerate() {
        let measurement_uuid = Uuid::parse_str(record.measurement_uuid.to_string().as_str())
            .map_err(|e| anyhow::Error::from(e))?;

        let sensor_uuid = Uuid::parse_str(record.sensor_uuid.to_string().as_str())
            .map_err(|e| anyhow::Error::from(e))?;

        let object_uuid = Uuid::parse_str(record.object_uuid.to_string().as_str())
            .map_err(|e| anyhow::Error::from(e))?;

        measurements.push(Measurement {
            measurement_uuid: Some(measurement_uuid),
            object_uuid,
            sensor_uuid,
            measured_at: record.measured_at,
            recorded_at: Some(record.recorded_at),
            latitude: record.latitude,
            longitude: record.longitude,
            object_length: record.object_length,
        });
    }

    Ok(Json(measurements))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(RocketApiDatabase::init())
        .mount("/api", routes![insert_measurement, find_measurements])
}
