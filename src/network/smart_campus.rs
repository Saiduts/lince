// src/network/smart_campus.rs

//! Formatea valores de sensores al protocolo JSON Smart Campus.
//!
//! El formatter recibe valores ya extraídos (no `SensorOutput` crudo),
//! lo que permite usarlo después del parser o con valores calculados.

use std::collections::HashMap;

/// Encabezado del mensaje Smart Campus.
#[derive(Debug, Clone)]
pub struct SmartCampusHeader {
    pub device_id: String,
    pub topic:     String,
}

impl SmartCampusHeader {
    pub fn new(device_id: &str, topic: &str) -> Self {
        Self {
            device_id: device_id.to_string(),
            topic:     topic.to_string(),
        }
    }
}

/// Una métrica individual dentro del payload Smart Campus.
#[derive(Debug, Clone)]
pub struct Metric {
    pub measurement: String,
    pub value:       f64,
}

impl Metric {
    pub fn new(measurement: &str, value: f64) -> Self {
        Self {
            measurement: measurement.to_string(),
            value,
        }
    }
}

/// Formatea valores numéricos al JSON estructurado Smart Campus.
///
/// El formatter no sabe nada de `SensorOutput` ni de sensores concretos.
/// Recibe valores ya extraídos, que pueden venir del parser, de un cálculo
/// o de cualquier otra fuente.
///
/// ```json
/// {
///   "header": { "deviceId": "raspberry-lince", "topic": "device/messages" },
///   "metrics": [
///     { "measurement": "temperatura", "value": 24.50 },
///     { "measurement": "humedad",     "value": 60.10 }
///   ]
/// }
/// ```
///
/// # Flujo típico con el parser
/// ```rust
/// // 1. Sensor lee
/// let output = sensor.read()?;
///
/// // 2. Parser extrae valores para cálculos
/// let valores = SensorParser::dht(&output)?;
/// let temp = valores["temperatura"];
/// let hum  = valores["humedad"];
///
/// // 3. Formatter convierte a JSON para enviar
/// let json = formatter.desde_mapa(&valores);
/// mqtt.send(json.as_bytes())?;
/// ```
pub struct SmartCampusFormatter {
    header: SmartCampusHeader,
}

impl SmartCampusFormatter {
    pub fn new(header: SmartCampusHeader) -> Self {
        Self { header }
    }

    /// Formatea una lista de métricas como JSON Smart Campus.
    ///
    /// Es el método base. Los demás lo llaman internamente.
    ///
    /// ```rust
    /// let metrics = vec![
    ///     Metric::new("temperatura", 24.5),
    ///     Metric::new("humedad", 60.0),
    /// ];
    /// let json = formatter.format(&metrics);
    /// ```
    pub fn format(&self, metrics: &[Metric]) -> String {
        let metrics_json: Vec<String> = metrics
            .iter()
            .map(|m| format!(
                r#"{{"measurement":"{}","value":{:.2}}}"#,
                m.measurement, m.value
            ))
            .collect();

        format!(
            r#"{{"header":{{"deviceId":"{}","topic":"{}"}},"metrics":[{}]}}"#,
            self.header.device_id,
            self.header.topic,
            metrics_json.join(",")
        )
    }

    /// Formatea directamente desde un `HashMap<String, f32>`,
    /// que es exactamente lo que devuelve `SensorParser::dht()`.
    ///
    /// El orden de las métricas en el JSON sigue el orden del mapa.
    ///
    /// # Ejemplo con DHT
    /// ```rust
    /// let valores = SensorParser::dht(&output)?;
    /// // valores = {"temperatura": 24.5, "humedad": 60.0}
    ///
    /// let json = formatter.desde_mapa(&valores);
    /// ```
    ///
    /// # Ejemplo con valores calculados
    /// ```rust
    /// let mut calculados = HashMap::new();
    /// calculados.insert("temp_promedio".to_string(), promedio);
    /// calculados.insert("temp_maxima".to_string(), maxima);
    ///
    /// let json = formatter.desde_mapa(&calculados);
    /// ```
    pub fn desde_mapa(&self, valores: &HashMap<String, f32>) -> String {
        let metrics: Vec<Metric> = valores
            .iter()
            .map(|(k, v)| Metric::new(k, *v as f64))
            .collect();

        self.format(&metrics)
    }

    /// Formatea un valor escalar (DS18B20, cualquier `f32` simple).
    ///
    /// # Ejemplo con DS18B20
    /// ```rust
    /// let temp = SensorParser::ds18b20(&output)?;
    /// let json = formatter.desde_valor("temperatura", temp);
    /// ```
    ///
    /// # Ejemplo con valor calculado
    /// ```rust
    /// let temp = SensorParser::ds18b20(&output)?;
    /// let corregida = temp - 1.5;  // corrección de calibración
    /// let json = formatter.desde_valor("temperatura_corregida", corregida);
    /// ```
    pub fn desde_valor(&self, nombre: &str, valor: f32) -> String {
        self.format(&[Metric::new(nombre, valor as f64)])
    }

    /// Formatea un valor booleano (MH-RD).
    ///
    /// Convierte `true` → `1.0`, `false` → `0.0`.
    ///
    /// # Ejemplo
    /// ```rust
    /// let mojado = SensorParser::mhrd(&output)?;
    /// let json = formatter.desde_bool("lluvia", mojado);
    /// ```
    pub fn desde_bool(&self, nombre: &str, valor: bool) -> String {
        self.format(&[Metric::new(nombre, if valor { 1.0 } else { 0.0 })])
    }
}