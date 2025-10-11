use std::thread;
use std::time::Duration;

use iot_framework::devices::sensors::dht11::Dht11Sensor;
use iot_framework::devices::sensors::dht22::Dht22Sensor;
use iot_framework::storage::memory::MemoryStorage;
use iot_framework::core::traits::sensor::Sensor;
use iot_framework::core::traits::storage::Storage;

fn main() {
    println!("Iniciando lectura de sensores DHT...");

    let mut dht22 = Dht22Sensor::new(23).unwrap();
    let mut dht11 = Dht11Sensor::new(17).unwrap();
    let mut storage = MemoryStorage::new();

    println!("Esperando 3s para estabilización inicial...");
    thread::sleep(Duration::from_secs(3));

    for i in 1..=3 {
        println!("Iteración {}/3", i);

        // Leer DHT22
        match dht22.read() {
            Ok(data) => {
                println!("DHT22 => {:?}", data);
                storage.save(data).unwrap();
            }
            Err(e) => println!("Error DHT22: {:?}", e),
        }

        thread::sleep(Duration::from_secs(3));

        // Leer DHT11
        match dht11.read() {
            Ok(data) => {
                println!("DHT11 => {:?}", data);
                storage.save(data).unwrap();
            }
            Err(e) => println!("Error DHT11: {:?}", e),
        }

        if i < 3 {
            println!("Esperando 8s antes de la siguiente lectura...");
            thread::sleep(Duration::from_secs(8));
        }
    }

    // Mostrar resultados almacenados
    println!("Datos almacenados en memoria:");

    let data = storage.list().unwrap();
    for (i, entry) in data.iter().enumerate() {
        println!("{}. {:?}", i + 1, entry);
    }

    println!("Lectura finalizada.");
}
