# IoT Framework para Gateways en Rust

## Descripción

Este framework permite desarrollar aplicaciones IoT en **dispositivos tipo gateway** (por ejemplo, Raspberry Pi).  
Está diseñado para ser **modular, ligero y flexible**, facilitando la integración de:

- Sensores y actuadores.
- Almacenamiento temporal o persistente.
- Comunicaciones MQTT y consola.
- Drivers hardware (GPIO, I2C, SPI).

Es ideal para proyectos educativos, prototipos o soluciones IoT de bajo recurso.

---

## Características

- **Modularidad:** cada componente (sensores, almacenamiento, comunicación) está separado.
- **Compatibilidad con `embedded-hal`:** facilita la integración con sensores y actuadores.
- **Almacenamiento en memoria:** rápido y simple para pruebas.
- **Conectividad MQTT y consola:** permite envío de datos a brokers o depuración local.
- **Drivers GPIO:** abstrae la lectura y escritura de pines físicos.

---

## Sensores soportados

- DHT11
- DHT22
- MH-RD (sensor de lluvia)
- Otros sensores digitales que implementen el trait `Sensor`

---

## Almacenamiento disponible

- `MemoryStorage` (en RAM)
- Futuras implementaciones podrían incluir almacenamiento en archivos o bases de datos

---

## Comunicación soportada

- `ConsoleCommunicator` (depuración local)
- `MqttCommunicator` (publicación a broker MQTT)

---

