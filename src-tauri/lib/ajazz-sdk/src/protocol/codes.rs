/// Feature report ID for firmware version
pub const FEATURE_REPORT_ID_VERSION: u8 = 0x01;

/// A Mirabox v1 vendor ID
pub const VENDOR_ID_MIRABOX_V1: u16 = 0x5548;
/// A Mirabox v2 vendor ID
pub const VENDOR_ID_MIRABOX_V2: u16 = 0x0300;

/// Product ID of Ajazz AKP153
pub const PID_AJAZZ_AKP153: u16 = 0x6674;
/// Product ID of Ajazz AKP815
pub const PID_AJAZZ_AKP815: u16 = 0x6672;
/// Product ID of Ajazz AKP153E
pub const PID_AJAZZ_AKP153E: u16 = 0x1010;
/// Product ID of Ajazz AKP153R
pub const PID_AJAZZ_AKP153R: u16 = 0x1020;
/// Product ID of Ajazz AKP03
pub const PID_AJAZZ_AKP03: u16 = 0x1001;
/// Product ID of Ajazz AKP03E
pub const PID_AJAZZ_AKP03E: u16 = 0x3002;
/// Product ID of Ajazz AKP03R
pub const PID_AJAZZ_AKP03R: u16 = 0x1003;
/// Product ID of Ajazz AKP03R rev 2
pub const PID_AJAZZ_AKP03R_REV2: u16 = 0x3003;
/// Product ID of Ajazz AKP05
pub const PID_AJAZZ_AKP05: u16 = 0x3004;

/// Offset of the button index in the input data
pub const OFFSET_ACTION_CODE: usize = 9;
/// Offset of the data length in the input data
pub const OFFSET_DATA_LENGTH: usize = 0;

/// Length of the input packet
pub const INPUT_PACKET_LENGTH: usize = 512;

/// Action code for no operation
pub const ACTION_CODE_NOP: u8 = 0x00;
/// Action code for button 7
pub const ACTION_CODE_BUTTON_7: u8 = 0x25;
/// Action code for button 8
pub const ACTION_CODE_BUTTON_8: u8 = 0x30;
/// Action code for button 9
pub const ACTION_CODE_BUTTON_9: u8 = 0x31;

/// Action code for encoder 0 counter-clockwise
pub const ACTION_CODE_ENCODER_0_CCW: u8 = 0x90;
/// Action code for encoder 0 clockwise
pub const ACTION_CODE_ENCODER_0_CW: u8 = 0x91;
/// Action code for encoder 1 counter-clockwise
pub const ACTION_CODE_ENCODER_1_CCW: u8 = 0x50;
/// Action code for encoder 1 clockwise
pub const ACTION_CODE_ENCODER_1_CW: u8 = 0x51;
/// Action code for encoder 2 counter-clockwise
pub const ACTION_CODE_ENCODER_2_CCW: u8 = 0x60;
/// Action code for encoder 2 clockwise
pub const ACTION_CODE_ENCODER_2_CW: u8 = 0x61;

/// Action code for encoder 0 press
pub const ACTION_CODE_ENCODER_0_PRESS: u8 = 0x33;
/// Action code for encoder 1 press
pub const ACTION_CODE_ENCODER_1_PRESS: u8 = 0x35;
/// Action code for encoder 2 press
pub const ACTION_CODE_ENCODER_2_PRESS: u8 = 0x34;

/// Header of the request packet
pub const REQUEST_HEADER: &[u8] = &[0x00, 0x43, 0x52, 0x54, 0x00, 0x00];

/// Request for flush command
pub const CMD_CLEAR_ALL: u8 = 0xFF;

/// Request for initialize command
pub const REQUEST_CMD_DIS: &[u8] = &[0x44, 0x49, 0x53, 0x00, 0x00];
/// Request for brightness command
pub const REQUEST_CMD_LIG: &[u8] = &[0x4c, 0x49, 0x47, 0x00, 0x00];
/// Request for keep alive command
pub const REQUEST_CMD_KEEP_ALIVE: &[u8] = &[0x43, 0x4F, 0x4E, 0x4E, 0x45, 0x43, 0x54];
/// Request for shutdown command
pub const REQUEST_CMD_SHUTDOWN: &[u8] = &[0x43, 0x4C, 0x45, 0x00, 0x00, 0x44, 0x43];
/// Request for sleep command
pub const REQUEST_CMD_SLEEP: &[u8] = &[0x48, 0x41, 0x4E];
/// Request for clear button image command
pub const REQUEST_CMD_CLEAR_BUTTON_IMAGE: &[u8] = &[0x43, 0x4c, 0x45, 0x00, 0x00, 0x00];
/// Request for flush command
pub const REQUEST_CMD_FLUSH: &[u8] = &[0x53, 0x54, 0x50];
/// Request for image packet.
/// This packet should be sent before sending image data.
pub const REQUEST_CMD_IMAGE_ANNOUNCE: &[u8] = &[0x42, 0x41, 0x54, 0x00, 0x00];
/// Request for logo image command
pub const REQUEST_CMD_LOGO_IMAGE_V1: &[u8] = &[0x4c, 0x4f, 0x47, 0x00, 0x12, 0xc3, 0xc0, 0x01];
/// Request for logo image command
pub const REQUEST_CMD_LOGO_IMAGE_V2: &[u8] = &[0x4c, 0x4f, 0x47, 0x00, 0x00];

/// Response for ACK packet
pub const RESPONSE_ACK_OK: &[u8] = &[0x41, 0x43, 0x4b, 0x00, 0x00, 0x4f, 0x4b];
