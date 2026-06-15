// Generated Rust reference translation from Intel Compute Runtime.
// SPDX-License-Identifier: MIT

#![allow(non_snake_case)]

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DebugData {
    pub vIsaSize: u32,
    pub genIsaSize: u32,
    pub vIsa: *const core::ffi::c_char,
    pub genIsa: *const core::ffi::c_char,
}

impl Default for DebugData {
    fn default() -> Self {
        Self {
            vIsaSize: 0,
            genIsaSize: 0,
            vIsa: core::ptr::null(),
            genIsa: core::ptr::null(),
        }
    }
}
