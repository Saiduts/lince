# Módulo storage

Implementaciones de almacenamiento de datos para el framework IoT.

## Módulo: `lince::storage`

```rust
pub mod memory;
```

Este módulo contiene implementaciones del trait `Storage` para persistir lecturas de sensores.

## Implementaciones Disponibles

### MemoryStorage

Almacenamiento en RAM (no persistente).

```rust
use lince::storage::memory::MemoryStorage;

pub struct MemoryStorage {
    data: Vec<(SystemTime, SensorOutput)>,
}

impl MemoryStorage {
    pub fn new() -> Self;
}
```

**Características:**
-   Muy rápido (acceso en memoria)
-   No persistente (datos se pierden al reiniciar)
-   Capacidad limitada por RAM
-   Sin dependencias externas

**Uso típico:**
- Buffer temporal
- Testing y desarrollo
- Caché de datos recientes

**Ejemplo:**
```rust
let mut storage = MemoryStorage::new();

storage.save(SensorOutput::Float(24.5))?;
storage.save(SensorOutput::Float(25.0))?;

let datos = storage.list()?;
println!("Almacenados: {} lecturas", datos.len());

storage.clear()?;
```

**Documentación:** [MemoryStorage Reference](../storage/memory_storage.md)

## Uso con Trait Storage

Todas las implementaciones cumplen con el trait `Storage`:

```rust
use lince::core::traits::storage::Storage;

fn guardar_lecturas<S: Storage>(
    storage: &mut S,
    sensor: &mut impl Sensor,
    n: usize
) -> Result<(), StorageError> {
    for _ in 0..n {
        let data = sensor.read()
            .map_err(|_| StorageError::SaveError)?;
        storage.save(data)?;
    }
    Ok(())
}

// Funciona con cualquier Storage
let mut memory = MemoryStorage::new();
guardar_lecturas(&mut memory, &mut dht22, 10)?;
```


## Recursos Adicionales

### Documentación Detallada
- [MemoryStorage Reference](../storage/memory_storage.md)
- [Crear Storage Personalizado](../storage/custom_storage.md)

### Interfaces
- [Trait Storage](./traits_storage.md)
- [StorageError](./core_types.md)

## Ver También

- [Trait Storage](./traits_storage.md)
- [Core Types](./core_types.md)
- [Arquitectura del Framework](../user_guide/architecture.md)