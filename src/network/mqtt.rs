use std::time::Duration;
use rumqttc::{Client, MqttOptions, QoS};
use crate::core::traits::communicator::{Communicator, CommunicatorError};

/// `MqttCommunicator` es un comunicador que envía datos a un broker MQTT.
///
/// Implementa el trait [`Communicator`], permitiendo la publicación de mensajes
/// en tópicos MQTT usando la librería [`rumqttc`].
///
/// Está diseñado para aplicaciones IoT donde los datos deben enviarse a un servidor/broker
/// que pueda distribuirlos a múltiples suscriptores.

pub struct MqttCommunicator {
    client: Client,
    topic_prefix: String,
}

impl MqttCommunicator {
    /// Crea una nueva instancia de `MqttCommunicator` configurada para conectarse a un broker MQTT.
    ///
    /// # Parámetros
    /// - `broker_url`: Dirección del broker MQTT (por ejemplo, `"mqtt.example.com"`).
    /// - `topic_prefix`: Prefijo que se antepondrá a todos los tópicos publicados.
    ///
    /// # Ejemplo
    /// ```
    /// let mut mqtt_comm = MqttCommunicator::new("test.mosquitto.org", "sensores/");
    /// ```
    /// 
    /// 

    
    pub fn new(broker_url: &str, topic_prefix: &str) -> Self {
        // Configuración básica de conexión MQTT
        let mut mqtt_options = MqttOptions::new(broker_url, "iot_framework", 1883);
        mqtt_options.set_keep_alive(Duration::from_secs(20));

        // Crea el cliente MQTT
        let (client, _connection) = Client::new(mqtt_options, 10);

        MqttCommunicator {
            client,
            topic_prefix: topic_prefix.to_string(),
        }
    }
}

impl Communicator for MqttCommunicator {
    /// Tipo de datos a enviar: `(topic, payload)`.
    type Command = (String, String);
    /// Tipo de respuesta: `()`, ya que no se espera un retorno inmediato.
    type Response = ();

    /// Publica un mensaje en un tópico MQTT.
    ///
    /// # Detalles
    /// - Concatena `topic_prefix` con el tópico recibido para formar el tópico final.
    /// - Publica el mensaje con [`QoS::AtLeastOnce`], garantizando entrega mínima una vez.
    ///
    /// # Parámetros
    /// - `command`: Una tupla `(topic, payload)` donde:
    ///   - `topic`: Tópico relativo al `topic_prefix`.
    ///   - `payload`: Contenido del mensaje.
    ///
    /// # Retorna
    /// - `Ok(())` si la publicación fue exitosa.
    /// - [`CommunicatorError::SendError`] si hubo un fallo en la publicación.
    fn send(&mut self, command: Self::Command) -> Result<Self::Response, CommunicatorError> {
        let (topic, payload) = command;
        let full_topic = format!("{}{}", self.topic_prefix, topic);

        self.client
            .publish(full_topic, QoS::AtLeastOnce, false, payload.as_bytes())
            .map_err(|e| CommunicatorError::SendError(e.to_string()))
            .map(|_| ())
    }

    fn receive(&mut self) -> Result<Self::Response, CommunicatorError> {
        unimplemented!()
    }
}