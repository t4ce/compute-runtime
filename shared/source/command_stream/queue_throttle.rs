// Generated Rust reference translation from Intel Compute Runtime.
// SPDX-License-Identifier: MIT

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum QueueThrottle {
    Low = 0,
    Medium = 1,
    High = 2,
}

pub fn get_throttle_from_power_saving_uint(value: u8) -> QueueThrottle {
    if value == 0 {
        QueueThrottle::Medium
    } else {
        QueueThrottle::Low
    }
}
