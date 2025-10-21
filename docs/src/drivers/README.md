# Módulo de Drivers (`drivers`)

El módulo `drivers` de **Lince** se encarga de la **interacción directa con el hardware**, proporcionando una capa de acceso segura y uniforme a los pines GPIO y otros periféricos del dispositivo gateway (por ejemplo, una Raspberry Pi).

Su objetivo es **abstraer el acceso físico al hardware** para que los sensores y componentes superiores del framework no dependan de detalles específicos del sistema operativo o la librería subyacente.



## Conceptos clave

- **`GpioDriver`**  
  Es el driver principal para controlar pines digitales.  
  Permite configurar un pin como entrada o salida, leer su estado y modificarlo, garantizando seguridad en el acceso concurrente.

  ```rust
  use lince::drivers::gpio::GpioDriver;
  use lince::core::SensorError;

  fn main() -> Result<(), SensorError> {
      let mut pin = GpioDriver::new(17)?;  // GPIO 17
      pin.set_high()?;                     // Activa el pin
      let nivel = pin.read_bool();         // Lee su estado
      println!("Nivel del pin: {}", nivel);
      Ok(())
  }
  ```

- **Seguridad y aislamiento**
  - Cada pin pertenece a un único driver mientras esté en uso.  
  - Al liberar la instancia, el pin queda disponible para otros procesos.  
  - Se usan las bibliotecas del sistema (`rppal` en Raspberry Pi) de forma encapsulada.

---

##  Funcionalidades principales

| Función | Descripción |
|----------|--------------|
| `new(pin: u8)` | Inicializa el pin GPIO indicado. |
| `set_high()` / `set_low()` | Cambia el estado lógico de un pin configurado como salida. |
| `read_level()` / `read_bool()` | Lee el valor actual del pin. |
| `set_mode_input()` / `set_mode_output()` | Configura el modo del pin. |

---
