use crate::core::types::{SensorOutput, SensorError};

/// Trait que define el comportamiento general de un **sensor** dentro del framework IoT.
///
/// Un sensor es cualquier dispositivo capaz de **medir una variable física o lógica**
/// y entregar un valor procesable por el sistema, como temperatura, humedad, presión,
/// luminosidad, entre otros.
///
/// Este trait proporciona una interfaz unificada para interactuar con diferentes tipos de sensores,
/// permitiendo que el runtime del framework los gestione de manera genérica.
///
/// # Associated Types
/// - `Output`: Tipo de dato que el sensor produce (por ejemplo `f32`, `bool`, o una estructura personalizada).
///
/// # Ejemplo
/// ```
/// use crate::core::traits::sensor::Sensor;
/// use crate::core::types::{SensorOutput, SensorError};
///
/// struct DummySensor;
///
/// impl Sensor for DummySensor {
///     type Output = f32;
///
///     fn read(&mut self) -> Result<SensorOutput, SensorError> {
///         Ok(SensorOutput::new("temperatura".into(), 24.5))
///     }
/// }
/// ```
pub trait Sensor {
    /// Tipo de dato que el sensor devuelve al ser leído.
    /// Puede representar un valor numérico, lógico o estructurado.
    type Output;

    /// Lee el valor actual del sensor.
    ///
    /// Retorna un [`SensorOutput`] si la lectura fue exitosa,
    /// o un [`SensorError`] si ocurrió algún problema durante la operación.
    ///
    /// Este método se ejecuta típicamente dentro del ciclo de lectura del runtime,
    /// y su resultado puede ser almacenado o enviado mediante un comunicador.
    fn read(&mut self) -> Result<SensorOutput, SensorError>;
}
