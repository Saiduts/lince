use crate::core::traits::sensor::Sensor;
use crate::drivers::pin::PinDriver;
use crate::core::SensorError;
use crate::core::SensorOutput;

/// MhRdSensor: representa un **sensor de lluvia digital (Rain Sensor)**.
///
/// Este sensor interpreta la salida digital (DO) de un módulo de lluvia.
/// Muchos módulos digitales tienen **salida activa baja** (`active_low = true`), 
/// lo que significa que el pin DO está en LOW cuando se detecta agua.
pub struct MhRdSensor {
    /// Controlador GPIO asociado al pin del sensor.
    gpio: PinDriver,
    
    /// Indica si la salida digital es activa en LOW (`true`) o HIGH (`false`).
    /// Por defecto, muchos módulos digitales son `active_low = true`.
    active_low: bool,
}

impl MhRdSensor {
    /// Crea un nuevo sensor de lluvia en el pin BCM indicado.
    ///
    /// # Parámetros
    /// - `pin`: número del pin BCM donde está conectado el sensor.
    /// - `active_low`: `true` si el módulo es activo en LOW (pin LOW = mojado).
    ///
    /// # Retorno
    /// - `Ok(Self)`: sensor inicializado correctamente.
    /// - `Err(SensorError)`: error al inicializar el pin GPIO.
    pub fn new(pin: u8, active_low: bool) -> Result<Self, SensorError> {
        // Inicializa el driver GPIO; mapea errores a SensorError::IoError
        let gpio = PinDriver::new(pin).map_err(|_| SensorError::IoError)?;
        Ok(Self { gpio, active_low })
    }
}

impl Sensor for MhRdSensor {
    type Output = SensorOutput;

    /// Lee el estado actual del sensor de lluvia.
    ///
    /// # Lógica
    /// 1. `read_bool()` devuelve `true` si el pin está en HIGH.
    /// 2. Si el sensor es `active_low`, entonces LOW indica **mojado**.
    /// 3. Devuelve `SensorOutput::Text` con "HÚMEDO" o "SECO".
    ///
    /// # Retorno
    /// - `Ok(SensorOutput)`: estado del sensor.
    /// - `Err(SensorError)`: error durante la lectura.
    fn read(&mut self) -> Result<Self::Output, SensorError> {
        // Lee el estado lógico del pin (HIGH = true, LOW = false)
        let raw_high = self.gpio.read_bool();

        // Ajusta según si el sensor es active_low
        let wet = if self.active_low { !raw_high } else { raw_high };

        // Devuelve el estado como texto
        Ok(SensorOutput::Text(
            if wet { "HÚMEDO".to_string() } else { "SECO".to_string() }
        ))
    }
}
