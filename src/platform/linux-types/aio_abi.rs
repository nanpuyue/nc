use super::fs::*;

pub type aio_context_t = usize;

pub const IOCB_CMD_PREAD: i32 = 0;
pub const IOCB_CMD_PWRITE: i32 = 1;
pub const IOCB_CMD_FSYNC: i32 = 2;
pub const IOCB_CMD_FDSYNC: i32 = 3;
// 4 was the experimental IOCB_CMD_PREADX
pub const IOCB_CMD_POLL: i32 = 5;
pub const IOCB_CMD_NOOP: i32 = 6;
pub const IOCB_CMD_PREADV: i32 = 7;
pub const IOCB_CMD_PWRITEV: i32 = 8;

/// Valid flags for the "aio_flags" member of the "struct iocb".
/// IOCB_FLAG_RESFD - Set if the "aio_resfd" member of the "struct iocb" is valid.
/// IOCB_FLAG_IOPRIO - Set if the "aio_reqprio" member of the "struct iocb" is valid.
pub const IOCB_FLAG_RESFD: i32 = (1 << 0);
pub const IOCB_FLAG_IOPRIO: i32 = (1 << 1);

/// read() from /dev/aio returns these structures.
#[repr(C)]
pub struct io_event_t {
    /// the data field from the iocb
    pub data: u64,

    /// what iocb this event came from
    pub obj: u64,

    /// result code for this event
    pub res: i64,

    /// secondary result
    pub res2: i64,
}

/// we always use a 64bit off_t when communicating
/// with userland.  its up to libraries to do the
/// proper padding and aio_error abstraction
#[repr(C)]
// TODO(Shaohua): Check int types to pre-defined types
pub struct iocb_t {
    /// these are internal to the kernel/libc.
    /// data to be returned in event's data */
    pub aio_data: u64,
    /// the kernel sets aio_key to the req #
    pub aio_key: u32,

    /// RWF_* flags
    pub aio_rw_flags: rwf_t,

    /// common fields
    pub aio_lio_opcode: u16, // see IOCB_CMD_ above
    pub aio_reqprio: i16,
    pub aio_fildes: u32,

    pub aio_buf: u64,
    pub aio_nbytes: u64,
    pub aio_offset: i64,

    /// extra parameters
    pub aio_reserved2: u64, // TODO: use this for a (struct sigevent *)

    /// flags for the "struct iocb"
    pub aio_flags: u32,

    /// If the IOCB_FLAG_RESFD flag of "aio_flags" is set, this is an eventfd
    /// to signal AIO readiness to
    pub aio_resfd: u32,
}
