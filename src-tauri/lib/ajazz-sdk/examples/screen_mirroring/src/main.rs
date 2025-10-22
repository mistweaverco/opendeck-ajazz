use image::{DynamicImage, ImageBuffer, Rgb};
use rustyline::Editor;

use ajazz_sdk::{list_devices, new_hidapi, AjazzError, Ajazz};
use scap::Target;

mod layout;

fn select_display() -> Target {
    let targets = scap::get_all_targets();
    let displays: Vec<&Target> = targets
        .iter()
        .filter(|target| matches!(target, Target::Display(_)))
        .collect();

    if displays.is_empty() {
        panic!("No displays found for capture");
    }

    if displays.len() == 1 {
        println!("Found one display, using it automatically");
        return displays[0].clone();
    }

    println!("Available displays:");
    for (i, target) in displays.iter().enumerate() {
        if let Target::Display(display) = target {
            println!("{}. Display: {}", i + 1, display.title);
        }
    }

    let mut rl = Editor::<(), rustyline::history::FileHistory>::new().unwrap();

    loop {
        let readline = rl.readline("Select display number (1-{}): ");
        match readline {
            Ok(line) => {
                if let Ok(choice) = line.trim().parse::<usize>() {
                    if choice >= 1 && choice <= displays.len() {
                        return displays[choice - 1].clone();
                    } else {
                        println!(
                            "Invalid choice. Enter a number from 1 to {}",
                            displays.len()
                        );
                    }
                } else {
                    println!("Invalid input. Enter a number.");
                }
            }
            Err(_) => {
                println!("Input error. Exiting...");
                std::process::exit(1);
            }
        }
    }
}

fn render_buttons(device: &Ajazz, frame: &scap::frame::RGBFrame) -> Result<(), AjazzError> {
    let image_buffer = ImageBuffer::<Rgb<u8>, _>::from_raw(
        frame.width as u32,
        frame.height as u32,
        frame.data.clone(),
    )
    .unwrap();

    let dyn_image = DynamicImage::ImageRgb8(image_buffer);

    let (button_size, button_rects) =
        layout::calculate_button_rect(frame.width as u32, frame.height as u32);

    for (i, rect) in button_rects.iter().enumerate() {
        if rect.x + button_size > dyn_image.width() {
            panic!(
                "Invalid width: {} + {} > {}",
                rect.x,
                button_size,
                dyn_image.width()
            );
        }

        if rect.y + button_size > dyn_image.height() {
            panic!(
                "Invalid height: {} + {} > {}",
                rect.y,
                button_size,
                dyn_image.height()
            );
        }

        let button_image = dyn_image.crop_imm(rect.x, rect.y, button_size, button_size);

        device.set_button_image(i as u8, button_image)?;
    }

    device.flush()?;

    Ok(())
}

fn main() {
    // Connect to device
    let hid = match new_hidapi() {
        Ok(hid) => hid,
        Err(e) => {
            panic!("Failed to create HidApi instance: {}", e);
        }
    };

    let devices = list_devices(&hid);
    if devices.is_empty() {
        panic!("No devices found");
    }

    let (kind, serial) = devices.first().unwrap();
    let device = Ajazz::connect_with_retries(&hid, *kind, serial, 5).unwrap();

    // Check if we have permission to capture screen
    // If we don't, request it.
    if !scap::has_permission() {
        println!("❌ Permission not granted. Requesting permission...");
        if !scap::request_permission() {
            println!("❌ Permission denied");
            return;
        }
    }

    let display = select_display();

    let options = scap::capturer::Options {
        fps: 10,
        show_cursor: true,
        show_highlight: true,
        target: Some(display.clone()),
        excluded_targets: None,
        output_type: scap::frame::FrameType::RGB,
        output_resolution: scap::capturer::Resolution::_480p,
        crop_area: None,
    };

    let mut recorder = scap::capturer::Capturer::build(options).unwrap_or_else(|err| {
        println!("Problem with building Capturer: {err}");
        std::process::exit(1);
    });

    // Start Capture
    recorder.start_capture();
    device.clear_all_button_images().unwrap();

    loop {
        let frame = recorder.get_next_frame().expect("Error");

        match frame {
            scap::frame::Frame::RGB(frame) => {
                render_buttons(&device, &frame).unwrap();
            }
            _ => {
                println!("Unknown frame type");
            }
        }
    }
}
