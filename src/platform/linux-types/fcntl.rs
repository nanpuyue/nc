use super::types::*;

pub const O_ACCMODE: i32 = 0o0000003;
pub const O_RDONLY: i32 = 0o0000000;
pub const O_WRONLY: i32 = 0o0000001;
pub const O_RDWR: i32 = 0o0000002;
/// not fcntl
pub const O_CREAT: i32 = 0o0000100;
/// not fcntl
pub const O_EXCL: i32 = 0o0000200;
/// not fcntl
pub const O_NOCTTY: i32 = 0o0000400;
/// not fcntl
pub const O_TRUNC: i32 = 0o0001000;
pub const O_APPEND: i32 = 0o0002000;
pub const O_NONBLOCK: i32 = 0o0004000;
/// used to be O_SYNC, see below
pub const O_DSYNC: i32 = 0o0010000;
/// fcntl, for BSD compatibility
pub const FASYNC: i32 = 0o0020000;
/// direct disk access hint
pub const O_DIRECT: i32 = 0o0040000;
pub const O_LARGEFILE: i32 = 0o0100000;
/// must be a directory
pub const O_DIRECTORY: i32 = 0o0200000;
/// don't follow links
pub const O_NOFOLLOW: i32 = 0o0400000;
pub const O_NOATIME: i32 = 0o1000000;
/// set close_on_exec
pub const O_CLOEXEC: i32 = 0o2000000;

///  Before Linux 2.6.33 only O_DSYNC semantics were implemented, but using
/// the O_SYNC flag.  We continue to use the existing numerical value
/// for O_DSYNC semantics now, but using the correct symbolic name for it.
/// This new value is used to request true Posix O_SYNC semantics.  It is
/// defined in this strange way to make sure applications compiled against
/// new headers get at least O_DSYNC semantics on older kernels.
///
/// This has the nice side-effect that we can simply test for O_DSYNC
/// wherever we do not care if O_DSYNC or O_SYNC is used.
///
/// Note: __O_SYNC must never be used directly.
pub const __O_SYNC: i32 = 0o4000000;
pub const O_SYNC: i32 = (__O_SYNC | O_DSYNC);

pub const O_PATH: i32 = 0o10000000;

pub const __O_TMPFILE: i32 = 0o20000000;

/// a horrid kludge trying to make sure that this will fail on old kernels
pub const O_TMPFILE: i32 = (__O_TMPFILE | O_DIRECTORY);
pub const O_TMPFILE_MASK: i32 = (__O_TMPFILE | O_DIRECTORY | O_CREAT);

pub const O_NDELAY: i32 = O_NONBLOCK;

pub const F_DUPFD: i32 = 0; // dup
pub const F_GETFD: i32 = 1; // get close_on_exec
pub const F_SETFD: i32 = 2; // set/clear close_on_exec
pub const F_GETFL: i32 = 3; // get file->f_flags
pub const F_SETFL: i32 = 4; // set file->f_flags
pub const F_GETLK: i32 = 5;
pub const F_SETLK: i32 = 6;
pub const F_SETLKW: i32 = 7;
pub const F_SETOWN: i32 = 8; // for sockets.
pub const F_GETOWN: i32 = 9; // for sockets.
pub const F_SETSIG: i32 = 10; // for sockets.
pub const F_GETSIG: i32 = 11; // for sockets.

///  using 'struct flock64'
pub const F_GETLK64: i32 = 12;
pub const F_SETLK64: i32 = 13;
pub const F_SETLKW64: i32 = 14;

pub const F_SETOWN_EX: i32 = 15;
pub const F_GETOWN_EX: i32 = 16;

pub const F_GETOWNER_UIDS: i32 = 17;

/// Open File Description Locks
///
/// Usually record locks held by a process are released on *any* close and are
/// not inherited across a fork().
///
/// These cmd values will set locks that conflict with process-associated
/// record  locks, but are "owned" by the open file description, not the
/// process. This means that they are inherited across fork() like BSD (flock)
/// locks, and they are only released automatically when the last reference to
/// the the open file against which they were acquired is put.
pub const F_OFD_GETLK: i32 = 36;
pub const F_OFD_SETLK: i32 = 37;
pub const F_OFD_SETLKW: i32 = 38;

pub const F_OWNER_TID: i32 = 0;
pub const F_OWNER_PID: i32 = 1;
pub const F_OWNER_PGRP: i32 = 2;

#[repr(C)]
pub struct f_owner_ex_t {
    pub type_: i32,
    pub pid: pid_t,
}

/// for F_[GET|SET]FL
/// actually anything with low bit set goes
pub const FD_CLOEXEC: i32 = 1;

/// for posix fcntl() and lockf()
pub const F_RDLCK: i32 = 0;
pub const F_WRLCK: i32 = 1;
pub const F_UNLCK: i32 = 2;

/// for old implementation of bsd flock ()
pub const F_EXLCK: i32 = 4; // or 3
pub const F_SHLCK: i32 = 8; // or 4

/// operations for bsd flock(), also used by the kernel implementation
/// shared lock
pub const LOCK_SH: i32 = 1;
/// exclusive lock
pub const LOCK_EX: i32 = 2;
/// or'd with one of the above to prevent blocking
pub const LOCK_NB: i32 = 4;
/// remove lock
pub const LOCK_UN: i32 = 8;

/// This is a mandatory flock ...
pub const LOCK_MAND: i32 = 32;
/// which allows concurrent read operations
pub const LOCK_READ: i32 = 64;
/// which allows concurrent write operations
pub const LOCK_WRITE: i32 = 128;
/// which allows concurrent read & write ops
pub const LOCK_RW: i32 = 192;

pub const F_LINUX_SPECIFIC_BASE: i32 = 1024;

//#ifndef HAVE_ARCH_STRUCT_FLOCK
//#ifndef __ARCH_FLOCK_PAD
//#define __ARCH_FLOCK_PAD
//#endif

#[repr(C)]
pub struct flock_t {
    pub l_type: i16,
    pub l_whence: i16,
    pub l_start: off_t,
    pub l_len: off_t,
    pub l_pid: pid_t,
    // TODO(Shaohua): FLOCK_PAD
    //__ARCH_FLOCK_PAD
}

//#ifndef HAVE_ARCH_STRUCT_FLOCK64
//#ifndef __ARCH_FLOCK64_PAD
//#define __ARCH_FLOCK64_PAD
//#endif

#[repr(C)]
pub struct flock64_t {
    pub l_type: i16,
    pub l_whence: i16,
    pub l_start: loff_t,
    pub l_len: loff_t,
    pub l_pid: pid_t,
    //__ARCH_FLOCK64_PAD
}
