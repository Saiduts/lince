# core::traits

Resumen de todos los traits fundamentales del framework IoT.

## Módulo: `lince::core::traits`

```rust
pub mod sensor;
pub mod storage;
pub mod communicator;
```

El módulo `core::traits` define las abstracciones principales que permiten la extensibilidad y modularidad del framework.

## Traits Disponibles

### Sensor

Define cómo leer datos de dispositivos físicos.

```rust
pub trait Sensor {
    type Output;
    fn read(&mut self) -> Result<SensorOutput, SensorError>;
}
```

**Propósito:** Abstracción unificada para cualquier sensor  
**Ubicación:** `lince::core::traits::sensor`  
**Documentación:** [Trait Sensor](./traits_sensor.md)

**Implementaciones:**
- `Dht11Sensor` - Temperatura/humedad básica
- `Dht22Sensor` - Temperatura/humedad precisa
- `Ds18b20Sensor` - Temperatura OneWire
- `MhRdSensor` - Sensor de lluvia

**Ejemplo:**
```rust
use lince::core::traits::sensor::Sensor;

fn leer_sensor<S: Sensor>(sensor: &mut S) -> Result<SensorOutput, SensorError> {
    sensor.read()
}
```

### Storage

Define cómo almacenar y recuperar lecturas de sensores.

```rust
pub trait Storage {
    fn save(&mut self, data: SensorOutput) -> Result<(), StorageError>;
    fn list(&self) -> Result<Vec<SensorOutput>, StorageError>;
    fn clear(&mut self) -> Result<(), StorageError>;
}
```

**Propósito:** Persistencia y gestión de datos  
**Ubicación:** `lince::core::traits::storage`  
**Documentación:** [Trait Storage](./traits_storage.md)

**Implementaciones:**
- `MemoryStorage` - Almacenamiento en RAM

**Ejemplo:**
```rust
use lince::core::traits::storage::Storage;

fn guardar_lecturas<S: Storage>(
    storage: &mut S,
    datos: Vec<SensorOutput>
) -> Result<(), StorageError> {
    for dato in datos {
        storage.save(dato)?;
    }
    Ok(())
}
```

### Communicator

Define cómo enviar datos a destinos externos.

```rust
pub trait Communicator {
    fn send(&mut self, data: &[u8]) -> Result<(), CommunicatorError>;
}
```

**Propósito:** Transmisión de datos a sistemas externos  
**Ubicación:** `lince::core::traits::communicator`  
**Documentación:** [Trait Communicator](./traits_communicator.md)

**Implementaciones:**
- `ConsoleCommunicator` - Salida a consola
- `MqttCommunicator` - Publicación MQTT

**Ejemplo:**
```rust
use lince::core::traits::communicator::Communicator;

fn publicar<C: Communicator>(
    comm: &mut C,
    mensaje: &str
) -> Result<(), CommunicatorError> {
    comm.send(mensaje.as_bytes())
}
```

## Uso Conjunto

### Pipeline Completo

```rust
use lince::devices::sensors::dht22::Dht22Sensor;
use lince::storage::memory::MemoryStorage;
use lince::network::mqtt::MqttCommunicator;
use lince::core::traits::{Sensor, Storage, Communicator};

fn pipeline_completo() -> Result<(), Box<dyn std::error::Error>> {
    // Componentes
    let mut sensor = Dht22Sensor::new(4)?;
    let mut storage = MemoryStorage::new();
    let mut mqtt = MqttCommunicator::new("sensor", "localhost", 1883, "topic")?;
    
    loop {
        // 1. Leer sensor
        let dato = sensor.read()?;
        println!("Leído: {:?}", dato);
        
        // 2. Almacenar
        storage.save(dato.clone())?;
        
        // 3. Enviar
        let json = serde_json::to_string(&dato)?;
        mqtt.send(json.as_bytes())?;
        
        std::thread::sleep(std::time::Duration::from_secs(10));
    }
}
```

## Recursos Adicionales

### Guías de Implementación

- [Crear Sensores Personalizados](../sensors/custom_sensors.md)
- [Crear Storage Personalizado](../storage/custom_storage.md)
- [Crear Communicators Personalizados](../communication/custom_communicators.md)

### Referencias de Traits

- [Trait Sensor](./traits_sensor.md)
- [Trait Storage](./traits_storage.md)
- [Trait Communicator](./traits_communicator.md)

### Tipos Core

- [SensorOutput y Errores](./core_types.md)


## Ver También

- [Arquitectura del Framework](../user_guide/architecture.md)
- [Conceptos Básicos](../user_guide/basic_concepts.md)