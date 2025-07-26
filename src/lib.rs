//pub fn add(left: u64, right: u64) -> u64 {
//    left + right
//}

//#[cfg(test)]
//mod tests {
//    use super::*;

//    #[test]
//    fn it_works() {
//        let result = add(2, 2);
//        assert_eq!(result, 4);
  //  }
//}


//! API pública del framework IoT para dispositivos gateway.
//!
//! Este framework permite construir soluciones IoT modulares, ligeras y extensibles,
//! con soporte para sensores, almacenamiento, conectividad MQTT/AMQP, configuración externa
//! y despliegue en contenedores.

// Módulo de configuración basado en archivos .toml
pub mod config;

// Núcleo del framework: control de flujo, ciclo de vida, lógica principal
pub mod core;

// Drivers para sensores y actuadores (GPIO, I2C, SPI, etc.)
pub mod drivers;

// Comunicación de red (MQTT, AMQP, HTTP, WebSockets, etc.)
pub mod network;

// Soporte específico para la plataforma objetivo (Raspberry Pi, Jetson, etc.)
pub mod platform;

//Simulador de sensor
pub mod simulated_sensor;


// Reexportar interfaces clave si se desea una API unificada
pub use core::FrameworkCore;
pub use config::AppConfig;
pub use drivers::SensorDriver;
pub use network::CommClient;


//Funciones públicas que acceden a SimulatedSensor desde el exterior
use simulated_sensor::SimulatedSensor;

pub fn get_fake_data() -> (f32, f32, u64) {
    let sensor = SimulatedSensor::new();
    (
        sensor.read_temperature(),
        sensor.read_humidity(),
        sensor.read_timestamp(),
    )
}
