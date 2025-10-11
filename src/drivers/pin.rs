use rppal::gpio::{Gpio, IoPin, Mode, Level};
use embedded_hal::digital::v2::{InputPin, OutputPin};
use crate::core::SensorError;

/// PinDriver: driver mínimo y seguro para controlar un pin digital.
///
/// Este driver proporciona una **interfaz segura y consistente** para trabajar con pines digitales,
/// compatible con `rppal` y con los traits de `embedded-hal` (`InputPin` y `OutputPin`).
/// Se encarga de la inicialización del pin, lectura, escritura y cambio de modo.
pub struct PinDriver {
    /// Pin gestionado por RPPAL.
    pub pin: IoPin,

    /// Número del pin BCM usado.
    pub pin_number: u8,
}

impl PinDriver {
    /// Crea un nuevo `PinDriver` para un pin BCM específico.
    ///
    /// # Parámetros
    /// - `pin_number`: número del pin BCM donde se conectará el dispositivo.
    ///
    /// # Retorno
    /// - `Ok(Self)` si se inicializa correctamente el pin.
    /// - `Err(SensorError::IoError)` si falla la inicialización del pin.
    pub fn new(pin_number: u8) -> Result<Self, SensorError> {
        let gpio = Gpio::new().map_err(|_| SensorError::IoError)?;
        let pin = gpio
            .get(pin_number)
            .map_err(|_| SensorError::IoError)?
            .into_io(Mode::Input);

        Ok(Self { pin, pin_number })
    }

    /// Lee el nivel lógico actual del pin.
    ///
    /// # Retorno
    /// - `Level::High` si el pin está en alto.
    /// - `Level::Low` si el pin está en bajo.
    pub fn read_level(&self) -> Level {
        self.pin.read()
    }

    /// Devuelve true si el pin está en alto.
    pub fn read_bool(&self) -> bool {
        self.read_level() == Level::High
    }

    /// Cambia el modo del pin (Input/Output).
    ///
    /// # Parámetros
    /// - `mode`: `Mode::Input` o `Mode::Output`.
    pub fn set_mode(&mut self, mode: Mode) {
        self.pin.set_mode(mode);
    }

    /// Escribe un nivel lógico en el pin.
    ///
    /// # Parámetros
    /// - `level`: `Level::High` o `Level::Low`.
    pub fn write_level(&mut self, level: Level) {
        self.pin.write(level);
    }
}

// --------------------------------------------------------------------
// Implementación de traits de `embedded-hal`
// --------------------------------------------------------------------

impl InputPin for PinDriver {
    type Error = core::convert::Infallible;

    /// Retorna true si el pin está en alto.
    fn is_high(&self) -> Result<bool, Self::Error> {
        Ok(self.pin.read() == Level::High)
    }

    /// Retorna true si el pin está en bajo.
    fn is_low(&self) -> Result<bool, Self::Error> {
        Ok(self.pin.read() == Level::Low)
    }
}

impl OutputPin for PinDriver {
    type Error = core::convert::Infallible;

    /// Configura el pin en alto.
    ///
    /// Este método asegura que el pin esté en modo salida antes de escribir.
    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.pin.set_mode(Mode::Output);
        self.pin.write(Level::High);
        Ok(())
    }

    /// Configura el pin en bajo.
    ///
    /// Este método asegura que el pin esté en modo salida antes de escribir.
    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.pin.set_mode(Mode::Output);
        self.pin.write(Level::Low);
        Ok(())
    }
}
