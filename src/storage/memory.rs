use crate::core::traits::storage::{Storage, StorageError};
use crate::core::types::SensorOutput;
use std::time::SystemTime;

/// MemoryStorage: almacenamiento de lecturas de sensores en memoria.
///
/// Este almacenamiento mantiene los datos **temporalmente en RAM**, sin persistencia en disco.
/// Cada lectura se guarda junto con su timestamp (`SystemTime`) de creación.
/// Útil para pruebas, depuración o almacenamiento temporal de datos de sensores.
pub struct MemoryStorage {
    /// Vector que almacena tuplas `(timestamp, sensor_output)`.
    data: Vec<(SystemTime, SensorOutput)>,
}

impl MemoryStorage {
    /// Crea un nuevo `MemoryStorage` vacío.
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }
}

impl Storage for MemoryStorage {
    /// Guarda una nueva lectura en memoria.
    ///
    /// # Parámetros
    /// - `data`: salida del sensor a almacenar.
    ///
    /// # Retorno
    /// - `Ok(())` si se almacena correctamente.
    /// - `Err(StorageError)` nunca ocurre en esta implementación, pero se mantiene
    ///   la firma del trait para compatibilidad.
    fn save(&mut self, data: SensorOutput) -> Result<(), StorageError> {
        self.data.push((SystemTime::now(), data));
        Ok(())
    }

    /// Lista todas las lecturas almacenadas, descartando los timestamps.
    ///
    /// # Retorno
    /// - `Ok(Vec<SensorOutput>)` con todas las lecturas guardadas.
    /// - `Err(StorageError)` no se produce en esta implementación.
    fn list(&self) -> Result<Vec<SensorOutput>, StorageError> {
        Ok(self.data.iter().map(|(_, d)| d.clone()).collect())
    }

    /// Limpia todas las lecturas almacenadas en memoria.
    ///
    /// # Retorno
    /// - `Ok(())` si se limpia correctamente.
    /// - `Err(StorageError)` no se produce en esta implementación.
    fn clear(&mut self) -> Result<(), StorageError> {
        self.data.clear();
        Ok(())
    }
}
