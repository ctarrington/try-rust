use crate::RocketApiDatabase;
use rocket::get;
use rocket::serde::json::Json;
use rocket_api_server::{Path, PathPoint};
use rocket_db_pools::Connection;
use uuid::Uuid;

#[get("/get_path?<object_uuid>&<start>&<end>")]
pub async fn get_path(
    mut db: Connection<RocketApiDatabase>,
    object_uuid: &str,
    start: &str,
    end: &str,
) -> Result<Json<Path>, rocket::response::Debug<anyhow::Error>> {
    let sqlx_object_uuid =
        sqlx::types::Uuid::parse_str(object_uuid).map_err(|e| anyhow::Error::from(e))?;

    let object_uuid = Uuid::parse_str(object_uuid).map_err(|e| anyhow::Error::from(e))?;

    let start = chrono::NaiveDateTime::parse_from_str(&start, "%Y-%m-%dT%H:%M:%S")
        .map_err(|e| anyhow::Error::from(e))?;

    let end = chrono::NaiveDateTime::parse_from_str(&end, "%Y-%m-%dT%H:%M:%S")
        .map_err(|e| anyhow::Error::from(e))?;

    let query_result = sqlx::query!(
        "SELECT * FROM measurements m WHERE m.object_uuid = $1 AND m.measured_at >= $2 AND m.measured_at < $3 ORDER BY m.measured_at",
        sqlx_object_uuid,
        start,
        end
    )
        .fetch_all(&mut **db)
        .await
        .map_err(|e| rocket::response::Debug(anyhow::Error::from(e)))?;

    let mut path_points: Vec<PathPoint> = vec![];
    for (_record_id, record) in query_result.iter().enumerate() {
        let sensor_uuid = Uuid::parse_str(record.sensor_uuid.to_string().as_str())
            .map_err(|e| anyhow::Error::from(e))?;

        path_points.push(PathPoint {
            sensor_uuid,
            measured_at: record.measured_at,
            latitude: record.latitude,
            longitude: record.longitude,
        });
    }

    Ok(Json(Path {
        object_uuid,
        path_points,
    }))
}
