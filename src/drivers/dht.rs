use std::time::{SystemTime, UNIX_EPOCH};
use dht_embedded::{Dht11, Dht22, DhtSensor as _ , NoopInterruptControl};
use gpio_cdev::{Chip, LineRequestFlags};
use linux_embedded_hal::{CdevPin, Delay};

use crate::core::sensor::Sensor;
use crate::core::types::{Measurement, SensorError};

/// Métrica a leer del DHT.
#[derive(Debug, Clone)]
pub enum DhtMetric { Temperature, Humidity }

/// Driver DHT11/DHT22 (un sensor lógico = una métrica: temp o hum)
pub struct DhtSensor {
    id: String,
    pin: u32,           // número de línea en gpiochip (BCM)
    model: String,      // "DHT11" | "DHT22"
    metric: DhtMetric,  // qué vamos a devolver: temp o hum
    unit: String,       // "Celsius" | "%"
}

impl DhtSensor {
    pub fn new(
        id: impl Into<String>,
        pin: u32,
        model: impl Into<String>,
        metric: DhtMetric,
        unit: impl Into<String>,
    ) -> Self {
        Self { id: id.into(), pin, model: model.into(), metric, unit: unit.into() }
    }

    fn read_raw(&self) -> Result<(f32, f32), SensorError> {
        // Abre gpiochip0 y solicita la línea como IN/OUT (requerido por dht)
        let mut gpiochip = Chip::new("/dev/gpiochip0")
            .map_err(|e| SensorError::ReadFailed(format!("gpiochip: {e}")))?;
        let line = gpiochip.get_line(self.pin)
            .map_err(|e| SensorError::ReadFailed(format!("get_line {}: {e}", self.pin)))?;
        let handle = line.request(
            LineRequestFlags::INPUT | LineRequestFlags::OUTPUT,
            1,
            "dht-sensor",
        ).map_err(|e| SensorError::ReadFailed(format!("request line: {e}")))?;
        let pin = CdevPin::new(handle)
            .map_err(|e| SensorError::ReadFailed(format!("CdevPin: {e}")))?;

        // Selecciona driver según modelo y lee
        let reading = match self.model.as_str() {
            "DHT22" => Dht22::new(NoopInterruptControl, Delay, pin).read(),
            "DHT11" => Dht11::new(NoopInterruptControl, Delay, pin).read(),
            other => return Err(SensorError::InvalidConfig(format!("Modelo no soportado: {other}"))),
        }.map_err(|e| SensorError::ReadFailed(format!("DHT read: {e}")))?;

        Ok((reading.temperature(), reading.humidity()))
    }
}

impl Sensor for DhtSensor {
    fn id(&self) -> &str { &self.id }
    fn kind(&self) -> &str { "dht" }
    fn unit(&self) -> &str { &self.unit }

    fn read(&self) -> Result<Measurement, SensorError> {
        let (t, h) = self.read_raw()?;
        let (value, kind, unit) = match self.metric {
            DhtMetric::Temperature => (t as f64, "temperature", self.unit.clone()),
            DhtMetric::Humidity    => (h as f64, "humidity", self.unit.clone()),
        };
        let ts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        Ok(Measurement {
            sensor_id: self.id.clone(),
            kind: kind.into(),
            value,
            unit,
            timestamp_secs: ts,
        })
    }
}