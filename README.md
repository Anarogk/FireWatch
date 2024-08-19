# Fire Detection System with Rust Embedded

This project is an embedded fire detection system built using Rust, targeting STM32 microcontrollers. The system detects fire using temperature and smoke sensors, identifies affected areas within a building, and sends real-time alerts to a React Native application through MQTT.

## Features

- **Fire Detection**: Monitors temperature and smoke levels to detect fire.
- **Clustering**: Identifies clusters of sensors to pinpoint affected floors/rooms.
- **Communication**: Uses MQTT to send fire alerts to a mobile app with detailed information.
- **Robust Rust Code**: Leveraging advanced Rust features like error handling, concurrency, and JSON serialization.

## Hardware Setup

- **Board**: STM32F407G-DISC1
- **Sensors**:
  - **DHT22**: Temperature and humidity sensor
  - **MQ-2**: Smoke and gas sensor
  - (Optional) **Flame Sensor**: For direct fire detection
  - (Optional) **GPS Module**: For location tracking
- **Communication Module**: ESP8266/ESP32 for Wi-Fi communication or HM-10 for Bluetooth communication.

For detailed wiring instructions, see the [Hardware Setup Documentation](docs/hardware_setup.md).

## Software Setup

### Prerequisites

- **Rust**: Ensure Rust is installed with the correct target for embedded development:
  ```sh
  rustup target add thumbv7em-none-eabihf
  ```
- **OpenOCD**: For flashing the STM32 board.

### Installing Dependencies

Clone the repository and run:

```sh
cargo build
```

This will build the project for the STM32F407G-DISC1 target.

### Configuration

Configure the MQTT settings in `config/mqtt_config.toml`:

```toml
[broker]
url = "mqtt://broker.hivemq.com"
username = "your_username"
password = "your_password"
```

### Building and Flashing

Use `cargo-embed` to build and flash the code onto the microcontroller:

```sh
cargo embed
```

### Running the Example

To run the basic fire detection example:

```sh
cargo run --example basic_fire_detection
```

## Project Structure

- **src/**: Contains the main source code.
  - `main.rs`: Main application entry point.
  - `fire_detection.rs`: Fire detection logic.
  - `communication.rs`: MQTT communication.
  - `sensor_driver.rs`: Sensor drivers.
  - `cluster.rs`: Clustering logic for identifying affected areas.
- **boards/**: Board-specific configurations for STM32F407G-DISC1.
- **config/**: Configuration files for MQTT and other services.
- **examples/**: Example code for testing specific functionalities.
- **docs/**: Documentation on hardware setup and deployment.

## React Native Integration

This project communicates with a React Native app via MQTT. The app displays real-time fire alerts with detailed information about the affected area. Use `react-native-mqtt` for the mobile app setup.

## Contributing

Feel free to open issues or submit pull requests. Contributions are welcome!

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.


### Next Steps

1. **Create the Repository**: Initialize a new GitHub repository and push this structure.
2. **Add CI/CD**: Consider adding GitHub Actions for continuous integration, ensuring your code builds and passes tests on every commit.
3. **Extend Documentation**: Add more detailed documentation for each module in the `docs/` folder as needed, including API documentation if you expose reusable components.

This structure will help maintain a clean and organized project on GitHub, adhering to common Rust embedded project conventions.




## Dependency installation

```bash
rustup target add thumbv7em-none-eabihf
cargo install cargo-embed

```


if ``cargo install cargo-embed`` fails, try following command inside powershell if you are using windows.
```powershell
irm https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.ps1 | iex
```


for this to work, you need to have the set the execution policy to remote-signed.

```powershell
Set-ExecutionPolicy RemoteSigned -scope CurrentUser
```


