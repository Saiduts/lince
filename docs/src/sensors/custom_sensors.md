# Crear Sensores Personalizados

Esta guía te enseña a crear tus propios sensores implementando el trait `Sensor`, permitiéndote integrar cualquier hardware al framework.

## Anatomía de un Sensor

Todo sensor personalizado necesita:

1. **Estructura** que almacena el estado del sensor
2. **Constructor** que inicializa el hardware
3. **Implementación del trait `Sensor`**

```rust
use lince::core::traits::sensor::Sensor;
use lince::core::{SensorOutput, SensorError};

// 1. Estructura
pub struct MiSensor {
    // Estado interno
}

// 2. Constructor
impl MiSensor {
    pub fn new(/* parámetros */) -> Result<Self, SensorError> {
        // Inicialización
        Ok(Self { /* ... */ })
    }
}

// 3. Implementación del trait
impl Sensor for MiSensor {
    type Output = SensorOutput;
    
    fn read(&mut self) -> Result<SensorOutput, SensorError> {
        // Lógica de lectura
    }
}
```


## Ejemplo: Sensor Simulado


```rust
use rand::Rng;

pub struct SimulatedTemperatureSensor {
    base_temp: f32,
    variation: f32,
}

impl SimulatedTemperatureSensor {
    pub fn new(base_temp: f32, variation: f32) -> Result<Self, SensorError> {
        Ok(Self { base_temp, variation })
    }
}

impl Sensor for SimulatedTemperatureSensor {
    type Output = SensorOutput;
    
    fn read(&mut self) -> Result<SensorOutput, SensorError> {
        let mut rng = rand::thread_rng();
        let offset = rng.gen_range(-self.variation..=self.variation);
        let temp = self.base_temp + offset;
        
        Ok(SensorOutput::Float(temp))
    }
}
```

## Checklist de Implementación

Al crear un sensor personalizado, verifica:

###  Inicialización
- [ ] Constructor valida parámetros
- [ ] Hardware se configura correctamente
- [ ] Errores se mapean a `SensorError`

###  Lectura
- [ ] `read()` retorna `Result<SensorOutput, SensorError>`
- [ ] Se validan los datos antes de retornarlos
- [ ] Se manejan timeouts apropiadamente

###  Manejo de Errores
- [ ] Todos los errores de hardware se capturan
- [ ] Se usan los tipos de error apropiados


## Ver También

- [Trait Sensor API](../api_reference/traits_sensor.md)
- [GpioDriver Reference](../drivers/gpio.md)
- [SensorOutput y SensorError](../api_reference/core_types.md)