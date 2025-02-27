use super::hugetlb_encode::*;

/// flags for memfd_create(2) (unsigned int)
pub const MFD_CLOEXEC: u32 = 0x0001;
pub const MFD_ALLOW_SEALING: u32 = 0x0002;
pub const MFD_HUGETLB: u32 = 0x0004;

/// Huge page size encoding when MFD_HUGETLB is specified, and a huge page
/// size other than the default is desired.  See hugetlb_encode.h.
/// All known huge page size encodings are provided here.  It is the
/// responsibility of the application to know which sizes are supported on
/// the running system.  See mmap(2) man page for details.
pub const MFD_HUGE_SHIFT: i32 = HUGETLB_FLAG_ENCODE_SHIFT;
pub const MFD_HUGE_MASK: i32 = HUGETLB_FLAG_ENCODE_MASK;

pub const MFD_HUGE_64KB: usize = HUGETLB_FLAG_ENCODE_64KB;
pub const MFD_HUGE_512KB: usize = HUGETLB_FLAG_ENCODE_512KB;
pub const MFD_HUGE_1MB: usize = HUGETLB_FLAG_ENCODE_1MB;
pub const MFD_HUGE_2MB: usize = HUGETLB_FLAG_ENCODE_2MB;
pub const MFD_HUGE_8MB: usize = HUGETLB_FLAG_ENCODE_8MB;
pub const MFD_HUGE_16MB: usize = HUGETLB_FLAG_ENCODE_16MB;
pub const MFD_HUGE_32MB: usize = HUGETLB_FLAG_ENCODE_32MB;
pub const MFD_HUGE_256MB: usize = HUGETLB_FLAG_ENCODE_256MB;
pub const MFD_HUGE_512MB: usize = HUGETLB_FLAG_ENCODE_512MB;
pub const MFD_HUGE_1GB: usize = HUGETLB_FLAG_ENCODE_1GB;
pub const MFD_HUGE_2GB: usize = HUGETLB_FLAG_ENCODE_2GB;
pub const MFD_HUGE_16GB: usize = HUGETLB_FLAG_ENCODE_16GB;
