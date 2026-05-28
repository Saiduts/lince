// src/parser.rs

//! Extrae valores numéricos o booleanos de un `SensorOutput`
//! para usarlos en cálculos, umbrales o lógica de aplicación.
//!
//! El parser es independiente del formatter: no produce JSON ni
//! formatos de red, solo devuelve los valores en su tipo nativo.

use std::collections::HashMap;
use crate::core::SensorOutput;

/// Errores posibles al parsear un `SensorOutput`.
#[derive(Debug)]
pub enum ParseError {
    /// El formato del texto no coincide con el esperado para este sensor.
    FormatoInvalido(String),
    /// El tipo de `SensorOutput` no es compatible con el método llamado.
    TipoIncompatible,
}

/// Parser de lecturas de sensores a valores nativos.
///
/// Cada método corresponde a un sensor o familia de sensores.
/// El usuario llama solo el que necesita según el sensor que tiene.
///
/// # Ejemplo
/// ```rust
/// use lince::parser::SensorParser;
///
/// let output = sensor.read().unwrap();
///
/// // DHT11 / DHT22 → HashMap con "temperatura" y "humedad"
/// let valores = SensorParser::dht(&output).unwrap();
/// let temp = valores["temperatura"];
/// let hum  = valores["humedad"];
///
/// if temp > 30.0 {
///     println!("Temperatura alta: {}", temp);
/// }
///
/// // DS18B20 → f32
/// let temp = SensorParser::ds18b20(&output).unwrap();
///
/// // MH-RD → bool
/// let mojado = SensorParser::mhrd(&output).unwrap();
/// ```
pub struct SensorParser;

impl SensorParser {
    /// Parsea la salida de un sensor DHT11 o DHT22.
    ///
    /// Devuelve un `HashMap` con las claves `"temperatura"` y `"humedad"`.
    ///
    /// # Formato esperado
    /// `"Temp: 24.3°C, Hum: 58.2%"` (DHT22)
    /// `"Temp: 24°C, Hum: 58%"` (DHT11)
    ///
    /// # Ejemplo
    /// ```rust
    /// let valores = SensorParser::dht(&output)?;
    ///
    /// let temp = valores["temperatura"];  // f32
    /// let hum  = valores["humedad"];      // f32
    ///
    /// let promedio = (temp + temp_anterior) / 2.0;
    /// ```
    pub fn dht(output: &SensorOutput) -> Result<HashMap<String, f32>, ParseError> {
        let texto = Self::extraer_texto(output)?;

        let temp = Self::extraer_valor(texto, "Temp: ", "°C")
            .ok_or_else(|| ParseError::FormatoInvalido(
                format!("No se encontró temperatura en: '{}'", texto)
            ))?;

        let hum = Self::extraer_valor(texto, "Hum: ", "%")
            .ok_or_else(|| ParseError::FormatoInvalido(
                format!("No se encontró humedad en: '{}'", texto)
            ))?;

        let mut map = HashMap::new();
        map.insert("temperatura".to_string(), temp);
        map.insert("humedad".to_string(), hum);

        Ok(map)
    }

    /// Parsea la salida de un sensor DS18B20.
    ///
    /// Devuelve la temperatura como `f32`.
    ///
    /// # Formato esperado
    /// `"24.56 °C"`
    ///
    /// # Ejemplo
    /// ```rust
    /// let temp = SensorParser::ds18b20(&output)?;
    ///
    /// if temp > 85.0 {
    ///     eprintln!("Temperatura crítica: {}", temp);
    /// }
    /// ```
    pub fn ds18b20(output: &SensorOutput) -> Result<f32, ParseError> {
        let texto = Self::extraer_texto(output)?;

        texto
            .trim_end_matches("°C")
            .trim()
            .parse::<f32>()
            .map_err(|_| ParseError::FormatoInvalido(
                format!("No se pudo parsear temperatura DS18B20 de: '{}'", texto)
            ))
    }

    /// Parsea la salida de un sensor MH-RD.
    ///
    /// Devuelve `true` si el sensor detectó humedad ("HÚMEDO"),
    /// `false` si está seco ("SECO").
    ///
    /// # Ejemplo
    /// ```rust
    /// let mojado = SensorParser::mhrd(&output)?;
    ///
    /// if mojado {
    ///     println!("Está lloviendo");
    /// }
    /// ```
    pub fn mhrd(output: &SensorOutput) -> Result<bool, ParseError> {
        match output {
            SensorOutput::Text(s) => match s.as_str() {
                "HÚMEDO" => Ok(true),
                "SECO"   => Ok(false),
                otro     => Err(ParseError::FormatoInvalido(
                    format!("Valor MH-RD desconocido: '{}'", otro)
                )),
            },
            SensorOutput::Bool(b) => Ok(*b),
            _ => Err(ParseError::TipoIncompatible),
        }
    }

    // ------------------------------------------------------------------
    // Helpers internos
    // ------------------------------------------------------------------

    fn extraer_texto(output: &SensorOutput) -> Result<&str, ParseError> {
        match output {
            SensorOutput::Text(s) => Ok(s.as_str()),
            _ => Err(ParseError::TipoIncompatible),
        }
    }

    fn extraer_valor(texto: &str, prefijo: &str, sufijo: &str) -> Option<f32> {
        let inicio = texto.find(prefijo)? + prefijo.len();
        let resto  = &texto[inicio..];
        let fin    = resto.find(sufijo)?;
        resto[..fin].trim().parse::<f32>().ok()
    }
}