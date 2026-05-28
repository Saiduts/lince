# Comunicación (Network)

Este módulo gestiona la **transmisión de datos** en el framework Lince.
Incluye el comunicador MQTT y el formateador Smart Campus.

---

## Contenido

- [MqttCommunicator](mqtt.md) — publica mensajes a un broker MQTT.
- [SmartCampusFormatter](smart_campus.md) — convierte valores numéricos al formato JSON Smart Campus.
- [Crear Communicators Personalizados](custom_communicators.md)

---

## Flujo típico

```
SensorParser        →  valores numéricos
SmartCampusFormatter →  JSON Smart Campus
MqttCommunicator    →  broker MQTT
SqliteStorage       →  reintento si hay desconexión
```

El reintento por desconexión vive en `SqliteStorage`, no en este módulo.
Consulta [SqliteStorage](../storage/sqlite_storage.md) para más detalles.

---

## Convenciones

- Todas las comunicaciones usan el trait `Communicator` como interfaz.
- `SmartCampusFormatter` recibe valores ya extraídos por `SensorParser`,
  no `SensorOutput` crudo. Esto permite operar los valores antes de formatearlos.