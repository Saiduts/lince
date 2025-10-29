use crate::core::{SensorError, SensorOutput};
use crate::core::traits::sensor::Sensor;
use crate::devices::sensors::dht::DhtBase;

/// Implementación del **sensor DHT22** para el framework IoT.
///
/// Este módulo define la lógica específica para leer datos del sensor DHT22,
/// utilizando la capa base [`DhtBase`] que gestiona la secuencia del protocolo
/// (sincronización, captura de bits y validación del checksum).
///
/// El DHT22 ofrece **mayor precisión y rango de medición** que el DHT11,
/// entregando 16 bits para temperatura y 16 bits para humedad.
///
/// # Características
/// - Temperatura: -40 a 80 °C  
/// - Humedad: 0–100 % RH  
/// - Resolución: 0.1 °C / 0.1 %
///
/// # Ejemplo
/// ```
/// use lince::devices::sensors::dht22::Dht22Sensor;
///
/// let mut sensor = Dht22Sensor::new(23).unwrap(); // GPIO23
/// let lectura = sensor.read().unwrap();
/// println!("{:?}", lectura);
/// ```
pub struct Dht22Sensor {
    /// Capa base para comunicación a nivel de protocolo DHT.
    base: DhtBase,
}

impl Dht22Sensor {
    /// Crea una nueva instancia del sensor DHT22.
    ///
    /// # Parámetros
    /// - `pin`: Número del pin GPIO conectado al pin de datos del sensor.
    ///
    /// # Retorna
    /// - `Ok(Self)` si el sensor se inicializa correctamente.
    /// - `Err(SensorError)` si ocurre un error en la configuración del pin.
    pub fn new(pin: u8) -> Result<Self, SensorError> {
        Ok(Self { base: DhtBase::new(pin)? })
    }
}

impl Sensor for Dht22Sensor {
    /// Tipo de salida del sensor: [`SensorOutput`] (lectura textual formateada).
    type Output = SensorOutput;

    /// Lee una muestra de temperatura y humedad desde el DHT22.
    ///
    /// Este método ejecuta la secuencia de comunicación, verifica la integridad
    /// de los datos mediante checksum y decodifica los valores físicos.
    ///
    /// # Flujo
    /// 1. Inicia la secuencia de comunicación (`iniciar_secuencia`).
    /// 2. Lee los 40 bits de respuesta (`leer_bits`).
    /// 3. Valida el checksum (`validar_checksum`).
    /// 4. Interpreta los 16 bits de humedad y 16 bits de temperatura.
    /// 5. Aplica corrección de signo si la temperatura es negativa.
    ///
    /// # Retorna
    /// - `Ok(SensorOutput::Text(...))` con los valores formateados.
    /// - `Err(SensorError)` si ocurre un fallo durante la lectura o validación.
    fn read(&mut self) -> Result<Self::Output, SensorError> {
        self.base.iniciar_secuencia()?;
        let data = self.base.leer_bits()?;
        DhtBase::validar_checksum(&data)?;

        // DHT22: 16 bits para humedad y 16 bits para temperatura
        let humidity = ((data[0] as u16) << 8 | data[1] as u16) as f32 / 10.0;
        let mut temperature = ((data[2] as u16) << 8 | data[3] as u16) as f32 / 10.0;

        // Bit de signo: si está activo, la temperatura es negativa
        if data[2] & 0x80 != 0 {
            temperature *= -1.0;
        }

        Ok(SensorOutput::Text(format!("Temp: {:.1}°C, Hum: {:.1}%", temperature, humidity)))
    }
}
