use rocket::futures::TryStreamExt;
use rocket::response::status::Created;
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
) -> Result<Created<Json<Measurement>>, String> {
    let measured_at: chrono::ParseResult<chrono::NaiveDateTime> = chrono::NaiveDateTime::parse_from_str(&*measurement.measured_at, "%Y-%m-%dT%H:%M:%S.%6f");

    if let Err(e) = measured_at {
        return Err(e.to_string());
    }

    let measured_at = measured_at.unwrap();

    let results = sqlx::query!(
            "INSERT INTO measurements (measured_at, object_id, latitude, longitude, object_length) VALUES ($1, $2, $3, $4, $5) RETURNING measurement_id", measured_at, measurement.object_id, measurement.latitude, measurement.longitude, measurement.object_length
        )
        .fetch(&mut **db)
        .try_collect::<Vec<_>>()
        .await.map_err(|e| e.to_string())?;

    let measurement_id = results
        .first()
        .expect("returning result is empty")
        .measurement_id;

    println!("Inserted object with id: {}", measurement_id);
    Ok(Created::new("/measurement").body(measurement))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(RocketApiDatabase::init())
        .mount("/api", routes![insert_measurement])
}
