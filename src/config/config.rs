use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub device: Device,
    pub sensor: Sensor,
    pub actuator: Actuator,
    pub storage: Storage,
    pub communication: Communication,
    pub runtime: Runtime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceConfig {
    pub name: String,
    pub location: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SensorConfig {
    pub r#type_: String, //r#type_ es un alias para type
    pub pin: u8,
    pub unit: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActuatorConfig {
    pub r#type_: String,
    pub pin: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommunicationConfig {
    pub r#type_: String,
    pub broker_url: String,
    pub topic: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageConfig {
    pub r#type_: String,
    pub path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RuntimeConfig {
    pub interval_ms: u64,
}