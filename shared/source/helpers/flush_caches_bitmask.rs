// Generated Rust reference translation from Intel Compute Runtime.
// SPDX-License-Identifier: MIT

pub mod flush_caches_bitmask {
    // Keep bit 0 free for backwards compatibility with FlushAllCaches debug variable, which is set to 1 to flush all caches.
    pub const DC_FLUSH: i64 = 1 << 1;
    pub const RENDER_TARGET_CACHE: i64 = 1 << 2;
    pub const INSTRUCTION_CACHE: i64 = 1 << 3;
    pub const TEXTURE_CACHE: i64 = 1 << 4;
    pub const PIPE_CONTROL: i64 = 1 << 5;
    pub const VF_CACHE: i64 = 1 << 6;
    pub const CONSTANT_CACHE: i64 = 1 << 7;
    pub const STATE_CACHE: i64 = 1 << 8;
    pub const TLB: i64 = 1 << 9;
    pub const HDC_PIPELINE: i64 = 1 << 10;
    pub const UN_TYPED_DATA_PORT_CACHE: i64 = 1 << 11;
    pub const COMPRESSION_CONTROL_SURFACE_CCS: i64 = 1 << 12;
    pub const L2_FLUSH: i64 = 1 << 13;
    pub const L2_TRANSIENT_FLUSH: i64 = 1 << 14;

    pub const ALL_CACHES: i64 = DC_FLUSH
        | RENDER_TARGET_CACHE
        | INSTRUCTION_CACHE
        | TEXTURE_CACHE
        | PIPE_CONTROL
        | VF_CACHE
        | CONSTANT_CACHE
        | STATE_CACHE
        | TLB
        | HDC_PIPELINE
        | UN_TYPED_DATA_PORT_CACHE
        | COMPRESSION_CONTROL_SURFACE_CCS
        | L2_FLUSH
        | L2_TRANSIENT_FLUSH;
}
