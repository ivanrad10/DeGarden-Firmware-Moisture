use esp_hal::{
    analog::adc::{Adc, AdcConfig},
    gpio::Io,
    peripherals::ADC1,
    prelude::nb,
};

// Return current sensor reading
pub fn get_measurement(io: Io, adc1: ADC1) -> f64 {
    let analog_pin = io.pins.gpio2;
    let mut adc1_config = AdcConfig::new();
    let mut pin = adc1_config.enable_pin(
        analog_pin,
        esp_hal::analog::adc::Attenuation::Attenuation11dB,
    );
    let mut adc1 = Adc::new(adc1, adc1_config);
    let pin_value: u16 = nb::block!(adc1.read_oneshot(&mut pin)).unwrap();
    return pin_value as f64;
}
