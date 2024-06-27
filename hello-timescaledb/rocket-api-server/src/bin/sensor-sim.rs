use rocket::tokio;
use rocket_api_server::Measurement;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    for index in 0..1000 {
        let measurement = Measurement {
            measurement_id: None,
            object_uuid: uuid::Uuid::new_v4(),
            sensor_uuid: uuid::Uuid::new_v4(),
            measured_at: chrono::Utc::now().naive_utc(),
            latitude: 10.0 + index as f32,
            longitude: 2.0,
            object_length: 10.0,
        };

        dbg!(&measurement);

        let client = reqwest::Client::new();
        let response = client
            .post("http://localhost:8000/api/measurement")
            .json(&measurement)
            .send()
            .await?;

        println!("{:?}", response);
        let response_measurement = serde_json::from_str::<Measurement>(&response.text().await?)?;
        println!("{:?}", response_measurement);
    }
    Ok(())
}
