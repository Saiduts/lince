use crate::core::SensorError;
use crate::drivers::gpio::GpioDriver;
use embedded_hal::digital::v2::{InputPin, OutputPin};
use std::time::{Duration, Instant};
use std::thread;

/// DhtBase: base para sensores DHT11/DHT22.
///
/// Esta estructura maneja la comunicación de bajo nivel con un sensor DHT
/// usando un pin digital GPIO. Proporciona métodos para iniciar la secuencia,
/// leer bits y validar la información recibida.
pub struct DhtBase {
    /// Driver GPIO asociado al pin donde está conectado el sensor.
    pin: GpioDriver,
}

impl DhtBase {
    /// Crea una nueva instancia del sensor DHT en el pin BCM indicado.
    ///
    /// # Parámetros
    /// - `pin_number`: número del pin BCM donde está conectado el sensor.
    ///
    /// # Retorno
    /// - `Ok(Self)`: instancia inicializada correctamente.
    /// - `Err(SensorError)`: error al inicializar el GPIO.
    pub fn new(pin_number: u8) -> Result<Self, SensorError> {
        Ok(Self {
            pin: GpioDriver::new(pin_number)?,
        })
    }

    /// Inicia la secuencia de comunicación con el sensor.
    ///
    /// Esta secuencia consiste en:
    /// 1. Pull LOW del pin por 20 ms para indicar inicio al sensor.
    /// 2. Pull HIGH por 30 µs.
    /// 3. Cambio del pin a modo entrada para escuchar la respuesta del sensor.
    ///
    /// # Retorno
    /// - `Ok(())`: secuencia iniciada correctamente.
    /// - `Err(SensorError)`: error en el manejo del pin.
    pub fn iniciar_secuencia(&mut self) -> Result<(), SensorError> {
        // === Señal de inicio ===
        let _ = self.pin.set_low();
        thread::sleep(Duration::from_millis(20));
        let _ = self.pin.set_high();
        spin_sleep::sleep(Duration::from_micros(30));

        // === Cambiar a modo entrada para escuchar respuesta ===
        // (tu driver debe tener este método)
        self.pin.set_mode(rppal::gpio::Mode::Input);

        Ok(())
    }

    /// Espera hasta que el pin alcance el nivel lógico deseado o se agote el timeout.
    ///
    /// # Parámetros
    /// - `pin`: referencia al pin GPIO.
    /// - `nivel`: `true` para HIGH, `false` para LOW.
    /// - `timeout_us`: tiempo máximo de espera en microsegundos.
    ///
    /// # Retorno
    /// - `true` si el nivel fue alcanzado antes del timeout.
    /// - `false` si ocurrió timeout o error de lectura.
    pub fn esperar_nivel(pin: &GpioDriver, nivel: bool, timeout_us: u64) -> bool {
        let start = Instant::now();
        let timeout = Duration::from_micros(timeout_us);

        loop {
            match pin.is_high() {
                Ok(state) => {
                    if state == nivel {
                        return true;
                    }
                }
                Err(_) => return false,
            }

            if start.elapsed() > timeout {
                return false;
            }

            spin_sleep::sleep(Duration::from_micros(1));
        }
    }

    /// Lee los 40 bits de datos enviados por el sensor.
    ///
    /// La comunicación DHT consiste en 5 bytes (40 bits):
    /// - byte 0: humedad entera
    /// - byte 1: humedad decimal
    /// - byte 2: temperatura entera
    /// - byte 3: temperatura decimal
    /// - byte 4: checksum
    ///
    /// # Retorno
    /// - `Ok([u8;5])`: datos recibidos correctamente.
    /// - `Err(SensorError)`: timeout u error en la lectura.
    pub fn leer_bits(&mut self) -> Result<[u8; 5], SensorError> {
        let mut data = [0u8; 5];

        // === Secuencia inicial de sincronización ===
        if !Self::esperar_nivel(&self.pin, false, 100) {
            return Err(SensorError::Timeout);
        }
        if !Self::esperar_nivel(&self.pin, true, 100) {
            return Err(SensorError::Timeout);
        }
        if !Self::esperar_nivel(&self.pin, false, 100) {
            return Err(SensorError::Timeout);
        }

        // === Leer los 40 bits ===
        for byte_idx in 0..5 {
            for bit_idx in 0..8 {
                // Esperar pulso alto
                if !Self::esperar_nivel(&self.pin, true, 100) {
                    return Err(SensorError::Timeout);
                }

                // Medir duración del pulso
                let start = Instant::now();
                if !Self::esperar_nivel(&self.pin, false, 100) {
                    return Err(SensorError::Timeout);
                }

                let dur = start.elapsed();
                // Pulso >40µs = 1, <40µs = 0
                if dur.as_micros() > 40 {
                    data[byte_idx] |= 1 << (7 - bit_idx);
                }
            }
        }

        Ok(data)
    }

    /// Valida el checksum de los datos leídos.
    ///
    /// # Parámetros
    /// - `data`: array de 5 bytes del sensor.
    ///
    /// # Retorno
    /// - `Ok(())` si el checksum es correcto.
    /// - `Err(SensorError::InvalidData)` si hay inconsistencia.
    pub fn validar_checksum(data: &[u8; 5]) -> Result<(), SensorError> {
        let checksum = data[0]
            .wrapping_add(data[1])
            .wrapping_add(data[2])
            .wrapping_add(data[3]);
        if checksum != data[4] {
            Err(SensorError::InvalidData)
        } else {
            Ok(())
        }
    }
}
