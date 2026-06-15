// Generated Rust reference translation from Intel Compute Runtime.
// SPDX-License-Identifier: MIT

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum KernelExecutionType {
    DefaultType = 0x0,
    Concurrent = 0x1,
    NotApplicable = 0x2,
}
