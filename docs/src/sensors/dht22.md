# DHT22 - Sensor de Temperatura y Humedad

El DHT22 es un sensor digital de temperatura y humedad de alta precisión, ideal para aplicaciones meteorológicas y de monitoreo ambiental.

## Características

| **Parámetro**               | **Valor**                                                                 |
|-----------------------------|---------------------------------------------------------------------------|
| **Modelo**                  | DHT22                                                                    |
| **Alimentación**            | 3.3–6 V DC                                                               |
| **Señal de salida**         | Digital mediante bus único                                               |
| **Elemento sensor**         | Condensador de polímero                                                  |
| **Rango de operación**      | Humedad: 0–100 % RH<br>Temperatura: −40 °C a 80 °C                      |
| **Precisión**               | Humedad: ±2 % RH (máx. ±5 % RH)<br>Temperatura: < ±0.5 °C               |
| **Resolución / Sensibilidad** | Humedad: 0.1 % RH<br>Temperatura: 0.1 °C                                |
| **Repetibilidad**           | Humedad: ±1 % RH<br>Temperatura: ±0.2 °C                                |
| **Histéresis de humedad**   | ±0.3 % RH                                                                |
| **Estabilidad a largo plazo** | ±0.5 % RH/año                                                          |
| **Período de medición**     | Promedio: 2 s                                                            |
| **Intercambiabilidad**      | Totalmente intercambiable                                                |
| **Dimensiones**             | Tamaño pequeño: 14 × 18 × 5.5 mm<br>Tamaño grande: 22 × 28 × 5 mm        |


## Conexión de Hardware

## Esquema de Pines

![Esquema de pines DHT22](../images/DHT22-Sensor-Pinout.png)

**Nota**: Se recomienda una resistencia pull-up de 10kΩ entre VCC y DATA.

## Uso Básico

### Importar el Módulo

```rust
use lince::devices::sensors::dht22::Dht22Sensor;
use lince::core::traits::sensor::Sensor;
use lince::core::{SensorOutput, SensorError};
```

### Crear una Instancia

```rust
// Crear sensor en GPIO 4
let mut sensor = Dht22Sensor::new(4)?;
```

### Leer Datos

```rust
match sensor.read() {
    Ok(SensorOutput::Text(data)) => {
        println!("Lectura: {}", data);
        // Salida: "Temp: 24.3°C, Hum: 58.2%"
    },
    Err(e) => eprintln!("Error: {:?}", e),
}
```

## Ejemplos

### Lectura Simple

```rust
use lince::devices::sensors::dht22::Dht22Sensor;
use lince::core::traits::sensor::Sensor;

fn main() {
    let mut sensor = Dht22Sensor::new(4)
        .expect("Error al inicializar DHT22");
    
    loop {
        match sensor.read() {
            Ok(data) => println!(" {:?}", data),
            Err(e) => eprintln!(" {:?}", e),
        }
        
        std::thread::sleep(std::time::Duration::from_secs(3));
    }
}
```


## API Reference

### `Dht22Sensor`

#### Constructor

```rust
pub fn new(pin: u8) -> Result<Self, SensorError>
```

Crea una nueva instancia del sensor DHT22.

**Parámetros:**
- `pin`: Número del pin GPIO (numeración BCM)

**Retorna:**
- `Ok(Dht22Sensor)`: Sensor inicializado correctamente
- `Err(SensorError)`: Error en la inicialización

**Errores posibles:**
- `SensorError::IoError`: No se pudo acceder al GPIO

#### Método `read()`

```rust
fn read(&mut self) -> Result<SensorOutput, SensorError>
```

Lee temperatura y humedad del sensor.

**Retorna:**
- `Ok(SensorOutput::Text)`: Lectura formateada como texto
- `Err(SensorError)`: Error durante la lectura

**Errores posibles:**
- `SensorError::Timeout`: El sensor no respondió a tiempo
- `SensorError::InvalidData`: Checksum inválido o datos corruptos

**Formato de salida:**
```
"Temp: 24.3°C, Hum: 58.2%"
```


## Notas Técnicas

### Protocolo de Comunicación

El DHT22 usa un protocolo propietario de 1-Wire:
1. MCU envía señal de inicio (LOW por 1-10ms)
2. Sensor responde con secuencia de sincronización
3. Sensor envía 40 bits de datos (5 bytes)
4. Formato: [HUM_H][HUM_L][TEMP_H][TEMP_L][CHECKSUM]

### Formato de Datos

DHT22: Usa 5 bytes (40 bits)

Humedad: [02] [88] → (0x0288 / 10) = 65.2%
Temperatura: [00] [FA] → (0x00FA / 10) = 25.0°C
Checksum: [82] → (2 + 136 + 0 + 250) & 0xFF = 82


### Limitaciones

- **Frecuencia máxima**: 0.5 Hz (una lectura cada 2 segundos)
- **Tiempo de respuesta**: ~2 segundos
- **Sensibilidad a interferencias**: Media (usar cables apantallados si es necesario)
- **Dependencia de calibración interna**: No puede recalibrarse manualmente


## Recursos Adicionales

- [Datasheet oficial DHT22](https://www.sparkfun.com/datasheets/Sensors/Temperature/DHT22.pdf)


## Ver También

- [DHT11 Reference](./dht11.md)
- [DS18B20 Reference](./ds18b20.md)