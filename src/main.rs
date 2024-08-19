#![no_std]
#![no_main]

use panic_halt as _; // Halt on panic
use cortex_m_rt::entry;
use stm32f4xx_hal::{pac, prelude::*};
use fire_detection::{FireDetection, SensorCluster};
use communication::send_fire_alert;
use sensor_driver::FireSensors;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    // Initialize RTT for debugging
    rtt_init_print!();

    let peripherals = pac::Peripherals::take().unwrap();
    let rcc = peripherals.RCC.constrain();
    let clocks = rcc.cfgr.freeze();

    let gpioa = peripherals.GPIOA.split();
    let adc = peripherals.ADC1;
    
    // Initialize sensors
    let temp_sensor = FireSensors::new(adc, gpioa.pa0);
    let fire_detection = FireDetection::new(50.0, 300);
    let cluster = SensorCluster::new();

    loop {
        let temperature = temp_sensor.read_temperature().unwrap_or(0.0);
        let smoke_level = temp_sensor.read_smoke_level().unwrap_or(0);

        if fire_detection.detect_fire(temperature, smoke_level) {
            if let Some(affected_cluster) = fire_detection.identify_affected_cluster("Floor 1", &cluster) {
                let alert = FireAlert {
                    location: "Room A, Floor 1",
                    temperature,
                    smoke_level,
                    cluster: affected_cluster,
                };
                send_fire_alert(&mut mqtt_client, alert);
            }
        }

        // Sleep or handle other tasks
    }
}
