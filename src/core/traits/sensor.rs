
pub trait Sensor {
    //Output es el tipo de dato que se obtiene al leer el sensor
    type Output;
    //lee el sensor y devuelve un valor de tipo Output y un mensaje de error si ocurre algún problema
    fn read(&mut self) -> Result<Self::Output, SensorError>;
}

//Error de lectura del sensor
#[derive(Debug)]
pub enum SensorError {
    //Error de lectura del sensor
    ReadError(String),
}