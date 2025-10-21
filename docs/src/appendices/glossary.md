# Glosario

Este glosario define los términos técnicos utilizados en la documentación del IoT Framework.

## A

### Associated Type
Tipo asociado a un trait en Rust. Permite que las implementaciones del trait definan tipos específicos. Ejemplo: `type Output` en el trait `Sensor`.

### Active Low
Configuración donde una señal digital se considera "activa" cuando está en estado LOW (0V). Común en sensores digitales donde LOW = detectado.

## B

### BCM (Broadcom)
Sistema de numeración de pines GPIO usado por Broadcom (fabricante del chip de Raspberry Pi). BCM 4 se refiere al GPIO 4 del chip.

### BOARD
Sistema alternativo de numeración de pines basado en la posición física en el conector de 40 pines de la Raspberry Pi.

### Broker MQTT
Servidor que gestiona las conexiones y mensajes entre clientes MQTT. Ejemplo: Mosquitto.

## C

### Checksum
Suma de verificación usada para detectar errores en transmisión de datos. Los sensores DHT usan un checksum de 8 bits.

### Communicator
Trait del framework que define cómo enviar datos a un destino externo (MQTT, consola, HTTP).

### Cross-compilation
Proceso de compilar código en una máquina (PC) para ejecutarlo en otra arquitectura (Raspberry Pi ARM).

## D

### DHT (Digital Humidity and Temperature)
Familia de sensores digitales de bajo costo para medir temperatura y humedad (DHT11, DHT22).

### Driver
Capa de software que maneja la comunicación de bajo nivel con hardware específico.

### Duty Cycle
Proporción de tiempo que una señal está en estado HIGH vs LOW. Usado en protocolos de comunicación digital.

## E

### Embedded HAL
Hardware Abstraction Layer para sistemas embebidos en Rust. Define traits estándar como `InputPin` y `OutputPin`.

### enum
Tipo de dato en Rust que representa un valor que puede ser una de varias variantes. Ejemplo: `SensorOutput`, `SensorError`.

## G

### Gateway
Dispositivo que actúa como puente entre sensores locales y servicios en la nube o red.

### GPIO (General Purpose Input/Output)
Pines digitales programables que pueden configurarse como entrada o salida. La Raspberry Pi tiene 40 pines GPIO.

## H

### High/LOW
Estados lógicos de una señal digital. HIGH = voltaje alto (3.3V), LOW = voltaje bajo (0V).

## I

### I2C (Inter-Integrated Circuit)
Protocolo de comunicación serial sincrónico que usa 2 cables (SDA y SCL). Permite conectar múltiples dispositivos.

### IoT (Internet of Things)
Red de dispositivos físicos conectados que intercambian datos a través de internet.

## M

### MQTT (Message Queuing Telemetry Transport)
Protocolo ligero de mensajería pub/sub diseñado para dispositivos IoT con ancho de banda limitado.

### Mutex
Mecanismo de sincronización que permite acceso exclusivo a un recurso compartido entre hilos.

## O

### OneWire
Protocolo de comunicación serial que usa un solo cable de datos. Usado por sensores DS18B20.

### Ownership
Concepto central de Rust donde cada valor tiene un único "dueño" que controla su ciclo de vida.

## P

### Pin Driver
Componente del framework que abstrae el control de pines GPIO, proporcionando una interfaz segura.

### Polling
Técnica donde el software pregunta repetidamente al hardware por su estado, en lugar de usar interrupciones.

### Protocol
Conjunto de reglas que define cómo se comunican dos dispositivos. Ejemplos: I2C, SPI, OneWire.

### Pull-up Resistor
Resistencia conectada entre un pin de datos y VCC para mantener el pin en HIGH cuando no está siendo controlado activamente.

### PWM (Pulse Width Modulation)
Técnica para controlar potencia variando el duty cycle de una señal digital.

## Q

### QoS (Quality of Service)
En MQTT, nivel de garantía de entrega de mensajes (0=at most once, 1=at least once, 2=exactly once).

## R

### Result<T, E>
Tipo de Rust para manejo de errores. Puede ser `Ok(T)` si la operación fue exitosa o `Err(E)` si falló.

### RPPAL (Raspberry Pi Peripheral Access Library)
Biblioteca Rust para acceder a periféricos de Raspberry Pi (GPIO, I2C, SPI, etc.).

## S

### Sensor
Dispositivo que mide una variable física (temperatura, humedad, luz) y la convierte en una señal eléctrica.

### SensorOutput
Enum del framework que representa los diferentes tipos de datos que un sensor puede producir.

### SPI (Serial Peripheral Interface)
Protocolo de comunicación serial sincrónico de alta velocidad que usa 4 cables (MISO, MOSI, SCK, CS).

### Storage
Trait del framework que define cómo almacenar y recuperar lecturas de sensores.

### Struct
Tipo de dato en Rust que agrupa varios valores relacionados. Similar a "class" en otros lenguajes.

## T

### Timeout
Tiempo máximo de espera para que una operación se complete. Si se excede, se genera un error.

### Topic
En MQTT, canal jerárquico donde se publican y suscriben mensajes. Ejemplo: `sensores/temperatura/salon`.

### Trait
Interface en Rust que define comportamiento compartido. Similar a interfaces en Java o protocolos en Swift.

### Transductor
Dispositivo que convierte una forma de energía en otra. Los sensores son tipos de transductores.

## U

### UART (Universal Asynchronous Receiver-Transmitter)
Protocolo de comunicación serial asíncrono. Usado para comunicación básica (TX/RX).

## V

### VCC
Terminal de alimentación positiva en un circuito. Generalmente 3.3V o 5V en sistemas embebidos.

### Volatility
En electrónica, tendencia de una señal a cambiar de estado inesperadamente sin intervención del software.

## W

### Wiring
Esquema de conexión física entre componentes electrónicos.

## Términos Específicos del Framework

### `Communicator`
Trait que abstrae el envío de datos a destinos externos.

### `DhtBase`
Driver base compartido por DHT11 y DHT22 que maneja el protocolo de comunicación.

### `MemoryStorage`
Implementación de `Storage` que mantiene datos en RAM (no persistente).

### `MqttCommunicator`
Implementación de `Communicator` que publica mensajes a un broker MQTT.

### `GpioDriver`
Driver GPIO que implementa traits de `embedded-hal` para máxima compatibilidad.

### `SensorError`
Enum que representa los posibles errores al leer un sensor.

## Acrónimos Comunes

| Acrónimo | Significado |
|----------|-------------|
| ADC | Analog-to-Digital Converter |
| API | Application Programming Interface |
| ARM | Advanced RISC Machine |
| BCM | Broadcom Corporation |
| CPU | Central Processing Unit |
| DAC | Digital-to-Analog Converter |
| GPIO | General Purpose Input/Output |
| HAL | Hardware Abstraction Layer |
| HTTP | Hypertext Transfer Protocol |
| I2C | Inter-Integrated Circuit |
| IoT | Internet of Things |
| JSON | JavaScript Object Notation |
| LED | Light-Emitting Diode |
| MCU | Microcontroller Unit |
| MISO | Master In Slave Out |
| MOSI | Master Out Slave In |
| MQTT | Message Queuing Telemetry Transport |
| OS | Operating System |
| PWM | Pulse Width Modulation |
| RAM | Random Access Memory |
| REST | Representational State Transfer |
| RH | Relative Humidity |
| RTC | Real-Time Clock |
| SCL | Serial Clock Line |
| SDA | Serial Data Line |
| SDK | Software Development Kit |
| SPI | Serial Peripheral Interface |
| UART | Universal Asynchronous Receiver-Transmitter |
| USB | Universal Serial Bus |
| UUID | Universally Unique Identifier |

## Unidades de Medida

| Unidad | Descripción |
|--------|-------------|
| °C | Grados Celsius (temperatura) |
| °F | Grados Fahrenheit (temperatura) |
| % RH | Porcentaje de humedad relativa |
| V | Voltios (voltaje) |
| mA | Miliamperios (corriente) |
| kΩ | Kilohmios (resistencia) |
| MHz | Megahertz (frecuencia) |
| µs | Microsegundos (tiempo) |
| ms | Milisegundos (tiempo) |
| Hz | Hertz (frecuencia) |
| Pa | Pascales (presión) |
| lux | Lux (iluminación) |

## Ver También

- [Códigos de Error](./error_codes.md)
- [Hardware Compatible](./hardware.md)
- [Arquitectura del Framework](../user_guide/architecture.md)