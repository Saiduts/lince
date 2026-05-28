// src/network/mqtt.rs

use std::sync::{Arc, Mutex};
use rumqttc::{Client, Event, MqttOptions, Packet, QoS};
use crate::core::traits::communicator::{Communicator, CommunicatorError};

/// Communicator que publica mensajes a un broker MQTT.
///
/// Internamente lanza un hilo de red que procesa los eventos de la conexión
/// y mantiene una bandera `connected` que refleja el estado real del broker.
/// Esta bandera es la que permite al sistema de reintento distinguir un fallo
/// por desconexión de uno por error de protocolo.
///
/// # Errores que devuelve `send()`
///
/// - `CommunicatorError::Disconnected` — el broker no está alcanzable.
///   El dato debería guardarse para reintento.
/// - `CommunicatorError::SendError` — error de protocolo o parámetros inválidos.
///   No tiene sentido reintentar.
pub struct MqttCommunicator {
    client: Client,
    topic: String,
    /// Estado de la conexión compartido con el hilo de red.
    /// `true` = broker alcanzable y conexión activa.
    connected: Arc<Mutex<bool>>,
}

impl MqttCommunicator {
    /// Crea un nuevo `MqttCommunicator` y lanza el hilo de red.
    ///
    /// # Parámetros
    /// - `client_id`: identificador único del cliente MQTT. No puede ser vacío.
    /// - `broker`: dirección del broker (hostname o IP). No puede ser vacío.
    /// - `port`: puerto del broker. Rango válido: 1–65535.
    /// - `topic`: tópico de publicación. No puede ser vacío ni contener `#` o `+`.
    ///
    /// # Errores
    /// - `CommunicatorError::SendError` si algún parámetro es inválido.
    pub fn new(
        client_id: &str,
        broker: &str,
        port: u16,
        topic: &str,
    ) -> Result<Self, CommunicatorError> {
        // --- Validaciones de parámetros ---
        if client_id.is_empty() {
            eprintln!("[MqttCommunicator] client_id no puede ser vacío");
            return Err(CommunicatorError::SendError);
        }
        if broker.is_empty() {
            eprintln!("[MqttCommunicator] broker no puede ser vacío");
            return Err(CommunicatorError::SendError);
        }
        if port == 0 {
            eprintln!("[MqttCommunicator] port no puede ser 0");
            return Err(CommunicatorError::SendError);
        }
        if topic.is_empty() || topic.contains('#') || topic.contains('+') {
            eprintln!("[MqttCommunicator] topic inválido: '{}'", topic);
            return Err(CommunicatorError::SendError);
        }

        let mut mqttoptions = MqttOptions::new(client_id, broker, port);
        mqttoptions.set_keep_alive(std::time::Duration::from_secs(5));

        let (client, mut connection) = Client::new(mqttoptions, 10);

        // Bandera compartida entre este struct y el hilo de red
        let connected = Arc::new(Mutex::new(false));
        let connected_clone = Arc::clone(&connected);

        // Hilo de red: actualiza `connected` según los eventos del broker
        std::thread::spawn(move || {
            for event in connection.iter() {
                match event {
                    Ok(Event::Incoming(Packet::ConnAck(_))) => {
                        if let Ok(mut c) = connected_clone.lock() {
                            *c = true;
                        }
                    }
                    Err(_) => {
                        if let Ok(mut c) = connected_clone.lock() {
                            *c = false;
                        }
                    }
                    _ => {}
                }
            }
            // Si el iterador termina, la conexión se perdió definitivamente
            if let Ok(mut c) = connected_clone.lock() {
                *c = false;
            }
        });

        Ok(Self {
            client,
            topic: topic.to_string(),
            connected,
        })
    }

    /// Retorna `true` si la conexión con el broker está activa en este momento.
    ///
    /// Útil para que el sistema de reintento compruebe si tiene sentido
    /// intentar vaciar la cola de pendientes antes de hacerlo.
    pub fn is_connected(&self) -> bool {
        self.connected.lock().map(|c| *c).unwrap_or(false)
    }
}

impl Communicator for MqttCommunicator {
    /// Publica `data` en el tópico configurado.
    ///
    /// # Errores
    /// - `CommunicatorError::Disconnected` si el broker no está alcanzable.
    /// - `CommunicatorError::SendError` si el payload está vacío o hay un
    ///   error de protocolo no relacionado con la conectividad.
    fn send(&mut self, data: &[u8]) -> Result<(), CommunicatorError> {
        if data.is_empty() {
            eprintln!("[MqttCommunicator] payload vacío");
            return Err(CommunicatorError::SendError);
        }

        // Si sabemos que no hay conexión, fallamos rápido con Disconnected
        if !self.is_connected() {
            return Err(CommunicatorError::Disconnected);
        }

        self.client
            .publish(&self.topic, QoS::AtLeastOnce, false, data)
            .map_err(|e| {
                eprintln!("[MqttCommunicator] Error al publicar: {}", e);
                // Marcar como desconectado para que el siguiente send() falle rápido
                if let Ok(mut c) = self.connected.lock() {
                    *c = false;
                }
                CommunicatorError::Disconnected
            })
    }
}