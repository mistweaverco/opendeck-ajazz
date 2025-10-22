use once_cell::sync::Lazy;

use crate::info::Kind;

use super::{codes, AjazzProtocolParser};

fn format_request(cmd: &[u8]) -> Vec<u8> {
    let mut buf = vec![];
    buf.extend(codes::REQUEST_HEADER);
    buf.extend(cmd);
    buf
}

/// Request for keep alive command
static REQUEST_KEEP_ALIVE: Lazy<Vec<u8>> =
    Lazy::new(|| format_request(codes::REQUEST_CMD_KEEP_ALIVE));

/// Request for initialize command
static REQUEST_INITIALIZE: Lazy<Vec<u8>> =
    Lazy::new(|| format_request(codes::REQUEST_CMD_DIS));

/// Request for brightness command
static REQUEST_BRIGHTNESS: Lazy<Vec<u8>> =
    Lazy::new(|| format_request(codes::REQUEST_CMD_LIG));

/// Request for sleep command
static REQUEST_SLEEP: Lazy<Vec<u8>> = Lazy::new(|| format_request(codes::REQUEST_CMD_SLEEP));

/// Request for shutdown command
static REQUEST_SHUTDOWN: Lazy<Vec<u8>> =
    Lazy::new(|| format_request(codes::REQUEST_CMD_SHUTDOWN));

/// Request for clear button image command
static REQUEST_CLEAR_BUTTON_IMAGE: Lazy<Vec<u8>> =
    Lazy::new(|| format_request(codes::REQUEST_CMD_CLEAR_BUTTON_IMAGE));

/// Request for flush command
static REQUEST_FLUSH: Lazy<Vec<u8>> = Lazy::new(|| format_request(codes::REQUEST_CMD_FLUSH));

/// Request for image announce packet
static REQUEST_IMAGE_ANNOUNCE: Lazy<Vec<u8>> =
    Lazy::new(|| format_request(codes::REQUEST_CMD_IMAGE_ANNOUNCE));

/// Request for logo image command
static REQUEST_LOGO_IMAGE_V1: Lazy<Vec<u8>> =
    Lazy::new(|| format_request(codes::REQUEST_CMD_LOGO_IMAGE_V1));

/// Request for logo image command
static REQUEST_LOGO_IMAGE_V2: Lazy<Vec<u8>> =
    Lazy::new(|| format_request(codes::REQUEST_CMD_LOGO_IMAGE_V2));

pub(crate) static FEATURE_REPORT_VERSION: Lazy<Vec<u8>> = Lazy::new(|| {
    let mut buff = vec![0x00; 20];
    buff.insert(0, codes::FEATURE_REPORT_ID_VERSION);
    buff
});

pub(crate) trait AjazzRequestBuilder {
    fn brightness_packet(&self, percent: u8) -> Vec<u8>;
    fn keep_alive_packet(&self) -> Vec<u8>;
    fn initialize_packet(&self) -> Vec<u8>;
    fn sleep_packet(&self) -> Vec<u8>;
    fn shutdown_packet(&self) -> Vec<u8>;
    fn clear_button_image_packet(&self, key: u8) -> Vec<u8>;
    fn flush_packet(&self) -> Vec<u8>;

    fn image_announce_packet(&self, index: u8, image_data: &[u8]) -> Vec<u8>;
    fn key_image_announce_packet(&self, key: u8, image_data: &[u8]) -> Vec<u8>;

    fn logo_image_packet(&self, image_data: &[u8]) -> Vec<u8>;
}

impl Kind {
    fn packet_length(&self) -> usize {
        if self.is_v2_api() {
            1024
        } else {
            512
        }
    }

    /// Extends buffer up to required packet length
    pub fn pad_packet(&self, buf: &mut Vec<u8>) {
        let length = self.packet_length() + 1;

        buf.extend(vec![0x00; length - buf.len()]);
    }
}

impl AjazzRequestBuilder for Kind {
    fn brightness_packet(&self, percent: u8) -> Vec<u8> {
        let mut buf = REQUEST_BRIGHTNESS.clone();
        buf.push(percent);

        self.pad_packet(&mut buf);
        buf
    }

    fn keep_alive_packet(&self) -> Vec<u8> {
        let mut buf = REQUEST_KEEP_ALIVE.clone();
        self.pad_packet(&mut buf);
        buf
    }

    fn initialize_packet(&self) -> Vec<u8> {
        let mut buf = REQUEST_INITIALIZE.clone();
        self.pad_packet(&mut buf);
        buf
    }

    fn sleep_packet(&self) -> Vec<u8> {
        let mut buf = REQUEST_SLEEP.clone();
        self.pad_packet(&mut buf);
        buf
    }

    fn shutdown_packet(&self) -> Vec<u8> {
        let mut buf = REQUEST_SHUTDOWN.clone();
        self.pad_packet(&mut buf);
        buf
    }

    fn clear_button_image_packet(&self, key: u8) -> Vec<u8> {
        let key = self.index_from_native_v1(key).unwrap_or(key);
        let key = if key == 0xff { 0xff } else { key + 1 };

        let mut buf = REQUEST_CLEAR_BUTTON_IMAGE.clone();
        buf.push(key);
        self.pad_packet(&mut buf);
        buf
    }

    fn flush_packet(&self) -> Vec<u8> {
        let mut buf = REQUEST_FLUSH.clone();
        self.pad_packet(&mut buf);
        buf
    }

    fn image_announce_packet(&self, index: u8, image_data: &[u8]) -> Vec<u8> {
        let mut buf = REQUEST_IMAGE_ANNOUNCE.clone();
        buf.push((image_data.len() >> 8) as u8);
        buf.push(image_data.len() as u8);
        buf.push(index);
        self.pad_packet(&mut buf);
        buf
    }

    fn key_image_announce_packet(&self, key: u8, image_data: &[u8]) -> Vec<u8> {
        let index = self.index_to_native_v1(key).unwrap_or(key);
        self.image_announce_packet(index + 1, image_data)
    }

    fn logo_image_packet(&self, image_data: &[u8]) -> Vec<u8> {
        let mut buf = if self.is_v2_api() {
            let mut buf = REQUEST_LOGO_IMAGE_V2.clone();
            buf.push((image_data.len() >> 8) as u8);
            buf.push(image_data.len() as u8);
            buf
        } else {
            REQUEST_LOGO_IMAGE_V1.clone()
        };
        self.pad_packet(&mut buf);
        buf
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Pads packet up to required packet length.
    /// Function is used to generate expected packet for testing.
    fn padded_packet(kind: Kind, buf: Vec<u8>) -> Vec<u8> {
        let length = if kind.is_v2_api() { 1025 } else { 513 };
        let mut buf = buf;
        buf.extend(vec![0u8; length - buf.len()]);
        buf
    }

    #[test]
    fn test_brightness_packet() {
        let kind = Kind::Akp153;
        let brightness = 50;
        let packet = kind.brightness_packet(brightness);
        let expected = padded_packet(
            kind,
            vec![
                0x00, 0x43, 0x52, 0x54, 0x00, 0x00, 0x4c, 0x49, 0x47, 0x00, 0x00, brightness,
            ],
        );

        assert_eq!(packet, expected);
    }

    #[test]
    fn test_keep_alive_packet() {
        let kind = Kind::Akp153;
        let packet = kind.keep_alive_packet();
        let expected = padded_packet(
            kind,
            vec![
                0x00, 0x43, 0x52, 0x54, 0x00, 0x00, 0x43, 0x4F, 0x4E, 0x4E, 0x45, 0x43, 0x54,
            ],
        );
        assert_eq!(packet, expected);
    }

    #[test]
    fn test_initialize_packet() {
        let kind = Kind::Akp153;
        let packet = kind.initialize_packet();
        let expected = padded_packet(
            kind,
            vec![0x00, 0x43, 0x52, 0x54, 0x00, 0x00, 0x44, 0x49, 0x53],
        );

        assert_eq!(packet, expected);
    }

    #[test]
    fn test_sleep_packet() {
        let kind = Kind::Akp153;
        let packet = kind.sleep_packet();
        let expected = padded_packet(
            kind,
            vec![0x00, 0x43, 0x52, 0x54, 0x00, 0x00, 0x48, 0x41, 0x4E],
        );
        assert_eq!(packet, expected);
    }

    #[test]
    fn test_shutdown_packet() {
        let kind = Kind::Akp153;
        let packet = kind.shutdown_packet();
        let expected = padded_packet(
            kind,
            vec![
                0x00, 0x43, 0x52, 0x54, 0x00, 0x00, 0x43, 0x4c, 0x45, 0x00, 0x00, 0x44, 0x43,
            ],
        );
        assert_eq!(packet, expected);
    }

    #[test]
    fn test_clear_button_image_packet() {
        let kind = Kind::Akp153;

        fn assert_clear_packet(kind: Kind, key: u8, expected: Vec<u8>) {
            let packet = kind.clear_button_image_packet(key);
            let expected = padded_packet(kind, expected);
            assert_eq!(packet, expected);
        }

        assert_clear_packet(
            kind,
            0,
            vec![
                0x00, 0x43, 0x52, 0x54, 0x00, 0x00, 0x43, 0x4c, 0x45, 0x00, 0x00, 0x00, 0x05,
            ],
        );
        assert_clear_packet(
            kind,
            1,
            vec![
                0x00, 0x43, 0x52, 0x54, 0x00, 0x00, 0x43, 0x4c, 0x45, 0x00, 0x00, 0x00, 0x0B,
            ],
        );
        assert_clear_packet(
            kind,
            0xff,
            vec![
                0x00, 0x43, 0x52, 0x54, 0x00, 0x00, 0x43, 0x4c, 0x45, 0x00, 0x00, 0x00, 0xff,
            ],
        );
    }

    #[test]
    fn test_flush_packet() {
        let kind = Kind::Akp153;
        let packet = kind.flush_packet();
        let expected = padded_packet(
            kind,
            vec![0x00, 0x43, 0x52, 0x54, 0x00, 0x00, 0x53, 0x54, 0x50],
        );
        assert_eq!(packet, expected);
    }

    #[test]
    fn test_image_announce_packet() {
        let kind = Kind::Akp03RRev2;
        let packet = kind.image_announce_packet(0, &[0x00, 0x01]);
        let expected = padded_packet(
            kind,
            vec![
                0x00, 0x43, 0x52, 0x54, 0x00, 0x00, 0x42, 0x41, 0x54, 0x00, 0x00, 0x00, 0x02,
                0x00, 0x00, 0x00,
            ],
        );
        assert_eq!(packet, expected);
    }

    #[test]
    fn test_logo_image_packet() {
        let kind = Kind::Akp153;
        let packet = kind.logo_image_packet(&[0x00, 0x01]);
        let expected = padded_packet(
            kind,
            vec![
                0x00, 0x43, 0x52, 0x54, 0x00, 0x00, 0x4c, 0x4f, 0x47, 0x00, 0x12, 0xc3, 0xc0,
                0x01,
            ],
        );
        assert_eq!(packet, expected);
    }

    // TODO: Add test for apply logo image packet
}
