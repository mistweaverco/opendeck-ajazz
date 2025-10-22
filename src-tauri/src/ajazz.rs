use crate::events::outbound::{encoder, keypad};

use std::collections::HashMap;

use base64::Engine as _;
use ajazz_sdk::{
  asynchronous::AsyncAjazz, convert_image_with_format_async, Event, ImageRect, Kind
};
use once_cell::sync::Lazy;
use tokio::sync::RwLock;

static AJAZZ_DEVICES: Lazy<RwLock<HashMap<String, AsyncAjazz>>> = Lazy::new(|| RwLock::new(HashMap::new()));

pub async fn update_image(context: &crate::shared::Context, image: Option<&str>) -> Result<(), anyhow::Error> {
	if let Some(device) = AJAZZ_DEVICES.read().await.get(&context.device) {
		if let Some(image) = image {
			let data = image.split_once(',').unwrap().1;
			let bytes = base64::engine::general_purpose::STANDARD.decode(data)?;
			if context.controller == "Encoder" {
				device
					.write_lcd(
						(context.position as u16 * 200) + 64,
						14,
						&ImageRect::from_image_async(image::load_from_memory(&bytes)?.resize(72, 72, image::imageops::FilterType::Nearest))?,
					)
					.await?;
			} else {
				device.set_button_image(context.position, image::load_from_memory(&bytes)?).await?;
			}
		} else if context.controller == "Encoder" {
			device
				.write_lcd(context.position as u16 * 200, 0, &ImageRect::from_image_async(image::DynamicImage::new_rgb8(200, 100))?)
				.await?;
		} else {
			device.clear_button_image(context.position).await?;
		}
		device.flush().await?;
	}
	Ok(())
}

pub async fn clear_screen(id: &str) -> Result<(), anyhow::Error> {
	if let Some(device) = AJAZZ_DEVICES.read().await.get(id) {
		device.clear_all_button_images().await?;
		if device.kind() == Kind::Akp05 {
			device
				.write_lcd_fill(&convert_image_with_format_async(device.kind().lcd_image_format().unwrap(), image::DynamicImage::new_rgb8(800, 100))?)
				.await?;
		}
		device.flush().await?;
	}
	Ok(())
}

pub async fn set_brightness(brightness: u8) {
	for (_id, device) in AJAZZ_DEVICES.read().await.iter() {
		let _ = device.set_brightness(brightness.clamp(0, 100)).await;
		let _ = device.flush().await;
	}
}

pub async fn reset_devices() {
	for (_id, device) in AJAZZ_DEVICES.read().await.iter() {
		let _ = device.reset().await;
		let _ = device.flush().await;
	}
}

async fn init(device: AsyncAjazz, device_id: String) {
	if AJAZZ_DEVICES.read().await.contains_key(&device_id) {
		return;
	}

	let kind = device.kind();
	let device_type = match kind {
		Kind::Akp153 | Kind::Akp153E | Kind::Akp153R => 2,
		Kind::Akp815 => 2,
		Kind::Akp03 | Kind::Akp03E | Kind::Akp03R => 2,
		Kind::Akp03RRev2 => 2,
		Kind::Akp05 => 7,
	};
	let _ = device.clear_all_button_images().await;
	if let Ok(settings) = crate::store::get_settings() {
		let _ = device.set_brightness(settings.value.brightness).await;
	}
	let _ = device.flush().await;
	crate::events::inbound::devices::register_device(
		"",
		crate::events::inbound::PayloadEvent {
			payload: crate::shared::DeviceInfo {
				id: device_id.clone(),
				plugin: String::new(),
				name: device.product_name.to_owned(),
				rows: kind.row_count(),
				columns: kind.column_count(),
				encoders: kind.encoder_count(),
				r#type: device_type,
			},
		},
	)
	.await
	.unwrap();

	let reader = device.get_reader();
	AJAZZ_DEVICES.write().await.insert(device_id.clone(), device);
	loop {
		let updates = match reader.read(100.0).await {
			Ok(updates) => updates,
			Err(_) => break,
		};
		for update in updates {
			match match update {
				Event::ButtonDown(key) => keypad::key_down(&device_id, key).await,
				Event::ButtonUp(key) => keypad::key_up(&device_id, key).await,
				Event::EncoderTwist(dial, ticks) => encoder::dial_rotate(&device_id, dial, ticks.into()).await,
				Event::EncoderDown(dial) => encoder::dial_press(&device_id, "dialDown", dial).await,
				Event::EncoderUp(dial) => encoder::dial_press(&device_id, "dialUp", dial).await,
				_ => Ok(()),
			} {
				Ok(_) => (),
				Err(error) => log::warn!("Failed to process device event {update:?}: {error}"),
			}
		}
	}

	AJAZZ_DEVICES.write().await.remove(&device_id);
	crate::events::inbound::devices::deregister_device("", crate::events::inbound::PayloadEvent { payload: device_id })
		.await
		.unwrap();
}

/// Attempt to initialise all connected devices.
pub async fn initialise_devices() {
	if let Ok(settings) = crate::store::get_settings() {
		if settings.value.disabledevices {
			crate::plugins::DEVICE_NAMESPACES
				.write()
				.await
				.insert("sd".to_owned(), "opendeck_alternative_ajazz_implementation".to_owned());
			return;
		} else {
			crate::plugins::DEVICE_NAMESPACES.write().await.remove("sd");
		}
	}

	// Iterate through detected Ajazz devices and attempt to register them.
	match ajazz_sdk::new_hidapi() {
		Ok(hid) => {
			for (kind, serial) in ajazz_sdk::list_devices(&hid) {
				let device_id = format!("sd-{serial}");
				if AJAZZ_DEVICES.read().await.contains_key(&device_id) {
					continue;
				}
				match AsyncAjazz::connect(&hid, kind, &serial) {
					Ok(device) => {
						tokio::spawn(init(device, device_id));
					}
					Err(error) => log::warn!("Failed to connect to Ajazz device: {error}"),
				}
			}
		}
		Err(error) => log::warn!("Failed to initialise hidapi: {error}"),
	}
}
