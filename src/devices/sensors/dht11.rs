use crate::core::{SensorError, SensorOutput};
use crate::core::traits::sensor::Sensor;
use crate::drivers::dht::DhtBase;

/// Implementación del **sensor DHT11** para el framework IoT.
///
/// Este módulo encapsula la lógica de comunicación con el sensor DHT11
/// utilizando una capa base (`DhtBase`) que maneja la secuencia de lectura
/// a nivel de protocolo (sincronización, captura de bits, y verificación de checksum).
///
/// El DHT11 mide **temperatura y humedad relativa**, devolviendo ambos valores
/// en una lectura formateada como texto.
///
/// # Ejemplo
/// ```
/// use iot_framework::devices::sensors::dht11::Dht11Sensor;
///
/// let mut sensor = Dht11Sensor::new(4).unwrap(); // GPIO4
/// let lectura = sensor.read().unwrap();
/// println!("{:?}", lectura);
/// ```
pub struct Dht11Sensor {
    /// Capa base de comunicación con el sensor DHT (maneja protocolo y tiempos).
    base: DhtBase,
}

impl Dht11Sensor {
    /// Crea una nueva instancia del sensor DHT11.
    ///
    /// # Parámetros
    /// - `pin`: Número de pin GPIO conectado a la línea de datos del sensor.
    ///
    /// # Retorna
    /// - `Ok(Self)` si la inicialización fue correcta.
    /// - `Err(SensorError)` si ocurre un error al configurar el pin.
    pub fn new(pin: u8) -> Result<Self, SensorError> {
        Ok(Self { base: DhtBase::new(pin)? })
    }
}

impl Sensor for Dht11Sensor {
    /// Tipo de salida: [`SensorOutput`] (texto con temperatura y humedad).
    type Output = SensorOutput;

    /// Lee una muestra del sensor DHT11.
    ///
    /// El proceso realiza la secuencia de inicialización, lectura de bits,
    /// validación de checksum y conversión de los datos crudos en valores legibles.
    ///
    /// # Flujo
    /// 1. Envía la señal de inicio al sensor (`iniciar_secuencia`).
    /// 2. Lee los 40 bits de respuesta (`leer_bits`).
    /// 3. Verifica integridad con `validar_checksum`.
    /// 4. Interpreta los bytes como humedad y temperatura.
    ///
    /// # Retorna
    /// - `Ok(SensorOutput::Text(...))` con la lectura formateada.
    /// - `Err(SensorError)` si ocurre algún fallo durante la lectura.
    fn read(&mut self) -> Result<Self::Output, SensorError> {
        self.base.iniciar_secuencia()?;
        let data = self.base.leer_bits()?;
        DhtBase::validar_checksum(&data)?;

        let humidity = data[0];
        let temperature = data[2] as i8;

        Ok(SensorOutput::Text(format!("Temp: {}°C, Hum: {}%", temperature, humidity)))
    }
}
