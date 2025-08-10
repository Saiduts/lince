/// Representa un actuador en el sistema (motor, relé, LED, etc.).
/// 
/// Un actuador recibe **comandos** y los ejecuta para realizar una acción física.
/// 
/// # Asociated Types
/// - `Command`: Tipo de datos que describe la instrucción enviada al actuador.
/// 
/// # Ejemplo
/// Un actuador de tipo `LED` podría recibir un `bool` como comando (`true` = encender, `false` = apagar).
pub trait Actuator {
    /// Tipo del comando que el actuador acepta.
    type Command;

    /// Envía un comando al actuador y ejecuta la acción física.
    ///
    /// # Errores
    /// Devuelve `ActuatorError::ExecuteError` si el comando falla.
    fn execute(&mut self, command: Self::Command) -> Result<(), ActuatorError>;
}

/// Posibles errores que pueden ocurrir al operar un actuador.
#[derive(Debug)]
pub enum ActuatorError {
    /// Error al ejecutar el comando en el actuador.
    ExecuteError(String),
}
