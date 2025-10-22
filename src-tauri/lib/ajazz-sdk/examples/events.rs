use std::sync::Arc;
use std::time::Duration;

use ajazz_sdk::{list_devices, new_hidapi, Ajazz, Event};

#[allow(clippy::arc_with_non_send_sync)]
fn main() {
    let hid = match new_hidapi() {
        Ok(hid) => hid,
        Err(e) => {
            eprintln!("Failed to create HidApi instance: {}", e);
            return;
        }
    };

    let devices = list_devices(&hid);
    let (kind, serial) = devices.first().unwrap();

    let Ok(device) = Ajazz::connect_with_retries(&hid, *kind, serial, 10) else {
        println!("Failed to connect");
        return;
    };
    // Print out some info from the device
    println!(
        "Connected to '{}' with version '{}'",
        device.serial_number().unwrap(),
        device.firmware_version().unwrap()
    );

    let device = Arc::new(device);
    let reader = device.get_reader();

    device.set_brightness(50).unwrap();
    device.clear_all_button_images().unwrap();

    loop {
        let updates = match reader.read(Some(Duration::from_secs_f64(100.0))) {
            Ok(updates) => updates,
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
        };
        for update in updates {
            match update {
                Event::ButtonDown(button) => {
                    println!("Button {} down", button);
                }
                Event::ButtonUp(button) => {
                    println!("Button {} up", button);
                }
                Event::EncoderTwist(dial, ticks) => {
                    println!("Dial {} twisted by {}", dial, ticks);
                }
                Event::EncoderDown(dial) => {
                    println!("Dial {} down", dial);
                }
                Event::EncoderUp(dial) => {
                    println!("Dial {} up", dial);
                }
            }
        }
    }

    device.shutdown().ok();
}
