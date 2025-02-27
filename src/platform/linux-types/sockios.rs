/// Socket-level I/O control calls.
pub const FIOSETOWN: i32 = 0x8901;
pub const SIOCSPGRP: i32 = 0x8902;
pub const FIOGETOWN: i32 = 0x8903;
pub const SIOCGPGRP: i32 = 0x8904;
pub const SIOCATMARK: i32 = 0x8905;
/// Get stamp (timeval)
pub const SIOCGSTAMP: i32 = 0x8906;
/// Get stamp (timespec)
pub const SIOCGSTAMPNS: i32 = 0x8907;
