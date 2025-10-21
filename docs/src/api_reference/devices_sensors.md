# Módulo devices::sensors

Implementaciones de sensores físicos para el framework IoT.

## Módulo: `lince::devices::sensors`

```rust
pub mod dht11;
pub mod dht22;
pub mod ds18b20;
pub mod mhrd;
```

Este módulo contiene todas las implementaciones concretas de sensores que implementan el trait `Sensor`.

## Sensores Disponibles

### DHT11

Sensor de temperatura y humedad de bajo costo.

```rust
use lince::devices::sensors::dht11::Dht11Sensor;

pub struct Dht11Sensor { /* ... */ }

impl Dht11Sensor {
    pub fn new(pin: u8) -> Result<Self, SensorError>;
}

impl Sensor for Dht11Sensor {
    type Output = SensorOutput;
    fn read(&mut self) -> Result<SensorOutput, SensorError>;
}
```


**Ejemplo:**
```rust
let mut sensor = Dht11Sensor::new(17)?;
let data = sensor.read()?;  // "Temp: 24°C, Hum: 60%"
```

**Documentación:** [DHT11 Reference](../sensors/dht11.md)

### DHT22

Sensor de temperatura y humedad de alta precisión.

```rust
use lince::devices::sensors::dht22::Dht22Sensor;

pub struct Dht22Sensor { /* ... */ }

impl Dht22Sensor {
    pub fn new(pin: u8) -> Result<Self, SensorError>;
}
```


**Ejemplo:**
```rust
let mut sensor = Dht22Sensor::new(4)?;
let data = sensor.read()?;  // "Temp: 24.3°C, Hum: 58.2%"
```

**Documentación:** [DHT22 Reference](../sensors/dht22.md)

### DS18B20

Sensor de temperatura digital con protocolo OneWire.

```rust
use lince::devices::sensors::ds18b20::Ds18b20Sensor;

pub struct Ds18b20Sensor { /* ... */ }

impl Ds18b20Sensor {
    pub fn new(device_id: &str) -> Result<Self, SensorError>;
}
```



**Ejemplo:**
```rust
let mut sensor = Ds18b20Sensor::new("28-00000a1b2c3d")?;
let data = sensor.read()?;  // "24.56 °C"

// Múltiples sensores en el mismo bus
let s1 = Ds18b20Sensor::new("28-00000a1b2c3d")?;
let s2 = Ds18b20Sensor::new("28-00000e4f5a6b")?;
```

**Documentación:** [DS18B20 Reference](../sensors/ds18b20.md)

### MH-RD

Sensor de lluvia digital.

```rust
use lince::devices::sensors::mhrd::MhRdSensor;

pub struct MhRdSensor { /* ... */ }

impl MhRdSensor {
    pub fn new(pin: u8, active_low: bool) -> Result<Self, SensorError>;
}
```


**Ejemplo:**
```rust
let mut sensor = MhRdSensor::new(17, true)?;  // active_low
let data = sensor.read()?;  // "HÚMEDO" o "SECO"

if let SensorOutput::Text(estado) = data {
    if estado == "HÚMEDO" {
        println!("¡Está lloviendo!");
    }
}
```

**Documentación:** [MH-RD Reference](../sensors/mhrd.md)



## Uso Polimórfico

### Con Trait Objects

```rust
let sensores: Vec<Box<dyn Sensor<Output = SensorOutput>>> = vec![
    Box::new(Dht11Sensor::new(17)?),
    Box::new(Dht22Sensor::new(4)?),
    Box::new(Ds18b20Sensor::new("28-abc")?),
    Box::new(MhRdSensor::new(27, true)?),
];

for sensor in sensores.iter_mut() {
    match sensor.read() {
        Ok(data) => println!("{:?}", data),
        Err(e) => eprintln!("Error: {:?}", e),
    }
}
```

### Con Genéricos

```rust
fn leer_n_veces<S: Sensor>(sensor: &mut S, n: usize) -> Vec<SensorOutput> {
    let mut resultados = Vec::new();
    
    for _ in 0..n {
        if let Ok(data) = sensor.read() {
            resultados.push(data);
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
    
    resultados
}

// Funciona con cualquier sensor
let lecturas_dht = leer_n_veces(&mut dht22, 10);
let lecturas_ds = leer_n_veces(&mut ds18b20, 10);
```

## Dependencias

```toml
[dependencies]
# Para sensores DHT
rppal = "0.14"

# Para DS18B20 (lectura de sysfs)
# No requiere dependencias adicionales, usa std::fs

# Opcionalmente para timestamps
chrono = "0.4"
```

## Notas de Hardware

### GPIO Disponibles

Raspberry Pi tiene GPIO limitados. Planifica tu uso:

```
GPIO Recomendados para sensores:
- GPIO 4:  DS18B20 (OneWire por defecto)
- GPIO 17: DHT11/DHT22 o MH-RD
- GPIO 22: DHT11/DHT22 o MH-RD
- GPIO 23: DHT11/DHT22
- GPIO 27: Disponible

GPIO a evitar:
- GPIO 0-1:  I2C (reservados)
- GPIO 7-11: SPI (reservados si usas SPI)
- GPIO 14-15: UART (reservados si usas serial)
```

### Pull-up Resistors

| Sensor | Valor Pull-up Requerido | Ubicación |
|----------|----|-----------|
| DHT11/22 |   4.7-10kΩ | Entre DATA y VCC |
| DS18B20 |   4.7kΩ | Entre DQ y VCC |
| MH-RD |   - | Módulo incluye |

### Alimentación

```
Todos los sensores: 3.3V o 5V
Recomendado: Usar 3.3V para compatibilidad directa con GPIO

Consumo típico:
- DHT11/22: ~2.5mA
- DS18B20: ~1.5mA
- MH-RD: ~20mA (módulo)

Total <100mA es seguro desde pines 3.3V de RPi
```

## Testing

### Test de Conectividad

```rust
#[test]
#[ignore]  // Solo con hardware conectado
fn test_all_sensors() {
    let mut dht11 = Dht11Sensor::new(17).expect("DHT11 no disponible");
    let mut dht22 = Dht22Sensor::new(4).expect("DHT22 no disponible");
    let mut ds = Ds18b20Sensor::new("28-abc").expect("DS18B20 no disponible");
    let mut rain = MhRdSensor::new(27, true).expect("MH-RD no disponible");
    
    assert!(dht11.read().is_ok());
    assert!(dht22.read().is_ok());
    assert!(ds.read().is_ok());
    assert!(rain.read().is_ok());
}
```

## Troubleshooting

### Error Común: `IoError`

```rust
//   Error
let sensor = Dht22Sensor::new(99)?;  // Pin inválido

//   Verificar permisos
// sudo usermod -a -G gpio $USER

//   Verificar que el pin existe
// gpio readall
```

### Error Común: `Timeout`

```rust
// Agregar delay entre lecturas
loop {
    sensor.read()?;
    std::thread::sleep(Duration::from_secs(3));  // DHT necesita 2-3s
}
```

## Recursos

### Documentación Detallada
- [DHT11](../sensors/dht11.md)
- [DHT22](../sensors/dht22.md)
- [DS18B20](../sensors/ds18b20.md)
- [MH-RD](../sensors/mhrd.md)

### Guías
- [Crear Sensores Personalizados](../sensors/custom_sensors.md)

### API
- [Trait Sensor](./traits_sensor.md)
- [SensorOutput y SensorError](./core_types.md)

