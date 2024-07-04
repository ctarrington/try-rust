use clap::Parser;
use rand::Rng;
use rocket::tokio;
use rocket_api_server::Measurement;

/// Simulate a sensor sending measurements to the API server
#[derive(Parser)]
#[command(version, about, long_about=None)]
struct Args {
    /// Number of real world objects to simulate
    #[arg(short, long, default_value_t = 1000)]
    object_count: usize,

    /// Number of measurements to send in parallel
    #[arg(short, long, default_value_t = 5)]
    future_count: usize,

    /// Number of iterations or ticks to simulate
    /// Each tick sends a measurement for each object
    /// zero means forever
    #[arg(short, long, default_value_t = 0)]
    tick_count: usize,

    /// Interval between ticks in milliseconds
    /// It is not adjusted for the time it takes to build and
    /// send the measurements
    #[arg(short, long, default_value_t = 1000)]
    interval_milliseconds: usize,

    // The probability that an object will be evicted and replaced as a percentage
    #[arg(short, long, default_value_t = 1)]
    eviction_percentage: usize,

    /// URL of the API server
    #[arg(short, long, default_value = "http://localhost:8000/api/measurement")]
    server_url: String,
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = rand::thread_rng();

    let sensor_uuid = uuid::Uuid::new_v4();
    let client = reqwest::Client::new();

    let args = Args::parse();
    let eviction_probability = args.eviction_percentage as f64 / 100.0;

    let mut tick = 0;
    let mut object_index = 0;

    let mut object_uuids: Vec<uuid::Uuid> = (0..args.object_count)
        .map(|_| uuid::Uuid::new_v4())
        .collect();

    // The main loop is a little tricky because we want to send a batches of measurements on the futures
    // until we get through all the objects.  Then we sleep for the interval and reset the object index.
    while tick < args.tick_count || args.tick_count == 0 {
        // send a batch of measurements using the specified number of futures
        let mut futures = Vec::new();
        for _index in 0..args.future_count {
            if object_index >= args.object_count {
                break;
            }

            let measurement = Measurement {
                measurement_uuid: None,
                recorded_at: None,
                object_uuid: object_uuids[object_index],
                sensor_uuid,
                measured_at: chrono::Utc::now().naive_utc(),
                latitude: 10.0 + object_index as f32 * 0.2,
                longitude: 2.0 + tick as f32 * 0.002,
                object_length: 10.0 + object_index as f32 * 0.5,
            };

            let client = client.clone();
            let server_url = args.server_url.clone();
            let future = async move {
                let _result = client.post(server_url).json(&measurement).send().await;
            };
            futures.push(future);
            object_index += 1;
        }

        futures::future::join_all(futures).await;

        // If we have sent all the objects, sleep for the interval
        // and reset the object index.
        if object_index >= args.object_count {
            object_index = 0;
            tick += 1;
            println!("Sent {} objects for tick {}", args.object_count, tick);
            tokio::time::sleep(tokio::time::Duration::from_millis(
                args.interval_milliseconds as u64,
            ))
            .await;

            if rng.gen_bool(eviction_probability) {
                let evicted = object_uuids.pop();
                println!("Evicted object {:?}", evicted);

                object_uuids.push(uuid::Uuid::new_v4());
                println!("Added new object {:?}", object_uuids.last());
            }
        }
    }

    Ok(())
}
