use crate::routes::find_measurements::find_measurements;
use crate::routes::get_diagnostics::get_diagnostics;
use crate::routes::get_path::get_path;
use crate::routes::insert_measurement::insert_measurement;

use rocket::{launch, routes};
use rocket_db_pools::Database;

pub mod routes;

#[derive(Database)]
#[database("rocket_api_database")]
struct RocketApiDatabase(sqlx::PgPool);

#[launch]
fn rocket() -> _ {
    rocket::build().attach(RocketApiDatabase::init()).mount(
        "/api",
        routes![
            insert_measurement,
            find_measurements,
            get_diagnostics,
            get_path
        ],
    )
}
