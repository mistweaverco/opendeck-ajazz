use std::str::{from_utf8, Utf8Error};

use crate::info::Kind;
use crate::protocol::codes;
use crate::{AjazzError, AjazzInput};

pub(crate) trait AjazzProtocolParser {
    fn parse_input(&self, data: &[u8]) -> Result<AjazzInput, AjazzError>;
    fn index_from_native_v1(&self, i: u8) -> Option<u8>;
    fn index_to_native_v1(&self, key: u8) -> Option<u8>;
    fn is_ack_ok(&self, data: &[u8]) -> bool;
}

/// Extracts string from byte array, removing \0 symbols
pub(crate) fn extract_string(bytes: &[u8]) -> Result<String, Utf8Error> {
    Ok(from_utf8(bytes)?.replace('\0', "").to_string())
}

impl AjazzProtocolParser for Kind {
    fn parse_input(&self, data: &[u8]) -> Result<AjazzInput, AjazzError> {
        if data[codes::OFFSET_DATA_LENGTH] == 0 {
            return Ok(AjazzInput::NoData);
        }

        let action_code = data[codes::OFFSET_ACTION_CODE];

        match self {
            kind if kind.is_v1_api() => {
                let mut states = vec![false; self.key_count() as usize];
                if action_code != codes::ACTION_CODE_NOP {
                    let raw_index = action_code - 1;
                    let Some(index) = kind.index_from_native_v1(raw_index) else {
                        return Err(AjazzError::BadData);
                    };
                    states[index as usize] = true;
                }

                Ok(AjazzInput::ButtonStateChange(states))
            }

            kind if kind.is_v2_api() => {
                if is_akp03_button_press(action_code) {
                    parse_akp03_button_press(action_code)
                } else if is_akp03_encoder_value(action_code) {
                    parse_akp03_encoder_value(action_code)
                } else if is_akp03_encoder_press(action_code) {
                    parse_akp03_encoder_press(action_code)
                } else {
                    println!("Bad data: {:?}", data);
                    Err(AjazzError::BadData)
                }
            }

            _ => Err(AjazzError::UnsupportedOperation),
        }
    }

    /// Converts Ajazz native key index to normalized key index
    fn index_from_native_v1(&self, i: u8) -> Option<u8> {
        if i >= self.key_count() || !self.is_v1_api() {
            return None;
        }

        match self {
            Kind::Akp153 | Kind::Akp153E | Kind::Akp153R => {
                if i < self.key_count() {
                    Some(
                        [4, 10, 16, 3, 9, 15, 2, 8, 14, 1, 7, 13, 0, 6, 12, 5, 11, 17]
                            [i as usize],
                    )
                } else {
                    Some(i)
                }
            }
            Kind::Akp815 => {
                if i < self.key_count() {
                    Some(self.key_count() - 1 - i)
                } else {
                    Some(i)
                }
            }
            _ => None,
        }
    }

    /// Converts normalized key index to Ajazz native key index
    fn index_to_native_v1(&self, key: u8) -> Option<u8> {
        if self.is_v1_api() {
            if key < self.key_count() {
                Some(
                    [12, 9, 6, 3, 0, 15, 13, 10, 7, 4, 1, 16, 14, 11, 8, 5, 2, 17]
                        [key as usize],
                )
            } else {
                Some(key)
            }
        } else {
            None
        }
    }

    fn is_ack_ok(&self, data: &[u8]) -> bool {
        data.starts_with(codes::RESPONSE_ACK_OK)
    }
}

fn parse_akp03_button_press(input: u8) -> Result<AjazzInput, AjazzError> {
    let mut button_states = vec![false; Kind::Akp03.key_count() as usize];
    if input == 0 {
        return Ok(AjazzInput::ButtonStateChange(button_states));
    }

    let pressed_index: usize = match input {
        // Six buttons with displays
        (1..=6) => input as usize,
        // Three buttons without displays
        codes::ACTION_CODE_BUTTON_7 => 7,
        codes::ACTION_CODE_BUTTON_8 => 8,
        codes::ACTION_CODE_BUTTON_9 => 9,
        _ => return Err(AjazzError::BadData),
    };
    button_states[pressed_index - 1] = true;

    Ok(AjazzInput::ButtonStateChange(button_states))
}

fn parse_akp03_encoder_value(input: u8) -> Result<AjazzInput, AjazzError> {
    let mut encoder_values = vec![0i8; Kind::Akp03.encoder_count() as usize];

    let (encoder, value): (usize, i8) = match input {
        // Left encoder
        codes::ACTION_CODE_ENCODER_0_CCW => (0, -1),
        codes::ACTION_CODE_ENCODER_0_CW => (0, 1),
        // Middle (top) encoder
        codes::ACTION_CODE_ENCODER_1_CCW => (1, -1),
        codes::ACTION_CODE_ENCODER_1_CW => (1, 1),
        // Right encoder
        codes::ACTION_CODE_ENCODER_2_CCW => (2, -1),
        codes::ACTION_CODE_ENCODER_2_CW => (2, 1),
        _ => return Err(AjazzError::BadData),
    };

    encoder_values[encoder] = value;
    Ok(AjazzInput::EncoderTwist(encoder_values))
}

fn parse_akp03_encoder_press(input: u8) -> Result<AjazzInput, AjazzError> {
    let mut encoder_states = vec![false; Kind::Akp03.encoder_count() as usize];

    let encoder: usize = match input {
        codes::ACTION_CODE_ENCODER_0_PRESS => 0,
        codes::ACTION_CODE_ENCODER_1_PRESS => 1,
        codes::ACTION_CODE_ENCODER_2_PRESS => 2,
        _ => return Err(AjazzError::BadData),
    };

    encoder_states[encoder] = true;
    Ok(AjazzInput::EncoderStateChange(encoder_states))
}

fn is_akp03_encoder_value(input: u8) -> bool {
    matches!(
        input,
        codes::ACTION_CODE_ENCODER_0_CCW
            | codes::ACTION_CODE_ENCODER_0_CW
            | codes::ACTION_CODE_ENCODER_1_CCW
            | codes::ACTION_CODE_ENCODER_1_CW
            | codes::ACTION_CODE_ENCODER_2_CCW
            | codes::ACTION_CODE_ENCODER_2_CW
    )
}

fn is_akp03_encoder_press(input: u8) -> bool {
    matches!(
        input,
        codes::ACTION_CODE_ENCODER_0_PRESS
            | codes::ACTION_CODE_ENCODER_1_PRESS
            | codes::ACTION_CODE_ENCODER_2_PRESS
    )
}

fn is_akp03_button_press(input: u8) -> bool {
    matches!(
        input,
        // Six buttons with displays
        1..=6 |
        // Three buttons without displays
        codes::ACTION_CODE_BUTTON_7
            | codes::ACTION_CODE_BUTTON_8
            | codes::ACTION_CODE_BUTTON_9
    )
}
