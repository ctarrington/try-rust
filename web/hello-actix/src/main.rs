use actix_web::web::Data;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

struct SharedState {
    count: Mutex<u32>,
}

#[get("/")]
async fn hello(data: web::Data<SharedState>) -> impl Responder {
    let count = &data.count;
    HttpResponse::Ok().body(format!(
        "Hello world! From actix-web. How are you? count: {}",
        *count.lock().unwrap()
    ))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let shared_state = Data::new(SharedState {
        count: Mutex::new(0),
    });

    // copy of pointer for use in the thread
    let shared_state_for_thread = shared_state.clone();
    let handle = thread::spawn(move || loop {
        println!("about to increment");
        {
            let mut count = shared_state_for_thread.count.lock().unwrap();
            *count += 1;
        }
        println!("back from increment, about to sleep");
        thread::sleep(Duration::from_secs(1));
        println!("back from sleep");
    });

    // move a copy of the reference counted pointer to the shared state
    HttpServer::new(move || App::new().app_data(shared_state.clone()).service(hello))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
