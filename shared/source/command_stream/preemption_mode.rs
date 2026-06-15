// Generated Rust reference translation from Intel Compute Runtime.
// SPDX-License-Identifier: MIT

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PreemptionMode {
    // Keep in sync with ForcePreemptionMode debug variable.
    Initial = 0,
    Disabled = 1,
    MidBatch = 2,
    ThreadGroup = 3,
    MidThread = 4,
}
