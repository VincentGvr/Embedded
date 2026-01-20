use tokio::time;
use azure_iot_sdk::{IoTHubClient, DeviceKeyTokenSource, Message};
use rand::Rng;
use rand::distributions::Uniform;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

// Time Interval between messages in milliseconds - change this value if needed
const TIME_INTERVAL_MS: u64 = 400; 

// Max & Min values and variation for each sensor
const TEMP_MIN: f32 = -10.0;
const TEMP_MAX: f32 = 40.0;
const TEMP_VARIATION: f32 = 0.5;

const HUMIDITY_MIN: f32 = 20.0;
const HUMIDITY_MAX: f32 = 100.0;
const HUMIDITY_VARIATION: f32 = 2.0;

const AIR_QUALITY_MIN: f32 = 350.0;
const AIR_QUALITY_MAX: f32 = 1200.0;
const AIR_QUALITY_VARIATION: f32 = 10.0;

const PRESSURE_MIN: f32 = 14.0;
const PRESSURE_MAX: f32 = 15.5;
const PRESSURE_VARIATION: f32 = 0.1;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct EnvironmentalConditions {
    device_id: u32,
    temperature_celsius: f32,
    humidity_percent: f32,
    air_quality_ppm: f32,
    pressure_psi: f32,
}

fn serialize_to_json(data: &EnvironmentalConditions) -> String {
    serde_json::to_string_pretty(data)
        .expect("Failed to serialize environmental conditions to JSON")
}

// Function to ensure value stays within bounds
fn constrain_value(value: f32, min: f32, max: f32) -> f32 {
    value.max(min).min(max)
}

// Function to generate a random variation
fn get_random_variation(variation: f32) -> f32 {
    let mut rng = rand::thread_rng();
    let distribution = Uniform::new(-variation, variation);
    rng.sample(distribution)
}

// Function to initialize random conditions within bounds
fn initialize_random_conditions(device_id: u32) -> EnvironmentalConditions {
    let mut rng = rand::thread_rng();
    EnvironmentalConditions {
        device_id,
        temperature_celsius: rng.gen_range(TEMP_MIN..TEMP_MAX),
        humidity_percent: rng.gen_range(HUMIDITY_MIN..HUMIDITY_MAX),
        air_quality_ppm: rng.gen_range(AIR_QUALITY_MIN..AIR_QUALITY_MAX),
        pressure_psi: rng.gen_range(PRESSURE_MIN..PRESSURE_MAX),
    }
}

// Function to generate new conditions based on previous values
fn generate_random_conditions(device_id: u32, last_conditions: Option<&EnvironmentalConditions>) -> EnvironmentalConditions {
    match last_conditions {
        Some(last) => {
            EnvironmentalConditions {
                device_id,
                temperature_celsius: constrain_value(
                    last.temperature_celsius + get_random_variation(TEMP_VARIATION),
                    TEMP_MIN,
                    TEMP_MAX
                ),
                humidity_percent: constrain_value(
                    last.humidity_percent + get_random_variation(HUMIDITY_VARIATION),
                    HUMIDITY_MIN,
                    HUMIDITY_MAX
                ),
                air_quality_ppm: constrain_value(
                    last.air_quality_ppm + get_random_variation(AIR_QUALITY_VARIATION),
                    AIR_QUALITY_MIN,
                    AIR_QUALITY_MAX
                ),
                pressure_psi: constrain_value(
                    last.pressure_psi + get_random_variation(PRESSURE_VARIATION),
                    PRESSURE_MIN,
                    PRESSURE_MAX
                ),
            }
        },
        None => initialize_random_conditions(device_id)
    }
}

#[tokio::main]
async fn main() -> azure_iot_sdk::Result<()> {
    let iothub_hostname = "<nameofIoTHubInstance>.azure-devices.net";
    let device_id = "vmIoTSimulator";
    let token_source = DeviceKeyTokenSource::new(
        iothub_hostname,
        device_id,
        "<devicePrimaryKey>",
    ).unwrap();

    let mut client = IoTHubClient::new(iothub_hostname, device_id.into(), token_source).await?;

    let mut interval = time::interval(time::Duration::from_millis(TIME_INTERVAL_MS)); //::from_secs(1));
    let mut count: u32 = 0;
    let num_devices: u32 = 3;
    
    let mut device_states: HashMap<u32, EnvironmentalConditions> = HashMap::new();

    loop {
        interval.tick().await;

        for device_number in 1..=num_devices {

            let last_conditions = device_states.get(&device_number);
            let new_conditions = generate_random_conditions(device_number, last_conditions);

            device_states.insert(device_number, new_conditions.clone());

            let json_output = serialize_to_json(&new_conditions);
            let msg = Message::builder()
                .set_body(json_output.as_bytes().to_vec())
                .set_message_id(format!("device{}-msg-{}", device_number, count))
                .build();

            println!("Device {} - Message {} sent: {}", device_number, count, json_output);        
            client.send_message(msg).await;
        }
        count += 1;
    };

}