// Generated Rust reference translation from Intel Compute Runtime.
// SPDX-License-Identifier: MIT

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EngineGroupType {
    Compute = 0,
    RenderCompute = 1,
    Copy = 2,
    LinkedCopy = 3,
    CooperativeCompute = 4,
    MaxEngineGroups = 5,
}

impl EngineGroupType {
    pub const fn is_copy_only_engine_type(self) -> bool {
        matches!(self, Self::Copy | Self::LinkedCopy)
    }
}
