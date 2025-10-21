# Módulo drivers

Drivers de hardware de bajo nivel para comunicación con dispositivos.

## Módulo: `lince::drivers`

```rust
pub mod gpio;
```

Este módulo contiene drivers que abstraen el acceso a hardware y protocolos de comunicación.

## Drivers Disponibles

### GpioDriver

Driver para control de GPIO individual.

```rust
use lince::drivers::pin::GpioDriver;

pub struct GpioDriver {
    pin: IoPin,
    pin_number: u8,
}

impl GpioDriver {
    pub fn new(pin_number: u8) -> Result<Self, SensorError>;
    pub fn read_level(&self) -> Level;
    pub fn read_bool(&self) -> bool;
    pub fn set_high(&mut self) -> Result<(), rppal::gpio::Error>;
    pub fn set_low(&mut self) -> Result<(), rppal::gpio::Error>;
    pub fn set_mode(&mut self, mode: Mode);
}
```

**Características:**
-   Control directo de GPIO
-   Implementa `embedded-hal` traits
-   Acceso de bajo nivel
-  ️ Manejo seguro de recursos

**Uso típico:**
- Base para sensores digitales
- Control de LEDs, relés
- Lectura de botones
- Protocolos custom

**Ejemplo:**
```rust
let mut pin = GpioDriver::new(17)?;

// Leer
let state = pin.read_bool();

// Escribir
pin.set_high()?;
pin.set_low()?;

// Configurar modo
pin.set_mode(Mode::Output);
```

**Documentación:** [GpioDriver Reference](../drivers/gpio.md)


## Compatibilidad con embedded-hal

Los drivers implementan traits estándar de `embedded-hal`:

```rust
use embedded_hal::digital::v2::{InputPin, OutputPin};

// GpioDriver implementa ambos
impl InputPin for GpioDriver {
    type Error = rppal::gpio::Error;
    
    fn is_high(&self) -> Result<bool, Self::Error> {
        Ok(self.pin.is_high())
    }
    
    fn is_low(&self) -> Result<bool, Self::Error> {
        Ok(self.pin.is_low())
    }
}

impl OutputPin for GpioDriver {
    type Error = rppal::gpio::Error;
    
    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.pin.set_low();
        Ok(())
    }
    
    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.pin.set_high();
        Ok(())
    }
}
```

**Ventaja:** Puedes usar drivers con cualquier librería que espere `embedded-hal`.

## Protocolos Soportados

### GPIO (Digital)
- **Driver**: `GpioDriver`
- **Uso**: Sensores digitales (MH-RD), control ON/OFF
- **Pins**: Cualquier GPIO (0-27)


## Recursos Adicionales

### Documentación Detallada
- [GpioDriver (GPIO)](../drivers/gpio.md)

### HAL
- [RPPAL Documentation](https://docs.rs/rppal/)
- [embedded-hal Documentation](https://docs.rs/embedded-hal/)
- [linux-embedded-hal](https://docs.rs/linux-embedded-hal/)

### Hardware
- [Raspberry Pi GPIO Pinout](https://pinout.xyz/)

## Ver También

- [GpioDriver Reference](../drivers/gpio.md)
- [Crear Sensores Personalizados](../sensors/custom_sensors.md)
- [Arquitectura del Framework](../user_guide/architecture.md)