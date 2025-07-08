pub trait Storage {
    //Data es el tipo de dato que se almacena en el almacenamiento
    type Data;
    //load es el método que se ejecuta para cargar datos del almacenamiento
    fn load(&mut self) -> Result<Self::Data, StorageError>;
    //save es el método que se ejecuta para almacenar datos en el almacenamiento
    fn save(&mut self, data: Self::Data) -> Result<(), StorageError>;
}

//Error de almacenamiento
#[derive(Debug)]
pub enum StorageError {
    //Error de carga de datos
    LoadError(String),
    //Error de almacenamiento de datos
    SaveError(String),
}