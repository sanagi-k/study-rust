#[cfg(target_os = "linux")]
mod gpio_rpi {
    use rppal::gpio::Gpio;
    use std::{thread, time::Duration};

    pub fn control_gpio() {
        let gpio = Gpio::new().unwrap();
        let mut pin = gpio.get(23).unwrap().into_output();
        pin.set_high();
        thread::sleep(Duration::from_secs(1));
        pin.set_low();
    }
}

fn main() {
    println!("RPi GPIO!");

    #[cfg(target_os = "linux")]
    gpio_rpi::control_gpio();
}
