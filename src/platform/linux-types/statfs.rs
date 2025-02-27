use super::types::*;

/// Most 64-bit platforms use 'long', while most 32-bit platforms use '__u32'.
/// Yes, they differ in signedness as well as size.
/// Special cases can override it for themselves -- except for S390x, which
/// is just a little too special for us. And MIPS, which I'm not touching
/// with a 10' pole.
pub type _statfs_word = isize;

#[repr(C)]
pub struct statfs_t {
    pub f_type: isize,
    pub f_bsize: isize,
    pub f_blocks: isize,
    pub f_bfree: isize,
    pub f_bavail: isize,
    pub f_files: isize,
    pub f_ffree: isize,
    pub f_fsid: isize,
    pub f_namelen: isize,
    pub f_frsize: isize,
    pub f_flags: isize,
    pub f_spare: [isize; 4],
}

/// ARM needs to avoid the 32-bit padding at the end, for consistency
/// between EABI and OABI
#[repr(C)]
pub struct statfs64_t {
    pub f_type: isize,
    pub f_bsize: isize,
    pub f_blocks: u64,
    pub f_bfree: u64,
    pub f_bavail: u64,
    pub f_files: u64,
    pub f_ffree: u64,
    pub f_fsid: fsid_t,
    pub f_namelen: isize,
    pub f_frsize: isize,
    pub f_flags: isize,
    pub f_spare: [isize; 4],
}

/// IA64 and x86_64 need to avoid the 32-bit padding at the end,
/// to be compatible with the i386 ABI
#[repr(C)]
pub struct compat_statfs64_t {
    pub f_type: u32,
    pub f_bsize: u32,
    pub f_blocks: u64,
    pub f_bfree: u64,
    pub f_bavail: u64,
    pub f_files: u64,
    pub f_ffree: u64,
    pub f_fsid: fsid_t,
    pub f_namelen: u32,
    pub f_frsize: u32,
    pub f_flags: u32,
    pub f_spare: [u32; 4],
}
