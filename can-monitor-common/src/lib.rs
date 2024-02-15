#![no_std]
// use bytes::Bytes;

// Define constants for CAN frame properties
const CAN_MAX_DLEN: usize = 8;

#[repr(C)]
#[derive(Debug)]
pub struct CanFrame {
    pub can_id: u32,                    // 32 bit CAN_ID + EFF/RTR/ERR flags
    pub can_dlc: u8,                    // Frame payload length in bytes (0 .. CAN_MAX_DLEN)
    _pad: u8,                           // Padding
    _res0: u8,                          // Reserved / padding
    _res1: u8,                          // Reserved / padding
    pub data: [u8; CAN_MAX_DLEN],       // Frame payload data
}

impl CanFrame {
    // Constructor function for creating a new CAN frame
    pub fn new(can_id: u32, can_dlc: u8, data: [u8; CAN_MAX_DLEN]) -> CanFrame {
        CanFrame {
            can_id,
            can_dlc,
            _pad: 0,
            _res0: 0,
            _res1: 0,
            data,
        }
    }
}

