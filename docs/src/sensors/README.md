# Referencia de Sensores

Esta sección documenta los **sensores compatibles con Lince**, el framework IoT diseñado para aplicaciones modulares y ligeras que se ejecutan en dispositivos tipo gateway, como la Raspberry Pi.

Cada driver de sensor implementa una **interfaz basada en traits**, lo que permite un manejo uniforme de los datos leídos, procesados y enviados a través del framework.

---

##   Estructura

- **[DHT11 – Temperatura y Humedad](dht11.md)**  
  Sensor digital de bajo costo para mediciones básicas de temperatura y humedad.

- **[DHT22 – Temperatura y Humedad](dht22.md)**  
  Versión de mayor precisión del DHT11, con mejor rango de medición y exactitud.

- **[DS18B20 – Temperatura OneWire](ds18b20.md)**  
  Sensor digital de temperatura que utiliza el protocolo OneWire, ideal para mediciones a distancia o en exteriores.

- **[MH-RD – Sensor de Lluvia](mhrd.md)**  
  Sensor analógico y digital para detección de lluvia, útil en estaciones meteorológicas o proyectos de automatización.

- **[Crear Sensores Personalizados](custom_sensors.md)**  
  Guía para implementar nuevos sensores compatibles con los traits y el flujo de datos del núcleo de Lince.

---

## Modelo de Integración

Cada módulo de sensor sigue un **patrón de interfaz consistente**:

```rust
use lince::core::Sensor;
use lince::core::SensorError;

pub struct ExampleSensor;

impl Sensor for ExampleSensor {
    fn read(&mut self) -> Result<f32, SensorError> {
        // Leer y retornar datos del sensor
        Ok(42.0)
    }
}
