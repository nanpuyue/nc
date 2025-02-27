use super::ipc::*;
use super::types::*;

/// generic msqid64_ds structure.
///
/// Note extra padding because this structure is passed back and forth
/// between kernel and user space.
///
/// msqid64_ds was originally meant to be architecture specific, but
/// everyone just ended up making identical copies without specific
/// optimizations, so we may just as well all use the same one.
///
/// 64 bit architectures typically define a 64 bit __kernel_time_t,
/// so they do not need the first three padding words.
/// On big-endian systems, the padding is in the wrong place.
///
/// Pad space is left for:
/// - 2 miscellaneous 32-bit values

#[repr(C)]
pub struct msqid64_ds_t {
    pub msg_perm: ipc_perm_t,
    /// last msgsnd time
    pub msg_stime: time_t,
    /// last msgrcv time
    pub msg_rtime: time_t,
    /// last change time
    pub msg_ctime: time_t,
    /// current number of bytes on queue
    pub msg_cbytes: usize,
    /// number of messages in queue
    pub msg_qnum: usize,
    /// max number of bytes on queue
    pub msg_qbytes: usize,
    /// pid of last msgsnd
    pub msg_lspid: pid_t,
    /// last receive pid
    pub msg_lrpid: pid_t,
    unused4: usize,
    unused5: usize,
}

#[cfg(target_pointer_size = "32")]
#[repr(C)]
pub struct msqid64_ds_t {
    pub msg_perm: ipc_perm_t,
    /// last msgsnd time
    pub msg_stime: usize,
    pub msg_stime_high: usize,
    /// last msgrcv time
    pub msg_rtime: usize,
    pub msg_rtime_high: usize,
    /// last change time
    pub msg_ctime: usize,
    pub msg_ctime_high: usize,
    /// current number of bytes on queue
    pub msg_cbytes: usize,
    /// number of messages in queue
    pub msg_qnum: usize,
    /// max number of bytes on queue
    pub msg_qbytes: usize,
    /// pid of last msgsnd
    pub msg_lspid: pid_t,
    /// last receive pid
    pub msg_lrpid: pid_t,
    unused4: u64,
    unused5: u64,
}

pub type msqid_ds_t = msqid64_ds_t;
