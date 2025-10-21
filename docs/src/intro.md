# IoT Framework para Rust

Bienvenido a la documentación oficial del **Framework para IoT Lince**, un sistema modular y extensible diseñado para construir soluciones IoT en dispositivos *gateway* basados en Linux, especialmente **Raspberry Pi**.

---

## ¿Qué es Lince?

**Lince** es una biblioteca escrita en **Rust** que proporciona abstracciones de alto nivel para:

- **Lectura de sensores**: temperatura, humedad, lluvia, entre otros.  
- **Almacenamiento de datos**: memoria, bases de datos o archivos.  
- **Comunicación**: MQTT, consola o HTTP.  
- **Control de hardware**: acceso a GPIO y protocolos de comunicación.

---

## Características Principales

###  Modular y Extensible
Arquitectura basada en **traits** que permite implementar fácilmente nuevos sensores, almacenamientos y comunicadores personalizados.

###  Alto Rendimiento
Desarrollado en **Rust**, garantiza seguridad de memoria, eficiencia y rendimiento nativo sin *garbage collector*.

###  Fácil de Usar
API intuitiva inspirada en frameworks como **MicroPython**, pero con la potencia, seguridad y concurrencia de Rust.

###  Listo para Producción
Incluye soporte para **MQTT**, manejo robusto de errores y una arquitectura validada en dispositivos reales como Raspberry Pi.

---

## Arquitectura del Proyecto

La estructura general del framework se organiza en módulos principales:

```text
lince/
├── core/       # Traits y tipos fundamentales  
├── devices/    # Implementaciones de sensores  
├── storage/    # Sistemas de almacenamiento  
├── network/    # Comunicadores (MQTT, Console)  
└── drivers/    # Drivers de hardware (GPIO)
```

Cada módulo cumple un rol específico, facilitando la extensión e integración de nuevos componentes.

---

## Ejemplo Rápido

```rust
use lince::devices::sensors::dht22::Dht22Sensor;
use lince::core::traits::sensor::Sensor;

fn main() {
    let mut sensor = Dht22Sensor::new(4).unwrap();
    
    match sensor.read() {
        Ok(data) => println!("Lectura: {:?}", data),
        Err(e) => eprintln!("Error: {:?}", e),
    }
}

```

Este ejemplo muestra cómo inicializar y leer un sensor DHT22 conectado al pin GPIO 4 en una Raspberry Pi.

# Plataformas Soportadas

- **Raspberry Pi** (todos los modelos con soporte GPIO)  
- **Linux ARM** (dispositivos compatibles con `rppal`)  
- **Otros sistemas Linux** con soporte de pines GPIO  

---

# Requisitos

- **Rust** 1.70 o superior  
- **Sistema operativo:** Linux  
- **Hardware:** con GPIO (para sensores físicos)  

---

# Próximos Pasos

1. Lee la [Guía de Instalación](../getting_started/installation.md)
2. Sigue la [Guía Rápida](../getting_started/quick_start.md)
3. Explora la [Referencia de Sensores](./sensors/index.html)

---

# Comunidad y Soporte

- **Repositorio:** [GitHub - Saiduts/lince](https://github.com/Saiduts/lince)  
- **Issues:** Reporta errores o solicita nuevas características  
- **Discusiones:** Comparte ideas, dudas o implementaciones  
