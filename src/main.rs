use std::thread;
use std::time::Duration;

use lince::devices::sensors::dht11::Dht11Sensor;
use lince::devices::sensors::ds18b20::Ds18b20Sensor;
use lince::core::traits::sensor::Sensor;
use lince::core::traits::storage::Storage;
use lince::core::traits::communicator::{Communicator, CommunicatorError};
use lince::parser::SensorParser;
use lince::network::smart_campus::{SmartCampusFormatter, SmartCampusHeader};
use lince::network::mqtt::MqttCommunicator;
use lince::storage::sqlite::SqliteStorage;

pub mod comunicacion {
    pub const BROKER:     &str = "159.65.231.88";
    pub const PUERTO:     u16  = 1883;
    pub const TOPIC:      &str = "device/messages";
    pub const CLIENT_ID:  &str = "lince";
}

pub mod sensores {
    pub const DHT11_PIN:  u8  = 17;
    pub const DS18B20_ID: &str = "28-00000b0e60f1";
}

fn main() {
    // --- Sensores ---
    let mut dht11 = Dht11Sensor::new(sensores::DHT11_PIN)
        .expect("No se pudo inicializar DHT11");

    let mut ds18b20 = Ds18b20Sensor::new(sensores::DS18B20_ID)
        .expect("No se pudo inicializar DS18B20");

    // --- Formatter ---
    let formatter = SmartCampusFormatter::new(
        SmartCampusHeader::new(comunicacion::CLIENT_ID, comunicacion::TOPIC)
    );

    // --- Comunicación ---
    let mut mqtt = MqttCommunicator::new(
        comunicacion::CLIENT_ID,
        comunicacion::BROKER,
        comunicacion::PUERTO,
        comunicacion::TOPIC,
    ).expect("No se pudo conectar al broker MQTT");

    // --- Almacenamiento ---
    let mut storage = SqliteStorage::new("pendientes.db")
        .expect("No se pudo abrir la base de datos");

    println!("Iniciando lectura de sensores...");
    println!("Broker: {}:{}", comunicacion::BROKER, comunicacion::PUERTO);
    println!("Topic:  {}", comunicacion::TOPIC);
    println!("Pendientes al arrancar: {}\n", storage.pending_count());

    // Espera de estabilización
    thread::sleep(Duration::from_secs(2));

    loop {
        // -------------------------------------------------------
        // DHT11 — temperatura y humedad
        // -------------------------------------------------------
        match dht11.read() {
            Ok(output) => {
                match SensorParser::dht(&output) {
                    Ok(valores) => {
                        let temp = valores["temperatura"];
                        let hum  = valores["humedad"];
                        println!("[DHT11] Temp: {}°C  Hum: {}%", temp, hum);

                        let json = formatter.desde_mapa(&valores);
                        storage.save(output).unwrap();

                        match mqtt.send(json.as_bytes()) {
                            Ok(()) => println!("[DHT11] Enviado: {}", json),
                            Err(CommunicatorError::Disconnected) => {
                                eprintln!("[DHT11] Sin conexión. Reintentando pendientes...");
                                let n = storage.flush_pending(&mut mqtt);
                                println!("[DHT11] Reenviados: {}", n);
                            }
                            Err(CommunicatorError::SendError) => {
                                eprintln!("[DHT11] Error de protocolo, descartado");
                            }
                        }
                    }
                    Err(e) => eprintln!("[DHT11] Error al parsear: {:?}", e),
                }
            }
            Err(e) => eprintln!("[DHT11] Error al leer: {:?}", e),
        }

        // DHT11 necesita mínimo 1 s entre lecturas
        thread::sleep(Duration::from_secs(2));

        // -------------------------------------------------------
        // DS18B20 — temperatura
        // -------------------------------------------------------
        match ds18b20.read() {
            Ok(output) => {
                match SensorParser::ds18b20(&output) {
                    Ok(temp) => {
                        println!("[DS18B20] Temp: {}°C", temp);

                        let json = formatter.desde_valor("temperatura_ds18b20", temp);
                        storage.save(output).unwrap();

                        match mqtt.send(json.as_bytes()) {
                            Ok(()) => println!("[DS18B20] Enviado: {}", json),
                            Err(CommunicatorError::Disconnected) => {
                                eprintln!("[DS18B20] Sin conexión. Reintentando pendientes...");
                                let n = storage.flush_pending(&mut mqtt);
                                println!("[DS18B20] Reenviados: {}", n);
                            }
                            Err(CommunicatorError::SendError) => {
                                eprintln!("[DS18B20] Error de protocolo, descartado");
                            }
                        }
                    }
                    Err(e) => eprintln!("[DS18B20] Error al parsear: {:?}", e),
                }
            }
            Err(e) => eprintln!("[DS18B20] Error al leer: {:?}", e),
        }

        println!("Pendientes: {}  |  Esperando 10s...\n", storage.pending_count());
        thread::sleep(Duration::from_secs(10));
    }
}