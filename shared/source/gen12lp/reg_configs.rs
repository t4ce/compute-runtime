// Generated Rust reference translation from Intel Compute Runtime.
// SPDX-License-Identifier: MIT

pub struct L3CntlRegConfigIgfxTigerlakeLp;

impl L3CntlRegConfigIgfxTigerlakeLp {
    pub const VALUE_FOR_SLM: u32 = 0xD0000020;
    pub const VALUE_FOR_NO_SLM: u32 = 0xD0000020;
}

pub struct L3CntlRegisterOffsetGen12LpFamily;

impl L3CntlRegisterOffsetGen12LpFamily {
    pub const REGISTER_OFFSET: u32 = 0xB134;
    pub const REGISTER_OFFSET_CCS: u32 = 0xB234;
}

pub struct DebugModeRegisterOffsetGen12LpFamily;

impl DebugModeRegisterOffsetGen12LpFamily {
    pub const REGISTER_OFFSET: u32 = 0x20d8;
    pub const DEBUG_ENABLED_VALUE: u32 = (1 << 5) | (1 << 21);
}
