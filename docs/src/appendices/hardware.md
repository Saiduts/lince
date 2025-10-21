
# Hardware Compatible

Este framework está diseñado específicamente para **Raspberry Pi 4** y dispositivos compatibles con su arquitectura GPIO.

## Raspberry Pi 4

### Especificaciones Técnicas

| Característica | Especificación |
|----------------|----------------|
| **Procesador** | Broadcom BCM2711, Quad core Cortex-A72 (ARM v8) 64-bit SoC @ 1.5GHz |
| **RAM** | 2GB, 4GB u 8GB LPDDR4-3200 SDRAM |
| **GPIO** | 40 pines (compatible con Pi 3/2/B+) |
| **Voltaje GPIO** | 3.3V |
| **Corriente máxima** | 16mA por pin (50mA total recomendado) |
| **Sistema Operativo** | Raspberry Pi OS (32-bit o 64-bit) |

### Pinout GPIO

![Pinout de Raspberry Pi 4](../images/rasppinout.png)

> **Nota:** El framework utiliza numeración **BCM**, no física.

---

## Requisitos del Sistema

### Sistema Operativo

- **Recomendado:** Raspberry Pi OS (64-bit) Bookworm o posterior
- **Kernel:** Linux 5.15 o superior

### Software Necesario

```bash
# Actualizar sistema
sudo apt update && sudo apt upgrade -y

# Instalar dependencias
sudo apt install -y build-essential git curl

# Instalar Rust (si no está instalado)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### Permisos GPIO

Para acceder a GPIO sin `sudo`:

```bash
# Agregar usuario al grupo gpio
sudo usermod -a -G gpio $USER

# Aplicar cambios (reiniciar sesión o usar)
newgrp gpio
```

---

## Pines Recomendados por Protocolo

### GPIO Digital (Sensores DHT, MH-RD)

**Pines disponibles para uso general:**
- GPIO4, GPIO17, GPIO27, GPIO22
- GPIO23, GPIO24, GPIO25, GPIO5, GPIO6
- GPIO12, GPIO13, GPIO16, GPIO19, GPIO20, GPIO21, GPIO26

**Ejemplo de configuración:**
```rust
// DHT11 en GPIO17
let mut dht11 = Dht11Sensor::new(17)?;

// MH-RD en GPIO27
let mut rain = MhRdSensor::new(27, true)?;
```

### OneWire (DS18B20)

**Pin recomendado:** GPIO4 (pin físico 7)

**Configuración en `/boot/config.txt`:**

# Habilitar OneWire en GPIO4
dtoverlay=w1-gpio,gpiopin=4


**Verificar dispositivos detectados:**
```bash
# Listar dispositivos OneWire
ls /sys/bus/w1/devices/
# Ejemplo de salida: 28-00000abcdef1
```

## Consideraciones Eléctricas

### Voltajes

| Componente | Voltaje | Notas |
|------------|---------|-------|
| **GPIO Logic** | 3.3V | **NO tolerante a 5V** |
| **Alimentación sensores** | 3.3V o 5V | Verificar datasheet |
| **Salida digital sensores** | 3.3V | Usar level shifter si es 5V |

**ADVERTENCIA:** Conectar señales de 5V directamente a pines GPIO puede **dañar permanentemente** la Raspberry Pi.

### Corrientes Máximas

- **Por pin GPIO:** 16mA
- **Total GPIO:** 50mA recomendado
- **3.3V rail:** 500mA máximo
- **5V rail:** Limitado por fuente de alimentación

### Protección Recomendada

```
Sensor → Resistencia 330Ω → GPIO Pin
```

Protege contra corrientes excesivas en caso de cortocircuito.

---

## Alimentación de la Raspberry Pi 4

### Fuente de Alimentación

| Especificación | Valor |
|----------------|-------|
| **Voltaje** | 5V DC |
| **Corriente mínima** | 3A |
| **Conector** | USB-C |
| **Potencia** | 15W (mínimo recomendado) |


## Limitaciones y Restricciones

### Pines No Disponibles para GPIO General

| Pin | Uso Reservado |
|-----|---------------|
| GPIO0, GPIO1 | I2C ID EEPROM (HATs) |
| GPIO2, GPIO3 | I2C principal |
| GPIO14, GPIO15 | UART (consola serial) |


## Solución de Problemas

### Error: "Permission denied" al acceder GPIO

**Solución:**
```bash
sudo usermod -a -G gpio $USER
# Cerrar sesión y volver a entrar
```

### Sensor DHT no responde

**Verificar:**
1. Resistencia pull-up instalada (4.7kΩ - 10kΩ)
2. Conexiones correctas (VCC, DATA, GND)
3. Voltaje de alimentación (3.3V o 5V según modelo)
4. Esperar 2 segundos entre lecturas

### DS18B20 no detectado

**Solución:**
```bash
# Verificar configuración en /boot/config.txt
grep w1-gpio /boot/config.txt

# Si no existe, agregar:
echo "dtoverlay=w1-gpio,gpiopin=4" | sudo tee -a /boot/config.txt

# Reiniciar
sudo reboot
```

---

## Recursos Adicionales

### Documentación Oficial

- [Raspberry Pi GPIO Documentation](https://www.raspberrypi.com/documentation/computers/raspberry-pi.html)
- [RPPAL Rust Crate](https://docs.rs/rppal/)

### Diagramas y Herramientas

- [Pinout.xyz](https://pinout.xyz/) - Referencia interactiva de pines
- [Fritzing](https://fritzing.org/) - Diseño de circuitos

### Datasheets de Sensores

- [DHT11 Datasheet](https://www.mouser.com/datasheet/2/758/DHT11-Technical-Data-Sheet-Translated-Version-1143054.pdf)
- [DHT22/AM2302 Datasheet](https://www.sparkfun.com/datasheets/Sensors/Temperature/DHT22.pdf)
- [DS18B20 Datasheet](https://datasheets.maximintegrated.com/en/ds/DS18B20.pdf)

---

## Próximos Pasos

- Consulta [Instalación](../getting_started/installation.md) para configurar el entorno
- Revisa [Guía Rápida](../getting_started/quick_start.md) para tu primer proyecto
- Explora [Drivers GPIO](../drivers/gpio.md) para detalles de implementación
```