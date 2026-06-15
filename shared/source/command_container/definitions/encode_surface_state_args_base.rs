// Generated Rust reference translation from Intel Compute Runtime.
// SPDX-License-Identifier: MIT

#![allow(non_snake_case)]

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EncodeSurfaceStateArgsBase {
    pub graphicsAddress: u64,
    pub size: usize,
    pub outMemory: usize,
    pub inTemplateMemory: usize,
    pub allocation: usize,
    pub gmmHelper: usize,
    pub numAvailableDevices: u32,
    pub mocs: u32,
    pub cpuCoherent: bool,
    pub forceNonAuxMode: bool,
    pub isReadOnly: bool,
    pub areMultipleSubDevicesInContext: bool,
    pub implicitScaling: bool,
    pub isDebuggerActive: bool,
}
