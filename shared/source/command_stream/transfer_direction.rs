// Generated Rust reference translation from Intel Compute Runtime.
// SPDX-License-Identifier: MIT

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TransferDirection {
    HostToHost = 0,
    HostToLocal = 1,
    LocalToHost = 2,
    LocalToLocal = 3,
    Remote = 4,
}

pub const fn create_transfer_direction(
    src_local: bool,
    dst_local: bool,
    remote_copy: bool,
) -> TransferDirection {
    if remote_copy {
        return TransferDirection::Remote;
    }

    if src_local {
        if dst_local {
            TransferDirection::LocalToLocal
        } else {
            TransferDirection::LocalToHost
        }
    } else if dst_local {
        TransferDirection::HostToLocal
    } else {
        TransferDirection::HostToHost
    }
}
