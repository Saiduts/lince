use serde::Serialize;

/// Representa los **datos producidos por un sensor** en el framework IoT.
///
/// Este tipo enum permite manejar lecturas de sensores de distintos tipos de datos
/// sin necesidad de definir una estructura separada para cada caso.
/// 
/// Gracias a esta abstracción, los módulos del framework (almacenamiento, comunicación, etc.)
/// pueden trabajar con sensores heterogéneos de manera unificada.
///
/// # Variantes
/// - `Bool(bool)`: Valor lógico (por ejemplo, detección de movimiento).
/// - `Int(i64)`: Valor entero (por ejemplo, conteo de pulsos, presión en Pa).
/// - `Float(f32)`: Valor decimal (por ejemplo, temperatura, humedad, voltaje).
/// - `Text(String)`: Texto libre (por ejemplo, identificadores o etiquetas).
/// - `Bytes(Vec<u8>)`: Datos binarios sin procesar.
///
/// # Ejemplo
/// ```
/// use crate::core::types::SensorOutput;
///
/// let lectura = SensorOutput::Float(23.7);
///
/// match lectura {
///     SensorOutput::Float(v) => println!("Temperatura: {} °C", v),
///     _ => println!("Tipo de dato no esperado"),
/// }
/// ```
#[derive(Debug, Clone, Serialize)]
pub enum SensorOutput {
    /// Valor booleano (por ejemplo: presencia detectada o no).
    Bool(bool),

    /// Valor entero (por ejemplo: conteo o nivel en unidades discretas).
    Int(i64),

    /// Valor flotante (por ejemplo: temperatura o humedad relativa).
    Float(f32),

    /// Cadena de texto (por ejemplo: mensaje o estado textual).
    Text(String),

    /// Datos binarios sin procesar (por ejemplo: lecturas en bytes crudos).
    Bytes(Vec<u8>),
}

/// Representa los **errores comunes que pueden ocurrir al interactuar con un sensor**.
///
/// Este tipo enum permite identificar la causa de un fallo en la lectura
/// sin depender del tipo de sensor o protocolo usado.
///
/// # Variantes
/// - `IoError`: Fallo de entrada/salida al intentar acceder al sensor.
/// - `Timeout`: El sensor no respondió dentro del tiempo esperado.
/// - `InvalidData`: Los datos recibidos son inválidos o corruptos.
/// - `InitializationError`: El sensor no pudo inicializarse correctamente.
///
/// # Ejemplo
/// ```
/// use crate::core::types::SensorError;
///
/// fn leer_sensor() -> Result<f32, SensorError> {
///     Err(SensorError::Timeout)
/// }
/// ```
#[derive(Debug, Clone)]
pub enum SensorError {
    /// Error de entrada/salida al acceder al sensor.
    IoError,
    /// El sensor no respondió dentro del tiempo límite.
    Timeout,
    /// Los datos leídos son inválidos o no interpretables.
    InvalidData,
    /// El sensor falló al inicializarse o no está disponible.
    InitializationError,
}
