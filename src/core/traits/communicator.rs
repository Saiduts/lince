pub trait Communicator {
    //Command es el tipo de dato que se envía al comunicador
    type Command;
    //Response es el tipo de dato que se obtiene al enviar un comando al comunicador
    type Response;
    //send es el método que se ejecuta para enviar un comando al comunicador
    fn send(&mut self, command: Self::Command) -> Result<Self::Response, CommunicatorError>;
    //receive es el método que se ejecuta para recibir un comando del comunicador
    fn receive(&mut self) -> Result<Self::Response, CommunicatorError>;
}

//Error de ejecución del comando
#[derive(Debug)]
pub enum CommunicatorError {
    //Error de envio del comando
    SendError(String),
    //Error de ejecución del comando
    ExecuteError(String),
}