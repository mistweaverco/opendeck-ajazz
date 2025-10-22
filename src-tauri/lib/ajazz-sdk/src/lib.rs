//! Ajazz library
//!
//! Library for interacting with Ajazz devices through [hidapi](https://crates.io/crates/hidapi).

#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]

use std::str::Utf8Error;

use hidapi::HidError;
use image::ImageError;
use thiserror::Error;

mod info;
mod images;
mod device;
mod protocol;
mod hid;

pub use info::Kind;
pub use device::{Ajazz, DeviceStateReader};
pub use images::{
    convert_image, convert_image_with_format, ImageFormat, ImageMode, ImageMirroring,
    ImageRect, ImageRotation,
};
pub use hid::{new_hidapi, refresh_device_list, list_devices};

/// Async Ajazz
#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
pub mod asynchronous;
#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
pub use asynchronous::AsyncAjazz;
#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
pub use images::{convert_image_async, convert_image_with_format_async};

/// Errors that can occur while working with Ajazz devices
#[derive(Debug, Error)]
pub enum AjazzError {
    /// HidApi error
    #[error("HidApi error: {0}")]
    HidError(#[from] HidError),

    /// Failed to convert bytes into string
    #[error("Failed to convert bytes into string: {0}")]
    Utf8Error(#[from] Utf8Error),

    /// Failed to encode image
    #[error("Failed to encode image: {0}")]
    ImageError(#[from] ImageError),

    /// Tokio join error
    #[cfg(feature = "async")]
    #[cfg_attr(docsrs, doc(cfg(feature = "async")))]
    #[error("Tokio join error: {0}")]
    JoinError(#[from] tokio::task::JoinError),

    /// Reader mutex was poisoned
    #[error("Reader mutex was poisoned")]
    PoisonError,

    /// Key index is invalid
    #[error("Key index is invalid: {0}")]
    InvalidKeyIndex(u8),

    /// Unrecognized Product ID
    #[error("Unrecognized Product ID: {0}")]
    UnrecognizedPID(u16),

    /// The device doesn't support doing that
    #[error("The device doesn't support doing that")]
    UnsupportedOperation,

    /// Device sent unexpected data
    #[error("Device sent unexpected data")]
    BadData,

    /// Invalid image size
    #[error("Invalid image size: {0}x{1}, expected {2}x{3}")]
    InvalidImageSize(usize, usize, usize, usize),

    /// Device didn't respond with ACK
    #[error("Device didn't respond with ACK")]
    NoAck,
}

/// Type of input that the device produced
#[derive(Clone, Debug)]
pub enum AjazzInput {
    /// No data was passed from the device
    NoData,

    /// Button was pressed
    ButtonStateChange(Vec<bool>),

    /// Encoder/Knob was pressed
    EncoderStateChange(Vec<bool>),

    /// Encoder/Knob was twisted/turned
    EncoderTwist(Vec<i8>),
}

impl AjazzInput {
    /// Checks if there's data received or not
    pub fn is_empty(&self) -> bool {
        matches!(self, AjazzInput::NoData)
    }
}

/// Tells what changed in button states
#[derive(Copy, Clone, Debug, Hash)]
pub enum Event {
    /// Button got pressed down
    ButtonDown(u8),

    /// Button got released
    ButtonUp(u8),

    /// Encoder got pressed down
    EncoderDown(u8),

    /// Encoder was released from being pressed down
    EncoderUp(u8),

    /// Encoder was twisted
    EncoderTwist(u8, i8),
}

#[derive(Default)]
struct DeviceState {
    pub buttons: Vec<bool>,
    pub encoders: Vec<bool>,
}
