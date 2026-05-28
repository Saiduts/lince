# SensorParser

`SensorParser` extrae los valores numéricos o booleanos de un `SensorOutput` para usarlos en cálculos, umbrales o lógica de aplicación.

Es independiente del formatter: no produce JSON ni formatos de red, solo devuelve los valores en su tipo nativo.

---

## ¿Por qué existe un parser separado?

Sin el parser, el único uso posible de una lectura de sensor es enviarla. Con el parser, el dato puede usarse antes de enviarlo:

```rust
// Sin parser — solo se puede enviar el texto crudo
let output = sensor.read()?;  // "Temp: 24.3°C, Hum: 58.2%"

// Con parser — los valores son números reales
let valores = SensorParser::dht(&output)?;
let temp = valores["temperatura"];  // f32: 24.3
let hum  = valores["humedad"];      // f32: 58.2

if temp > 30.0 { /* activar ventilador */ }
let promedio = (temp + temp_anterior) / 2.0;
```

---

## Importar el Módulo

```rust
use lince::parser::SensorParser;
```

---

## Métodos por Sensor

### `SensorParser::dht()` — DHT11 / DHT22

Devuelve un `HashMap<String, f32>` con las claves `"temperatura"` y `"humedad"`.

```rust
use std::collections::HashMap;
use lince::parser::SensorParser;

let output = sensor.read()?;
let valores = SensorParser::dht(&output)?;

let temp = valores["temperatura"];  // f32
let hum  = valores["humedad"];      // f32
```

**Formato esperado del texto:**
- DHT22: `"Temp: 24.3°C, Hum: 58.2%"`
- DHT11: `"Temp: 24°C, Hum: 58%"`

**Errores:**
- `ParseError::FormatoInvalido` si el texto no tiene la estructura esperada.
- `ParseError::TipoIncompatible` si el `SensorOutput` no es de tipo `Text`.

---

### `SensorParser::ds18b20()` — DS18B20

Devuelve la temperatura como `f32`.

```rust
let output = sensor.read()?;
let temp = SensorParser::ds18b20(&output)?;  // f32
```

**Formato esperado del texto:** `"24.56 °C"`

**Errores:**
- `ParseError::FormatoInvalido` si el texto no puede convertirse a número.
- `ParseError::TipoIncompatible` si el `SensorOutput` no es de tipo `Text`.

---

### `SensorParser::mhrd()` — MH-RD

Devuelve `true` si el sensor detectó humedad, `false` si está seco.

```rust
let output = sensor.read()?;
let mojado = SensorParser::mhrd(&output)?;  // bool

if mojado {
    println!("Está lloviendo");
}
```

**Valores esperados:** `"HÚMEDO"` → `true`, `"SECO"` → `false`.

También acepta `SensorOutput::Bool` directamente.

**Errores:**
- `ParseError::FormatoInvalido` si el texto no es ni `"HÚMEDO"` ni `"SECO"`.
- `ParseError::TipoIncompatible` si el tipo no es `Text` ni `Bool`.

---

## Ejemplos de Uso

### Umbral de temperatura

```rust
let valores = SensorParser::dht(&output)?;
let temp = valores["temperatura"];

if temp > 35.0 {
    eprintln!("Temperatura crítica: {}°C", temp);
}
```

### Corrección de calibración

```rust
let temp = SensorParser::ds18b20(&output)?;
let temp_corregida = temp - 1.5;  // offset de calibración

let json = formatter.desde_valor("temperatura", temp_corregida);
```

### Promedio de lecturas

```rust
let mut acumulado = 0.0_f32;
let n = 10;

for _ in 0..n {
    let valores = SensorParser::dht(&sensor.read()?)?;
    acumulado += valores["temperatura"];
    thread::sleep(Duration::from_secs(3));
}

let promedio = acumulado / n as f32;
println!("Temperatura promedio: {:.1}°C", promedio);
```

### Valores calculados con el formatter

```rust
// El parser extrae los valores
let valores = SensorParser::dht(&output)?;
let temp = valores["temperatura"];
let hum  = valores["humedad"];

// Se pueden operar antes de formatear
let mut calculados = HashMap::new();
calculados.insert("temperatura".to_string(),  temp);
calculados.insert("humedad".to_string(),      hum);
calculados.insert("indice_calor".to_string(), calcular_indice_calor(temp, hum));

// El formatter recibe el mapa ya listo
let json = formatter.desde_mapa(&calculados);
```

---

## Relación con SmartCampusFormatter

El parser y el formatter son independientes entre sí. El flujo típico es:

```
sensor.read()          →  SensorOutput
SensorParser::dht()    →  HashMap<String, f32>   ← aquí se pueden hacer cálculos
formatter.desde_mapa() →  JSON Smart Campus
mqtt.send()
```

Si no se necesitan cálculos, se puede pasar el mapa directamente del parser al formatter sin modificarlo.

---

## Referencia de Interfaces

### `SensorParser`

```rust
pub fn dht(output: &SensorOutput) -> Result<HashMap<String, f32>, ParseError>

pub fn ds18b20(output: &SensorOutput) -> Result<f32, ParseError>

pub fn mhrd(output: &SensorOutput) -> Result<bool, ParseError>
```

### `ParseError`

```rust
pub enum ParseError {
    FormatoInvalido(String),  // el texto no tiene la estructura esperada
    TipoIncompatible,         // el SensorOutput no es del tipo correcto
}
```

---

## Ver También

- [SmartCampusFormatter](./communication/smart_campus.md)
- [SensorOutput y tipos](./reference/core_types.md)
- [DHT22](./sensors/dht22.md)
- [DS18B20](./sensors/ds18b20.md)
- [MH-RD](./sensors/mhrd.md)