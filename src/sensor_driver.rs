use embedded_hal::digital::v2::InputPin;
use stm32f4xx_hal::{adc::Adc, gpio::*, pac};

pub struct FireSensors<TEMP, SMOKE> {
    temp_sensor: TEMP,
    smoke_sensor: SMOKE,
}

impl<TEMP, SMOKE> FireSensors<TEMP, SMOKE>
where
    TEMP: Adc<pac::ADC1, u16>,
    SMOKE: InputPin,
{
    pub fn new(temp_sensor: TEMP, smoke_sensor: SMOKE) -> Self {
        Self {
            temp_sensor,
            smoke_sensor,
        }
    }

    pub fn read_temperature(&mut self) -> Result<f32, ()> {
        let raw_temp = self.temp_sensor.read().map_err(|_| ())?;
        Ok((raw_temp as f32) / 100.0) // Convert to Celsius
    }

    pub fn read_smoke_level(&self) -> Result<u16, ()> {
        let raw_smoke = self.smoke_sensor.read().map_err(|_| ())?;
        Ok(raw_smoke) // Raw smoke value
    }
}
