use std::{fs, path}; //Utilidad para leer archivos
use crate::config::config::Config; //Para leer la configuración

pub fn load_config(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let config_content = fs::read_to_string(path)?; //Leer el contenido del archivo
    let config: Config = toml::from_str(&config_content)?; //Convertir el contenido del archivo a un tipo Config
    Ok(config)
}