# Lince — IoT Framework para Gateways en Rust

Framework modular y ligero para construir aplicaciones IoT en dispositivos tipo gateway (Raspberry Pi). Escrito en Rust, ofrece abstracciones para sensores, almacenamiento, comunicación MQTT y control de hardware GPIO.

---

## Requisitos

- **Hardware:** Raspberry Pi 4 (o compatible con `rppal`)
- **OS:** Raspberry Pi OS (32 o 64 bits)
- **Rust:** 1.70 o superior

Si no tienes Rust instalado:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

---

## Instalación y uso

Lince se usa como dependencia Git desde tu propio proyecto Cargo. No necesitas clonar el repositorio manualmente a menos que quieras contribuir o explorar el código fuente.

### Crear un nuevo proyecto

```bash
cargo new mi-proyecto-iot
cd mi-proyecto-iot
```

### Agregar Lince como dependencia

Edita tu `Cargo.toml`:

```toml
[dependencies]
lince = { git = "https://github.com/Saiduts/lince" }
```

Luego ejecuta `cargo build` para descargar y compilar la dependencia.


---

## Importar en tu proyecto

Lince expone sus módulos principales desde la raíz del crate:

```rust
// Traits principales
use lince::Sensor;
use lince::Storage;
use lince::Communicator;

// Sensores
use lince::devices::sensors::dht22::Dht22Sensor;
use lince::devices::sensors::dht11::Dht11Sensor;
use lince::devices::sensors::ds18b20::Ds18b20Sensor;
use lince::devices::sensors::mhrd::MhRdSensor;

// Almacenamiento
use lince::storage::memory::MemoryStorage;

// Comunicación
use lince::MqttCommunicator;
use lince::ConsoleCommunicator;

// Tipos
use lince::core::SensorOutput;
use lince::core::traits::sensor::Sensor;
use lince::core::traits::storage::Storage;
use lince::core::traits::communicator::Communicator;
```

---

## Documentación

La documentación completa del framework está disponible en:

**[Introducción — Documentación de Framework para IoT Lince](https://documentacionlincefrm.web.app)**

Incluye guías de instalación, referencia de todos los sensores soportados, descripción de los traits y ejemplos de uso.

---

## Generar la documentación localmente

Si la página no está disponible, puedes generar la documentación en tu máquina a partir del repositorio.

### Requisitos previos

```bash
# Instalar mdBook
cargo install mdbook
```

### Pasos

```bash
# 1. Clonar el repositorio
git clone https://github.com/Saiduts/lince.git
cd lince

# 2. Entrar a la carpeta de documentación
cd docs

# 3. Generar y abrir la documentación en el navegador
mdbook serve --open
```

Esto levanta un servidor local en `http://localhost:3000` con toda la documentación navegable.

Si solo quieres generar los archivos estáticos sin abrirlos:

```bash
mdbook build
# Los archivos quedan en docs/book/
```

---

## Sensores soportados

| Sensor | Variable | Protocolo |
|--------|----------|-----------|
| DHT11 | Temperatura y humedad | Digital 1-Wire |
| DHT22 | Temperatura y humedad | Digital 1-Wire |
| DS18B20 | Temperatura | OneWire |
| MH-RD | Lluvia / humedad superficial | Digital (DO) |

---
