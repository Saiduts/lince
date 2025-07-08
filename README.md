# 📡 Framework IoT para Dispositivos Gateway – SmartCampus UIS

Este proyecto es un framework modular, liviano y extensible en Rust, diseñado para facilitar el desarrollo de aplicaciones IoT en dispositivos tipo gateway, como Raspberry Pi. Está orientado a la plataforma SmartCampus UIS, pero puede ser adaptado a cualquier entorno similar.


## 🎯 Objetivo

Permitir la rápida implementación de soluciones IoT en gateways

# Procedimiento
🧭 FASE 1 – AMBIENTACIÓN TECNOLÓGICA
🔹 Paso 1. Instalar herramientas (Hecho)
🛠 Qué hacer:

Instala Rust:

bash
Copiar
Editar
curl https://sh.rustup.rs -sSf | sh
Instala cargo-generate y cargo-edit:

bash
Copiar
Editar
cargo install cargo-generate cargo-edit
🎯 Resultado: Ambiente de desarrollo Rust listo y funcionando.

🔹 Paso 2. Crear proyecto base (Hecho)
🛠 Qué hacer:

bash
Copiar
Editar
cargo new iot_framework --lib
cd iot_framework
🎯 Resultado: Carpeta iot_framework/ con Cargo.toml y src/lib.rs.

🔹 Paso 3. Estructurar carpetas modulares (Hecho)
🛠 Qué hacer:

bash
Copiar
Editar
mkdir -p src/core src/drivers src/network src/config src/platform
🎯 Resultado: Proyecto organizado por capas funcionales (sensores, red, config, etc.)

🧭 FASE 2 – DISEÑO Y DEFINICIÓN DEL FRAMEWORK
🔹 Paso 4. Definir traits base (Hecho)
🛠 Qué hacer: Crear estos traits (interfaces):

Sensor (medición)

Actuator (control de dispositivos)

Communicator (envío de datos)

Storage (almacenamiento local)

RuntimeController (manejo de ciclo principal)

🎯 Resultado: Abstracciones limpias que otros podrán implementar.

🔹 Paso 5. Definir estructuras de configuración (Hecho)
🛠 Qué hacer:

Crear archivo config.toml

Crear structs con serde para deserializar

Leer config al iniciar el framework

🎯 Resultado: El framework se puede configurar sin tocar código.

🔹 Paso 6. Diseñar arquitectura modular
🛠 Qué hacer:

Diagrama de componentes y relaciones (puedes usar draw.io o Excalidraw)

Definir API pública (lib.rs)

Documentar los módulos (/// + cargo doc)

🎯 Resultado: Arquitectura clara para mantenimiento y extensibilidad.

🧭 FASE 3 – IMPLEMENTACIÓN DEL FRAMEWORK
🔹 Paso 7. Implementar sensor simulado
🛠 Qué hacer:

Crear SimulatedSensor que devuelve valores falsos.

Implementa el trait Sensor.

🎯 Resultado: Sensor listo para pruebas sin hardware real.

🔹 Paso 8. Implementar comunicador por consola y MQTT
🛠 Qué hacer:

ConsoleCommunicator: imprime en consola

MqttCommunicator: usa rumqttc para publicar

🎯 Resultado: Puedes enviar datos a la nube o verlos localmente.

🔹 Paso 9. Implementar controlador del ciclo principal
🛠 Qué hacer:

RuntimeController que:

Lee sensores

Publica con Communicator

Controla actuadores (si aplica)

Ejecuta cada N segundos (configurable)

🎯 Resultado: El framework corre automáticamente sin intervención manual.

🔹 Paso 10. Implementar almacenamiento local opcional
🛠 Qué hacer:

FileStorage: guarda datos en .csv o .json

MemoryStorage: uso en RAM (útil para tests)

🎯 Resultado: Se puede guardar data localmente si se cae la red.

🔹 Paso 11. Soportar múltiples sensores y actuadores
🛠 Qué hacer:

Usa Vec<Box<dyn Sensor>> y Vec<Box<dyn Actuator>>

Permite registrar nuevos componentes dinámicamente

🎯 Resultado: El framework se vuelve escalable.

🔹 Paso 12. Crear estructura de extensión tipo plugin
🛠 Qué hacer:

Permitir que el usuario registre su propio Sensor o Actuator
desde su código sin modificar el framework

🎯 Resultado: El framework es reutilizable por otros desarrolladores.

🧭 FASE 4 – VALIDACIÓN FUNCIONAL
🔹 Paso 13. Pruebas en Raspberry Pi
🛠 Qué hacer:

Usa rppal para GPIO reales

Conecta sensor DHT22 o BME280

Prueba lectura real + publicación MQTT

🎯 Resultado: Framework validado con hardware real

🔹 Paso 14. Pruebas de desempeño
🛠 Qué hacer:

Mide uso de CPU, RAM, errores, reconexiones

Usa herramientas: htop, iotop, valgrind

🎯 Resultado: Informe técnico de eficiencia y estabilidad

🧭 FASE 5 – DOCUMENTACIÓN Y PUBLICACIÓN
🔹 Paso 15. Documentación para desarrolladores
🛠 Qué hacer:

Documentar cada trait y struct (///)

Generar documentación con:

bash
Copiar
Editar
cargo doc --open
Escribir README y ejemplos en examples/

🎯 Resultado: Framework entendible por otros desarrolladores

🔹 Paso 16. Publicación como librería
🛠 Qué hacer:

Crear cuenta en crates.io

Añadir metadatos en Cargo.toml

Publicar:

bash
Copiar
Editar
cargo publish
🎯 Resultado: Tu framework estará disponible públicamente como librería Rust

🧭 FASE 6 – ENTREGABLES DEL TRABAJO DE GRADO
🔹 Paso 17. Manual de uso y guía práctica
🛠 Qué hacer:

Crear un manual paso a paso

Mostrar cómo extender el framework

Incluir ejemplos funcionales (sensores, publicación, controladores)

🎯 Resultado: Cumples el objetivo académico del trabajo de grado

🔹 Paso 18. Preparar informe final y presentación
🛠 Qué hacer:

Resumen de cada fase

Resultados obtenidos

Comparación con soluciones como MicroPython, Mongoose OS

Diagrama de arquitectura final

🎯 Resultado: Proyecto completo y defendible
