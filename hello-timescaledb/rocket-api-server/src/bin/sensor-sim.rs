use clap::Parser;
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
    #[arg(short, long, default_value_t = 1000)]
    interval_milliseconds: usize,

    /// URL of the API server
    #[arg(short, long, default_value = "http://localhost:8000/api/measurement")]
    server_url: String,
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let sensor_uuid = uuid::Uuid::new_v4();
    let client = reqwest::Client::new();

    let object_count = args.object_count;
    let tick_count = args.tick_count;

    let future_count = args.future_count;

    let mut tick = 0;
    let mut object_index = 0;
    let mut total = 0;

    let object_uuids: Vec<uuid::Uuid> = (0..object_count).map(|_| uuid::Uuid::new_v4()).collect();

    while tick < tick_count || tick_count == 0 {
        let mut futures = Vec::new();
        for _index in 0..future_count {
            if object_index >= object_count {
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
            total += 1;
        }

        futures::future::join_all(futures).await;

        if object_index >= object_count {
            object_index = 0;
            tick += 1;
            println!("Sent objects {} for tick {}", object_count, tick);
            tokio::time::sleep(tokio::time::Duration::from_millis(
                args.interval_milliseconds as u64,
            ))
            .await;
        }
    }

    println!(
        "Finished sending {} measurements with {} futures",
        total, future_count
    );
    Ok(())
}
