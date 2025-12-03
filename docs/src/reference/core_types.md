
# Types

Este módulo define los tipos fundamentales que representan datos y errores en el framework IoT.

## Descripción General

`Types` proporciona las estructuras básicas para:
- Representar lecturas de sensores de diferentes tipos
- Manejar errores comunes en operaciones con sensores
- Garantizar interoperabilidad entre componentes del framework



## SensorOutput

Enum que representa los **datos producidos por un sensor**.

### Definición

```rust
pub enum SensorOutput {
    Bool(bool),
    Int(i64),
    Float(f32),
    Text(String),
    Bytes(Vec<u8>),
}
```

### Variantes

| Variante | Tipo | Descripción | Ejemplo de Uso |
|----------|------|-------------|----------------|
| `Bool` | `bool` | Valor lógico | Detección de movimiento, presencia |
| `Int` | `i64` | Valor entero | Conteo de pulsos, presión en Pa |
| `Float` | `f32` | Valor decimal | Temperatura, humedad, voltaje |
| `Text` | `String` | Cadena de texto | Identificadores, estados textuales |
| `Bytes` | `Vec<u8>` | Datos binarios | Lecturas crudas sin procesar |

### Ejemplo de Uso

```rust
use iot_framework::core::SensorOutput;

// Lectura de temperatura
let temp = SensorOutput::Float(23.7);

// Detección de presencia
let motion = SensorOutput::Bool(true);

// Mensaje de estado
let status = SensorOutput::Text("Sistema OK".to_string());

// Procesamiento con pattern matching
match temp {
    SensorOutput::Float(v) => println!("Temperatura: {} °C", v),
    SensorOutput::Int(v) => println!("Valor entero: {}", v),
    SensorOutput::Text(s) => println!("Mensaje: {}", s),
    _ => println!("Tipo de dato no esperado"),
}
```

### Casos de Uso Comunes

#### Sensores Analógicos
```rust
// DHT22 devuelve múltiples valores como texto formateado
SensorOutput::Text("Temp: 24.5°C, Hum: 60.2%".to_string())

// DS18B20 devuelve temperatura como texto
SensorOutput::Text("23.75 °C".to_string())
```

#### Sensores Digitales
```rust
// Sensor de lluvia MH-RD
SensorOutput::Text("HÚMEDO".to_string())

// PIR motion sensor
SensorOutput::Bool(true)
```

#### Contadores
```rust
// Contador de pulsos
SensorOutput::Int(1542)
```

---

## SensorError

Enum que representa los **errores comunes en operaciones con sensores**.

### Definición

```rust
pub enum SensorError {
    IoError,
    Timeout,
    InvalidData,
    InitializationError,
}
```

### Variantes

| Error | Descripción | Cuándo Ocurre |
|-------|-------------|---------------|
| `IoError` | Error de entrada/salida | Fallo al acceder al GPIO, bus I2C/SPI |
| `Timeout` | Timeout de comunicación | Sensor no responde en tiempo esperado |
| `InvalidData` | Datos inválidos o corruptos | Checksum incorrecto, formato inesperado |
| `InitializationError` | Fallo en inicialización | Sensor no detectado, pin inválido |

### Ejemplo de Uso

```rust
use iot_framework::core::SensorError;

fn leer_sensor() -> Result<f32, SensorError> {
    // Simulación de timeout
    Err(SensorError::Timeout)
}

// Manejo de errores
match leer_sensor() {
    Ok(valor) => println!("Lectura exitosa: {}", valor),
    Err(SensorError::Timeout) => eprintln!("Sensor no respondió a tiempo"),
    Err(SensorError::IoError) => eprintln!("Error de comunicación"),
    Err(SensorError::InvalidData) => eprintln!("Datos corruptos"),
    Err(SensorError::InitializationError) => eprintln!("Sensor no inicializado"),
}
```


---

## Integración con el Framework

### En Implementaciones de Sensores

```rust
use iot_framework::core::traits::sensor::Sensor;
use iot_framework::core::{SensorOutput, SensorError};

struct MiSensor;

impl Sensor for MiSensor {
    type Output = SensorOutput;

    fn read(&mut self) -> Result<SensorOutput, SensorError> {
        // Lectura del sensor
        let valor = 25.3;
        Ok(SensorOutput::Float(valor))
    }
}
```

### En Storage

```rust
use iot_framework::core::traits::storage::Storage;
use iot_framework::core::SensorOutput;

let mut storage = MemoryStorage::new();

// Guardar diferentes tipos de datos
storage.save(SensorOutput::Float(23.5))?;
storage.save(SensorOutput::Text("Sensor OK".to_string()))?;
storage.save(SensorOutput::Bool(true))?;
```

### En Comunicadores

```rust
use iot_framework::core::SensorOutput;
use serde_json;

// Serializar para envío por red
let output = SensorOutput::Float(24.8);
let json = match output {
    SensorOutput::Float(v) => serde_json::json!({"temp": v}),
    SensorOutput::Text(s) => serde_json::json!({"message": s}),
    _ => serde_json::json!({"data": "unknown"}),
};
```

---


## Próximos Pasos

- Consulta [Sensor Trait](traits_sensor.md) para ver cómo usar `SensorOutput`
- Revisa [Implementar Sensores Personalizados](../sensors/custom_sensors.md)
- Explora [Manejo de Errores](../user_guide/basic_concepts.md)
```