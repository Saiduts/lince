
//! API pública del framework IoT para dispositivos gateway.
//!
//! Este framework permite construir soluciones IoT modulares, ligeras y extensibles,
//! con soporte para sensores, almacenamiento, conectividad MQTT.


// Núcleo del framework: control de flujo, ciclo de vida, lógica principal
pub mod core;

// Sensores fisicos
pub mod devices{
    pub mod sensors;
}

pub mod storage;

// Comunicación de red (MQTT, HTTP.)
pub mod network;

//Drivers GPIO.
pub mod drivers;

// Reexportar interfaces clave para una API unificada
pub use core::traits::{
    communicator::Communicator,
    sensor::Sensor,
    storage::Storage,
};
pub use network::console::ConsoleCommunicator;
pub use network::mqtt::MqttCommunicator;


