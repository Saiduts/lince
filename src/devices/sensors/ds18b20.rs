use crate::core::traits::sensor::Sensor;
use std::fs;
use crate::core::{SensorError, SensorOutput};

/// Implementación del **sensor DS18B20** para el framework IoT.
///
/// Este sensor de temperatura utiliza el bus **OneWire**, y en sistemas Linux
/// su lectura se expone como un archivo de texto en:
/// `/sys/bus/w1/devices/{id}/w1_slave`.
///
/// El kernel gestiona la comunicación con el dispositivo, por lo que el framework
/// solo necesita leer y procesar el contenido del archivo correspondiente.
///
/// # Características
/// - Rango: −55 °C a +125 °C  
/// - Resolución: 12 bits (0.0625 °C)  
/// - Protocolo: OneWire  
///
/// # Ejemplo
/// ```
/// use iot_framework::devices::sensors::ds18b20::Ds18b20Sensor;
///
/// let mut sensor = Ds18b20Sensor::new("28-00000abcdef").unwrap();
/// let lectura = sensor.read().unwrap();
/// println!("{:?}", lectura);
/// ```
pub struct Ds18b20Sensor {
    /// Ruta completa del archivo `w1_slave` donde el kernel expone los datos del sensor.
    ///
    /// Ejemplo: `/sys/bus/w1/devices/28-00000abcdef/w1_slave`
    device_path: String,
}

impl Ds18b20Sensor {
    /// Crea una nueva instancia del sensor DS18B20 a partir del identificador del dispositivo.
    ///
    /// # Parámetros
    /// - `device_id`: ID único asignado por el bus OneWire (ej. `"28-00000abcdef"`).
    ///
    /// # Retorna
    /// - `Ok(Self)` si la ruta se construyó correctamente.
    /// - `Err(SensorError)` solo en caso de error de inicialización futura.
    pub fn new(device_id: &str) -> Result<Self, SensorError> {
        Ok(Self {
            device_path: format!("/sys/bus/w1/devices/{}/w1_slave", device_id),
        })
    }

    /// Lee directamente el archivo `w1_slave` y obtiene los datos crudos del sensor.
    ///
    /// # Retorna
    /// - `Ok(String)` con el contenido del archivo si la lectura fue exitosa.  
    /// - `Err(SensorError::IoError)` si el archivo no puede leerse.
    fn read_temp_raw(&self) -> Result<String, SensorError> {
        fs::read_to_string(&self.device_path)
            .map_err(|_| SensorError::IoError)
    }
}

impl Sensor for Ds18b20Sensor {
    /// Tipo de salida: [`SensorOutput`] (valor textual formateado).
    type Output = SensorOutput;

    /// Lee la temperatura actual desde el DS18B20.
    ///
    /// # Flujo
    /// 1. Llama a [`read_temp_raw`] para leer el archivo del kernel.  
    /// 2. Busca el marcador `"t="` en el texto (donde se encuentra la lectura).  
    /// 3. Convierte el valor crudo a grados Celsius dividiendo entre 1000.  
    /// 4. Devuelve el resultado como texto formateado (`"xx.xx °C"`).  
    ///
    /// # Retorna
    /// - `Ok(SensorOutput::Text(...))` si la lectura fue válida.  
    /// - `Err(SensorError::InvalidData)` si el formato del archivo no es el esperado.  
    /// - `Err(SensorError::IoError)` si ocurre un problema al leer el archivo.
    fn read(&mut self) -> Result<Self::Output, SensorError> {
        let data = self.read_temp_raw()?;

        if let Some(eq_pos) = data.find("t=") {
            let temp_str = &data[eq_pos + 2..].trim();

            let temp_c = temp_str
                .parse::<f32>()
                .map_err(|_| SensorError::InvalidData)? / 1000.0;

            Ok(SensorOutput::Text(format!("{:.2} °C", temp_c)))
        } else {
            Err(SensorError::InvalidData)
        }
    }
}
