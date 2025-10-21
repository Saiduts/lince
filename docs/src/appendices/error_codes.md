# Códigos de Error

Referencia completa de todos los errores del framework IoT y cómo resolverlos.

## Categorías de Errores

```
lince::core
├── SensorError       - Errores de sensores
├── StorageError      - Errores de almacenamiento
└── CommunicatorError - Errores de comunicación
```

## SensorError

### `IoError`

**Código:** `SensorError::IoError`

**Descripción:** Error de entrada/salida al acceder al hardware.

**Causas comunes:**
- Sin permisos para acceder a GPIO
- Dispositivo no encontrado
- Pin GPIO no existe
- Recurso ya en uso

**Soluciones:**

```bash
# 1. Verificar permisos
groups $USER | grep gpio
# Si no aparece:
sudo usermod -a -G gpio $USER
# Cerrar sesión y volver a entrar

# 2. Verificar que el GPIO existe
gpio readall

# 3. Ejecutar con sudo (temporal)
sudo ./mi-programa

# 4. Verificar que el pin no esté en uso
lsof | grep gpio
```

---

### `Timeout`

**Código:** `SensorError::Timeout`

**Descripción:** El sensor no respondió dentro del tiempo esperado.

**Causas comunes:**
- Sensor no conectado físicamente
- Alimentación insuficiente
- Cable demasiado largo
- Sensor dañado
- Timing incorrecto

**Soluciones:**

```rust
// 1. Verificar conexiones físicas
// 2. Aumentar tiempo entre lecturas
use std::thread;
use std::time::Duration;

let mut sensor = Dht22Sensor::new(4)?;

//   Mal: muy rápido
sensor.read()?;
sensor.read()?;  // ¡Timeout!

//   Bien: esperar entre lecturas
sensor.read()?;
thread::sleep(Duration::from_secs(3));
sensor.read()?;  // OK

// 3. Implementar reintentos
fn leer_con_reintentos(sensor: &mut impl Sensor, max: u32) -> Result<SensorOutput, SensorError> {
    for intento in 1..=max {
        match sensor.read() {
            Ok(data) => return Ok(data),
            Err(SensorError::Timeout) if intento < max => {
                eprintln!("Timeout en intento {}, reintentando...", intento);
                thread::sleep(Duration::from_secs(2));
                continue;
            },
            Err(e) => return Err(e),
        }
    }
    Err(SensorError::Timeout)
}
```

**Tiempos mínimos por sensor:**

| Sensor | Tiempo mínimo entre lecturas |
|--------|------------------------------|
| DHT11 | 1 segundo |
| DHT22 | 2 segundos |
| DS18B20 | 750 ms |
| MH-RD | Instantáneo |

---

### `InvalidData`

**Código:** `SensorError::InvalidData`

**Descripción:** Los datos recibidos son inválidos o están corruptos.

**Causas comunes:**
- Checksum incorrecto
- Interferencia electromagnética
- Cable de mala calidad
- Sensor defectuoso
- Datos fuera de rango

**Rangos válidos por sensor:**

| Sensor | Rango Temperatura | Rango Humedad |
|--------|-------------------|---------------|
| DHT11 | 0-50°C | 20-90% |
| DHT22 | -40-80°C | 0-100% |
| DS18B20 | -55-125°C | N/A |

---

### `InitializationError`

**Código:** `SensorError::InitializationError`

**Descripción:** El sensor no pudo inicializarse correctamente.

**Causas comunes:**
- Parámetros incorrectos
- Hardware no soportado
- Configuración del sistema incorrecta
- Chip ID incorrecto

---

## StorageError

### `SaveError`

**Código:** `StorageError::SaveError`

**Descripción:** Error al intentar guardar datos.

**Causas comunes:**
- Disco lleno
- Sin permisos de escritura
- Base de datos bloqueada
- Conexión de red perdida (storage remoto)

**Soluciones:**

```bash
# 1. Verificar espacio en disco
df -h

# 2. Verificar permisos
ls -la datos.db
chmod 666 datos.db  # Si es necesario

# 3. Verificar inodos disponibles
df -i
```

```rust
// En código: manejo de errores
match storage.save(data) {
    Err(StorageError::SaveError) => {
        // Intentar limpiar espacio
        storage.clear()?;
        
        // Reintentar
        storage.save(data)?;
    },
    Ok(_) => println!("Guardado OK"),
}

// Implementar límite de tamaño
const MAX_ENTRIES: usize = 10_000;

if storage.list()?.len() >= MAX_ENTRIES {
    // Rotar: eliminar los más viejos
    storage.clear()?;
}

storage.save(data)?;
```

---

### `ReadError`

**Código:** `StorageError::ReadError`

**Descripción:** Error al intentar leer datos almacenados.

**Causas comunes:**
- Archivo corrupto
- Formato de datos inválido
- Conexión perdida
- Archivo no existe

**Soluciones:**

```rust
match storage.list() {
    Err(StorageError::ReadError) => {
        eprintln!("Error al leer storage");
        
        // Verificar que el archivo existe
        if !std::path::Path::new("datos.db").exists() {
            eprintln!("Archivo no existe, creando nuevo...");
            // Recrear storage
        }
        
        // Intentar reparar
        // backup_and_recreate()?;
    },
    Ok(data) => println!("Leídos {} registros", data.len()),
}

---

### `ClearError`

**Código:** `StorageError::ClearError`

**Descripción:** Error al intentar limpiar el almacenamiento.

**Causas comunes:**
- Sin permisos
- Sistema de archivos de solo lectura
- Base de datos bloqueada

**Soluciones:**

```bash
# Verificar permisos
ls -la datos.db
sudo chmod 666 datos.db

# Verificar sistema de archivos
mount | grep " / "
```

```rust
match storage.clear() {
    Err(StorageError::ClearError) => {
        eprintln!("No se pudo limpiar storage");
        
        // Alternativa: eliminar archivo directamente
        std::fs::remove_file("datos.db").ok();
        
        // Recrear storage
        storage = MemoryStorage::new();
    },
    Ok(_) => println!("Storage limpiado"),
}
```

---

## CommunicatorError

### `SendError`

**Código:** `CommunicatorError::SendError`

**Descripción:** Error al intentar enviar datos.

**Causas comunes:**
- Red no disponible
- Broker/servidor no accesible
- Timeout de conexión
- Credenciales incorrectas
- Client ID duplicado (MQTT)

**Soluciones:**

```bash
# 1. Verificar conectividad de red
ping -c 3 localhost
ping -c 3 192.168.1.100

# 2. Verificar que el servicio está corriendo
sudo systemctl status mosquitto

# 3. Verificar puertos
netstat -tuln | grep 1883

# 4. Probar conexión manualmente
mosquitto_pub -h localhost -t test -m "hola"
```


## Tabla de Referencia Rápida

| Error | Código | Verificar | Solución Rápida |
|-------|--------|-----------|-----------------|
| **IoError** | `SensorError::IoError` | Permisos GPIO | `sudo usermod -a -G gpio $USER` |
| **Timeout** | `SensorError::Timeout` | Conexiones físicas | Esperar 2-3s entre lecturas |
| **InvalidData** | `SensorError::InvalidData` | Cables, interferencias | Usar cables más cortos |
| **InitializationError** | `SensorError::InitializationError` | Pin válido, hardware | Verificar pin 0-27 |
| **SaveError** | `StorageError::SaveError` | Espacio en disco | `df -h`, limpiar datos |
| **ReadError** | `StorageError::ReadError` | Archivo existe | Recrear storage |
| **ClearError** | `StorageError::ClearError` | Permisos | `chmod 666 archivo` |
| **SendError** | `CommunicatorError::SendError` | Red, broker | Verificar conectividad |

## Debugging por Error

### Proceso de Debugging General

```rust
use log::{error, warn, info, debug};

fn procesar_con_debug() {
    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or("debug")
    ).init();
    
    info!("Iniciando aplicación");
    
    match inicializar_sensor() {
        Ok(sensor) => {
            info!("Sensor inicializado correctamente");
            debug!("Sensor: {:?}", sensor);
        },
        Err(e) => {
            error!("Error al inicializar: {:?}", e);
            debug!("Backtrace: {}", std::backtrace::Backtrace::capture());
            
            // Información de diagnóstico
            diagnosticar_error(&e);
            
            std::process::exit(1);
        }
    }
}

fn diagnosticar_error(error: &SensorError) {
    match error {
        SensorError::IoError => {
            warn!("Diagnóstico IoError:");
            warn!("  1. Verificar grupos: {}", std::process::Command::new("groups").output().unwrap());
            warn!("  2. Verificar GPIO: gpio readall");
        },
        SensorError::Timeout => {
            warn!("Diagnóstico Timeout:");
            warn!("  1. Verificar conexiones físicas");
            warn!("  2. Verificar alimentación del sensor");
        },
        _ => {}
    }
}
```

## Herramientas de Diagnóstico

```bash
# Script de diagnóstico completo
#!/bin/bash

echo "=== Diagnóstico IoT Framework ==="

echo "1. Permisos GPIO:"
groups $USER | grep -q gpio && echo "  Usuario en grupo gpio" || echo "  Usuario NO en grupo gpio"

echo "2. GPIO disponibles:"
gpio readall 2>/dev/null && echo "  GPIO accesibles" || echo "  GPIO no accesibles"

echo "3. I2C:"
i2cdetect -y 1 2>/dev/null && echo "  I2C funcional" || echo "  I2C no disponible"

echo "4. OneWire:"
ls /sys/bus/w1/devices/ 2>/dev/null && echo "  OneWire activo" || echo "  OneWire no configurado"

echo "5. Espacio en disco:"
df -h | grep "/$"

echo "=== Fin del diagnóstico ==="
```

## Ver También

- [Glosario](./glossary.md)
- [Hardware Compatible](./hardware.md)
- [Guia de instalación](../getting_started/installation.md)