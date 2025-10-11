use crate::core::traits::communicator::{Communicator, CommunicatorError};

/// ConsoleCommunicator: comunica datos enviándolos a la **salida estándar (consola)**.
///
/// Este comunicador es útil principalmente para:
/// - Depuración de sensores o datos de IoT.
/// - Pruebas locales sin necesidad de conectividad de red.
///
/// Todos los datos se muestran como texto en la consola, precedidos por `[CONSOLE]`.
pub struct ConsoleCommunicator;

impl ConsoleCommunicator {
    /// Crea un nuevo `ConsoleCommunicator`.
    ///
    /// # Retorno
    /// Una instancia lista para enviar datos a la consola.
    pub fn new() -> Self {
        Self
    }
}

impl Communicator for ConsoleCommunicator {
    /// Envía datos a la consola.
    ///
    /// # Parámetros
    /// - `data`: slice de bytes a mostrar. Se interpreta como UTF-8.
    ///
    /// # Retorno
    /// - `Ok(())` siempre que la conversión a UTF-8 sea exitosa.
    /// - `Err(CommunicatorError)` no se produce en esta implementación, 
    ///   pero se mantiene la firma del trait para compatibilidad.
    fn send(&mut self, data: &[u8]) -> Result<(), CommunicatorError> {
        let s = String::from_utf8_lossy(data);
        println!("[CONSOLE] {}", s);
        Ok(())
    }
}
