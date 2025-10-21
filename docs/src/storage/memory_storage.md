# MemoryStorage

`MemoryStorage` es una implementación del trait `Storage` que almacena lecturas de sensores en memoria RAM, proporcionando acceso rápido sin persistencia.

## Uso Básico

### Importar el Módulo

```rust
use lince::storage::memory::MemoryStorage;
use lince::core::traits::storage::Storage;
use lince::core::SensorOutput;
```

### Crear una Instancia

```rust
let mut storage = MemoryStorage::new();
```

### Operaciones CRUD

```rust
// CREATE: Guardar dato
storage.save(SensorOutput::Float(24.5))?;

// READ: Listar todos
let datos = storage.list()?;
for dato in datos {
    println!("{:?}", dato);
}

// DELETE: Limpiar todo
storage.clear()?;
```


### Ejemplo: Estadísticas en Memoria

```rust
fn calcular_estadisticas(storage: &MemoryStorage) -> Option<Estadisticas> {
    let datos = storage.list().ok()?;
    
    let temperaturas: Vec<f32> = datos
        .iter()
        .filter_map(|d| match d {
            SensorOutput::Float(v) => Some(*v),
            _ => None,
        })
        .collect();
    
    if temperaturas.is_empty() {
        return None;
    }
    
    let suma: f32 = temperaturas.iter().sum();
    let promedio = suma / temperaturas.len() as f32;
    
    let max = temperaturas.iter()
        .fold(f32::NEG_INFINITY, |a, &b| a.max(b));
    let min = temperaturas.iter()
        .fold(f32::INFINITY, |a, &b| a.min(b));
    
    Some(Estadisticas {
        promedio,
        max,
        min,
        muestras: temperaturas.len(),
    })
}

struct Estadisticas {
    promedio: f32,
    max: f32,
    min: f32,
    muestras: usize,
}

fn main() {
    let mut sensor = Dht22Sensor::new(4).unwrap();
    let mut storage = MemoryStorage::new();
    
    // Recolectar datos
    for _ in 0..20 {
        if let Ok(data) = sensor.read() {
            storage.save(data).ok();
        }
        thread::sleep(Duration::from_secs(2));
    }
    
    // Analizar
    if let Some(stats) = calcular_estadisticas(&storage) {
        println!("  Estadísticas de {} muestras:", stats.muestras);
        println!("  Promedio: {:.2}°C", stats.promedio);
        println!("  Máxima:   {:.2}°C", stats.max);
        println!("  Mínima:   {:.2}°C", stats.min);
    }
}
```


## API Reference

### `MemoryStorage`

#### Constructor

```rust
pub fn new() -> Self
```

Crea una instancia vacía de MemoryStorage.

**Ejemplo:**
```rust
let mut storage = MemoryStorage::new();
```

#### Trait `Storage`

##### `save()`

```rust
fn save(&mut self, data: SensorOutput) -> Result<(), StorageError>
```

Guarda una lectura con timestamp automático.

**Parámetros:**
- `data`: Dato a almacenar

**Retorna:**
- `Ok(())`: Siempre exitoso en MemoryStorage
- `Err(StorageError)`: No ocurre en esta implementación

##### `list()`

```rust
fn list(&self) -> Result<Vec<SensorOutput>, StorageError>
```

Retorna todas las lecturas almacenadas (sin timestamps).

**Retorna:**
- `Ok(Vec<SensorOutput>)`: Vector con todas las lecturas
- `Err(StorageError)`: No ocurre en esta implementación

**Nota**: Los timestamps internos no se exponen en esta API.

##### `clear()`

```rust
fn clear(&mut self) -> Result<(), StorageError>
```

Elimina todas las lecturas almacenadas.

**Retorna:**
- `Ok(())`: Siempre exitoso



## Próximos Pasos

-   Aprende a [crear storage personalizado](./custom_storage.md)
-   Explora [trait storage API](../api_reference/storage.md)

## Ver También

- [Arquitectura del Framework](../user_guide/architecture.md)