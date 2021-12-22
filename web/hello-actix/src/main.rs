use std::ops::Deref;
use std::sync::Mutex;
use std::time::Duration;
use std::{thread, time};

use rand::Rng;

use actix_web::web::Data;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

use serde::Serialize;

/// simplistic proof of work scheme to introduce a little variability how long a process takes
/// picks a random number until it is at or below a target
fn calculate_proof_of_work(target: u32, range_max: u32) -> u32 {
    let mut rng = rand::thread_rng();

    loop {
        let value: u32 = rng.gen_range(0..range_max);
        if value <= target {
            break value;
        }
    }
}

/// calculated state can have information that is not shared with the UI
struct CalculatedState {
    tick_count: u32,
    proofs: Vec<u32>,
}

impl CalculatedState {
    fn new() -> Self {
        CalculatedState {
            tick_count: 0,
            proofs: (0..850)
                .map(|_| calculate_proof_of_work(10, 10_000))
                .collect(),
        }
    }

    fn tick(&mut self) {
        self.tick_count = self.tick_count + 1;
        self.proofs
            .iter_mut()
            .for_each(|mut proof| *proof += calculate_proof_of_work(10, 10_000));
    }
}

/// projection of the calculated state - just the fields that are needed for the UI
/// having this level of indirection does require cloning
#[derive(Serialize)]
struct SharedState {
    tick_count: u32,
    proof_of_work_list: Vec<u32>,
}

impl SharedState {
    fn new(calculated_state: &CalculatedState) -> Self {
        SharedState {
            tick_count: calculated_state.tick_count,
            proof_of_work_list: calculated_state.proofs.clone(),
        }
    }
}

struct WrappedState {
    current: Mutex<SharedState>,
}

#[get("/")]
async fn get_current(data: web::Data<WrappedState>) -> impl Responder {
    // just using the derefed (unpacked) SharedState in the to_string works fine
    // assigning it to a temp variable gives move issues
    let serialized: String = serde_json::to_string(
        data.current
            .lock()
            .expect("unable to lock the data")
            .deref(),
    )
    .expect("unable to serialize the current state");
    HttpResponse::Ok().body(serialized)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let wrapped_state = Data::new(WrappedState {
        current: Mutex::new(SharedState::new(&CalculatedState::new())),
    });

    // copy of pointer for use in the thread
    let wrapped_state_for_thread = wrapped_state.clone();
    thread::spawn(move || {
        let mut calculated_state = CalculatedState::new();
        loop {
            let begin = time::Instant::now();
            calculated_state.tick();

            // grap the lock, swap the shared state, release the lock when current goes out of scope
            {
                let mut current = wrapped_state_for_thread
                    .current
                    .lock()
                    .expect("unable to lock the wrapped_state_for_thread");
                *current = SharedState::new(&calculated_state);
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
