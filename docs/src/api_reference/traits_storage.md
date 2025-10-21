# Trait `Storage`

Define la interfaz para sistemas de almacenamiento de lecturas de sensores.

```rust
pub trait Storage {
    fn save(&mut self, data: SensorOutput) -> Result<(), StorageError>;
    fn list(&self) -> Result<Vec<SensorOutput>, StorageError>;
    fn clear(&mut self) -> Result<(), StorageError>;
}
```

## Descripción

El trait `Storage` proporciona una abstracción unificada para persistir, recuperar y gestionar lecturas de sensores. Cualquier implementación de este trait puede almacenar datos en diferentes backends (memoria, disco, bases de datos, cloud).

## Métodos Requeridos

### `save()`

```rust
fn save(&mut self, data: SensorOutput) -> Result<(), StorageError>
```

Guarda una lectura de sensor en el almacenamiento.

**Parámetros:**
- `data`: Dato del sensor a almacenar

**Retorna:**
- `Ok(())`: Dato guardado exitosamente
- `Err(StorageError)`: Error durante el almacenamiento

**Errores posibles:**
- `StorageError::SaveError`: Fallo al guardar (disco lleno, permisos, red)

**Comportamiento esperado:**
- Las implementaciones DEBEN preservar el orden de inserción cuando sea posible
- Las implementaciones PUEDEN agregar metadata (timestamps, IDs)
- Las implementaciones DEBEN manejar múltiples llamadas concurrentes de forma segura

**Ejemplo:**
```rust
let mut storage = MemoryStorage::new();

storage.save(SensorOutput::Float(24.5))?;
storage.save(SensorOutput::Text("Estado: OK".to_string()))?;
```

### `list()`

```rust
fn list(&self) -> Result<Vec<SensorOutput>, StorageError>
```

Retorna todas las lecturas almacenadas.

**Retorna:**
- `Ok(Vec<SensorOutput>)`: Vector con todas las lecturas
- `Err(StorageError)`: Error durante la recuperación

**Errores posibles:**
- `StorageError::ReadError`: Fallo al leer (archivo corrupto, conexión perdida)

**Comportamiento esperado:**
- Las implementaciones DEBEN retornar datos en orden de inserción cuando sea posible
- Las implementaciones PUEDEN omit metadata interno (timestamps) si no está en el trait
- Las implementaciones DEBEN retornar un vector vacío si no hay datos

**Ejemplo:**
```rust
let datos = storage.list()?;

for (i, dato) in datos.iter().enumerate() {
    println!("{}. {:?}", i + 1, dato);
}
```

### `clear()`

```rust
fn clear(&mut self) -> Result<(), StorageError>
```

Elimina todas las lecturas almacenadas.

**Retorna:**
- `Ok(())`: Almacenamiento limpiado exitosamente
- `Err(StorageError)`: Error durante la limpieza

**Errores posibles:**
- `StorageError::ClearError`: Fallo al limpiar (permisos, bloqueo)

**Comportamiento esperado:**
- Las implementaciones DEBEN eliminar TODAS las lecturas
- Las implementaciones DEBEN preservar la estructura del storage (no eliminar archivos/tablas)
- Las implementaciones DEBEN ser idempotentes (llamar múltiples veces es seguro)

**Ejemplo:**
```rust
storage.clear()?;

assert_eq!(storage.list()?.len(), 0);
```


## Ver También

- [SensorOutput y StorageError](./core_types.md)
- [MemoryStorage](../storage/memory_storage.md)
- [Crear Storage Personalizado](../storage/custom_storage.md)
- [Trait Sensor](./traits_sensor.md)