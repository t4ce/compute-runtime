// Generated Rust reference translation from Intel Compute Runtime.
// SPDX-License-Identifier: MIT

pub mod fp_atomic_ext_flags {
    pub const GLOBAL_LOAD_STORE: u64 = 1 << 0;
    pub const GLOBAL_ADD: u64 = 1 << 1;
    pub const GLOBAL_MIN_MAX: u64 = 1 << 2;
    pub const LOCAL_LOAD_STORE: u64 = 1 << 16;
    pub const LOCAL_ADD: u64 = 1 << 17;
    pub const LOCAL_MIN_MAX: u64 = 1 << 18;

    pub const LOAD_STORE_ATOMIC_CAPS: u32 = (GLOBAL_LOAD_STORE | LOCAL_LOAD_STORE) as u32;
    pub const MIN_MAX_ATOMIC_CAPS: u32 = (GLOBAL_MIN_MAX | LOCAL_MIN_MAX) as u32;
    pub const ADD_ATOMIC_CAPS: u32 = (GLOBAL_ADD | LOCAL_ADD) as u32;
}
