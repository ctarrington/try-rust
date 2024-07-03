use crate::RocketApiDatabase;
use rocket::get;
use rocket::serde::json::Json;
use rocket_api_server::{convert_to_uuid, parse_datetime, Measurement};
use rocket_db_pools::Connection;

#[get("/find_measurements?<start>&<end>")]
pub async fn find_measurements(
    mut db: Connection<RocketApiDatabase>,
    start: &str,
    end: &str,
) -> Result<Json<Vec<Measurement>>, rocket::response::Debug<anyhow::Error>> {
    let start = parse_datetime(&start).map_err(|e| anyhow::Error::from(e))?;
    let end = parse_datetime(&end).map_err(|e| anyhow::Error::from(e))?;

    // Distinct on object_uuid and order by measured_at descending combine to give the most recent
    // measurement for each object
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
        let measurement_uuid =
            convert_to_uuid(&record.measurement_uuid).map_err(|e| anyhow::Error::from(e))?;

        let sensor_uuid =
            convert_to_uuid(&record.sensor_uuid).map_err(|e| anyhow::Error::from(e))?;

        let object_uuid =
            convert_to_uuid(&record.object_uuid).map_err(|e| anyhow::Error::from(e))?;

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
