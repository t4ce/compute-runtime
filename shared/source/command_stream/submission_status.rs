// Generated Rust reference translation from Intel Compute Runtime.
// SPDX-License-Identifier: MIT

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SubmissionStatus {
    Success = 0,
    Failed = 1,
    OutOfMemory = 2,
    OutOfHostMemory = 3,
    Unsupported = 4,
    DeviceUninitialized = 5,
}
