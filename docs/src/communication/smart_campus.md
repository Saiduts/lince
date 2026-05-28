# SmartCampusFormatter

`SmartCampusFormatter` convierte valores numéricos o booleanos al JSON estructurado requerido por el protocolo Smart Campus.

Recibe valores ya extraídos, no `SensorOutput` crudo. Esto permite usarlo con valores que vienen del parser, de cálculos propios o de cualquier otra fuente.

```json
{
  "header": { "deviceId": "raspberry-lince", "topic": "device/messages" },
  "metrics": [
    { "measurement": "temperatura", "value": 24.50 },
    { "measurement": "humedad",     "value": 60.10 }
  ]
}
```

---

## Importar el Módulo

```rust
use lince::network::smart_campus::{SmartCampusFormatter, SmartCampusHeader, Metric};
```

---

## Crear una Instancia

```rust
let formatter = SmartCampusFormatter::new(
    SmartCampusHeader::new("raspberry-lince", "device/messages")
);
```

---

## Métodos de Formateo

### `desde_mapa()` — para DHT11 / DHT22

Recibe el `HashMap<String, f32>` que devuelve `SensorParser::dht()`.

```rust
use lince::parser::SensorParser;

let output = sensor.read()?;
let valores = SensorParser::dht(&output)?;

let json = formatter.desde_mapa(&valores);
// {"header":{...},"metrics":[{"measurement":"temperatura","value":24.50},{"measurement":"humedad","value":60.10}]}
```

También funciona con mapas de valores calculados:

```rust
let mut calculados = HashMap::new();
calculados.insert("temp_promedio".to_string(), 24.5_f32);
calculados.insert("temp_maxima".to_string(),  27.3_f32);

let json = formatter.desde_mapa(&calculados);
```

---

### `desde_valor()` — para DS18B20 o cualquier valor escalar

```rust
use lince::parser::SensorParser;

let output = sensor.read()?;
let temp = SensorParser::ds18b20(&output)?;

let json = formatter.desde_valor("temperatura_ds18b20", temp);
```

Con corrección de calibración:

```rust
let temp_corregida = SensorParser::ds18b20(&output)? - 1.5;
let json = formatter.desde_valor("temperatura_corregida", temp_corregida);
```

---

### `desde_bool()` — para MH-RD

Convierte `true` → `1.0`, `false` → `0.0`.

```rust
use lince::parser::SensorParser;

let output = sensor.read()?;
let mojado = SensorParser::mhrd(&output)?;

let json = formatter.desde_bool("lluvia", mojado);
// {"header":{...},"metrics":[{"measurement":"lluvia","value":1.00}]}
```

---

### `format()` — control total

Si necesitas construir las métricas manualmente:

```rust
let metrics = vec![
    Metric::new("temperatura", 24.5),
    Metric::new("humedad",     60.0),
    Metric::new("indice_calor", 26.1),
];
let json = formatter.format(&metrics);
```

---

## Flujo Completo con Parser

```rust
use lince::parser::SensorParser;
use lince::network::smart_campus::{SmartCampusFormatter, SmartCampusHeader};

// 1. Sensor lee
let output = sensor.read()?;

// 2. Parser extrae valores para cálculos
let valores = SensorParser::dht(&output)?;
let temp = valores["temperatura"];
let hum  = valores["humedad"];

// 3. Lógica de aplicación (opcional)
if temp > 30.0 {
    println!("Temperatura alta");
}

// 4. Formatter convierte a JSON para enviar
let json = formatter.desde_mapa(&valores);
mqtt.send(json.as_bytes())?;
```

---

## Referencia de Interfaces

### `SmartCampusHeader`

```rust
pub fn new(device_id: &str, topic: &str) -> Self
```

### `Metric`

```rust
pub fn new(measurement: &str, value: f64) -> Self
```

### `SmartCampusFormatter`

```rust
pub fn new(header: SmartCampusHeader) -> Self

pub fn format(&self, metrics: &[Metric]) -> String

pub fn desde_mapa(&self, valores: &HashMap<String, f32>) -> String

pub fn desde_valor(&self, nombre: &str, valor: f32) -> String

pub fn desde_bool(&self, nombre: &str, valor: bool) -> String
```

---

## Ver También

- [SensorParser](../parser.md)
- [MqttCommunicator](./mqtt.md)
- [RetryLayer en SqliteStorage](../storage/sqlite_storage.md)
- [Trait Communicator](../reference/traits_communicator.md)