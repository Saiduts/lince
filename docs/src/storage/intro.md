# Descripción General de Storage

El módulo de almacenamiento proporciona abstracciones para persistir lecturas de sensores.

## ¿Qué es Storage?

**Storage** es un sistema que permite:
-  Guardar lecturas de sensores

## Trait Storage

Todos los sistemas de almacenamiento implementan el trait `Storage`:

```rust
pub trait Storage {
    fn save(&mut self, data: SensorOutput) -> Result<(), StorageError>;
    fn list(&self) -> Result<Vec<SensorOutput>, StorageError>;
    fn clear(&mut self) -> Result<(), StorageError>;
}
```

## Próximos Pasos

-  Lee [MemoryStorage](./memory_storage.md)
-  Aprende a [crear storage personalizado](./custom_storage.md)

## Ver También

- [Trait Storage](../reference/traits_storage.md)
- [Core Types](../reference/core_types.md)