/// Define un medio de comunicación (ej. consola, MQTT, HTTP).
///
/// Un comunicador envía y recibe mensajes de otros sistemas o de la nube.
///
/// # Associated Types
/// - `Command`: Tipo de datos enviados.
/// - `Response`: Tipo de datos recibidos.
pub trait Communicator {
    /// Tipo del comando que se envía.
    type Command;

    /// Tipo de la respuesta que se recibe.
    type Response;

    /// Envía un comando y devuelve una respuesta.
    ///
    /// # Errores
    /// - `SendError` si falla el envío.
    /// - `ExecuteError` si hay fallo interno en la ejecución.
    fn send(&mut self, command: Self::Command) -> Result<Self::Response, CommunicatorError>;

    /// Recibe datos del canal de comunicación.
    ///
    /// # Errores
    /// - `ExecuteError` si hay fallo en la lectura.
    fn receive(&mut self) -> Result<Self::Response, CommunicatorError>;
}

/// Errores posibles de un comunicador.
#[derive(Debug)]
pub enum CommunicatorError {
    /// Fallo en el envío de datos.
    SendError(String),

    /// Fallo en la ejecución interna del comunicador.
    ExecuteError(String),
}
