use serde::Serialize;
use warp::http::Method;
use warp::reject::Reject;
use warp::Filter;
use warp::Rejection;
use warp::Reply;

use rand::prelude::*;

#[derive(Debug, Serialize)]
struct Thing {
    name: String,
    description: String,
}

#[derive(Debug)]
struct InvalidThing;
impl Reject for InvalidThing {}

async fn get_scary_things() -> Result<impl Reply, Rejection> {
    let things = vec![
        Thing {
            name: "Godzilla".to_string(),
            description: "Radioactive and hostile to cities".to_string(),
        },
        Thing {
            name: "Tiny Spider".to_string(),
            description: "Sure, it is probably harmless... but ewww, creepy".to_string(),
        },
    ];
    Ok(warp::reply::json(&things))
}
async fn get_things() -> Result<impl Reply, Rejection> {
    let things = vec![
        Thing {
            name: "Thing 1".to_string(),
            description: "This is the first thing".to_string(),
        },
        Thing {
            name: "Thing 2".to_string(),
            description: "This is the second thing".to_string(),
        },
    ];

    // Note: if there is no path to success then the compiler will complain that it cannot infer the type
    let value = random::<f64>();
    return if value > 0.3 {
        Ok(warp::reply::json(&things))
    } else {
        Err(warp::reject::custom(InvalidThing))
    };
}

async fn error_handler(_err: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(InvalidThing) = _err.find() {
        return Ok(warp::reply::with_status(
            warp::reply::json(&"Invalid thing"),
            warp::http::StatusCode::BAD_REQUEST,
        ));
    } else {
        Ok(warp::reply::with_status(
            warp::reply::json(&"Not found"),
            warp::http::StatusCode::NOT_FOUND,
        ))
    }
}

#[tokio::main]
async fn main() {
    /*
    curl -v -X OPTIONS http://localhost:3030/scary-things \
    -H "Access-Control-Request-Method: PUT" \
    -H "Access-Control-Request-Headers: content-type" \
    -H "Origin: https://www.example.com"
    */
    let cors = warp::cors()
        .allow_origin("https://www.example.com")
        .allow_headers(vec!["content-type"])
        .allow_methods(&[Method::PUT, Method::DELETE, Method::GET, Method::POST]);

    // http://localhost:3030/scary-things
    let scary_things_route = warp::get()
        .and(warp::path("scary-things"))
        .and(warp::path::end())
        .and_then(get_scary_things);

    // http://localhost:3030/things
    let things_route = warp::get()
        .and(warp::path("things"))
        .and(warp::path::end())
        .and_then(get_things);

    let get_things_route = scary_things_route.or(things_route).recover(error_handler);

    println!("Server started at http://localhost:3030/things");

    let routes = get_things_route.with(cors);
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
