use rocket::futures::TryStreamExt;
use rocket::response::status::Created;
use rocket::serde::json::Json;
use rocket::{launch, post, routes};

use rocket_db_pools::{Connection, Database};

use rocket_api_server::Measurement;

#[derive(Database)]
#[database("sqlx")]
struct DB(sqlx::PgPool);

#[post("/measurement", data = "<measurement>")]
async fn insert_measurement(
    mut db: Connection<DB>,
    measurement: Json<Measurement>,
) -> Result<Created<Json<Measurement>>, String> {
    let results = sqlx::query!(
            "INSERT INTO measurements (latitude, longitude, object_length) VALUES ($1, $2, $3) RETURNING object_id", measurement.latitude, measurement.longitude, measurement.object_length
        )
        .fetch(&mut **db)
        .try_collect::<Vec<_>>()
        .await.map_err(|e| e.to_string())?;

    let object_id = results
        .first()
        .expect("returning result is empty")
        .object_id;

    println!("Inserted object with id: {}", object_id);
    Ok(Created::new("/measurement").body(measurement))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(DB::init())
        .mount("/api", routes![insert_measurement])
}
