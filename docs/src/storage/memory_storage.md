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
use lince::devices::sensors::dht22::Dht22Sensor;
use lince::core::traits::sensor::Sensor;
use lince::storage::memory::MemoryStorage;
use lince::core::traits::storage::Storage;
use lince::core::SensorOutput;

use std::thread;
use std::time::Duration;

struct Estadisticas {
    promedio: f32,
    max: f32,
    min: f32,
    muestras: usize,
}

fn extraer_temperatura(texto: &str) -> Option<f32> {
    // Ejemplo de texto: "Temp: 24.3°C, Hum: 53.7%"
    let inicio = texto.find("Temp: ")? + 6;
    let fin = texto[inicio..].find("°C")? + inicio;
    let valor = &texto[inicio..fin];
    valor.trim().parse::<f32>().ok()
}

fn calcular_estadisticas(storage: &MemoryStorage) -> Option<Estadisticas> {
    let datos = storage.list().ok()?;
    
    let temperaturas: Vec<f32> = datos
        .iter()
        .filter_map(|d| match d {
            SensorOutput::Text(t) => extraer_temperatura(t),
            _ => None,
        })
        .collect();
    
    if temperaturas.is_empty() {
        return None;
    }
    
    let suma: f32 = temperaturas.iter().sum();
    let promedio = suma / temperaturas.len() as f32;
    let max = temperaturas.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
    let min = temperaturas.iter().cloned().fold(f32::INFINITY, f32::min);
    
    Some(Estadisticas {
        promedio,
        max,
        min,
        muestras: temperaturas.len(),
    })
}

fn main() {
    let mut sensor = Dht22Sensor::new(23).unwrap();
    let mut storage = MemoryStorage::new();
    
    println!("Recolectando datos del DHT22...");
    for _ in 0..10 {
        if let Ok(data) = sensor.read() {
            storage.save(data.clone()).ok();
            println!("  Guardado: {:?}", data);
        } else {
            println!("  Error de lectura");
        }
        thread::sleep(Duration::from_secs(2));
    }
    
    println!("\nAnalizando resultados...");
    if let Some(stats) = calcular_estadisticas(&storage) {
        println!("  Estadísticas de {} muestras:", stats.muestras);
        println!("  Promedio: {:.2}°C", stats.promedio);
        println!("  Máxima:   {:.2}°C", stats.max);
        println!("  Mínima:   {:.2}°C", stats.min);
    } else {
        println!("  No se pudieron calcular estadísticas.");
    }
}

```



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


##### `clear()`

```rust
fn clear(&mut self) -> Result<(), StorageError>
```

Elimina todas las lecturas almacenadas.

**Retorna:**
- `Ok(())`: Siempre exitoso



## Próximos Pasos

-   Aprende a [crear storage personalizado](./custom_storage.md)
-   Explora [trait storage](../reference/storage.md)

## Ver También

- [Arquitectura del Framework](../user_guide/architecture.md)