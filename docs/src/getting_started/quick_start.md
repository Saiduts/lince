# Guía Rápida

---

## Prerrequisitos

Asegúrate de tener:

- Rust instalado (`rustc --version` debe funcionar)  
- Una Raspberry Pi con Raspbian / Raspberry Pi OS  
- Un sensor **DHT22** conectado al **GPIO 4**

---

## 1. Crear un Nuevo Proyecto

```bash
cargo new mi-proyecto-iot
cd mi-proyecto-iot
```

---

## 2. Agregar Dependencias

Edita tu `Cargo.toml` y agrega:

```toml
[dependencies]
lince = { git = "https://github.com/Saiduts/lince" }
```

---

## 3. Escribir el Código

Reemplaza el contenido de `src/main.rs`:

```rust
use lince::devices::sensors::dht22::Dht22Sensor;
use lince::core::traits::sensor::Sensor;
use std::thread;
use std::time::Duration;

fn main() {
    println!("  Iniciando sensor DHT22...");
    
    // Crear el sensor en GPIO 23
    let mut sensor = Dht22Sensor::new(23)
        .expect("No se pudo inicializar el sensor");
    
    // Esperar estabilización
    thread::sleep(Duration::from_secs(2));
    
    // Realizar 5 lecturas
    for i in 1..=5 {
        println!(" Lectura #{}", i);
        match sensor.read() {
            Ok(data) => println!("    Datos: {:?}", data),
            Err(e) => println!("    Error: {:?}", e),
        }
        
        thread::sleep(Duration::from_secs(3));
    }
    
    println!("Programa finalizado");
}
```

---

## 4. Compilar y Ejecutar

```bash
# Compilar (puede tardar en la primera vez)
cargo build --release

# Ejecutar con permisos de GPIO
sudo ./target/release/mi-proyecto-iot
```

---

## 5. Salida Esperada

```
  Iniciando sensor DHT22...

 Lectura #1
    Datos: Text("Temp: 24.3°C, Hum: 58.2%")

 Lectura #2
    Datos: Text("Temp: 24.4°C, Hum: 58.1%")

 Lectura #3
    Datos: Text("Temp: 24.3°C, Hum: 58.3%")

 Lectura #4
    Datos: Text("Temp: 24.5°C, Hum: 58.0%")

 Lectura #5
    Datos: Text("Temp: 24.4°C, Hum: 58.2%")

 Programa finalizado
```

---

## Agregar Almacenamiento

Para guardar los datos en memoria:

```rust
use lince::devices::sensors::dht22::Dht22Sensor;
use lince::storage::memory::MemoryStorage;
use lince::core::traits::sensor::Sensor;
use lince::core::traits::storage::Storage;
use std::thread;
use std::time::Duration;

fn main() {
    let mut sensor = Dht22Sensor::new(23).unwrap();
    let mut storage = MemoryStorage::new();
    
    for _ in 0..5 {
        if let Ok(data) = sensor.read() {
            storage.save(data).unwrap();
        }
        thread::sleep(Duration::from_secs(3));
    }
    
    // Mostrar datos almacenados
    let lecturas = storage.list().unwrap();
    println!("Total de lecturas: {}", lecturas.len());
    for (i, lectura) in lecturas.iter().enumerate() {
        println!("{}. {:?}", i + 1, lectura);
    }
}
```

---

## Enviar Datos a MQTT

Para publicar en un broker MQTT:

```rust
use lince::devices::sensors::dht22::Dht22Sensor;
use lince::network::mqtt::MqttCommunicator;
use lince::core::traits::communicator::Communicator;
use lince::core::traits::sensor::Sensor;
use std::thread;
use std::time::Duration;

fn main() {
    let mut sensor = Dht22Sensor::new(23).unwrap();
    let mut mqtt = MqttCommunicator::new(
        "mi-sensor",
        "localhost",
        1883,
        "sensores/temperatura"
    ).unwrap();
    
    loop {
        if let Ok(data) = sensor.read() {
            let mensaje = format!("{:?}", data);
            mqtt.send(mensaje.as_bytes()).unwrap();
            println!(" Enviado: {}", mensaje);
        }
        thread::sleep(Duration::from_secs(60));
    }
}
```

---

## Solución de Problemas

**Error: “Permission denied”**  
- Ejecuta con `sudo` para acceder a GPIO  
- O configura permisos:  
  ```bash
  sudo usermod -a -G gpio $USER
  ```

**Error: “Timeout” o “InvalidData”**  
- Verifica las conexiones del sensor  
- Usa el número de pin **BCM**, no **BOARD**  
- Espera 2–3 segundos entre lecturas  

**Error de compilación**  
- Actualiza Rust: `rustup update`  
- Verifica dependencias en `Cargo.toml`

---

