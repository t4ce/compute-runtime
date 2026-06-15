// Generated Rust reference translation from Intel Compute Runtime.
// SPDX-License-Identifier: MIT

#![allow(non_snake_case)]

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HeapInfo {
    pub pKernelHeap: *const core::ffi::c_void,
    pub pGsh: *const core::ffi::c_void,
    pub pDsh: *const core::ffi::c_void,
    pub pSsh: *const core::ffi::c_void,
    pub kernelHeapSize: u32,
    pub generalStateHeapSize: u32,
    pub dynamicStateHeapSize: u32,
    pub surfaceStateHeapSize: u32,
    pub kernelUnpaddedSize: u32,
}

impl Default for HeapInfo {
    fn default() -> Self {
        Self {
            pKernelHeap: core::ptr::null(),
            pGsh: core::ptr::null(),
            pDsh: core::ptr::null(),
            pSsh: core::ptr::null(),
            kernelHeapSize: 0,
            generalStateHeapSize: 0,
            dynamicStateHeapSize: 0,
            surfaceStateHeapSize: 0,
            kernelUnpaddedSize: 0,
        }
    }
}
