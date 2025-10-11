use crate::core::types::{SensorOutput};

/// Define un almacenamiento simple para lecturas de sensores.
pub trait Storage {
    /// Guarda una nueva lectura en el almacenamiento.
    fn save(&mut self, data: SensorOutput) -> Result<(), StorageError>;

    /// Devuelve todas las lecturas guardadas.
    fn list(&self) -> Result<Vec<SensorOutput>, StorageError>;

    /// Limpia el almacenamiento (borra todos los datos).
    fn clear(&mut self) -> Result<(), StorageError>;
}

/// Errores básicos de almacenamiento.
#[derive(Debug, Clone)]
pub enum StorageError {
    SaveError,
    ReadError,
    ClearError,
}
