// Generated Rust reference translation from Intel Compute Runtime.
// SPDX-License-Identifier: MIT

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PipeControlArgs {
    pub post_sync_cmd: *mut core::ffi::c_void,

    pub block_setting_post_sync_properties: bool,
    pub cs_stall_only: bool,
    pub disable_cs_stall: bool,
    pub dc_flush_enable: bool,
    pub render_target_cache_flush_enable: bool,
    pub instruction_cache_invalidate_enable: bool,
    pub texture_cache_invalidation_enable: bool,
    pub pipe_control_flush_enable: bool,
    pub vf_cache_invalidation_enable: bool,
    pub constant_cache_invalidation_enable: bool,
    pub state_cache_invalidation_enable: bool,
    pub generic_media_state_clear: bool,
    pub hdc_pipeline_flush: bool,
    pub tlb_invalidation: bool,
    pub compression_control_surface_ccs_flush: bool,
    pub notify_enable: bool,
    pub workload_partition_offset: bool,
    pub amfs_flush_enable: bool,
    pub un_typed_data_port_cache_flush: bool,
    pub depth_cache_flush_enable: bool,
    pub depth_stall_enable: bool,
    pub protected_memory_disable: bool,
    pub is_walker_with_profiling_enqueued: bool,
    pub command_cache_invalidate_enable: bool,
    pub is_l1_invalidate_required: bool,
    pub is_l1_flush_required: bool,
}

impl Default for PipeControlArgs {
    fn default() -> Self {
        Self {
            post_sync_cmd: core::ptr::null_mut(),
            block_setting_post_sync_properties: false,
            cs_stall_only: false,
            disable_cs_stall: false,
            dc_flush_enable: false,
            render_target_cache_flush_enable: false,
            instruction_cache_invalidate_enable: false,
            texture_cache_invalidation_enable: false,
            pipe_control_flush_enable: false,
            vf_cache_invalidation_enable: false,
            constant_cache_invalidation_enable: false,
            state_cache_invalidation_enable: false,
            generic_media_state_clear: false,
            hdc_pipeline_flush: false,
            tlb_invalidation: false,
            compression_control_surface_ccs_flush: false,
            notify_enable: false,
            workload_partition_offset: false,
            amfs_flush_enable: false,
            un_typed_data_port_cache_flush: false,
            depth_cache_flush_enable: false,
            depth_stall_enable: false,
            protected_memory_disable: false,
            is_walker_with_profiling_enqueued: false,
            command_cache_invalidate_enable: false,
            is_l1_invalidate_required: false,
            is_l1_flush_required: false,
        }
    }
}
