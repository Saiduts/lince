use rumqttc::{Client, MqttOptions, QoS};
use crate::core::traits::communicator::{Communicator, CommunicatorError};

/// Communicator que publica mensajes a un broker MQTT.
pub struct MqttCommunicator {
    client: Client,
    topic: String,
}

impl MqttCommunicator {
    /// Crea un nuevo MqttCommunicator.
    ///
    /// # Parámetros
    /// - `client_id`: identificador del cliente MQTT
    /// - `broker`: dirección del broker (ej: "localhost")
    /// - `port`: puerto del broker (ej: 1883)
    /// - `topic`: tópico donde se publicarán los mensajes
    pub fn new(client_id: &str, broker: &str, port: u16, topic: &str) -> Result<Self, CommunicatorError> {
        let mut mqttoptions = MqttOptions::new(client_id, broker, port);
        mqttoptions.set_keep_alive(std::time::Duration::from_secs(5));

        let (client, mut _connection) = Client::new(mqttoptions, 10);

        // Opcional: puedes lanzar un hilo que procese los paquetes entrantes,
        // pero si solo envías, puedes ignorarlo.
        std::thread::spawn(move || {
            for _notification in _connection.iter() {
                // Aquí podrías loguear si lo deseas
            }
        });

        Ok(Self {
            client,
            topic: topic.to_string(),
        })
    }
}

impl Communicator for MqttCommunicator {
    fn send(&mut self, data: &[u8]) -> Result<(), CommunicatorError> {
        self.client
            .publish(&self.topic, QoS::AtLeastOnce, true, data)
            .map_err(|_| CommunicatorError::SendError)?;
        Ok(())
    }
}
