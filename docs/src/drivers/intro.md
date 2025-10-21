# Desarrollar Drivers Propios

Esta guía te enseña a crear drivers de hardware de bajo nivel para integrar nuevos protocolos y dispositivos al framework.

## ¿Qué es un Driver?

Un **driver** es una capa de abstracción entre el hardware físico y las implementaciones de sensores de alto nivel. Maneja:

-   Protocolos de comunicación (I2C, SPI, UART, OneWire)
-   Timing crítico y señales
-   Parsing de datos binarios
-  ️ Manejo de errores de hardware

## Recursos

- [embedded-hal Documentation](https://docs.rs/embedded-hal/)
- [RPPAL Documentation](https://docs.rs/rppal/)

## Ver También

- [GpioDriver (GPIO)](./gpio.md)
- [Crear Sensores Personalizados](../sensors/custom_sensors.md)
- [Arquitectura del Framework](../user_guide/architecture.md)