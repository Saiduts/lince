pub trait Actuator {
    //Command es el tipo de dato que se envía al actuador
    type Command;
    //execute es el método que se ejecuta para enviar un comando al actuador
    fn execute(&mut self, command: Self::Command) -> Result<(), ActuatorError>;
}

//Error de ejecución del comando
#[derive(Debug)]
pub enum ActuatorError {
    //Error de ejecución del comando
    ExecuteError(String),
}