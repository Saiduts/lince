/// Define un sistema de almacenamiento (archivo local, base de datos, memoria, etc.).
///
/// # Associated Types
/// - `Data`: Tipo de datos que se guardan/cargan.
pub trait Storage {
    /// Tipo de los datos manejados por el almacenamiento.
    type Data;

    /// Carga datos desde el almacenamiento.
    ///
    /// # Errores
    /// - `LoadError` si no se puede leer.
    fn load(&mut self) -> Result<Self::Data, StorageError>;

    /// Guarda datos en el almacenamiento.
    ///
    /// # Errores
    /// - `SaveError` si no se puede guardar.
    fn save(&mut self, data: Self::Data) -> Result<(), StorageError>;
}

/// Posibles errores del almacenamiento.
#[derive(Debug)]
pub enum StorageError {
    /// Error al cargar datos.
    LoadError(String),

    /// Error al guardar datos.
    SaveError(String),
}
