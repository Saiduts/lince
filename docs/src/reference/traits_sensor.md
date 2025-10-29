# Trait `Sensor`

Define el comportamiento general de un sensor dentro del framework IoT.

```rust
pub trait Sensor {
    type Output;
    fn read(&mut self) -> Result<SensorOutput, SensorError>;
}
```

## Descripción

El trait `Sensor` proporciona una interfaz unificada para interactuar con diferentes tipos de sensores físicos. Cualquier dispositivo capaz de medir una variable física o lógica puede implementar este trait para integrarse al framework.

## Associated Types

### `Output`

Tipo de dato que el sensor produce. Aunque está definido como associated type, todas las implementaciones actuales retornan `SensorOutput` en el método `read()`.

```rust
type Output = SensorOutput;
```

## Métodos Requeridos

### `read()`

```rust
fn read(&mut self) -> Result<SensorOutput, SensorError>
```

Lee el valor actual del sensor.

**Parámetros:** Ninguno (método mutable)

**Retorna:**
- `Ok(SensorOutput)`: Lectura exitosa con los datos del sensor
- `Err(SensorError)`: Error durante la operación de lectura

**Errores:**
- `SensorError::IoError`: Fallo de entrada/salida al acceder al sensor
- `SensorError::Timeout`: El sensor no respondió en el tiempo esperado
- `SensorError::InvalidData`: Datos recibidos inválidos o corruptos
- `SensorError::InitializationError`: El sensor no está inicializado

## Implementaciones en el Framework

El trait `Sensor` está implementado para:

| Sensor | Módulo | Mide |
|--------|--------|------|
| `Dht11Sensor` | `devices::sensors::dht11` | Temperatura y humedad (baja precisión) |
| `Dht22Sensor` | `devices::sensors::dht22` | Temperatura y humedad (alta precisión) |
| `Ds18b20Sensor` | `devices::sensors::ds18b20` | Temperatura (OneWire) |
| `MhRdSensor` | `devices::sensors::mhrd` | Detección de lluvia (digital) |

## Ver También

- [SensorOutput](../reference/core_types.md)
- [SensorError](../reference/core_types.md)
- [Crear Sensores Personalizados](../sensors/custom_sensors.md)
- [GpioDriver](../drivers/gpio.md)