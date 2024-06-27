use rocket::tokio;
use rocket_api_server::Measurement;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sensor_uuid = uuid::Uuid::new_v4();
    let client = reqwest::Client::new();

    let object_count = 10_000;
    let tick_count = 60;

    let future_count = 20;
    let loops = object_count * tick_count / future_count;

    let mut tick = 0;
    let mut object_index = 0;
    let mut total = 0;

    let object_uuids: Vec<uuid::Uuid> = (0..object_count).map(|_| uuid::Uuid::new_v4()).collect();

    for _outer_index in 0..loops {
        let mut futures = Vec::new();
        for _index in 0..future_count {
            if object_index >= object_count {
                break;
            }

            let measurement = Measurement {
                measurement_id: None,
                recorded_at: None,
                object_uuid: object_uuids[object_index],
                sensor_uuid,
                measured_at: chrono::Utc::now().naive_utc(),
                latitude: 10.0 + object_index as f32 * 0.2,
                longitude: 2.0 + tick as f32 * 0.002,
                object_length: 10.0 + object_index as f32 * 0.5,
            };

            let client = client.clone();
            let future = async move {
                let _result = client
                    .post("http://localhost:8000/api/measurement")
                    .json(&measurement)
                    .send()
                    .await;
            };
            futures.push(future);
            object_index += 1;
            total += 1;
        }

        futures::future::join_all(futures).await;

        if object_index >= object_count {
            object_index = 0;
            tick += 1;
            println!("Sent measurements for tick {}", tick);
        }
    }

    println!(
        "Finished sending {} measurements with {} futures",
        total, future_count
    );
    Ok(())
}
