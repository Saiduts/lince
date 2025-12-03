# Conceptos Básicos

Esta guía introduce los conceptos fundamentales del  Framework que necesitas entender para trabajar efectivamente con la biblioteca.

## Traits: El Corazón del Framework

Los **traits** son interfaces que definen comportamientos compartidos. El framework se basa en tres traits principales:

### Sensor

Define cómo leer datos de un dispositivo:

```rust
pub trait Sensor {
    type Output;
    fn read(&mut self) -> Result<SensorOutput, SensorError>;
}
```

**¿Por qué es importante?** Cualquier sensor (DHT22, DS18B20, o uno personalizado) puede usarse de la misma forma:

```rust
fn process_sensor<S: Sensor>(sensor: &mut S) {
    match sensor.read() {
        Ok(data) => println!("Datos: {:?}", data),
        Err(e) => eprintln!("Error: {:?}", e),
    }
}

// Funciona con cualquier sensor
process_sensor(&mut dht22);
process_sensor(&mut ds18b20);
```

### Storage

Define cómo almacenar lecturas:

```rust
pub trait Storage {
    fn save(&mut self, data: SensorOutput) -> Result<(), StorageError>;
    fn list(&self) -> Result<Vec<SensorOutput>, StorageError>;
    fn clear(&mut self) -> Result<(), StorageError>;
}
```

### Communicator

Define cómo enviar datos:

```rust
pub trait Communicator {
    fn send(&mut self, data: &[u8]) -> Result<(), CommunicatorError>;
}
```

## SensorOutput: Datos Unificados

`SensorOutput` es un enum que representa cualquier tipo de dato de sensor:

```rust
pub enum SensorOutput {
    Bool(bool),        // Ej: sensor de movimiento (detectado/no detectado)
    Int(i64),          // Ej: contador de pulsos
    Float(f32),        // Ej: temperatura en °C
    Text(String),      // Ej: "Temp: 24.5°C, Hum: 60%"
    Bytes(Vec<u8>),    // Ej: datos binarios crudos
}
```

### Uso

```rust
let temp = SensorOutput::Float(24.5);
let state = SensorOutput::Bool(true);
let read = SensorOutput::Text("Humedad: 60%".to_string());

// Pattern matching
match temp {
    SensorOutput::Float(v) => println!("Temperatura: {}°C", v),
    _ => println!("No es temperatura"),
}
```

## Manejo de Errores

El framework usa `Result` para manejar errores de forma segura:

```rust
pub enum SensorError {
    IoError,              // Error de entrada/salida
    Timeout,              // Sensor no respondió
    InvalidData,          // Datos corruptos
    InitializationError,  // No se pudo inicializar
}
```

### Propagación de Errores

```rust
fn read_and_save(
    sensor: &mut impl Sensor,
    storage: &mut impl Storage
) -> Result<(), String> {
    // El operador ? propaga errores automáticamente
    let data = sensor.read()
        .map_err(|e| format!("Error al leer: {:?}", e))?;
    
    storage.save(data)
        .map_err(|e| format!("Error al guardar: {:?}", e))?;
    
    Ok(())
}
```

### Manejo Robusto

```rust
//   BIEN: Manejo explícito
match sensor.read() {
    Ok(data) => println!("Éxito: {:?}", data),
    Err(SensorError::Timeout) => eprintln!("Timeout, reintentando..."),
    Err(SensorError::InvalidData) => eprintln!("Datos inválidos"),
    Err(e) => eprintln!("Otro error: {:?}", e),
}

//   MAL: Ignorar errores
sensor.read().ok();  // Pierde información del error
```

## GPIO y Numeración de Pines

### BCM vs BOARD

Raspberry Pi tiene dos sistemas de numeración:

```
BOARD (Física)         BCM (Broadcom)
┌─────────┐           ┌─────────┐
│ Pin 1   │           │ 3.3V    │
│ Pin 2   │           │ 5V      │
│ Pin 3   │──────────>│ GPIO 2  │ ◄── BCM 2
│ Pin 4   │           │ 5V      │
│ Pin 5   │──────────>│ GPIO 3  │ ◄── BCM 3
└─────────┘           └─────────┘
```

**El framework usa numeración BCM:**

```rust
// BCM 4 = Pin físico 7
let sensor = Dht22Sensor::new(23)?;  // GPIO 4 (BCM)
```


## Ownership y Borrowing

### Ownership

En Rust, ownership es un sistema que asegura que cada valor tiene un único propietario, y cuando ese propietario sale de alcance, el valor se libera automáticamente. Esto permite manejar memoria de forma segura sin necesidad de un recolector de basura.

```rust
let sensor = Dht22Sensor::new(23)?;  // sensor posee GPIO 4

//   Error: no puedes crear otro sensor en el mismo pin
let sensor2 = Dht22Sensor::new(23)?;  // Falla: GPIO ya en uso
```

### Borrowing

En Rust, borrowing es el mecanismo que permite prestar referencias a un valor sin transferir su propiedad. Se hace mediante referencias inmutables (&valor) o mutables (&mut valor), permitiendo leer o modificar datos sin tomar ownership.

```rust
fn read_sensor(sensor: &mut Dht22Sensor) {  // Préstamo mutable
    sensor.read().ok();
}

let mut sensor = Dht22Sensor::new(23)?;
read_sensor(&mut sensor);  // Préstamo temporal
// sensor aún existe aquí
```

### Move Semantics

En Rust, move semantics significa que al asignar o pasar un valor, su ownership se transfiere (“mueve”) al nuevo propietario, dejando al original inaccesible. Esto evita copias innecesarias y asegura seguridad de memoria.

```rust
let sensor = Dht22Sensor::new(23)?;

let sensor2 = sensor;  // sensor se "mueve" a sensor2

//   Error: sensor ya no es válido
// sensor.read();  // No compila

//   OK: sensor2 ahora es el dueño
sensor2.read()?;
```

## Lifetimes y Referencias

En Rust, lifetimes son una forma de especificar la validez de referencias a valores.

```rust
// Lifetime explícito: 'a
fn first_read<'a>(
    reads: &'a [SensorOutput]
) -> Option<&'a SensorOutput> {
    reads.first()
}

// Uso
let data = vec![
    SensorOutput::Float(24.5),
    SensorOutput::Float(25.0),
];

if let Some(first) = first_read(&data) {
    println!("Primera lectura: {:?}", first);
}
```


## Módulos y Visibilidad

En Rust, los módulos son una forma de organizar el código en diferentes archivos y namespaces.

```rust
mod my_module {
    pub struct Publico { /* accesible fuera */ }
    struct Privado { /* solo interno */ }
    
    pub fn public_function() {}
    fn private_function() {}
}

use my_module::Publico;
// use my_module::Privado;  //   Error: privado
```

## Macros Útiles

### println! y format!

println! es un macro que imprime texto en la consola, con salto de línea automático.
format! es un macro que formatea texto con variables.

```rust
let temp = 24.5;

println!("Temp: {}°C", temp);
println!("Temp: {:.1}°C", temp);  // 1 decimal
println!("Temp: {:>6.1}°C", temp);  // Alineado derecha

let texto = format!("Temp: {:.1}°C", temp);
```

### vec! y array

vec! y array son macros que crean vectores y arrays.

```rust
let v1 = vec![1, 2, 3];  // Vector (heap)
let a1 = [1, 2, 3];      // Array (stack)

let v2 = vec![0; 10];    // Vector de 10 ceros
```

### dbg!

dbg! es un macro que imprime información de depuración en la consola.

```rust
let valor = 42;
dbg!(valor);  // Imprime: [src/main.rs:10] valor = 42

let resultado = dbg!(sensor.read());  // Imprime y retorna
```

## Timing y Delays

En Rust, para timing y delays se usan principalmente:

Delays / Sleep: bloquea la ejecución del hilo por un tiempo determinado usando

Medir tiempo: se usa Instant::now() para capturar un instante y elapsed() para calcular la duración transcurrida.

```rust
use std::thread;
use std::time::Duration;

// Sleep (bloquea el hilo)
thread::sleep(Duration::from_secs(1));
thread::sleep(Duration::from_millis(500));

// Medir tiempo
use std::time::Instant;

let start = Instant::now();
sensor.read()?;
let duration = start.elapsed();
println!("Lectura tomó: {:?}", duration);
```

## JSON y Serialización

En Rust, JSON y serialización se refieren a convertir estructuras de datos entre objetos de Rust y texto JSON, generalmente usando la crate serde:

Serialización: convertir un valor de Rust a JSON.
Deserialización: convertir JSON a una estructura de Rust.

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct DataSensor {
    temperature: f32,
    humidity: f32,
    timestamp: String,
}

let data = DataSensor {
    temperature: 24.5,
    humidity: 60.0,
    timestamp: "2025-10-15T14:30:00Z".to_string(),
};

// A JSON
let json = serde_json::to_string(&datos)?;
println!("{}", json);

// Desde JSON
let restored: DataSensor = serde_json::from_str(&json)?;
```

## Conceptos de Tiempo Real

### Jitter

En Rust, jitter se refiere a la variación inesperada en el tiempo de ejecución de una operación o intervalo, especialmente en sistemas embebidos o IoT.

```rust
//   MAL: Jitter acumulativo
loop {
    sensor.read()?;
    thread::sleep(Duration::from_secs(1));  // Se acumula el tiempo de read()
}

//   BIEN: Timing preciso
let interval = Duration::from_secs(1);
let mut next = Instant::now() + interval;

loop {
    sensor.read()?;
    thread::sleep_until(next);
    next += interval;
}
```


## Mejores Prácticas

###   Hacer

```rust
// Manejo explícito de errores
match sensor.read() {
    Ok(data) => process(data),
    Err(e) => log_error(e),
}

// Validación de entradas
fn create_sensor(pin: u8) -> Result<Sensor, Error> {
    if pin > 27 {
        return Err(Error::PinInvalido);
    }
    // ...
}

// Liberar recursos
{
    let sensor = Dht22Sensor::new(23)?;
    sensor.read()?;
}  // sensor se libera aquí automáticamente
```

###   Evitar

```rust
// Ignorar errores
sensor.read().ok();

// Unwrap sin razón
let data = sensor.read().unwrap();  // Puede hacer panic!

// Magic numbers
thread::sleep(Duration::from_millis(2000));  // ¿Por qué 2000?

// Mejor:
const SENSOR_STABILIZATION_MS: u64 = 2000;
thread::sleep(Duration::from_millis(SENSOR_STABILIZATION_MS));
```


## Dependencias del Framework

El framework utiliza varias bibliotecas externas (crates) para funcionar. Aquí se explica para qué sirve cada una:

### Configuración y Serialización

**serde (v1.0)**
- **¿Qué hace?** Convierte estructuras de Rust a otros formatos (JSON, TOML, etc.) y viceversa
- **¿Por qué la usamos?** Para guardar datos de sensores en formato JSON y enviarlos por red
- **Feature "derive":** Genera automáticamente el código de conversión
```rust
#[derive(Serialize)]
struct SensorData {
    temperature: f32,
    humidity: f32,
}
// serde convierte esto a JSON automáticamente
```

**serde_json (v1.0)**
- **¿Qué hace?** Soporte específico para trabajar con JSON
- **¿Por qué la usamos?** Para leer y escribir datos en formato JSON
- **Ejemplo:** Convertir lecturas de sensores a JSON para enviar por MQTT

**toml (v0.8)**
- **¿Qué hace?** Lee archivos de configuración en formato TOML
- **¿Por qué la usamos?** Para cargar configuraciones del sistema (pines, brokers MQTT, etc.)

**spin_sleep (v1.1)**
- **¿Qué hace?** Proporciona delays de alta precisión
- **¿Por qué la usamos?** Los sensores DHT requieren timing de microsegundos exacto
- **Diferencia con `thread::sleep`:** Más preciso para intervalos cortos (<10ms)

---

### Hardware y Sensores

**rppal (v0.14)**
- **¿Qué hace?** Raspberry Pi Peripheral Access Library - acceso directo al hardware
- **¿Por qué la usamos?** Para controlar GPIO, I2C, SPI y PWM en Raspberry Pi
- **Funcionalidad:** Permite leer y escribir en los pines físicos

**embedded-hal (v0.2.7)**
- **¿Qué hace?** Hardware Abstraction Layer estándar para sistemas embebidos
- **¿Por qué la usamos?** Define traits comunes (`InputPin`, `OutputPin`) para portabilidad
- **Ventaja:** El código puede funcionar en diferentes plataformas sin cambios

---

### Comunicación

**rumqttc (v0.23)**
- **¿Qué hace?** Cliente MQTT para comunicación IoT
- **¿Por qué la usamos?** Para publicar datos de sensores a brokers MQTT
- **Funcionalidad:** Conecta con brokers como Mosquitto, HiveMQ, etc.

---

### Utilidades

**rand (v0.8)**
- **¿Qué hace?** Generador de números aleatorios
- **¿Por qué la usamos?** Para simulaciones y testing sin hardware real

**thiserror (v1.0)**
- **¿Qué hace?** Simplifica la creación de tipos de error personalizados
- **¿Por qué la usamos?** Para definir `SensorError`, `StorageError`, etc. de forma clara
- **Ventaja:** Genera automáticamente mensajes de error legibles

---

### Dependencias Específicas de Linux

Estas solo se usan en sistemas Linux:

**gpio-cdev (v0.5)**
- **¿Qué hace?** Acceso de bajo nivel a GPIO usando `/dev/gpiochipX`
- **¿Por qué la usamos?** Interfaz directa con el driver del kernel de Linux

**linux-embedded-hal (v0.3)**
- **¿Qué hace?** Implementación de `embedded-hal` para Linux
- **¿Por qué la usamos?** Permite usar traits estándar en sistemas Linux completos

---

## Próximos Pasos

Ahora que entiendes los conceptos básicos:

-  Explora [Arquitectura del Framework](./architecture.md)

## Ver También

- [Glosario de Términos](../appendices/glossary.md)
- [Rust Book (oficial)](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)