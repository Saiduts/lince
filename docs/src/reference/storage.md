# Módulo storage

Implementaciones de almacenamiento de datos para el framework IoT.

## Storage

```rust
pub mod memory;
pub mod sqlite;
```

---

## Implementaciones Disponibles

### MemoryStorage

Almacenamiento en RAM, sin persistencia.

```rust
use lince::storage::memory::MemoryStorage;

let mut storage = MemoryStorage::new();
storage.save(SensorOutput::Float(24.5))?;
let datos = storage.list()?;
storage.clear()?;
```

**Documentación:** [MemoryStorage](../storage/memory_storage.md)

---

### SqliteStorage

Almacenamiento persistente en SQLite, con soporte de reintento por desconexión.

```rust
use lince::storage::sqlite::SqliteStorage;

let mut storage = SqliteStorage::new("sensor_data.db")?;

// Guardar siempre antes de enviar
storage.save(dato)?;

// Si el envío falla por desconexión, reenviar la cola
match mqtt.send(json.as_bytes()) {
    Err(CommunicatorError::Disconnected) => {
        storage.flush_pending(&mut mqtt);
    }
    _ => {}
}

// Consultar cuántos pendientes hay
println!("{} pendientes", storage.pending_count());
```

**Métodos adicionales sobre el trait `Storage`:**

```rust
pub fn flush_pending<C: Communicator>(&mut self, comm: &mut C) -> usize
pub fn pending_count(&self) -> usize
```

**Documentación:** [SqliteStorage](../storage/sqlite_storage.md)

---

## Comparativa

| Característica | `MemoryStorage` | `SqliteStorage` |
|----------------|-----------------|-----------------|
| Persistencia | No | Sí |
| Reintento por desconexión | No | Sí, `flush_pending()` |
| Dependencias extra | Ninguna | `rusqlite` |
| Ideal para | Tests, desarrollo | Producción |

---

## Uso con Trait Storage

```rust
use lince::core::traits::storage::Storage;

fn guardar<S: Storage>(storage: &mut S, dato: SensorOutput) {
    storage.save(dato).unwrap();
}

// Funciona con cualquier implementación
guardar(&mut MemoryStorage::new(), dato.clone());
guardar(&mut SqliteStorage::new("datos.db").unwrap(), dato);
```

---

## Ver También

- [Trait Storage](./traits_storage.md)
- [Core Types](./core_types.md)
- [MqttCommunicator](../communication/mqtt.md)
- [Arquitectura del Framework](../user_guide/architecture.md)