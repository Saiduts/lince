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
/// use lince::devices::sensors::ds18b20::Ds18b20Sensor;
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
        let device_path = format!("/sys/bus/w1/devices/{}/w1_slave", device_id);
        
        // Validar que el archivo existe
        if !std::path::Path::new(&device_path).exists() {
            eprintln!(
                "[DS18B20] Dispositivo no encontrado: {}. \
                Verifica que OneWire está habilitado y el sensor conectado.",
                device_id
            );
            return Err(SensorError::InitializationError);
        }
        
        // Intentar lectura de prueba
        let test_read = fs::read_to_string(&device_path)
            .map_err(|e| {
                eprintln!("[DS18B20] Error al leer dispositivo: {}", e);
                SensorError::IoError
            })?;
            
        // Validar formato básico
        if !test_read.contains("t=") {
            eprintln!("[DS18B20] Formato de datos inválido del sensor");
            return Err(SensorError::InvalidData);
        }
        
        println!("[DS18B20] Sensor {} inicializado correctamente", device_id);
        
        Ok(Self { device_path })
    }

    /// Lee directamente el archivo `w1_slave` y obtiene los datos crudos del sensor.
    ///
    /// # Retorna
    /// - `Ok(String)` con el contenido del archivo si la lectura fue exitosa.  
    /// - `Err(SensorError::IoError)` si el archivo no puede leerse.
    fn read_temp_raw(&self) -> Result<String, SensorError> {
        fs::read_to_string(&self.device_path)
            .map_err(|e| {
                eprintln!("[DS18B20] Error de lectura: {}", e);
                SensorError::IoError
            })
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
        // Verificar CRC (checksum del sensor)
        if !data.contains("YES") {
            eprintln!("[DS18B20] CRC inválido - datos corruptos");
            return Err(SensorError::InvalidData);
        }

        if let Some(eq_pos) = data.find("t=") {
            let temp_str = data[eq_pos + 2..].trim();
            
            let temp_raw = temp_str
                .parse::<i32>()
                .map_err(|e| {
                    eprintln!("[DS18B20] Error al parsear temperatura: {}", e);
                    SensorError::InvalidData
                })?;
            
            let temp_c = temp_raw as f32 / 1000.0;
            
            // Validar rango físico del DS18B20 (-55°C a 125°C)
            if temp_c < -55.0 || temp_c > 125.0 {
                eprintln!(
                    "[DS18B20] Temperatura fuera de rango: {:.2}°C. \
                    Sensor posiblemente desconectado o defectuoso.",
                    temp_c
                );
                return Err(SensorError::InvalidData);
            }
            
            // Detectar valores sospechosos (exactamente 0°C o 85°C suelen ser errores)
            if temp_c == 0.0 || temp_c == 85.0 {
                eprintln!(
                    "[DS18B20] ADVERTENCIA: Temperatura sospechosa {:.2}°C. \
                    Puede indicar sensor desconectado o en estado de inicialización.",
                    temp_c
                );
            }

            Ok(SensorOutput::Text(format!("{:.2} °C", temp_c)))
        } else {
            eprintln!("[DS18B20] No se encontró marcador 't=' en los datos");
            Err(SensorError::InvalidData)
        }
    }
}
