use serde::{Deserialize, Serialize};

/// Configuración principal del framework IoT.
/// 
/// Esta estructura representa la configuración completa del sistema,
/// que se carga desde un archivo `config.toml` al iniciar.
/// 
/// Contiene subestructuras para cada componente clave:
/// - `device`: Información del dispositivo.
/// - `sensor`: Configuración de sensores.
/// - `actuator`: Configuración de actuadores.
/// - `storage`: Configuración de almacenamiento local.
/// - `communication`: Configuración de comunicación (MQTT, consola, etc.).
/// - `runtime`: Parámetros de ejecución (intervalos, etc.).
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub device: DeviceConfig,
    pub sensor: SensorConfig,
    pub actuator: ActuatorConfig,
    pub storage: StorageConfig,
    pub communication: CommunicationConfig,
    pub runtime: RuntimeConfig,
}

/// Información general del dispositivo.
/// 
/// Usada para identificar el nodo IoT.
#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceConfig {
    /// Nombre único del dispositivo.
    pub name: String,
    /// Ubicación física del dispositivo.
    pub location: String,
}

/// Configuración de un sensor.
/// 
/// Define el tipo de sensor, el pin en el que está conectado
/// y la unidad de medida que reporta.
#[derive(Debug, Serialize, Deserialize)]
pub struct SensorConfig {
    /// Tipo de sensor (ej. `"DHT22"`, `"BME280"`).
    /// 
    /// El prefijo `r#` permite usar palabras reservadas como `type`.
    pub r#type_: String,
    /// Pin físico al que está conectado el sensor.
    pub pin: u8,
    /// Unidad de medida (ej. `"Celsius"`, `"Humidity(%)"`).
    pub unit: String,
}

/// Configuración de un actuador.
/// 
/// Define el tipo de actuador y el pin de control.
#[derive(Debug, Serialize, Deserialize)]
pub struct ActuatorConfig {
    /// Tipo de actuador (ej. `"Relay"`, `"LED"`).
    pub r#type_: String,
    /// Pin físico de control del actuador.
    pub pin: u8,
}

/// Configuración del sistema de comunicación.
/// 
/// Permite definir si se usa MQTT, consola u otro método.
#[derive(Debug, Serialize, Deserialize)]
pub struct CommunicationConfig {
    /// Tipo de comunicación (ej. `"MQTT"`, `"Console"`).
    pub r#type_: String,
    /// Dirección del broker (solo para MQTT).
    pub broker_url: String,
    /// Tópico donde se publican los datos.
    pub topic: String,
}

/// Configuración del sistema de almacenamiento local.
/// 
/// Indica el método y ubicación del almacenamiento.
#[derive(Debug, Serialize, Deserialize)]
pub struct StorageConfig {
    /// Tipo de almacenamiento (ej. `"File"`, `"Database"`).
    pub r#type_: String,
    /// Ruta o ubicación de almacenamiento.
    pub path: String,
}

/// Configuración de parámetros de ejecución.
/// 
/// Controla la frecuencia del bucle principal.
#[derive(Debug, Serialize, Deserialize)]
pub struct RuntimeConfig {
    /// Intervalo en milisegundos entre cada ciclo de lectura/envío.
    pub interval_ms: u64,
}
