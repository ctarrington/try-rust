use crate::RocketApiDatabase;
use rocket::get;
use rocket::serde::json::Json;
use rocket_api_server::{
    convert_to_uuid, parse_datetime, InstrumentedResponse, Measurement, Times,
};
use rocket_db_pools::Connection;

#[get("/find_measurements?<start>&<end>&<flavor>&<page_index>&<page_size>")]
pub async fn find_measurements(
    mut db: Connection<RocketApiDatabase>,
    start: &str,
    end: &str,
    page_index: i64,
    page_size: i64,
    flavor: String,
) -> Result<Json<InstrumentedResponse<Vec<Measurement>>>, rocket::response::Debug<anyhow::Error>> {
    let start = parse_datetime(&start).map_err(anyhow::Error::from)?;
    let end = parse_datetime(&end).map_err(anyhow::Error::from)?;

    let query_start = chrono::Utc::now().naive_utc();
    // Distinct on object_uuid and order by measured_at descending combine to give the most recent
    // measurement for each object
    let query_results = sqlx::query!(
        "SELECT DISTINCT ON (object_uuid) * FROM measurements m WHERE m.measured_at >= $1 AND m.measured_at < $2 AND m.flavor LIKE $3 ORDER BY m.object_uuid, m.measured_at DESC LIMIT $4 OFFSET $5",
        start,
        end,
        flavor,
        page_size,
        page_index * page_size,
    )
        .fetch_all(&mut **db)
        .await
        .map_err(|e| rocket::response::Debug(anyhow::Error::from(e)))?;
    let query_complete = chrono::Utc::now().naive_utc();

    let mut measurements: Vec<Measurement> = vec![];
    for record in query_results {
        let measurement_uuid =
            convert_to_uuid(&record.measurement_uuid).map_err(anyhow::Error::from)?;

        let sensor_uuid = convert_to_uuid(&record.sensor_uuid).map_err(anyhow::Error::from)?;

        let object_uuid = convert_to_uuid(&record.object_uuid).map_err(anyhow::Error::from)?;

        measurements.push(Measurement {
            measurement_uuid: Some(measurement_uuid),
            object_uuid,
            sensor_uuid,
            measured_at: record.measured_at,
            recorded_at: Some(record.recorded_at),
            latitude: record.latitude,
            longitude: record.longitude,
            altitude: record.altitude,
            x_position: record.x_position,
            y_position: record.y_position,
            z_position: record.z_position,
            x_velocity: record.x_velocity,
            y_velocity: record.y_velocity,
            z_velocity: record.z_velocity,
            object_length: record.object_length,
            object_width: record.object_width,
            object_height: record.object_height,
            flavor: record.flavor,
            toppings: record.toppings,
            color: record.color,
            texture: record.texture,
        });
    }

    let data_mangling_complete = chrono::Utc::now().naive_utc();

    let times = Times {
        request_sent_at: Default::default(),
        query_start,
        query_complete,
        data_mangling_complete,
        response_received_at: Default::default(),
    };

    let instrumented_response = InstrumentedResponse {
        payload: measurements,
        times,
    };
    Ok(Json(instrumented_response))
}
