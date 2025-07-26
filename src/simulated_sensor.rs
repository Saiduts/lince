use rand::Rng;
use std::time::{SystemTime, UNIX_EPOCH};

/// SimulatedSensor representa un sensor de prueba que genera valores aleatorios.
pub struct SimulatedSensor;

impl SimulatedSensor {
    /// Crea una nueva instancia del sensor simulado.
    pub fn new() -> Self {
        SimulatedSensor
    }

    /// Retorna un valor simulado de temperatura en °C.
    pub fn read_temperature(&self) -> f32 {
        let mut rng = rand::thread_rng();
        rng.gen_range(18.0..30.0)
    }

    /// Retorna un valor simulado de humedad en %.
    pub fn read_humidity(&self) -> f32 {
        let mut rng = rand::thread_rng();
        rng.gen_range(40.0..80.0)
    }

    /// Devuelve un timestamp actual.
    pub fn read_timestamp(&self) -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}
