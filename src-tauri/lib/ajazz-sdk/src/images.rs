use image::{ColorType, DynamicImage, GenericImageView, ImageError};
use image::codecs::jpeg::JpegEncoder;
use image::imageops::FilterType;

use crate::{Kind, AjazzError};

/// Image rotation
#[derive(Copy, Clone, Debug, Hash)]
pub enum ImageRotation {
    /// No rotation
    Rot0,
    /// 90 degrees clockwise
    Rot90,
    /// 180 degrees
    Rot180,
    /// 90 degrees counter-clockwise
    Rot270,
}

/// Image mirroring
#[derive(Copy, Clone, Debug, Hash)]
pub enum ImageMirroring {
    /// No image mirroring
    None,
    /// Flip by X
    X,
    /// Flip by Y
    Y,
    /// Flip by both axes
    Both,
}

/// Image format
#[derive(Copy, Clone, Debug, Hash)]
pub enum ImageMode {
    /// No image
    None,
    /// Jpeg image
    JPEG,
}

/// Image format used by the device
#[derive(Copy, Clone, Debug, Hash)]
pub struct ImageFormat {
    /// Image format/mode
    pub mode: ImageMode,
    /// Image size
    pub size: (usize, usize),
    /// Image rotation
    pub rotation: ImageRotation,
    /// Image mirroring
    pub mirror: ImageMirroring,
}

impl Default for ImageFormat {
    fn default() -> Self {
        Self {
            mode: ImageMode::None,
            size: (0, 0),
            rotation: ImageRotation::Rot0,
            mirror: ImageMirroring::None,
        }
    }
}

/// Converts image into image data depending on provided kind of device
pub fn convert_image(kind: Kind, image: DynamicImage) -> Result<Vec<u8>, ImageError> {
    convert_image_with_format(kind.key_image_format(), image)
}

/// Converts image into image data depending on provided image format
pub fn convert_image_with_format(
    image_format: ImageFormat,
    image: DynamicImage,
) -> Result<Vec<u8>, ImageError> {
    // Ensuring size of the image
    let (ws, hs) = image_format.size;

    // Applying rotation
    let image = match image_format.rotation {
        ImageRotation::Rot0 => image,
        ImageRotation::Rot90 => image.rotate90(),
        ImageRotation::Rot180 => image.rotate180(),
        ImageRotation::Rot270 => image.rotate270(),
    };

    let image = image.resize_exact(ws as u32, hs as u32, FilterType::Triangle);

    // Applying mirroring
    let image = match image_format.mirror {
        ImageMirroring::None => image,
        ImageMirroring::X => image.fliph(),
        ImageMirroring::Y => image.flipv(),
        ImageMirroring::Both => image.fliph().flipv(),
    };

    let image_data = image.into_rgb8().to_vec();

    // Encoding image
    match image_format.mode {
        ImageMode::None => Ok(vec![]),
        ImageMode::JPEG => {
            let mut buf = Vec::new();
            let mut encoder = JpegEncoder::new_with_quality(&mut buf, 90);
            encoder.encode(&image_data, ws as u32, hs as u32, ColorType::Rgb8.into())?;
            Ok(buf)
        }
    }
}

/// Converts image into image data depending on provided kind of device, can be safely ran inside [multi_thread](tokio::runtime::Builder::new_multi_thread) runtime
#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
pub fn convert_image_async(kind: Kind, image: DynamicImage) -> Result<Vec<u8>, AjazzError> {
    Ok(tokio::task::block_in_place(move || {
        convert_image(kind, image)
    })?)
}

/// Converts image into image data depending on provided image format, can be safely ran inside [multi_thread](tokio::runtime::Builder::new_multi_thread) runtime
#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
pub fn convert_image_with_format_async(
    format: ImageFormat,
    image: DynamicImage,
) -> Result<Vec<u8>, AjazzError> {
    Ok(tokio::task::block_in_place(move || {
        convert_image_with_format(format, image)
    })?)
}

/// Rect to be used when trying to send image to lcd screen
pub struct ImageRect {
    /// Width of the image
    pub w: u16,

    /// Height of the image
    pub h: u16,

    /// Data of the image row by row as RGB
    pub data: Vec<u8>,
}

impl ImageRect {
    /// Converts image to image rect
    pub fn from_image(image: DynamicImage) -> Result<ImageRect, AjazzError> {
        let (image_w, image_h) = image.dimensions();

        let image_data = image.into_rgb8().to_vec();

        let mut buf = Vec::new();
        let mut encoder = JpegEncoder::new_with_quality(&mut buf, 90);
        encoder.encode(&image_data, image_w, image_h, ColorType::Rgb8.into())?;

        Ok(ImageRect {
            w: image_w as u16,
            h: image_h as u16,
            data: buf,
        })
    }

    /// Converts image to image rect, can be safely ran inside [multi_thread](tokio::runtime::Builder::new_multi_thread) runtime
    #[cfg(feature = "async")]
    #[cfg_attr(docsrs, doc(cfg(feature = "async")))]
    pub fn from_image_async(image: DynamicImage) -> Result<ImageRect, AjazzError> {
        tokio::task::block_in_place(move || ImageRect::from_image(image))
    }
}

#[derive(Clone, Copy)]
pub(crate) struct WriteImageParameters {
    pub image_report_length: usize,
    pub image_report_payload_length: usize,
}

impl WriteImageParameters {
    pub fn for_kind(kind: Kind) -> Self {
        let image_report_length = match kind {
            kind if kind.is_v1_api() => 513,
            kind if kind.is_v2_api() => 1025,
            _ => 1024,
        };

        let image_report_header_length = 1;
        let image_report_payload_length = image_report_length - image_report_header_length;

        Self {
            image_report_length,
            image_report_payload_length,
        }
    }
}
