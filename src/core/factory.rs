use crate::config::SensorConfig;
use crate::core::sensor::Sensor;
use crate::core::types::SensorError;
use crate::drivers::dht::{DhtSensor, DhtMetric};

pub fn build_sensor(scfg: &SensorConfig) -> Result<Box<dyn Sensor>, SensorError> {
    match scfg.kind.as_str() {
        "dht" => {
            let pin   = scfg.pin.ok_or_else(|| SensorError::InvalidConfig("falta pin".into()))?;
            let model = scfg.model.clone().ok_or_else(|| SensorError::InvalidConfig("falta model (DHT11/DHT22)".into()))?;
            let metric = match scfg.metric.as_deref() {
                Some("temperature") => DhtMetric::Temperature,
                Some("humidity")    => DhtMetric::Humidity,
                other => return Err(SensorError::InvalidConfig(format!("metric no válida: {:?}", other))),
            };
            let unit = scfg.unit.clone().unwrap_or_else(|| {
                if matches!(metric, DhtMetric::Temperature) { "Celsius".into() } else { "%".into() }
            });
            Ok(Box::new(DhtSensor::new(&scfg.id, pin as u32, model, metric, unit)))
        }
        // ... otros tipos: "simulated", etc.
        other => Err(SensorError::InvalidConfig(format!("tipo de sensor no soportado: {other}"))),
    }
}
