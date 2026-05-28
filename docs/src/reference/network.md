# Módulo network

Implementaciones de comunicadores y formateadores para enviar datos a sistemas externos.

## Network

```rust
pub mod console;
pub mod mqtt;
pub mod smart_campus;
```

---

## Implementaciones Disponibles

### ConsoleCommunicator

Imprime datos a stdout. Ideal para debugging.

```rust
use lince::network::console::ConsoleCommunicator;

let mut console = ConsoleCommunicator::new();
console.send(b"Temperatura: 24.5C")?;
// [CONSOLE] Temperatura: 24.5C
```

**Documentación:** no requiere configuración adicional.

---

### MqttCommunicator

Publica mensajes a un broker MQTT.

```rust
use lince::network::mqtt::MqttCommunicator;

let mut mqtt = MqttCommunicator::new(
    "lince-gateway",
    "localhost",
    1883,
    "device/messages"
)?;

mqtt.send(json.as_bytes())?;
```

**Errores que devuelve `send()`:**
- `CommunicatorError::Disconnected` — broker no alcanzable. El dato debe guardarse en `SqliteStorage` para reintento.
- `CommunicatorError::SendError` — error de protocolo. No tiene sentido reintentar.

**Documentación:** [MqttCommunicator](../communication/mqtt.md)

---

### SmartCampusFormatter

Convierte valores numéricos o booleanos al JSON estructurado Smart Campus.
Recibe valores ya extraídos por `SensorParser`, no `SensorOutput` crudo.

```rust
use lince::network::smart_campus::{SmartCampusFormatter, SmartCampusHeader};
use lince::parser::SensorParser;

let formatter = SmartCampusFormatter::new(
    SmartCampusHeader::new("raspberry-lince", "device/messages")
);

// Desde DHT
let valores = SensorParser::dht(&output)?;
let json = formatter.desde_mapa(&valores);

// Desde DS18B20
let temp = SensorParser::ds18b20(&output)?;
let json = formatter.desde_valor("temperatura", temp);

// Desde MH-RD
let mojado = SensorParser::mhrd(&output)?;
let json = formatter.desde_bool("lluvia", mojado);
```

**Documentación:** [SmartCampusFormatter](../communication/smart_campus.md)

---

## Flujo Completo

```rust
// Parser → Formatter → Communicator → Storage (si falla)

let valores = SensorParser::dht(&output)?;         // extraer valores
let json    = formatter.desde_mapa(&valores);       // formatear
storage.save(output)?;                              // guardar siempre

match mqtt.send(json.as_bytes()) {
    Ok(())                               => {}
    Err(CommunicatorError::Disconnected) => {
        storage.flush_pending(&mut mqtt); // reenviar cola
    }
    Err(CommunicatorError::SendError)    => {}
}
```

---

## Crear Communicator Personalizado

Ver guía: [Crear Communicators Personalizados](../communication/custom_communicators.md)

```rust
impl Communicator for MiCommunicator {
    fn send(&mut self, data: &[u8]) -> Result<(), CommunicatorError> {
        // implementación
    }
}
```

---

## Ver También

- [Parser](../parser.md)
- [SqliteStorage — flush_pending](../storage/sqlite_storage.md)
- [Trait Communicator](./traits_communicator.md)
- [CommunicatorError](./core_types.md)