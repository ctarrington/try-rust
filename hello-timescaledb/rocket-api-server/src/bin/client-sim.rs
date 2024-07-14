use clap::Parser;
use rocket::tokio;
use rocket_api_server::{Measurement, Path, Times, TIME_FORMAT};

/// Simulate a client getting measurements from the API server
#[derive(Parser)]
#[command(version, about, long_about=None)]
struct Args {
    /// Interval between gets in milliseconds
    #[arg(short, long, default_value_t = 1000)]
    interval_milliseconds: usize,

    /// window of time in minutes to get measurements
    #[arg(short, long, default_value_t = 10)]
    window_minutes: usize,

    /// Minutes behind now to get measurements
    /// now - ago - window_minutes is the start time for the query
    /// now - ago is the end time for the query
    #[arg(short, long, default_value_t = 0)]
    ago_minutes: usize,

    /// number of gets to perform
    /// 0 means forever
    #[arg(short, long, default_value_t = 0)]
    iterations: usize,

    /// number of objects to fetch path for
    #[arg(short, long, default_value_t = 1)]
    path_count: usize,

    /// URL of the API server
    #[arg(
        short,
        long,
        default_value = "http://localhost:8000/api/find_measurements"
    )]
    server_url: String,
}

//noinspection ALL
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let client = reqwest::Client::new();
    let mut iteration_count = 0;

    while args.iterations == 0 || iteration_count < args.iterations {
        let loop_start = chrono::Utc::now().naive_utc();

        let end =
            chrono::Utc::now().naive_utc() - chrono::Duration::minutes(args.ago_minutes as i64);
        let start = end - chrono::Duration::minutes(args.window_minutes as i64);

        let url = format!(
            "{}?start={}&end={}",
            args.server_url,
            start.format(TIME_FORMAT),
            end.format(TIME_FORMAT),
        );

        let request_sent_at = chrono::Utc::now().naive_utc();
        let result = client.get(&url).send().await;

        if let Err(err) = result {
            println!("Error: {}", err);
            continue;
        }

        let response = result.unwrap();
        let instrumented_response: rocket_api_server::InstrumentedResponse<Vec<Measurement>> =
            response.json().await?;
        let measurements = instrumented_response.payload;
        let times = Times {
            request_sent_at,
            response_received_at: chrono::Utc::now().naive_utc(),
            ..instrumented_response.times
        };
        println!("n: {}, {}", measurements.len(), times);

        if args.iterations != 0 {
            iteration_count += 1;
        }

        let max_path_index = std::cmp::min(args.path_count, measurements.len());

        // get a path for the first path_count objects
        for measurement in measurements.iter().take(max_path_index) {
            let url = format!(
                "http://localhost:8000/api/get_path?object_uuid={}&start={}&end={}",
                measurement.object_uuid,
                start.format("%Y-%m-%dT%H:%M:%S"),
                end.format("%Y-%m-%dT%H:%M:%S")
            );
            let response = client.get(&url).send().await?;
            let path: Path = response.json().await?;

            println!("Got path for object {}", path.object_uuid);
            println!("{}", serde_json::to_string_pretty(&path).unwrap());
        }

        // take a break
        let loop_end = chrono::Utc::now().naive_utc();
        let elapsed = loop_end - loop_start;
        let sleep_time =
            chrono::Duration::milliseconds(args.interval_milliseconds as i64) - elapsed;
        tokio::time::sleep(tokio::time::Duration::from_millis(
            sleep_time.num_milliseconds() as u64,
        ))
        .await;
    }

    Ok(())
}
