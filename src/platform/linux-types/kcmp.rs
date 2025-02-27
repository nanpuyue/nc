/// Comparison type - enum kcmp_type.
pub const KCMP_FILE: i32 = 0;
pub const KCMP_VM: i32 = 1;
pub const KCMP_FILES: i32 = 2;
pub const KCMP_FS: i32 = 3;
pub const KCMP_SIGHAND: i32 = 4;
pub const KCMP_IO: i32 = 5;
pub const KCMP_SYSVSEM: i32 = 6;
pub const KCMP_EPOLL_TFD: i32 = 7;
pub const KCMP_TYPES: i32 = 8;

/// Slot for KCMP_EPOLL_TFD
#[repr(C)]
pub struct kcmp_epoll_slot_t {
    /// epoll file descriptor
    pub efd: u32,

    /// target file number
    pub tfd: u32,

    /// target offset within same numbered sequence
    pub toff: u32,
}
