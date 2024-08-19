
# Hardware Setup for Fire Detection System

This guide will walk you through setting up the hardware for the fire detection system.

## Microcontroller Board: STM32F407G-DISC1

### Steps

1. **Power Supply**: Ensure your STM32F407G-DISC1 board is powered via USB or an external power source.
2. **Sensor Connections**:
   - **DHT22**: Connect the data pin to GPIOA0 with a 10k pull-up resistor, and power it with 3.3V or 5V.
   - **MQ-2**: Connect the analog output pin to an ADC pin (e.g., PA1), and power it with 5V.
   - (Optional) **Flame Sensor**: Connect the digital output to a GPIO pin, e.g., GPIOA2.
   - (Optional) **ESP8266**: Connect the RX/TX pins to the appropriate UART pins on the STM32 board (e.g., PA9 and PA10).
3. **Flashing the Code**: Use `cargo embed` to build and flash the code onto the STM32 board.
4. **Testing**: Ensure the sensors are reading correctly by monitoring the RTT output or connecting to a debugger.

