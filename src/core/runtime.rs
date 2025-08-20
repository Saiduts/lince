use crate::core::traits::actuator::Actuator;
use crate::core::traits::communicator::Communicator;
use crate::core::traits::sensor::Sensor;
/// Controla el ciclo de ejecución del sistema.
use std::time::Duration;
use tokio::time;

/// Se encarga de coordinar sensores, actuadores, comunicación y almacenamiento.
/// Normalmente se implementa en el **núcleo** del framework.
pub struct RuntimeController<S, C, A> {
    /// Sensores que se encargan de leer datos de la entorno.
    sensors: Vec<S>,

    /// Actuadores que se encargan de actuar en la entorno.
    actuators: Vec<A>,

    /// Comunicador que se encarga de comunicarse con el entorno.
    communicator: C,

    /// Intervalo de tiempo entre cada iteración del ciclo de ejecución.
    interval: Duration,
}

impl<S, C, A> RuntimeController<S, C, A>
where
    // Send es para que pueda ser transmitido por hilos diferentes.
    S: Sensor + Send,
    C: Communicator<Command = S::Output> + Send,
    A: Actuator<Command = S::Output> + Send,
    S::Output: Clone + std::fmt::Debug,
{
    /// Crea un nuevo controlador de ejecución.
    /// # Parámetros
    /// - `sensors`: Vector de sensores que se encargan de leer datos de la entorno.
    /// - `actuators`: Vector de actuadores que se encargan de actuar en la entorna.
    /// - `communicator`: Comunicador que se encarga de comunicarse con el entorno.
    /// - `interval`: Intervalo de tiempo en segundos entre cada iteración del ciclo de ejecución.
    pub fn new(sensors: Vec<S>, actuators: Vec<A>, communicator: C, interval: u64) -> Self {
        Self {
            sensors,
            actuators,
            communicator,
            interval: Duration::from_secs(interval),
        }
    }

    /// Inicia el ciclo de ejecución del sistema.
    /// Este método:
    /// - Utiliza un temporizador no bloqueante (`tokio::time::interval`).
    /// - Itera infinitamente hasta que el programa finalice o sea interrumpido.
    /// - Ejecuta lectura, publicación y control en cada ciclo.
    pub async fn run(&mut self) {

        // Declaramos un intervalo que se encargará de ejecutar la función `tick` cada `interval` segundos.
        let mut interval = time::interval(self.interval);
        loop {
            // Llamamos a la función `tick` del intervalo cada `interval` segundos.
            interval.tick().await;
            // En cada iteración del ciclo de ejecución, leemos los valores de los sensores y los ejecutamos en los actuadores.
            for sensor in &mut self.sensors {
                // Leemos el valor del sensor.
                match sensor.read() {
                    // Si se recibe un valor, lo imprimimos y lo enviamos al comunicador.
                    Ok(value) => {
                        println!("Sensor valor: {:?}", value);
                        
                        if let Err(e) = self.communicator.send(value.clone()) {
                            eprintln!("Error enviando dato: {:?}", e);
                        }

                        // Ejecutamos los actuadores necesarios con el valor recibido.
                        for actuator in &mut self.actuators {
                            if let Err(e) = actuator.execute(value.clone()) {
                                eprintln!("Error actuando: {:?}", e);
                            }
                        }
                    }
                    // Si ocurre algún error, lo imprimimos y lo ignoramos.
                    Err(e) => {
                        eprintln!("Error leyendo sensores: {:?}", e);
                    }
                }
            }
        }
    }
}
