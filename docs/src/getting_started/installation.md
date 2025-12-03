# Instalación

Esta guía te ayudará a instalar y configurar el **Framework para IoT Lince** en tu Raspberry Pi 4.

---

## Requisitos del Sistema

### Hardware

- **Raspberry Pi 4** compatible con `rppal`  
- **Tarjeta SD** de al menos 8GB  
- **Fuente de alimentación:** 5V, 2.5A mínimo  

### Software

- **Sistema Operativo:** Raspberry Pi OS (32-bit o 64-bit)  
- **Rust:** 1.70 o superior  
- **Git:** Para clonar el repositorio  

---

## Paso 1: Preparar el Sistema

### Actualizar el Sistema

### Actualizar el Sistema
```bash
# Actualiza la lista de paquetes disponibles desde los repositorios
sudo apt update

# Instala las versiones más recientes de los paquetes ya instalados
sudo apt upgrade -y  # -y acepta automáticamente todas las confirmaciones
```

### Instalar Dependencias

### Instalar Dependencias
```bash
# build-essential: herramientas de compilación (gcc, g++, make)
# git: sistema de control de versiones para clonar repositorios
# curl: herramienta para descargar archivos desde URLs
sudo apt install -y build-essential git curl
```

### Habilitar Interfaces (si es necesario)

Para usar sensores **OneWire** (como DS18B20):

```bash
# Editar /boot/config.txt
sudo nano /boot/config.txt

# Agregar al final:
dtoverlay=w1-gpio,gpiopin=4

# Guardar (Ctrl+O) y salir (Ctrl+X)
sudo reboot
```

---

## Paso 2: Instalar Rust

### Instalación Estándar

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Selecciona la opción por defecto (**1**).

### Configurar el Entorno

```bash
source $HOME/.cargo/env
```

### Verificar la Instalación

```bash
rustc --version
cargo --version
```

Deberías ver algo como:

```
rustc 1.75.0 (82e1608df 2024-01-15)
cargo 1.75.0 (1d8b05cdd 2024-01-01)
```

---


## Recursos Adicionales

- [Documentación oficial de Rust](https://www.rust-lang.org/learn)  
- [RPPAL Documentation](https://docs.rs/rppal/latest/rppal/)  
- [Raspberry Pi GPIO Pinout](https://pinout.xyz/)  
- [Documentación Raspberry](https://www.raspberrypi.com/documentation/)
