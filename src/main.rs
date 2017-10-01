#![feature(alloc)]
#![no_std]

extern crate alloc;
extern crate tock;

use alloc::fmt::Write;
use tock::console::Console;
use tock::ipc::ble_ess::{self, ReadingType};
use tock::sensors::*;
use tock::ipc::Client;
use alloc::String;

fn main() {
    let mut console = tock::console::Console::new();
    let mut client = Client::new(
            String::from("org.tockos.services.ble-led-ctrl")).unwrap();
    let buffer = client.share(5).unwrap();


    tock::timer::delay_ms(5000);

    
    write!(&mut console, "Init\n");

    client.led_register().unwrap();

    loop {
        tock::timer::delay_ms(10000);
        client.ping();
    }
}

