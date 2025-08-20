use rand::Rng;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::core::traits::sensor::{Sensor, SensorError};

#[derive(Debug, Clone)]
pub struct SensorData {
    pub temperature: f32,
    pub humidity: f32,
    pub timestamp: u64,
}

/// `SimulatedSensor` es un sensor virtual utilizado para pruebas.
/// 
/// Este sensor genera valores aleatorios para temperatura y humedad,
/// y devuelve marcas de tiempo actuales.  
/// Se utiliza principalmente en entornos de desarrollo y simulación
/// para verificar la lógica del sistema sin requerir hardware real.
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
    /// El rango es **40.0 a 80.0** por ciento.
    pub fn read_humidity(&self) -> f32 {
        let mut rng = rand::thread_rng();
        rng.gen_range(40.0..80.0)
    }

    /// Devuelve un timestamp actual.
    /// Devuelve un **timestamp** en segundos desde el 1 de enero de 1970 (UNIX epoch).
    pub fn read_timestamp(&self) -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}

impl Sensor for SimulatedSensor {
    type Output = SensorData;

    fn read(&mut self) -> Result<Self::Output, SensorError> {
        Ok(SensorData {
            temperature: self.read_temperature(),
            humidity: self.read_humidity(),
            timestamp: self.read_timestamp(),
        })
    }
}