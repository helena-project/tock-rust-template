#![feature(alloc)]
#![no_std]

extern crate alloc;
extern crate tock;

use alloc::fmt::Write;
use tock::console::Console;
use tock::sensors::*;

mod ble_ess;

use ble_ess::ReadingType;

fn main() {
    let mut console = Console::new();

    let mut client = ble_ess::connect().expect("Failed to connect");
    client.init().unwrap();

    tock::timer::delay_ms(5000);

    write!(&mut console, "Init\n").unwrap();

    client.led_register().expect("Failed to register LED");

    let mut humidity = HumiditySensor;
    let mut temperature = TemperatureSensor;
    let mut light = AmbientLightSensor;
    loop {
        // Temperature
        let temp = temperature.read();
        write!(&mut console, "Temperature: {}\n", temp).unwrap();
        if let Err(_) = client.set_reading(ReadingType::Temperature, temp) {
            write!(&mut console, "Failed to set temperature\n").unwrap_or(());
        }

        // Light
        let lx = light.read();
        write!(&mut console, "Light:       {}\n", lx).unwrap();
        if let Err(_) = client.set_reading(ReadingType::Light, lx) {
            write!(&mut console, "Failed to set temperature\n").unwrap_or(());
        }

        // Humidity
        let humid = humidity.read();
        write!(&mut console, "Humidity:    {}\n", humid).unwrap();
        if let Err(_) = client.set_reading(ReadingType::Humidity, humid) {
            write!(&mut console, "Failed to set temperature\n").unwrap_or(());
        }

        tock::timer::delay_ms(5000);
    }
}

