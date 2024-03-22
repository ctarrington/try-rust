use serde::Serialize;
use warp::Filter;

#[derive(Debug, Serialize)]
struct Thing {
    name: String,
    description: String,
}

async fn get_scary_things() -> Result<impl warp::Reply, warp::Rejection> {
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
async fn get_things() -> Result<impl warp::Reply, warp::Rejection> {
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
    Ok(warp::reply::json(&things))
}

#[tokio::main]
async fn main() {
    let get_things_route = warp::path!("things")
        .and(warp::get())
        .and_then(get_things)
        .or(warp::path!("scary-things")
            .and(warp::get())
            .and_then(get_scary_things));

    println!("Server started at http://localhost:3030/things");
    warp::serve(get_things_route)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
