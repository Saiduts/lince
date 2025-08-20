use std::{fs}; // Utilidades para leer archivos y manejar rutas
use crate::config::config::Config; // Estructura Config definida en el módulo config

/// Carga la configuración del framework desde un archivo TOML.
/// 
/// # Parámetros
/// - `path`: Ruta del archivo de configuración (`.toml`).
/// 
/// # Retorno
/// Devuelve un `Config` con todos los parámetros del sistema, o un error si:
/// - El archivo no existe.
/// - No se puede leer.
/// - El contenido no es un TOML válido.
/// 
/// # Ejemplo
/// ```rust
/// let config = load_config("config.toml").unwrap();
/// println!("Dispositivo: {}", config.device.name);
/// ```
pub fn load_config(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    // Lee todo el contenido del archivo como un String
    let config_content = fs::read_to_string(path)?;

    // Convierte el String desde formato TOML a la estructura Config
    // Esto utiliza Serde + toml para deserializar automáticamente
    let config: Config = toml::from_str(&config_content)?;

    // Devuelve la configuración cargada
    Ok(config)
}
