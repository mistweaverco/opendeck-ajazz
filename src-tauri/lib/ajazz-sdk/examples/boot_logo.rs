use ajazz_sdk::{list_devices, new_hidapi, Ajazz};

fn main() {
    let mut args = std::env::args();
    if args.len() < 2 {
        eprintln!("Usage: {} <image_path>", args.next().unwrap());
        return;
    }
    let image_path = args.nth(1).expect("image path is required");

    println!("Connecting to device");

    let hid = new_hidapi().unwrap();

    let devices = list_devices(&hid);
    if devices.is_empty() {
        eprintln!("No devices found");
        return;
    }

    let (kind, serial) = devices.first().unwrap();
    let device = Ajazz::connect_with_retries(&hid, *kind, serial, 10).unwrap();

    println!("Setting boot logo image: {}", image_path);

    let image = image::open(image_path).unwrap();
    device.set_logo_image(image).unwrap();

    println!("Boot logo image updated");
}
