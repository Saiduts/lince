# Códigos de Error

Referencia completa de todos los errores del framework IoT y cómo resolverlos.

## Categorías de Errores

```
lince::core
├── SensorError        — errores de sensores
├── StorageError       — errores de almacenamiento
└── CommunicatorError  — errores de comunicación

lince::parser
└── ParseError         — errores al extraer valores de SensorOutput
```

---

## SensorError

### `IoError`

**Cuándo ocurre:** error de entrada/salida al acceder al hardware.

**Causas comunes:** sin permisos GPIO, pin no existe, recurso en uso.

```bash
sudo usermod -a -G gpio $USER
# Cerrar sesión y volver a entrar
```

---

### `Timeout`

**Cuándo ocurre:** el sensor no respondió en el tiempo esperado.

**Causas comunes:** sensor desconectado, alimentación insuficiente, lecturas demasiado frecuentes.

**Tiempos mínimos entre lecturas:**

| Sensor | Mínimo |
|--------|--------|
| DHT11 | 1 s |
| DHT22 | 2 s |
| DS18B20 | 750 ms |
| MH-RD | Instantáneo |

---

### `InvalidData`

**Cuándo ocurre:** datos recibidos corruptos o fuera de rango físico.

**Causas comunes:** checksum incorrecto, interferencia electromagnética, sensor defectuoso.

---

### `InitializationError`

**Cuándo ocurre:** el sensor no pudo inicializarse.

**Causas comunes:** parámetros incorrectos, dispositivo no encontrado, archivo `w1_slave` inexistente.

---

## StorageError

### `SaveError`

**Cuándo ocurre:** fallo al guardar datos.

**Causas comunes:** disco lleno, sin permisos de escritura, ruta vacía.

```bash
df -h          # verificar espacio
ls -la datos.db  # verificar permisos
```

---

### `ReadError`

**Cuándo ocurre:** fallo al leer datos almacenados.

**Causas comunes:** archivo corrupto, tabla eliminada manualmente.

---

### `ClearError`

**Cuándo ocurre:** fallo al limpiar el almacenamiento.

**Causas comunes:** sin permisos, base de datos bloqueada.

---

## CommunicatorError

### `Disconnected`

**Cuándo ocurre:** el broker MQTT no está alcanzable o la conexión fue interrumpida.

**¿Se reintenta?** Sí. Usar `SqliteStorage::flush_pending()` para reenviar los datos pendientes cuando la conexión vuelva.

```rust
match mqtt.send(json.as_bytes()) {
    Err(CommunicatorError::Disconnected) => {
        storage.flush_pending(&mut mqtt);
    }
    _ => {}
}
```

**Diagnóstico:**
```bash
sudo systemctl status mosquitto
mosquitto_pub -h localhost -t test -m "ping"
```

---

### `SendError`

**Cuándo ocurre:** error de protocolo no relacionado con la conectividad.

**¿Se reintenta?** No. Indica un problema en los parámetros (payload vacío, topic inválido, client_id vacío). Corregir el código, no reintentar.

**Causas comunes:**

| Causa | Solución |
|-------|----------|
| `client_id` vacío | Proporcionar un ID no vacío |
| `topic` vacío | Proporcionar un topic válido |
| `topic` con `#` o `+` | Los wildcards no se pueden usar al publicar |
| `payload` vacío | Verificar que el dato tiene contenido |
| `port = 0` | Usar un puerto válido (ej: 1883) |

---

## ParseError

### `FormatoInvalido(String)`

**Cuándo ocurre:** el texto del `SensorOutput` no tiene la estructura esperada para el sensor indicado.

**Causas comunes:** llamar al método equivocado para el sensor, formato de texto modificado.

```rust
// Error: llamar ds18b20() con una salida de DHT
let output = dht22.read()?;  // "Temp: 24.3°C, Hum: 58.2%"
SensorParser::ds18b20(&output);  // FormatoInvalido

// Correcto
SensorParser::dht(&output);
```

El mensaje de error incluye el texto que se intentó parsear, lo que facilita el diagnóstico.

---

### `TipoIncompatible`

**Cuándo ocurre:** el tipo de la variante `SensorOutput` no es compatible con el método llamado.

**Causas comunes:** pasar un `SensorOutput::Float` a `SensorParser::dht()`, que espera `SensorOutput::Text`.

```rust
// Error
let output = SensorOutput::Float(24.5);
SensorParser::dht(&output);  // TipoIncompatible

// Correcto: dht() espera SensorOutput::Text
let output = dht22.read()?;
SensorParser::dht(&output);
```

---

## Tabla de Referencia Rápida

| Error | Módulo | ¿Se reintenta? | Acción |
|-------|--------|---------------|--------|
| `IoError` | Sensor | No | Verificar permisos GPIO |
| `Timeout` | Sensor | Sí | Esperar entre lecturas |
| `InvalidData` | Sensor | Sí | Verificar conexiones físicas |
| `InitializationError` | Sensor | No | Verificar parámetros y hardware |
| `SaveError` | Storage | No | Verificar disco y permisos |
| `ReadError` | Storage | No | Verificar integridad del archivo |
| `ClearError` | Storage | No | Verificar permisos |
| `Disconnected` | Communicator | Sí | `flush_pending()` en SqliteStorage |
| `SendError` | Communicator | No | Corregir parámetros |
| `FormatoInvalido` | Parser | No | Usar el método correcto para el sensor |
| `TipoIncompatible` | Parser | No | Verificar tipo de SensorOutput |

---

## Ver También

- [Glosario](./glossary.md)
- [Hardware Compatible](./hardware.md)
- [Parser](../parser.md)
- [SqliteStorage — flush_pending](../storage/sqlite_storage.md)