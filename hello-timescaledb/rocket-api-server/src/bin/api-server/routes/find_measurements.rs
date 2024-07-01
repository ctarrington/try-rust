use crate::RocketApiDatabase;
use rocket::get;
use rocket::serde::json::Json;
use rocket_api_server::Measurement;
use rocket_db_pools::Connection;
use uuid::Uuid;

#[get("/find_measurements?<start>&<end>")]
pub async fn find_measurements(
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
        "SELECT DISTINCT ON (object_uuid) * FROM measurements m WHERE m.measured_at >= $1 AND m.measured_at < $2 ORDER BY m.object_uuid, m.measured_at DESC",
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
