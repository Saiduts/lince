// src/lib.rs

//! API pública del framework IoT para dispositivos gateway.
//!
//! Este framework permite construir soluciones IoT modulares, ligeras y extensibles,
//! con soporte para sensores, almacenamiento, conectividad MQTT y reintento automático
//! por desconexión.

pub mod core;

pub mod devices {
    pub mod sensors;
}

pub mod storage;
pub mod network;
pub mod drivers;
pub mod parser;

// Traits principales
pub use core::traits::{
    communicator::Communicator,
    sensor::Sensor,
    storage::Storage,
};

// Communicators
pub use network::console::ConsoleCommunicator;
pub use network::mqtt::MqttCommunicator;


// Formato Smart Campus
pub use network::smart_campus::{SmartCampusFormatter, SmartCampusHeader, Metric};
pub use parser::SensorParser;