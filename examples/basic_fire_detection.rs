#![no_std]
#![no_main]

use panic_halt as _; // Halt on panic
use cortex_m_rt::entry;
use stm32f4xx_hal::{pac, prelude::*};
use fire_detection::FireDetection;
use sensor_driver::FireSensors;

#[entry]
fn main() -> ! {
    let peripherals = pac::Peripherals::take().unwrap();
    let rcc = peripherals.RCC.constrain();
    let clocks = rcc.cfgr.freeze();

    let gpioa = peripherals.GPIOA.split();
    let adc = peripherals.ADC1;

    // Initialize sensors
    let temp_sensor = FireSensors::new(adc, gpioa.pa0);
    let fire_detection = FireDetection::new(50.0, 300);

    loop {
        let temperature = temp_sensor.read_temperature().unwrap_or(0.0);
        let smoke_level = temp_sensor.read_smoke_level().unwrap_or(0);

        if fire_detection.detect_fire(temperature, smoke_level) {
            // Trigger an alert (e.g., LED, buzzer, etc.)
        }

        // Sleep or handle other tasks
    }
}
