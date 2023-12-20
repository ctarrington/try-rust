use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;

struct AppState {
    counter: Mutex<i32>, // <- Mutex is necessary to mutate safely across threads
}

#[get("/")]
async fn read(data: web::Data<AppState>) -> impl Responder {
    let counter = data.counter.lock().unwrap();
    HttpResponse::Ok().body(format!("counter = {counter}"))
}

#[get("/increment")]
async fn increment(data: web::Data<AppState>) -> impl Responder {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;

    HttpResponse::Ok().body(format!("counter = {counter}"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter = web::Data::new(AppState {
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        App::new()
            .service(read)
            .service(increment)
            .app_data(counter.clone()) // <- register the created data
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
