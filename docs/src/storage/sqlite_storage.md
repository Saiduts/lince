# SqliteStorage

`SqliteStorage` es una implementación del trait `Storage` que persiste las lecturas de sensores en SQLite. Opcionalmente puede reenviar los mensajes que no se pudieron enviar por desconexión mediante `flush_pending()`.

---

## ¿Cuándo usar SqliteStorage?

| Situación | Recomendación |
|-----------|---------------|
| Pruebas o depuración | [`MemoryStorage`](./memory_storage.md) |
| Datos que deben persistir entre reinicios | `SqliteStorage` |
| Conexión intermitente con reintento | `SqliteStorage` + `flush_pending()` |

---

## Importar el Módulo

```rust
use lince::storage::sqlite::SqliteStorage;
use lince::core::traits::storage::Storage;
```

---

## Uso Básico

```rust
// Archivo en disco (persistente)
let mut storage = SqliteStorage::new("sensor_data.db").unwrap();

// En memoria (útil para tests)
let mut storage = SqliteStorage::new(":memory:").unwrap();
```

```rust
storage.save(SensorOutput::Text("Temp: 24.5°C, Hum: 60%".to_string()))?;

let lecturas = storage.list()?;
println!("Total: {}", lecturas.len());

storage.clear()?;
```

---

## Reintento por Desconexión — `flush_pending()`

Cuando un envío falla por desconexión, el dato queda guardado con `sent = 0`.
`flush_pending()` intenta reenviar todos los pendientes usando el comunicador que se le pase.
Los que se envíen con éxito se marcan como `sent = 1` para no repetirlos.

### Activar el reintento

```rust
match mqtt.send(json.as_bytes()) {
    Ok(()) => println!("Enviado"),
    Err(CommunicatorError::Disconnected) => {
        // SQLite reenvía los pendientes automáticamente
        let enviados = storage.flush_pending(&mut mqtt);
        println!("Reenviados: {}", enviados);
    }
    Err(CommunicatorError::SendError) => eprintln!("Error de protocolo"),
}
```

### Sin reintento

Si no se llama `flush_pending()`, el dato queda guardado pero no se reenvía.
El comportamiento por defecto del storage no cambia:

```rust
match mqtt.send(json.as_bytes()) {
    Ok(())                               => println!("Enviado"),
    Err(CommunicatorError::Disconnected) => eprintln!("Sin conexión, dato guardado"),
    Err(CommunicatorError::SendError)    => eprintln!("Error de protocolo"),
}
```

### Consultar pendientes

```rust
println!("Pendientes: {}", storage.pending_count());
```

---

## Comportamiento de `flush_pending()`

Para cada mensaje pendiente:

| Resultado del envío | Acción |
|---------------------|--------|
| `Ok(())` | Marca como `sent = 1`, continúa con el siguiente |
| `Err(Disconnected)` | Para el ciclo, deja los restantes para después |
| `Err(SendError)` | Marca como `sent = 1` y descarta (no tiene sentido reintentar) |

---

## Flujo Completo

```rust
// 1. Sensor lee
let dato = sensor.read()?;

// 2. Parser extrae valores
let valores = SensorParser::dht(&dato)?;

// 3. Formatter convierte a JSON
let json = formatter.desde_mapa(&valores);

// 4. SQLite guarda siempre (sent = 0)
storage.save(dato)?;

// 5. MQTT envía
match mqtt.send(json.as_bytes()) {
    Ok(())                               => { /* sent = 1 implícito */ }
    Err(CommunicatorError::Disconnected) => {
        storage.flush_pending(&mut mqtt); // intenta reenviar cola
    }
    Err(CommunicatorError::SendError)    => { /* descarta */ }
}
```

---

## Esquema de la Base de Datos

```sql
CREATE TABLE IF NOT EXISTS sensor_readings (
    id        INTEGER PRIMARY KEY AUTOINCREMENT,
    timestamp TEXT    NOT NULL,
    kind      TEXT    NOT NULL,
    value     TEXT    NOT NULL,
    sent      INTEGER NOT NULL DEFAULT 0  -- 0 = pendiente, 1 = enviado
);
```

---

## Diferencias con MemoryStorage

| Característica | `MemoryStorage` | `SqliteStorage` |
|----------------|-----------------|-----------------|
| Persistencia | No | Sí |
| Reintento por desconexión | No | Sí, con `flush_pending()` |
| Dependencias extra | Ninguna | `rusqlite` |
| Ideal para | Tests, desarrollo | Producción |

---

## Referencia de Interfaces

### Constructor

```rust
pub fn new(path: &str) -> Result<Self, StorageError>
```

### Trait `Storage`

```rust
fn save(&mut self, data: SensorOutput) -> Result<(), StorageError>
fn list(&self) -> Result<Vec<SensorOutput>, StorageError>
fn clear(&mut self) -> Result<(), StorageError>
```

### Métodos adicionales

```rust
pub fn flush_pending<C: Communicator>(
    &mut self,
    comm: &mut C,
) -> usize

pub fn pending_count(&self) -> usize
```

---

## Configuración en Cargo.toml

```toml
[dependencies]
rusqlite = { version = "0.31", features = ["bundled"] }
```

---

## Ver También

- [MemoryStorage](./memory_storage.md)
- [MqttCommunicator](../communication/mqtt.md)
- [Trait Storage](../reference/traits_storage.md)