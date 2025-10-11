/// Trait que define un medio de comunicación dentro del framework IoT.
/// 
/// Su propósito es **abstraer la forma en que los datos se envían hacia un destino externo**,
/// como un broker MQTT, un servidor HTTP o simplemente la consola.
///
/// Cualquier estructura que implemente este trait podrá actuar como un canal de salida
/// de información desde el gateway hacia otros sistemas.
///

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
/// Permite manejar fallos de transmisión sin acoplarse a una implementación específica.
/// En el futuro se pueden agregar variantes adicionales como `ConnectionLost` o `Timeout`.
#[derive(Debug, Clone)]
pub enum CommunicatorError {
    /// Error genérico al intentar enviar datos.
    SendError,
}
