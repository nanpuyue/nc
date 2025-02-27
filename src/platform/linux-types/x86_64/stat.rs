
#[repr(C)]
#[derive(Clone, Default, Debug)]
pub struct stat_t {
	pub st_dev: usize,
	pub st_ino: usize,
	pub st_nlink: usize,

	pub st_mode: u32,
	pub st_uid: u32,
	pub st_gid: u32,
	pad0: u32,
	pub st_rdev: usize,
	pub st_size: isize,
	pub st_blksize: isize,
    /// Number 512-byte blocks allocated.
	pub st_blocks: isize,

	pub st_atime: usize,
	pub st_atime_nsec: usize,
	pub st_mtime: usize,
	pub st_mtime_nsec: usize,
	pub st_ctime: usize,
	pub	st_ctime_nsec: usize,
	unused: [isize; 3],
}

