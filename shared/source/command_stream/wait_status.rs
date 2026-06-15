// Generated Rust reference translation from Intel Compute Runtime.
// SPDX-License-Identifier: MIT

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum WaitStatus {
    NotReady = 0,
    Ready = 1,
    GpuHang = 2,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct WaitParams {
    pub indefinitely_poll: bool,
    pub enable_timeout: bool,
    pub skip_tbx_download: bool,
    pub wait_timeout: i64,
}

impl WaitParams {
    pub const fn new(
        indefinitely_poll: bool,
        enable_timeout: bool,
        skip_tbx_download: bool,
        wait_timeout: i64,
    ) -> Self {
        Self {
            indefinitely_poll,
            enable_timeout,
            skip_tbx_download,
            wait_timeout,
        }
    }
}

impl Default for WaitParams {
    fn default() -> Self {
        Self {
            indefinitely_poll: false,
            enable_timeout: false,
            skip_tbx_download: false,
            wait_timeout: 0,
        }
    }
}
