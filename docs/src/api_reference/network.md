# Módulo network

Implementaciones de comunicadores para enviar datos a sistemas externos.

## Módulo: `lince::network`

```rust
pub mod console;
pub mod mqtt;
```

Este módulo contiene implementaciones del trait `Communicator` para transmitir datos de sensores.

## Implementaciones Disponibles

### ConsoleCommunicator

Imprime datos a la consola (stdout).

```rust
use lince::network::console::ConsoleCommunicator;

pub struct ConsoleCommunicator;

impl ConsoleCommunicator {
    pub fn new() -> Self;
}
```

**Características:**
-   Salida a stdout
-   Ideal para debugging
-   Sin overhead de red
-   Sin dependencias

**Uso típico:**
- Desarrollo y debugging
- Testing local
- Logging simple

**Ejemplo:**
```rust
let mut console = ConsoleCommunicator::new();

console.send(b"Temperatura: 24.5C")?;
console.send(b"Humedad: 60%")?;

// Salida:
// [CONSOLE] Temperatura: 24.5C
// [CONSOLE] Humedad: 60%
```


### MqttCommunicator

Publica mensajes a un broker MQTT.

```rust
use lince::network::mqtt::MqttCommunicator;

pub struct MqttCommunicator { /* ... */ }

impl MqttCommunicator {
    pub fn new(
        client_id: &str,
        broker: &str,
        port: u16,
        topic: &str
    ) -> Result<Self, CommunicatorError>;
}
```

**Características:**
-   Protocolo IoT estándar
-   QoS (Quality of Service)
-   Publish/Subscribe
-   Reconexión automática (depende del cliente)

**Uso típico:**
- Comunicación IoT
- Integración con Home Assistant
- Dashboards en tiempo real
- Cloud IoT

**Ejemplo:**
```rust
let mut mqtt = MqttCommunicator::new(
    "sensor-cocina",
    "localhost",
    1883,
    "home/kitchen/temperature"
)?;

let json = r#"{"temp": 24.5, "hum": 60}"#;
mqtt.send(json.as_bytes())?;
```

**Documentación:** [MqttCommunicator Reference](../communication/mqtt.md)


## Uso con Trait Communicator

Todas las implementaciones cumplen con el trait `Communicator`:

```rust
use lince::core::traits::communicator::Communicator;

fn publicar_datos<C: Communicator>(
    comm: &mut C,
    sensor: &mut impl Sensor
) -> Result<(), CommunicatorError> {
    let data = sensor.read()
        .map_err(|_| CommunicatorError::SendError)?;
    
    let json = serde_json::to_string(&data)
        .map_err(|_| CommunicatorError::SendError)?;
    
    comm.send(json.as_bytes())
}

// Funciona con cualquier Communicator
publicar_datos(&mut console, &mut dht22)?;
publicar_datos(&mut mqtt, &mut dht22)?;
```

## Crear Communicator Personalizado

Ver guía completa: [Crear Communicators Personalizados](../communication/custom_communicators.md)

Ejemplo básico:

```rust
pub struct MiCommunicator {
    // Estado de conexión
}

impl Communicator for MiCommunicator {
    fn send(&mut self, data: &[u8]) -> Result<(), CommunicatorError> {
        // Implementación
    }
}
```



## Recursos Adicionales

### Documentación Detallada
- [MqttCommunicator Reference](../communication/mqtt.md)
- [Crear Communicators Personalizados](../communication/custom_communicators.md)

### API
- [Trait Communicator](./traits_communicator.md)
- [CommunicatorError](./core_types.md)


## Ver También

- [Trait Communicator](./traits_communicator.md)
- [Core Types](./core_types.md)
- [Arquitectura del Framework](../user_guide/architecture.md)