// Generated Rust reference translation from Intel Compute Runtime.
// SPDX-License-Identifier: MIT

#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HardwareIpVersion(u32);

impl HardwareIpVersion {
    pub const REVISION_MASK: u32 = 0x3f;
    pub const RESERVED_MASK: u32 = 0xff;
    pub const RELEASE_MASK: u32 = 0xff;
    pub const ARCHITECTURE_MASK: u32 = 0x3ff;

    pub const REVISION_SHIFT: u32 = 0;
    pub const RESERVED_SHIFT: u32 = 6;
    pub const RELEASE_SHIFT: u32 = 14;
    pub const ARCHITECTURE_SHIFT: u32 = 22;

    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    pub const fn make(revision: u32, reserved: u32, release: u32, architecture: u32) -> Self {
        Self(
            ((revision & Self::REVISION_MASK) << Self::REVISION_SHIFT)
                | ((reserved & Self::RESERVED_MASK) << Self::RESERVED_SHIFT)
                | ((release & Self::RELEASE_MASK) << Self::RELEASE_SHIFT)
                | ((architecture & Self::ARCHITECTURE_MASK) << Self::ARCHITECTURE_SHIFT),
        )
    }

    pub const fn value(self) -> u32 {
        self.0
    }

    pub const fn revision(self) -> u32 {
        (self.0 >> Self::REVISION_SHIFT) & Self::REVISION_MASK
    }

    pub const fn reserved(self) -> u32 {
        (self.0 >> Self::RESERVED_SHIFT) & Self::RESERVED_MASK
    }

    pub const fn release(self) -> u32 {
        (self.0 >> Self::RELEASE_SHIFT) & Self::RELEASE_MASK
    }

    pub const fn architecture(self) -> u32 {
        (self.0 >> Self::ARCHITECTURE_SHIFT) & Self::ARCHITECTURE_MASK
    }
}
