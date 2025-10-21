use std::thread;
use std::time::Duration;

use lince::devices::sensors::dht11::Dht11Sensor;
use lince::devices::sensors::dht22::Dht22Sensor;
use lince::storage::memory::MemoryStorage;
use lince::core::traits::sensor::Sensor;
use lince::core::traits::storage::Storage;
use lince::network::mqtt::MqttCommunicator;
use lince::core::traits::communicator::Communicator;
use lince::core::SensorOutput;

fn main() {
    println!("Iniciando lectura de sensores DHT...");

    // Inicializar sensores
    let mut dht22 = match Dht22Sensor::new(23) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error inicializando DHT22: {:?}", e);
            return;
        }
    };

    let mut dht11 = match Dht11Sensor::new(17) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error inicializando DHT11: {:?}", e);
            return;
        }
    };

    // Almacenamiento en memoria
    let mut storage = MemoryStorage::new();

    // Comunicador MQTT
    let mut mqtt = match MqttCommunicator::new("test_client", "localhost", 1883, "test/topic") {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error creando comunicador MQTT: {:?}", e);
            return;
        }
    };

    println!("Esperando 3s para estabilización inicial...");
    thread::sleep(Duration::from_secs(3));

    for i in 1..=3 {
        println!("Iteración {}/3", i);

        // --- Leer DHT22 con reintentos ---
        let mut attempts = 0;
        let data_dht22 = loop {
            match dht22.read() {
                Ok(d) => break d,
                Err(e) if attempts < 2 => {
                    attempts += 1;
                    eprintln!("Error DHT22 (intento {}): {:?}, reintentando...", attempts, e);
                    thread::sleep(Duration::from_millis(500));
                }
                Err(e) => {
                    eprintln!("Error DHT22 definitivo: {:?}", e);
                    break SensorOutput::Text("Error DHT22".to_string());
                }
            }
        };

        println!("DHT22 => {:?}", data_dht22);
        storage.save(data_dht22.clone()).unwrap();

        if let SensorOutput::Text(ref s) = data_dht22 {
            if let Err(e) = mqtt.send(s.as_bytes()) {
                eprintln!("Error enviando DHT22 via MQTT: {:?}", e);
            }
        }

        thread::sleep(Duration::from_secs(3));

        // --- Leer DHT11 con reintentos ---
        attempts = 0;
        let data_dht11 = loop {
            match dht11.read() {
                Ok(d) => break d,
                Err(e) if attempts < 2 => {
                    attempts += 1;
                    eprintln!("Error DHT11 (intento {}): {:?}, reintentando...", attempts, e);
                    thread::sleep(Duration::from_millis(500));
                }
                Err(e) => {
                    eprintln!("Error DHT11 definitivo: {:?}", e);
                    break SensorOutput::Text("Error DHT11".to_string());
                }
            }
        };

        println!("DHT11 => {:?}", data_dht11);
        storage.save(data_dht11.clone()).unwrap();

        if let SensorOutput::Text(ref s) = data_dht11 {
            if let Err(e) = mqtt.send(s.as_bytes()) {
                eprintln!("Error enviando DHT11 via MQTT: {:?}", e);
            }
        }

        if i < 3 {
            println!("Esperando 8s antes de la siguiente lectura...");
            thread::sleep(Duration::from_secs(8));
        }
    }

    // Mostrar resultados almacenados
    println!("Datos almacenados en memoria:");
    for (i, entry) in storage.list().unwrap().iter().enumerate() {
        println!("{}. {:?}", i + 1, entry);
    }

    println!("Lectura y envío finalizados.");

    // Espera final para asegurar que los últimos mensajes MQTT se transmitan
    thread::sleep(Duration::from_secs(1));
}
