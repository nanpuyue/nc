
#[repr(C)]
pub struct stat_t {
    pub st_dev: usize,
    pub st_ino: usize,
    pub st_nlink: usize,
    pub st_mode: u32,
    pub st_uid: u32,
    pub st_gid: u32,
    pad1: u32,
    pub st_rdev: usize,
    pub st_size: usize,
    pub st_atime: usize,
    pub st_atime_nsec: usize,
    pub st_mtime: usize,
    pub st_mtime_nsec: usize,
    pub st_ctime: usize,
    pub st_ctime_nsec: usize,
    pub st_blksize: usize,
    pub st_blocks: isize,
    unused: [usize; 3],
}

pub const STAT_HAVE_NSEC: i32 = 1;
