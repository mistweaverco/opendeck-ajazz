//! Code from this module is using [block_in_place](tokio::task::block_in_place),
//! and so they cannot be used in [current_thread](tokio::runtime::Builder::new_current_thread) runtimes

use std::sync::Arc;
use std::time::Duration;

use hidapi::{HidApi, HidResult};
use image::DynamicImage;
use tokio::sync::Mutex;
use tokio::task::block_in_place;
use tokio::time::sleep;

use crate::{AjazzError, AjazzInput, DeviceState, Event, Kind};
use crate::device::{handle_input_state_change, Ajazz};
use crate::hid::list_devices;
use crate::images::{convert_image_async, ImageRect};
use crate::info::get_product_name;

/// Actually refreshes the device list, can be safely ran inside [multi_thread](tokio::runtime::Builder::new_multi_thread) runtime
pub fn refresh_device_list_async(hidapi: &mut HidApi) -> HidResult<()> {
    block_in_place(move || hidapi.refresh_devices())
}

/// Returns a list of devices as (Kind, Serial Number) that could be found using HidApi,
/// can be safely ran inside [multi_thread](tokio::runtime::Builder::new_multi_thread) runtime
///
/// **WARNING:** To refresh the list, use [refresh_device_list]
pub fn list_devices_async(hidapi: &HidApi) -> Vec<(Kind, String)> {
    block_in_place(move || list_devices(hidapi))
}

/// Ajazz device interface suitable to be used in async, uses [block_in_place](block_in_place)
/// so this wrapper cannot be used in [current_thread](tokio::runtime::Builder::new_current_thread) runtimes
#[derive(Clone)]
pub struct AsyncAjazz {
    kind: Kind,
    pub product_name: String,
    device: Arc<Mutex<Ajazz>>,
}

/// Static functions of the struct
impl AsyncAjazz {
    /// Attempts to connect to the device, can be safely ran inside [multi_thread](tokio::runtime::Builder::new_multi_thread) runtime
    pub fn connect(
        hidapi: &HidApi,
        kind: Kind,
        serial: &str,
    ) -> Result<AsyncAjazz, AjazzError> {
        let device = block_in_place(move || Ajazz::connect(hidapi, kind, serial))?;
        let product_name = block_in_place(move || get_product_name(&kind));

        Ok(AsyncAjazz {
            kind,
            product_name,
            device: Arc::new(Mutex::new(device)),
        })
    }

    /// Attempts to connect to the device, can be safely ran inside [multi_thread](tokio::runtime::Builder::new_multi_thread) runtime
    pub fn connect_with_retries(
        hidapi: &HidApi,
        kind: Kind,
        serial: &str,
        attempts: u8,
    ) -> Result<AsyncAjazz, AjazzError> {
        let device = block_in_place(move || {
            Ajazz::connect_with_retries(hidapi, kind, serial, attempts)
        })?;
        let product_name = block_in_place(move || get_product_name(&kind));

        Ok(AsyncAjazz {
            kind,
            product_name,
            device: Arc::new(Mutex::new(device)),
        })
    }
}

/// Instance methods of the struct
impl AsyncAjazz {
    /// Returns kind of the Stream Deck
    pub fn kind(&self) -> Kind {
        self.kind
    }

    /// Returns manufacturer string of the device
    pub async fn manufacturer(&self) -> Result<String, AjazzError> {
        let device = self.device.lock().await;
        block_in_place(move || device.manufacturer())
    }

    /// Returns product string of the device
    pub async fn product(&self) -> Result<String, AjazzError> {
        let device = self.device.lock().await;
        block_in_place(move || device.product())
    }

    /// Returns serial number of the device
    pub async fn serial_number(&self) -> Result<String, AjazzError> {
        let device = self.device.lock().await;
        block_in_place(move || device.serial_number())
    }

    /// Returns firmware version of the StreamDeck
    pub async fn firmware_version(&self) -> Result<String, AjazzError> {
        let device = self.device.lock().await;
        block_in_place(move || device.firmware_version())
    }

    /// Reads button states, awaits until there's data.
    /// Poll rate determines how often button state gets checked
    pub async fn read_input(&self, poll_rate: f32) -> Result<AjazzInput, AjazzError> {
        loop {
            let device = self.device.lock().await;
            let data = block_in_place(move || device.read_input(None))?;

            if !data.is_empty() {
                return Ok(data);
            }

            sleep(Duration::from_secs_f32(1.0 / poll_rate)).await;
        }
    }

    /// Resets the device
    pub async fn reset(&self) -> Result<(), AjazzError> {
        let device = self.device.lock().await;
        block_in_place(move || device.reset())
    }

    /// Sets brightness of the device, value range is 0 - 100
    pub async fn set_brightness(&self, percent: u8) -> Result<(), AjazzError> {
        let device = self.device.lock().await;
        block_in_place(move || device.set_brightness(percent))
    }

    /// Sets button's image to blank, changes must be flushed with `.flush()` before
    /// they will appear on the device!
    pub async fn clear_button_image(&self, key: u8) -> Result<(), AjazzError> {
        let device = self.device.lock().await;
        block_in_place(move || device.clear_button_image(key))
    }

    /// Sets blank images to every button, changes must be flushed with `.flush()` before
    /// they will appear on the device!
    pub async fn clear_all_button_images(&self) -> Result<(), AjazzError> {
        let device = self.device.lock().await;
        block_in_place(move || device.clear_all_button_images())
    }

    /// Sets specified button's image, changes must be flushed with `.flush()` before
    /// they will appear on the device!
    pub async fn set_button_image(
        &self,
        key: u8,
        image: DynamicImage,
    ) -> Result<(), AjazzError> {
        let image = convert_image_async(self.kind, image)?;
        let device = self.device.lock().await;
        block_in_place(move || device.set_button_image_data(key, &image))
    }

    /// Sets specified button's image, changes must be flushed with `.flush()` before
    /// they will appear on the device!
    pub async fn set_button_image_data(
        &self,
        key: u8,
        image_data: &[u8],
    ) -> Result<(), AjazzError> {
        let device = self.device.lock().await;
        block_in_place(move || device.set_button_image_data(key, image_data))
    }

    /// Set logo image
    pub async fn set_logo_image(&self, image: DynamicImage) -> Result<(), AjazzError> {
        let device = self.device.lock().await;
        block_in_place(move || device.set_logo_image(image))
    }

    /// Writes image data to Stream Deck device's lcd strip/screen as region.
    /// Only Stream Deck Plus supports writing LCD regions, for Stream Deck Neo use write_lcd_fill
    pub async fn write_lcd(&self, x: u16, y: u16, rect: &ImageRect) -> Result<(), AjazzError> {
        let device = self.device.lock().await;
        block_in_place(move || device.write_lcd(x, y, rect))
    }

    /// Writes image data to Stream Deck device's lcd strip/screen as full fill
    ///
    /// You can convert your images into proper image_data like this:
    /// ```
    /// use elgato_streamdeck::images::{convert_image_with_format_async};
    /// let image_data = convert_image_with_format_async(device.kind().lcd_image_format(), image).await.unwrap();
    /// device.write_lcd_fill(&image_data).await;
    /// ```
    pub async fn write_lcd_fill(&self, image_data: &[u8]) -> Result<(), AjazzError> {
        let device = self.device.lock().await;
        block_in_place(move || device.write_lcd_fill(image_data))
    }


    /// Sleeps the device
    pub async fn sleep(&self) -> Result<(), AjazzError> {
        let device = self.device.lock().await;
        block_in_place(move || device.sleep())
    }

    /// Make periodic events to the device, to keep it alive
    pub async fn keep_alive(&self) -> Result<(), AjazzError> {
        let device = self.device.lock().await;
        block_in_place(move || device.keep_alive())
    }

    /// Shutdown the device
    pub async fn shutdown(&self) -> Result<(), AjazzError> {
        let device = self.device.lock().await;
        block_in_place(move || device.shutdown())
    }

    /// Flushes the button's image to the device
    pub async fn flush(&self) -> Result<(), AjazzError> {
        let device = self.device.lock().await;
        block_in_place(move || device.flush())
    }

    /// Returns button state reader for this device
    pub fn get_reader(&self) -> Arc<AsyncDeviceStateReader> {
        Arc::new(AsyncDeviceStateReader {
            device: self.clone(),
            states: Mutex::new(DeviceState {
                buttons: vec![false; self.kind.key_count() as usize],
                encoders: vec![false; self.kind.encoder_count() as usize],
            }),
        })
    }
}

/// Button reader that keeps state of the Stream Deck and returns events instead of full states
pub struct AsyncDeviceStateReader {
    device: AsyncAjazz,
    states: Mutex<DeviceState>,
}

impl AsyncDeviceStateReader {
    /// Reads states and returns updates
    pub async fn read(&self, poll_rate: f32) -> Result<Vec<Event>, AjazzError> {
        let input = self.device.read_input(poll_rate).await?;
        let mut current_state = self.states.lock().await;

        let updates = handle_input_state_change(input, &mut current_state)?;
        Ok(updates)
    }
}
