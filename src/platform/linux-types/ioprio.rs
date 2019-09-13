/// Gives us 8 prio classes with 13-bits of data for each class
pub const IOPRIO_CLASS_SHIFT: u8 = 13;
pub const IOPRIO_PRIO_MASK: usize = (1 << IOPRIO_CLASS_SHIFT) - 1;

#[inline]
pub const fn IOPRIO_PRIO_CLASS(mask: i32) -> i32 {
    mask >> IOPRIO_CLASS_SHIFT
}

#[inline]
pub const fn IOPRIO_PRIO_DATA(mask: i32) -> usize {
    (mask as usize) & IOPRIO_PRIO_MASK
}

#[inline]
pub const fn IOPRIO_PRIO_VALUE(class_: i32, data: usize) -> usize {
    ((class_ as usize) << IOPRIO_CLASS_SHIFT) | data
}

#[inline]
pub const fn ioprio_valid(mask: i32) -> bool {
    IOPRIO_PRIO_CLASS(mask) != IOPRIO_CLASS_NONE
}

/// These are the io priority groups as implemented by CFQ. RT is the realtime
/// class, it always gets premium service. BE is the best-effort scheduling
/// class, the default for any process. IDLE is the idle scheduling class, it
/// is only served when no one else is using the disk.
pub const IOPRIO_CLASS_NONE: i32 = 0;
pub const IOPRIO_CLASS_RT: i32 = 1;
pub const IOPRIO_CLASS_BE: i32 = 2;
pub const IOPRIO_CLASS_IDLE: i32 = 3;

/// 8 best effort priority levels are supported
pub const IOPRIO_BE_NR: i32 = 8;

pub const IOPRIO_WHO_PROCESS: i32 = 1;
pub const IOPRIO_WHO_PGRP: i32 = 2;
pub const IOPRIO_WHO_USER: i32 = 3;

/// Fallback BE priority
pub const IOPRIO_NORM: i32 = 4;
