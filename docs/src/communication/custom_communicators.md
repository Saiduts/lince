# Crear Communicators Personalizados

Esta guía te enseña a implementar tu propio sistema de comunicación creando estructuras que implementen el trait `Communicator`.

## Anatomía de un Communicator

Todo communicator personalizado necesita:

1. **Estructura** que mantiene la conexión/configuración
2. **Constructor** que establece la conexión
3. **Implementación del trait `Communicator`**

```rust
use lince::core::traits::communicator::{Communicator, CommunicatorError};

// 1. Estructura
pub struct MiCommunicator {
    // Estado de conexión
}

// 2. Constructor
impl MiCommunicator {
    pub fn new(/* parámetros */) -> Result<Self, CommunicatorError> {
        // Establecer conexión
        Ok(Self { /* ... */ })
    }
}

// 3. Implementación del trait
impl Communicator for MiCommunicator {
    fn send(&mut self, data: &[u8]) -> Result<(), CommunicatorError> {
        // Enviar datos
    }
}
```

## Ejemplo: ConsoleCommunicator

Comunicador que imprime en la consola (útil para debugging):

```rust
use lince::core::traits::communicator::{Communicator, CommunicatorError};

pub struct ConsoleCommunicator;

impl ConsoleCommunicator {
    
    pub fn new() -> Self {
        Self
    }
}

impl Communicator for ConsoleCommunicator {
    
    fn send(&mut self, data: &[u8]) -> Result<(), CommunicatorError> {
        let s = String::from_utf8_lossy(data);
        println!("[CONSOLE] {}", s);
        Ok(())
    }
}


fn main() {
    let mut console = ConsoleCommunicator::new();
    
    console.send(b"Temperatura: 24.5C")?;
    console.send(b"Humedad: 60%")?;
}

```


## Checklist de Implementación

- [ ] Constructor valida parámetros
- [ ] Establecer conexión en constructor
- [ ] Timeout configurado
- [ ] Manejo robusto de errores
- [ ] Reconexión si aplica
- [ ] Liberar recursos (Drop)
- [ ] Documentación con ejemplos
- [ ] Tests unitarios
- [ ] Tests de integración

## Dependencias Útiles

```toml
[dependencies]
# HTTP
reqwest = { version = "0.11", features = ["blocking", "json"] }

# WebSocket
tungstenite = "0.20"

# Serial
serialport = "4.2"

# Async
tokio = { version = "1", features = ["full"] }
async-trait = "0.1"
```

## Ver También

- [Trait Communicator](../api_reference/traits_communicator.md)
- [MqttCommunicator](./mqtt.md)
- [CommunicatorError](../api_reference/core_types.md)