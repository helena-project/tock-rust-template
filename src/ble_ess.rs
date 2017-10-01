use core::cell::RefCell;
use alloc::boxed::Box;
use alloc::String;
use tock;
use tock::ipc::Client;

#[repr(u32)]
pub enum ReadingType {
    Temperature = 0,
    Humidity = 1,
    Light = 2
}

struct Internal {
    client: Client,
    buffer: Box<[u8]>,
    reading_done: bool,
}

pub struct RustFestDemo {
    internal: RefCell<Internal>
}

pub fn connect() -> Result<RustFestDemo, ()> {
    let mut client = Client::new(
                String::from("org.tockos.services.ble-led-ctrl"))?;
    let buffer = client.share(5)?;
    let internal = RefCell::new(Internal {
        client: client,
        buffer: buffer,
        reading_done: false,
    });
    Ok(RustFestDemo { internal: internal })
}

impl RustFestDemo {
    pub fn init(&mut self) -> Result<(), ()> {
        unsafe {
            let client_ptr = &self.internal as *const _ as usize;
            self.internal.borrow_mut().client.subscribe(RustFestDemo::cb, client_ptr)
        }
    }

    extern fn cb(_: usize, _: usize, _: usize, ptr: usize) {
        let client: &RefCell<Internal> = unsafe { &*(ptr as *const _) };

        let kind = {
            let buffer = &client.borrow().buffer;
            buffer[buffer.len() - 1]
        };
        match kind {
            0 => {
                tock::led::toggle(0);
                client.borrow_mut().buffer[0] = 0;
                unsafe {
                    client.borrow_mut().client.notify_async().unwrap();
                }
            },
            1 => {
                client.borrow_mut().reading_done = true;
            },
            _ => {}
        }

    }

    pub fn led_register(&mut self) -> Result<(), ()> {
        let mut internal = self.internal.borrow_mut();
        let last_idx = internal.buffer.len() - 1;
        internal.buffer[last_idx] = 0;
        unsafe {
            internal.client.notify_async()
        }
    }

    pub fn set_reading<I>(&mut self, sensor: ReadingType, data: I) -> Result<(), ()>
            where I: Into<i32> {
        let sensor_type = sensor as u32;
        let data = Into::<i32>::into(data) as u32;
        {
            let buffer = &mut self.internal.borrow_mut().buffer;
            let last_idx = buffer.len() - 1;
            buffer[last_idx] = 1;
            buffer[0..4].copy_from_slice(&[(sensor_type & 0xff) as u8,
                                           ((sensor_type >> 8) & 0xff) as u8,
                                           ((sensor_type >> 16) & 0xff) as u8,
                                           ((sensor_type >> 24) & 0xff) as u8]);
            buffer[4..8].copy_from_slice(&[(data & 0xff) as u8,
                                           ((data >> 8) & 0xff) as u8,
                                           ((data >> 16) & 0xff) as u8,
                                           ((data >> 24) & 0xff) as u8]);
        }
        unsafe {
            self.internal.borrow_mut().client.notify_async()?;
        }
        tock::syscalls::yieldk_for(|| self.internal.borrow().reading_done);
        Ok(())
    }
}
