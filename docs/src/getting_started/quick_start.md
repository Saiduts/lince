# Guía Rápida

---

## Prerrequisitos

- Rust instalado (`rustc --version` debe funcionar)
- Raspberry Pi con Raspberry Pi OS
- Un sensor **DHT22** conectado al **GPIO 23**

---

## 1. Crear un Nuevo Proyecto

```bash
cargo new mi-proyecto-iot
cd mi-proyecto-iot
```

---

## 2. Agregar la Dependencia

```toml
[dependencies]
lince = { git = "https://github.com/Saiduts/lince" }
rusqlite = { version = "0.31", features = ["bundled"] }
```

---

## 3. Lectura Simple

```rust
use lince::devices::sensors::dht22::Dht22Sensor;
use lince::core::traits::sensor::Sensor;
use std::thread;
use std::time::Duration;

fn main() {
    let mut sensor = Dht22Sensor::new(23)
        .expect("No se pudo inicializar el sensor");

    thread::sleep(Duration::from_secs(2));

    for i in 1..=5 {
        match sensor.read() {
            Ok(data)  => println!("Lectura {}: {:?}", i, data),
            Err(e)    => eprintln!("Error: {:?}", e),
        }
        thread::sleep(Duration::from_secs(3));
    }
}
```

---

## 4. Con Parser — extraer valores para cálculos

```rust
use lince::devices::sensors::dht22::Dht22Sensor;
use lince::core::traits::sensor::Sensor;
use lince::parser::SensorParser;
use std::thread;
use std::time::Duration;

fn main() {
    let mut sensor = Dht22Sensor::new(23).unwrap();
    thread::sleep(Duration::from_secs(2));

    loop {
        if let Ok(output) = sensor.read() {
            // Parser extrae los valores como números
            if let Ok(valores) = SensorParser::dht(&output) {
                let temp = valores["temperatura"];
                let hum  = valores["humedad"];

                println!("Temp: {}°C  Hum: {}%", temp, hum);

                if temp > 30.0 {
                    println!("Alerta: temperatura alta");
                }
            }
        }
        thread::sleep(Duration::from_secs(3));
    }
}
```

---

## 5. Con Formatter — enviar a Smart Campus

```rust
use lince::devices::sensors::dht22::Dht22Sensor;
use lince::core::traits::sensor::Sensor;
use lince::core::traits::communicator::Communicator;
use lince::parser::SensorParser;
use lince::network::smart_campus::{SmartCampusFormatter, SmartCampusHeader};
use lince::network::mqtt::MqttCommunicator;
use std::thread;
use std::time::Duration;

fn main() {
    let mut sensor = Dht22Sensor::new(23).unwrap();

    let formatter = SmartCampusFormatter::new(
        SmartCampusHeader::new("raspberry-lince", "device/messages")
    );

    let mut mqtt = MqttCommunicator::new(
        "lince", "localhost", 1883, "device/messages"
    ).unwrap();

    thread::sleep(Duration::from_secs(2));

    loop {
        if let Ok(output) = sensor.read() {
            // 1. Parser extrae los valores
            if let Ok(valores) = SensorParser::dht(&output) {
                // 2. Formatter convierte a JSON Smart Campus
                let json = formatter.desde_mapa(&valores);

                // 3. MQTT envía
                match mqtt.send(json.as_bytes()) {
                    Ok(())  => println!("Enviado: {}", json),
                    Err(e)  => eprintln!("Error: {:?}", e),
                }
            }
        }
        thread::sleep(Duration::from_secs(10));
    }
}
```

---

## 6. Con Reintento por Desconexión

```rust
use lince::devices::sensors::dht22::Dht22Sensor;
use lince::core::traits::sensor::Sensor;
use lince::core::traits::storage::Storage;
use lince::core::traits::communicator::{Communicator, CommunicatorError};
use lince::parser::SensorParser;
use lince::network::smart_campus::{SmartCampusFormatter, SmartCampusHeader};
use lince::network::mqtt::MqttCommunicator;
use lince::storage::sqlite::SqliteStorage;
use std::thread;
use std::time::Duration;

fn main() {
    let mut sensor    = Dht22Sensor::new(23).unwrap();
    let formatter     = SmartCampusFormatter::new(
        SmartCampusHeader::new("raspberry-lince", "device/messages")
    );
    let mut mqtt      = MqttCommunicator::new(
        "lince", "localhost", 1883, "device/messages"
    ).unwrap();
    let mut storage   = SqliteStorage::new("pendientes.db").unwrap();

    thread::sleep(Duration::from_secs(2));

    loop {
        // 1. Sensor lee
        let output = match sensor.read() {
            Ok(d)  => d,
            Err(e) => { eprintln!("Error sensor: {:?}", e); continue; }
        };

        // 2. Parser extrae valores
        let valores = match SensorParser::dht(&output) {
            Ok(v)  => v,
            Err(e) => { eprintln!("Error parser: {:?}", e); continue; }
        };

        // 3. Formatter convierte a JSON
        let json = formatter.desde_mapa(&valores);

        // 4. SQLite guarda siempre
        storage.save(output).unwrap();

        // 5. MQTT envía — si no hay conexión, SQLite reenvía la cola
        match mqtt.send(json.as_bytes()) {
            Ok(())                               => println!("Enviado: {}", json),
            Err(CommunicatorError::Disconnected) => {
                eprintln!("Sin conexión. Reintentando pendientes...");
                let n = storage.flush_pending(&mut mqtt);
                println!("Reenviados: {}", n);
            }
            Err(CommunicatorError::SendError)    => eprintln!("Error de protocolo"),
        }

        thread::sleep(Duration::from_secs(10));
    }
}
```

---

## Solución de Problemas

**`Permission denied` al acceder GPIO**
```bash
sudo usermod -a -G gpio $USER
# Cerrar sesión y volver a entrar
```

**`Timeout` o `InvalidData`**
- Verificar conexiones del sensor
- Usar numeración **BCM**, no **BOARD**
- Esperar 2–3 segundos entre lecturas

**`ParseError::FormatoInvalido`**
- Verificar que el método del parser corresponde al sensor usado
- `SensorParser::dht()` para DHT11/DHT22
- `SensorParser::ds18b20()` para DS18B20
- `SensorParser::mhrd()` para MH-RD

**Error de compilación**
```bash
rustup update
```

---

## Ver También

- [Parser](../parser.md)
- [SmartCampusFormatter](../communication/smart_campus.md)
- [SqliteStorage](../storage/sqlite_storage.md)
- [Códigos de Error](../appendices/error_codes.md)