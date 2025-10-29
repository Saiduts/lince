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
fn procesar_sensor<S: Sensor>(sensor: &mut S) {
    match sensor.read() {
        Ok(data) => println!("Datos: {:?}", data),
        Err(e) => eprintln!("Error: {:?}", e),
    }
}

// Funciona con cualquier sensor
procesar_sensor(&mut dht22);
procesar_sensor(&mut ds18b20);
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
let estado = SensorOutput::Bool(true);
let lectura = SensorOutput::Text("Humedad: 60%".to_string());

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
fn leer_y_guardar(
    sensor: &mut impl Sensor,
    storage: &mut impl Storage
) -> Result<(), String> {
    // El operador ? propaga errores automáticamente
    let datos = sensor.read()
        .map_err(|e| format!("Error al leer: {:?}", e))?;
    
    storage.save(datos)
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

### Modos de Pin

```rust
use rppal::gpio::Mode;

// Entrada: para leer sensores
pin.set_mode(Mode::Input);

// Salida: para controlar LEDs, relés
pin.set_mode(Mode::Output);
```

### Pull-up/Pull-down

```rust
// Pull-up: pin por defecto en HIGH
pin.set_mode(Mode::Input);
pin.set_pullupdown(PullUpDown::PullUp);

// Pull-down: pin por defecto en LOW
pin.set_pullupdown(PullUpDown::PullDown);
```

## Ownership y Borrowing

### Ownership

Cada sensor "posee" su hardware:

```rust
let sensor = Dht22Sensor::new(23)?;  // sensor posee GPIO 4

//   Error: no puedes crear otro sensor en el mismo pin
let otro = Dht22Sensor::new(23)?;  // Falla: GPIO ya en uso
```

### Borrowing

Para compartir acceso temporalmente:

```rust
fn leer_sensor(sensor: &mut Dht22Sensor) {  // Préstamo mutable
    sensor.read().ok();
}

let mut sensor = Dht22Sensor::new(23)?;
leer_sensor(&mut sensor);  // Préstamo temporal
// sensor aún existe aquí
```

### Move Semantics

```rust
let sensor = Dht22Sensor::new(23)?;

let sensor2 = sensor;  // sensor se "mueve" a sensor2

//   Error: sensor ya no es válido
// sensor.read();  // No compila

//   OK: sensor2 ahora es el dueño
sensor2.read()?;
```

## Lifetimes y Referencias

```rust
// Lifetime explícito: 'a
fn primera_lectura<'a>(
    lecturas: &'a [SensorOutput]
) -> Option<&'a SensorOutput> {
    lecturas.first()
}

// Uso
let datos = vec![
    SensorOutput::Float(24.5),
    SensorOutput::Float(25.0),
];

if let Some(primera) = primera_lectura(&datos) {
    println!("Primera lectura: {:?}", primera);
}
```


## Módulos y Visibilidad

```rust
mod mi_modulo {
    pub struct Publico { /* accesible fuera */ }
    struct Privado { /* solo interno */ }
    
    pub fn funcion_publica() {}
    fn funcion_privada() {}
}

use mi_modulo::Publico;
// use mi_modulo::Privado;  //   Error: privado
```

## Macros Útiles

### println! y format!

```rust
let temp = 24.5;

println!("Temp: {}°C", temp);
println!("Temp: {:.1}°C", temp);  // 1 decimal
println!("Temp: {:>6.1}°C", temp);  // Alineado derecha

let texto = format!("Temp: {:.1}°C", temp);
```

### vec! y array

```rust
let v1 = vec![1, 2, 3];  // Vector (heap)
let a1 = [1, 2, 3];      // Array (stack)

let v2 = vec![0; 10];    // Vector de 10 ceros
```

### dbg!

```rust
let valor = 42;
dbg!(valor);  // Imprime: [src/main.rs:10] valor = 42

let resultado = dbg!(sensor.read());  // Imprime y retorna
```

## Timing y Delays

```rust
use std::thread;
use std::time::Duration;

// Sleep (bloquea el hilo)
thread::sleep(Duration::from_secs(1));
thread::sleep(Duration::from_millis(500));

// Medir tiempo
use std::time::Instant;

let inicio = Instant::now();
sensor.read()?;
let duracion = inicio.elapsed();
println!("Lectura tomó: {:?}", duracion);
```

## JSON y Serialización

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct DatosSensor {
    temperatura: f32,
    humedad: f32,
    timestamp: String,
}

let datos = DatosSensor {
    temperatura: 24.5,
    humedad: 60.0,
    timestamp: "2025-10-15T14:30:00Z".to_string(),
};

// A JSON
let json = serde_json::to_string(&datos)?;
println!("{}", json);

// Desde JSON
let recuperado: DatosSensor = serde_json::from_str(&json)?;
```

## Conceptos de Tiempo Real

### Jitter

Variación en el timing de las lecturas:

```rust
//   MAL: Jitter acumulativo
loop {
    sensor.read()?;
    thread::sleep(Duration::from_secs(1));  // Se acumula el tiempo de read()
}

//   BIEN: Timing preciso
let intervalo = Duration::from_secs(1);
let mut proximo = Instant::now() + intervalo;

loop {
    sensor.read()?;
    thread::sleep_until(proximo);
    proximo += intervalo;
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
fn crear_sensor(pin: u8) -> Result<Sensor, Error> {
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

## Próximos Pasos

Ahora que entiendes los conceptos básicos:

-  Explora [Arquitectura del Framework](./architecture.md)

## Ver También

- [Glosario de Términos](../appendices/glossary.md)
- [Rust Book (oficial)](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)