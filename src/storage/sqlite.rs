// src/storage/sqlite.rs

use rusqlite::{Connection, params};
use crate::core::traits::storage::{Storage, StorageError};
use crate::core::traits::communicator::{Communicator, CommunicatorError};
use crate::core::types::SensorOutput;

/// SqliteStorage: almacenamiento persistente con soporte opcional de reintento.
///
/// Cada lectura se guarda con una columna `sent`:
/// - `0` = pendiente de envío
/// - `1` = ya enviado
///
/// Si el usuario activa el reintento con [`flush_pending`], el propio storage
/// intenta reenviar los mensajes pendientes usando el comunicador que se le pase.
/// Los que se envíen con éxito se marcan como `sent = 1` para no repetirlos.
///
/// # Esquema
/// ```sql
/// CREATE TABLE IF NOT EXISTS sensor_readings (
///     id        INTEGER PRIMARY KEY AUTOINCREMENT,
///     timestamp TEXT    NOT NULL,
///     kind      TEXT    NOT NULL,
///     value     TEXT    NOT NULL,
///     sent      INTEGER NOT NULL DEFAULT 0
/// )
/// ```
pub struct SqliteStorage {
    conn: Connection,
}

impl SqliteStorage {
    /// Abre o crea una base de datos SQLite en la ruta indicada.
    /// Usa `":memory:"` para tests en RAM.
    pub fn new(path: &str) -> Result<Self, StorageError> {
        if path.is_empty() {
            eprintln!("[SqliteStorage] La ruta no puede estar vacía");
            return Err(StorageError::SaveError);
        }

        let conn = Connection::open(path).map_err(|e| {
            eprintln!("[SqliteStorage] Error al abrir '{}': {}", path, e);
            StorageError::SaveError
        })?;

        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS sensor_readings (
                id        INTEGER PRIMARY KEY AUTOINCREMENT,
                timestamp TEXT    NOT NULL,
                kind      TEXT    NOT NULL,
                value     TEXT    NOT NULL,
                sent      INTEGER NOT NULL DEFAULT 0
            );"
        ).map_err(|e| {
            eprintln!("[SqliteStorage] Error al crear la tabla: {}", e);
            StorageError::SaveError
        })?;

        Ok(Self { conn })
    }

    /// Marca un dato como enviado para que no vuelva a intentarse.
    fn mark_sent(&mut self, id: i64) -> Result<(), StorageError> {
        self.conn.execute(
            "UPDATE sensor_readings SET sent = 1 WHERE id = ?1",
            params![id],
        ).map_err(|e| {
            eprintln!("[SqliteStorage] Error al marcar id={} como enviado: {}", id, e);
            StorageError::SaveError
        })?;
        Ok(())
    }

    /// Recupera todos los datos con `sent = 0`, ordenados por id.
    fn list_pending_raw(&self) -> Result<Vec<(i64, SensorOutput)>, StorageError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, kind, value FROM sensor_readings
             WHERE sent = 0 ORDER BY id ASC"
        ).map_err(|e| {
            eprintln!("[SqliteStorage] Error preparando consulta de pendientes: {}", e);
            StorageError::ReadError
        })?;

        let rows = stmt.query_map([], |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
            ))
        }).map_err(|e| {
            eprintln!("[SqliteStorage] Error ejecutando consulta: {}", e);
            StorageError::ReadError
        })?;

        let mut results = Vec::new();
        for row in rows {
            let (id, kind, value) = row.map_err(|_| StorageError::ReadError)?;
            if let Some(data) = Self::deserialize(&kind, &value) {
                results.push((id, data));
            }
        }
        Ok(results)
    }

    /// Número de mensajes pendientes de envío.
    ///
    /// Útil para mostrar diagnósticos.
    ///
    /// ```rust
    /// println!("Pendientes: {}", storage.pending_count());
    /// ```
    pub fn pending_count(&self) -> usize {
        self.conn
            .query_row(
                "SELECT COUNT(*) FROM sensor_readings WHERE sent = 0",
                [],
                |row| row.get::<_, i64>(0),
            )
            .unwrap_or(0) as usize
    }

    /// Intenta reenviar los mensajes pendientes usando el comunicador dado.
    ///
    /// Para cada mensaje pendiente:
    /// - Si el envío tiene éxito → se marca como `sent = 1` (no se reenvía de nuevo).
    /// - Si falla por `Disconnected` → se detiene el ciclo y espera al siguiente llamado.
    /// - Si falla por `SendError` → se descarta marcándolo como enviado
    ///   (error de protocolo, no tiene sentido reintentar).
    ///
    /// Retorna cuántos mensajes se pudieron enviar en este ciclo.
    ///
    /// # Cuándo llamarlo
    ///
    /// Al inicio de cada iteración del bucle principal, antes de leer el sensor,
    /// o después de detectar que la conexión volvió. El usuario decide.
    ///
    /// # Ejemplo
    /// ```rust
    /// // Al inicio de cada iteración, intentar vaciar la cola
    /// let enviados = storage.flush_pending(&mut mqtt);
    /// if enviados > 0 {
    ///     println!("Reenviados {} mensajes pendientes", enviados);
    /// }
    /// ```
    pub fn flush_pending<C: Communicator>(
        &mut self,
        comm: &mut C,
    ) -> usize {
        let pending = match self.list_pending_raw() {
            Ok(p)  => p,
            Err(e) => {
                eprintln!("[SqliteStorage] Error recuperando pendientes: {:?}", e);
                return 0;
            }
        };

        if pending.is_empty() {
            return 0;
        }

        println!("[SqliteStorage] Intentando reenviar {} pendientes", pending.len());

        let mut enviados = 0;

        for (id, data) in &pending {
            let bytes = Self::to_bytes(data);

            match comm.send(&bytes) {
                Ok(()) => {
                    let _ = self.mark_sent(*id);
                    enviados += 1;
                }
                Err(CommunicatorError::Disconnected) => {
                    // Sigue sin conexión: no tiene sentido continuar
                    eprintln!(
                        "[SqliteStorage] Aún sin conexión. \
                        Enviados en este ciclo: {}/{}",
                        enviados, pending.len()
                    );
                    break;
                }
                Err(CommunicatorError::SendError) => {
                    // Error de protocolo: descartar para no bloquear la cola
                    eprintln!("[SqliteStorage] SendError en id={}, descartando", id);
                    let _ = self.mark_sent(*id);
                }
            }
        }

        enviados
    }

    // ------------------------------------------------------------------
    // Serialización interna
    // ------------------------------------------------------------------

    fn serialize(data: &SensorOutput) -> (String, String) {
        match data {
            SensorOutput::Bool(v)  => ("Bool".into(),  v.to_string()),
            SensorOutput::Int(v)   => ("Int".into(),   v.to_string()),
            SensorOutput::Float(v) => ("Float".into(), v.to_string()),
            SensorOutput::Text(v)  => ("Text".into(),  v.clone()),
            SensorOutput::Bytes(v) => (
                "Bytes".into(),
                v.iter().map(|b| format!("{:02x}", b)).collect::<Vec<_>>().join(" "),
            ),
        }
    }

    fn deserialize(kind: &str, value: &str) -> Option<SensorOutput> {
        match kind {
            "Bool"  => value.parse::<bool>().ok().map(SensorOutput::Bool),
            "Int"   => value.parse::<i64>().ok().map(SensorOutput::Int),
            "Float" => value.parse::<f32>().ok().map(SensorOutput::Float),
            "Text"  => Some(SensorOutput::Text(value.to_string())),
            "Bytes" => {
                let bytes: Option<Vec<u8>> = value
                    .split_whitespace()
                    .map(|hex| u8::from_str_radix(hex, 16).ok())
                    .collect();
                bytes.map(SensorOutput::Bytes)
            }
            _ => {
                eprintln!("[SqliteStorage] Tipo desconocido: {}", kind);
                None
            }
        }
    }

    /// Convierte un SensorOutput a bytes para reenviarlo por el comunicador.
    fn to_bytes(data: &SensorOutput) -> Vec<u8> {
        match data {
            SensorOutput::Bytes(v) => v.clone(),
            SensorOutput::Text(s)  => s.as_bytes().to_vec(),
            SensorOutput::Float(f) => f.to_string().into_bytes(),
            SensorOutput::Int(i)   => i.to_string().into_bytes(),
            SensorOutput::Bool(b)  => b.to_string().into_bytes(),
        }
    }
}

// ------------------------------------------------------------------
// Implementación del trait Storage
// ------------------------------------------------------------------

impl Storage for SqliteStorage {
    /// Guarda una lectura con `sent = 0` (pendiente de envío).
    fn save(&mut self, data: SensorOutput) -> Result<(), StorageError> {
        let (kind, value) = Self::serialize(&data);
        let timestamp = unix_timestamp();

        self.conn.execute(
            "INSERT INTO sensor_readings (timestamp, kind, value, sent)
             VALUES (?1, ?2, ?3, 0)",
            params![timestamp, kind, value],
        ).map_err(|e| {
            eprintln!("[SqliteStorage] Error al guardar: {}", e);
            StorageError::SaveError
        })?;

        Ok(())
    }

    /// Retorna todas las lecturas (enviadas y pendientes), ordenadas por id.
    fn list(&self) -> Result<Vec<SensorOutput>, StorageError> {
        let mut stmt = self.conn.prepare(
            "SELECT kind, value FROM sensor_readings ORDER BY id ASC"
        ).map_err(|_| StorageError::ReadError)?;

        let rows = stmt.query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        }).map_err(|_| StorageError::ReadError)?;

        let mut results = Vec::new();
        for row in rows {
            let (kind, value) = row.map_err(|_| StorageError::ReadError)?;
            if let Some(output) = Self::deserialize(&kind, &value) {
                results.push(output);
            }
        }
        Ok(results)
    }

    /// Elimina todas las filas, enviadas y pendientes.
    fn clear(&mut self) -> Result<(), StorageError> {
        self.conn.execute("DELETE FROM sensor_readings", [])
            .map_err(|_| StorageError::ClearError)?;
        Ok(())
    }
}

fn unix_timestamp() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
        .to_string()
}