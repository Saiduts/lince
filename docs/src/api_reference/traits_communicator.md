# Trait `Communicator`

Define la interfaz para enviar datos a destinos externos (MQTT, HTTP, consola, etc.).

```rust
pub trait Communicator {
    fn send(&mut self, data: &[u8]) -> Result<(), CommunicatorError>;
}
```

## Descripción

El trait `Communicator` abstrae la comunicación hacia sistemas externos, permitiendo que el framework envíe datos de sensores a diferentes destinos sin acoplarse a implementaciones específicas.

## Método Requerido

### `send()`

```rust
fn send(&mut self, data: &[u8]) -> Result<(), CommunicatorError>
```

Envía datos al destino configurado.

**Parámetros:**
- `data`: Slice de bytes a transmitir

**Retorna:**
- `Ok(())`: Datos enviados exitosamente
- `Err(CommunicatorError)`: Error durante el envío

**Errores posibles:**
- `CommunicatorError::SendError`: Fallo al enviar (red, permisos, destino no disponible)

**Comportamiento esperado:**
- Las implementaciones DEBEN intentar enviar los datos inmediatamente
- Las implementaciones PUEDEN implementar reintentos internos
- Las implementaciones NO DEBEN bloquear indefinidamente
- Las implementaciones DEBEN ser thread-safe cuando sea necesario

**Ejemplo:**
```rust
let mut mqtt = MqttCommunicator::new("client", "localhost", 1883, "topic")?;

// Texto
mqtt.send(b"temperatura: 24.5")?;

// JSON
let json = r#"{"temp": 24.5}"#;
mqtt.send(json.as_bytes())?;

// Binario
mqtt.send(&[0x01, 0x02, 0x03])?;
```

## Implementaciones en el Framework

| Implementación | Módulo | Destino | Uso |
|----------------|--------|---------|-----|
| `ConsoleCommunicator` | `network::console` | stdout | Debug, desarrollo |
| `MqttCommunicator` | `network::mqtt` | Broker MQTT | IoT, M2M |

## Ver También

- [CommunicatorError](./core_types.md)
- [MqttCommunicator](../communication/mqtt.md)
- [Crear Communicators Personalizados](../communication/custom_communicators.md)