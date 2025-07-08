use crate::config::loader::load_config;

fn main() {
    match load_config("config.toml") {
        Ok(config) => {
            println!("Device name: {}", config.device.name);
            println!("Device location: {}", config.device.location);
            println!("Sensor type: {}", config.sensor.r#type_);
            println!("Sensor pin: {}", config.sensor.pin);
            println!("Sensor unit: {}", config.sensor.unit);
            println!("Actuator type: {}", config.actuator.r#type_);
            println!("Actuator pin: {}", config.actuator.pin);
            println!("Communication type: {}", config.communication.r#type_);
            println!("Communication broker URL: {}", config.communication.broker_url);
            println!("Communication topic: {}", config.communication.topic);
            println!("Storage type: {}", config.storage.r#type_);
            println!("Storage path: {}", config.storage.path);
            println!("Runtime interval (ms): {}", config.runtime.interval_ms);
        }
        Err(e) => {
            println!("Error loading config: {}", e);
        }
    }
}