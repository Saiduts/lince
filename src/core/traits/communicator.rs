// src/core/traits/communicator.rs

/// Trait que define un medio de comunicación dentro del framework IoT.
///
/// Su propósito es **abstraer la forma en que los datos se envían hacia un destino externo**,
/// como un broker MQTT, un servidor HTTP o simplemente la consola.
///
/// Cualquier estructura que implemente este trait podrá actuar como un canal de salida
/// de información desde el gateway hacia otros sistemas.
pub trait Communicator {
    /// Envía un mensaje a través del medio de comunicación.
    ///
    /// El parámetro `data` representa la información a transmitir,
    /// que puede ser texto, JSON o bytes sin formato.
    ///
    /// Retorna `Ok(())` si el envío fue exitoso o un `CommunicatorError` en caso de fallo.
    fn send(&mut self, data: &[u8]) -> Result<(), CommunicatorError>;
}

/// Enumeración que representa los errores posibles al usar un comunicador.
///
/// La distinción entre variantes es fundamental para el sistema de reintento:
/// solo `Disconnected` activa el almacenamiento de pendientes; `SendError`
/// indica un fallo no relacionado con la conectividad (payload inválido,
/// topic mal formado, etc.) que no tiene sentido reintentar.
///
/// # Variantes
///
/// | Variante | Cuándo ocurre | ¿Se reintenta? |
/// |----------|---------------|----------------|
/// | `Disconnected` | Broker no alcanzable, hilo de red muerto | Sí |
/// | `SendError` | Error de protocolo, payload vacío, topic inválido | No |
///
/// # Ejemplo
/// ```rust
/// match comm.send(data) {
///     Ok(()) => {},
///     Err(CommunicatorError::Disconnected) => {
///         // Guardar para reintento posterior
///     }
///     Err(CommunicatorError::SendError) => {
///         // Loguear, no reintentar
///     }
/// }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum CommunicatorError {
    /// El comunicador no pudo alcanzar el destino por un problema de red o
    /// porque la conexión fue interrumpida. Los datos pueden reintentarse.
    Disconnected,

    /// Error en el envío no relacionado con la conectividad: payload vacío,
    /// topic inválido, error de protocolo. No tiene sentido reintentar.
    SendError,
}