use std::sync::Mutex;
use std::time::Duration;
use std::{thread, time};

use actix_web::web::Data;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

use serde::Serialize;

/// calculated state can have information that is not shared with the UI
struct CalculatedState {
    count: u32,
}

/// projection of the calculated state - just the fields that are needed for the UI
#[derive(Serialize)]
struct SharedState {
    count: u32,
}

impl SharedState {
    fn new(calculated_state: &CalculatedState) -> Self {
        SharedState {
            count: calculated_state.count,
        }
    }
}

struct WrappedState {
    current: Mutex<SharedState>,
}

#[get("/")]
async fn get_current(data: web::Data<WrappedState>) -> impl Responder {
    let current = &*data.current.lock().expect("unable to lock the data");
    let serialized: String =
        serde_json::to_string(current).expect("unable to serialize the current state");
    HttpResponse::Ok().body(serialized)
}

fn tick_state(current: &CalculatedState) -> CalculatedState {
    CalculatedState {
        count: current.count + 1,
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let wrapped_state = Data::new(WrappedState {
        current: Mutex::new(SharedState { count: 0 }),
    });

    // copy of pointer for use in the thread
    let wrapped_state_for_thread = wrapped_state.clone();
    thread::spawn(move || {
        let mut calculated_state = CalculatedState { count: 0 };
        loop {
            let begin = time::Instant::now();
            calculated_state = tick_state(&calculated_state);

            // grap the lock, swap the shared state, release the lock when current goes out of scope
            {
                let new_shared_state = SharedState::new(&calculated_state);
                let mut current = wrapped_state_for_thread
                    .current
                    .lock()
                    .expect("unable to lock the wrapped_state_for_thread");
                *current = new_shared_state;
            }

            let elapsed = time::Instant::now() - begin;
            let goal = Duration::from_secs(1);
            let pause: Duration = if elapsed > goal {
                println!(
                    "warning: falling behind in update loop: {:?} > {:?}",
                    elapsed, goal
                );
                Duration::from_secs(0)
            } else {
                goal - elapsed
            };
            println!(
                "back from increment, took {:?}, about to sleep: {:?}",
                elapsed, pause
            );
            thread::sleep(pause);
        }
    });

    println!("listening at http://localhost:8080");
    // move a copy of the reference counted pointer to the shared state
    HttpServer::new(move || {
        App::new()
            .app_data(wrapped_state.clone())
            .service(get_current)
    })
    .bind("127.0.0.1:8080")
    .expect("unable to bind to address")
    .run()
    .await
}
