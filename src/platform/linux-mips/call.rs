extern crate alloc;

use super::errno::*;
use super::sysno::*;
use super::{syscall0, syscall1, syscall2, syscall3, syscall4, syscall5, syscall6};
use crate::c_str::CString;
use crate::types::*;
use alloc::string::String;
use alloc::vec::Vec;

pub fn accept(sockfd: i32, addr: &mut sockaddr_in_t, addrlen: &mut socklen_t) -> Result<(), Errno> {
    unsafe {
        let sockfd = sockfd as usize;
        let addr_ptr = addr as *mut sockaddr_in_t as usize;
        let addrlen_ptr = addrlen as *mut socklen_t as usize;
        syscall3(SYS_ACCEPT, sockfd, addr_ptr, addrlen_ptr).map(drop)
    }
}

/// Accept a connection on a socket.
pub fn accept4(
    sockfd: i32,
    addr: &mut sockaddr_in_t,
    addrlen: &mut socklen_t,
    flags: i32,
) -> Result<(), Errno> {
    unsafe {
        let sockfd = sockfd as usize;
        let addr_ptr = addr as *mut sockaddr_in_t as usize;
        let addrlen_ptr = addrlen as *mut socklen_t as usize;
        let flags = flags as usize;
        syscall4(SYS_ACCEPT4, sockfd, addr_ptr, addrlen_ptr, flags).map(drop)
    }
}

/// Check user's permission for a file.
pub fn access(filename: &str, mode: i32) -> Result<(), Errno> {
    unsafe {
        let filename = CString::new(filename);
        let filename_ptr = filename.as_ptr() as usize;
        let mode = mode as usize;
        syscall2(SYS_ACCESS, filename_ptr, mode).map(drop)
    }
}

/// Switch process accounting.
pub fn acct(filename: &str) -> Result<(), Errno> {
    unsafe {
        let filename = CString::new(filename);
        let filename_ptr = filename.as_ptr() as usize;
        syscall1(SYS_ACCT, filename_ptr).map(drop)
    }
}

/// Add a key to the kernel's key management facility.
pub fn add_key(
    type_: &str,
    description: &str,
    payload: usize,
    plen: size_t,
    dest_keyring: key_serial_t,
) -> Result<key_serial_t, Errno> {
    unsafe {
        let type_ = CString::new(type_);
        let type_ptr = type_.as_ptr() as usize;
        let description = CString::new(description);
        let description_ptr = description.as_ptr() as usize;
        let plen = plen as usize;
        let dest_keyring = dest_keyring as usize;
        syscall5(
            SYS_ADD_KEY,
            type_ptr,
            description_ptr,
            payload,
            plen,
            dest_keyring,
        )
        .map(|ret| ret as key_serial_t)
    }
}

/// Tune kernel clock. Returns clock state on success.
pub fn adjtimex(buf: &mut timex_t) -> Result<i32, Errno> {
    unsafe {
        let buf_ptr = buf as *mut timex_t as usize;
        syscall1(SYS_ADJTIMEX, buf_ptr).map(|ret| ret as i32)
    }
}

pub fn afs_syscall() {
    core::unimplemented!();
    // syscall0(SYS_AFS_SYSCALL);
}

/// set an alarm clock for delivery of a signal.
pub fn alarm(seconds: u32) -> u32 {
    unsafe {
        let seconds = seconds as usize;
        syscall1(SYS_ALARM, seconds).expect("alarm() failed") as u32
    }
}

/// Start, flush or tune buffer-dirty-flush daemon.
/// There are no bdflush tunables left.  But distributions are
/// still running obsolete flush daemons, so we terminate them here.
///
/// Use of bdflush() is deprecated and will be removed in a future kernel.
/// The `flush-X' kernel threads fully replace bdflush daemons and this call.
/// Deprecated.
pub fn bdflush() {
    core::unimplemented!();
    // syscall0(SYS_BDFLUSH);
}

/// Bind a name to a socket.
pub fn bind(sockfd: i32, addr: &sockaddr_in_t, addrlen: socklen_t) -> Result<(), Errno> {
    unsafe {
        let sockfd = sockfd as usize;
        let addr_ptr = addr as *const sockaddr_in_t as usize;
        let addrlen = addrlen as usize;
        syscall3(SYS_BIND, sockfd, addr_ptr, addrlen).map(drop)
    }
}

/// Perform a command on an extended BPF map or program
pub fn bpf(cmd: i32, attr: &mut bpf_attr_t, size: u32) -> Result<i32, Errno> {
    unsafe {
        let cmd = cmd as usize;
        let attr_ptr = attr as *mut bpf_attr_t as usize;
        let size = size as usize;
        syscall3(SYS_BPF, cmd, attr_ptr, size).map(|ret| ret as i32)
    }
}

pub fn r#break() {
    core::unimplemented!();
    // syscall0(SYS_BREAK);
}

/// Change data segment size.
pub fn brk(addr: usize) -> Result<(), Errno> {
    unsafe { syscall1(SYS_BRK, addr).map(drop) }
}

pub fn cachectl() {
    core::unimplemented!();
    // syscall0(SYS_CACHECTL);
}

/// Flush contents of instruction and/or data cache.
pub fn cacheflush(addr: usize, nbytes: size_t, cache: i32) -> Result<(), Errno> {
    unsafe {
        let nbytes = nbytes as usize;
        let cache = cache as usize;
        syscall3(SYS_CACHEFLUSH, addr, nbytes, cache).map(drop)
    }
}

/// Get capabilities of thread.
pub fn capget(hdrp: &mut cap_user_header_t, data: &mut cap_user_data_t) -> Result<(), Errno> {
    unsafe {
        let hdrp_ptr = hdrp as *mut cap_user_header_t as usize;
        let data_ptr = data as *mut cap_user_data_t as usize;
        syscall2(SYS_CAPGET, hdrp_ptr, data_ptr).map(drop)
    }
}

/// Set capabilities of thread.
pub fn capset(hdrp: &mut cap_user_header_t, data: &cap_user_data_t) -> Result<(), Errno> {
    unsafe {
        let hdrp_ptr = hdrp as *mut cap_user_header_t as usize;
        let data_ptr = data as *const cap_user_data_t as usize;
        syscall2(SYS_CAPSET, hdrp_ptr, data_ptr).map(drop)
    }
}

/// Change working directory.
pub fn chdir(filename: &str) -> Result<(), Errno> {
    unsafe {
        let filename = CString::new(filename);
        let filename_ptr = filename.as_ptr() as usize;
        syscall1(SYS_CHDIR, filename_ptr).map(drop)
    }
}

/// Change permissions of a file.
pub fn chmod(filename: &str, mode: mode_t) -> Result<(), Errno> {
    unsafe {
        let filename = CString::new(filename);
        let filename_ptr = filename.as_ptr() as usize;
        let mode = mode as usize;
        syscall2(SYS_CHMOD, filename_ptr, mode).map(drop)
    }
}

/// Change ownership of a file.
pub fn chown(filename: &str, user: uid_t, group: gid_t) -> Result<(), Errno> {
    unsafe {
        let filename = CString::new(filename);
        let filename_ptr = filename.as_ptr() as usize;
        let user = user as usize;
        let group = group as usize;
        syscall3(SYS_CHOWN, filename_ptr, user, group).map(drop)
    }
}

/// Change the root directory.
pub fn chroot(filename: &str) -> Result<(), Errno> {
    unsafe {
        let filename = CString::new(filename);
        let filename_ptr = filename.as_ptr() as usize;
        syscall1(SYS_CHROOT, filename_ptr).map(drop)
    }
}

pub fn clock_adjtime(which_clock: clockid_t, tx: &mut timex_t) -> Result<(), Errno> {
    unsafe {
        let which_clock = which_clock as usize;
        let tx_ptr = tx as *mut timex_t as usize;
        syscall2(SYS_CLOCK_ADJTIME, which_clock, tx_ptr).map(drop)
    }
}

pub fn clock_adjtime64() {
    core::unimplemented!();
    // syscall0(SYS_CLOCK_ADJTIME64);
}

/// Get resolution(precision) of the specific clock.
pub fn clock_getres(which_clock: clockid_t, tp: &mut timespec_t) -> Result<(), Errno> {
    unsafe {
        let which_clock = which_clock as usize;
        let tp_ptr = tp as *mut timespec_t as usize;
        syscall2(SYS_CLOCK_GETRES, which_clock, tp_ptr).map(drop)
    }
}

pub fn clock_getres_time64() {
    core::unimplemented!();
    // syscall0(SYS_CLOCK_GETRES_TIME64);
}

/// Get time of specific clock.
pub fn clock_gettime(which_clock: clockid_t, tp: &mut timespec_t) -> Result<(), Errno> {
    unsafe {
        let which_clock = which_clock as usize;
        let tp_ptr = tp as *mut timespec_t as usize;
        syscall2(SYS_CLOCK_GETTIME, which_clock, tp_ptr).map(drop)
    }
}

pub fn clock_gettime64() {
    core::unimplemented!();
    // syscall0(SYS_CLOCK_GETTIME64);
}

/// High resolution sleep with a specific clock.
pub fn clock_nanosleep(
    which_clock: clockid_t,
    flags: i32,
    rqtp: &timespec_t,
    rmtp: &mut timespec_t,
) -> Result<(), Errno> {
    unsafe {
        let which_clock = which_clock as usize;
        let flags = flags as usize;
        let rqtp_ptr = rqtp as *const timespec_t as usize;
        let rmtp_ptr = rmtp as *mut timespec_t as usize;
        syscall4(SYS_CLOCK_NANOSLEEP, which_clock, flags, rqtp_ptr, rmtp_ptr).map(drop)
    }
}

pub fn clock_nanosleep_time64() {
    core::unimplemented!();
    // syscall0(SYS_CLOCK_NANOSLEEP_TIME64);
}

/// Set time of specific clock.
pub fn clock_settime(which_clock: clockid_t, tp: &timespec_t) -> Result<(), Errno> {
    unsafe {
        let which_clock = which_clock as usize;
        let tp_ptr = tp as *const timespec_t as usize;
        syscall2(SYS_CLOCK_SETTIME, which_clock, tp_ptr).map(drop)
    }
}

pub fn clock_settime64() {
    core::unimplemented!();
    // syscall0(SYS_CLOCK_SETTIME64);
}

/// Create a child process.
pub fn clone(
    clone_flags: i32,
    newsp: usize,
    parent_tid: &mut i32,
    child_tid: &mut i32,
    tls: usize,
) -> Result<pid_t, Errno> {
    unsafe {
        let clone_flags = clone_flags as usize;
        let parent_tid_ptr = parent_tid as *mut i32 as usize;
        let child_tid_ptr = child_tid as *mut i32 as usize;
        syscall5(
            SYS_CLONE,
            clone_flags,
            newsp,
            parent_tid_ptr,
            child_tid_ptr,
            tls,
        )
        .map(|ret| ret as pid_t)
    }
}

/// Close a file descriptor.
pub fn close(fd: i32) -> Result<(), Errno> {
    unsafe {
        let fd = fd as usize;
        syscall1(SYS_CLOSE, fd).map(drop)
    }
}

/// Initialize a connection on a socket.
pub fn connect(sockfd: i32, addr: &sockaddr_in_t, addrlen: socklen_t) -> Result<(), Errno> {
    unsafe {
        let sockfd = sockfd as usize;
        // TODO(Shaohua): Use sockaddr_t generic type.
        let addr_ptr = addr as *const sockaddr_in_t as usize;
        let addrlen = addrlen as usize;
        syscall3(SYS_CONNECT, sockfd, addr_ptr, addrlen).map(drop)
    }
}

/// Copy a range of data from one file to another.
pub fn copy_file_range(
    fd_in: i32,
    off_in: &mut loff_t,
    fd_out: i32,
    off_out: &mut loff_t,
    len: size_t,
    flags: u32,
) -> Result<ssize_t, Errno> {
    unsafe {
        let fd_in = fd_in as usize;
        let off_in_ptr = off_in as *mut loff_t as usize;
        let fd_out = fd_out as usize;
        let off_out_ptr = off_out as *mut loff_t as usize;
        let len = len as usize;
        let flags = flags as usize;
        syscall6(
            SYS_COPY_FILE_RANGE,
            fd_in,
            off_in_ptr,
            fd_out,
            off_out_ptr,
            len,
            flags,
        )
        .map(|ret| ret as ssize_t)
    }
}

/// Create a file.
/// equals to call `open()` with flags `O_CREAT|O_WRONLY|O_TRUNC`.
pub fn creat(filename: &str, mode: mode_t) -> Result<i32, Errno> {
    unsafe {
        let filename = CString::new(filename);
        let filename_ptr = filename.as_ptr() as usize;
        let mode = mode as usize;
        syscall2(SYS_CREAT, filename_ptr, mode).map(|ret| ret as i32)
    }
}

pub fn create_module() {
    core::unimplemented!();
    // syscall0(SYS_CREATE_MODULE);
}

/// Unlock a kernel module.
pub fn delete_module(name: &str, flags: i32) -> Result<(), Errno> {
    unsafe {
        let name = CString::new(name);
        let name_ptr = name.as_ptr() as usize;
        let flags = flags as usize;
        syscall2(SYS_DELETE_MODULE, name_ptr, flags).map(drop)
    }
}

/// Create a copy of the file descriptor `oldfd`, using the lowest available
/// file descriptor.
pub fn dup(oldfd: i32) -> Result<isize, Errno> {
    unsafe {
        let oldfd = oldfd as usize;
        syscall1(SYS_DUP, oldfd).map(|ret| ret as isize)
    }
}

/// Create a copy of the file descriptor `oldfd`, using the speficified file
/// descriptor `newfd`.
pub fn dup2(oldfd: i32, newfd: i32) -> Result<(), Errno> {
    unsafe {
        let oldfd = oldfd as usize;
        let newfd = newfd as usize;
        syscall2(SYS_DUP2, oldfd, newfd).map(drop)
    }
}

/// Save as `dup2()`, but can set the close-on-exec flag on `newfd`.
pub fn dup3(oldfd: i32, newfd: i32, flags: i32) -> Result<(), Errno> {
    unsafe {
        let oldfd = oldfd as usize;
        let newfd = newfd as usize;
        let flags = flags as usize;
        syscall3(SYS_DUP3, oldfd, newfd, flags).map(drop)
    }
}

/// Open an epoll file descriptor.
pub fn epoll_create(size: i32) -> Result<i32, Errno> {
    unsafe {
        let size = size as usize;
        syscall1(SYS_EPOLL_CREATE, size).map(|ret| ret as i32)
    }
}

/// Open an epoll file descriptor.
pub fn epoll_create1(flags: i32) -> Result<i32, Errno> {
    unsafe {
        let flags = flags as usize;
        syscall1(SYS_EPOLL_CREATE1, flags).map(|ret| ret as i32)
    }
}

/// Control interface for an epoll file descriptor.
pub fn epoll_ctl(epfd: i32, op: i32, fd: i32, event: &mut epoll_event_t) -> Result<(), Errno> {
    unsafe {
        let epfd = epfd as usize;
        let op = op as usize;
        let fd = fd as usize;
        let event_ptr = event as *mut epoll_event_t as usize;
        syscall4(SYS_EPOLL_CTL, epfd, op, fd, event_ptr).map(drop)
    }
}

/// Wait for an I/O event on an epoll file descriptor.
pub fn epoll_pwait(epfd: i32, op: i32, fd: i32, events: &mut epoll_event_t) -> Result<i32, Errno> {
    unsafe {
        let epfd = epfd as usize;
        let op = op as usize;
        let fd = fd as usize;
        let events_ptr = events as *mut epoll_event_t as usize;
        syscall4(SYS_EPOLL_PWAIT, epfd, op, fd, events_ptr).map(|ret| ret as i32)
    }
}

/// Wait for an I/O event on an epoll file descriptor.
pub fn epoll_wait(
    epfd: i32,
    events: &mut epoll_event_t,
    maxevents: i32,
    timeout: i32,
) -> Result<i32, Errno> {
    unsafe {
        let epfd = epfd as usize;
        let events_ptr = events as *mut epoll_event_t as usize;
        let maxevents = maxevents as usize;
        let timeout = timeout as usize;
        syscall4(SYS_EPOLL_WAIT, epfd, events_ptr, maxevents, timeout).map(|ret| ret as i32)
    }
}

/// Create a file descriptor for event notification.
pub fn eventfd(count: u32) -> Result<i32, Errno> {
    unsafe {
        let count = count as usize;
        syscall1(SYS_EVENTFD, count).map(|ret| ret as i32)
    }
}

/// Create a file descriptor for event notification.
pub fn eventfd2(count: u32, flags: i32) -> Result<i32, Errno> {
    unsafe {
        let count = count as usize;
        let flags = flags as usize;
        syscall2(SYS_EVENTFD2, count, flags).map(|ret| ret as i32)
    }
}

/// Execute a new program.
pub fn execve(filename: &str, argv: &[&str], env: &[&str]) -> Result<(), Errno> {
    unsafe {
        let filename = CString::new(filename);
        let filename_ptr = filename.as_ptr() as usize;
        let argv_ptr = argv.as_ptr() as usize;
        let env_ptr = env.as_ptr() as usize;
        syscall3(SYS_EXECVE, filename_ptr, argv_ptr, env_ptr).map(drop)
    }
}

/// Execute a new program relative to a directory file descriptor.
pub fn execveat(
    fd: i32,
    filename: &str,
    argv: &[&str],
    env: &[&str],
    flags: i32,
) -> Result<(), Errno> {
    unsafe {
        let fd = fd as usize;
        let filename = CString::new(filename);
        let filename_ptr = filename.as_ptr() as usize;
        let argv_ptr = argv.as_ptr() as usize;
        let env_ptr = env.as_ptr() as usize;
        let flags = flags as usize;
        syscall5(SYS_EXECVEAT, fd, filename_ptr, argv_ptr, env_ptr, flags).map(drop)
    }
}

/// Terminate current process.
pub fn exit(status: u8) {
    unsafe {
        let status = status as usize;
        let _ret = syscall1(SYS_EXIT, status);
    }
}

/// Exit all threads in a process's thread group.
pub fn exit_group(status: i32) {
    unsafe {
        let status = status as usize;
        let _ret = syscall1(SYS_EXIT_GROUP, status);
    }
}

/// Check user's permission for a file.
pub fn faccessat(dfd: i32, filename: &str, mode: i32) -> Result<(), Errno> {
    unsafe {
        let dfd = dfd as usize;
        let filename = CString::new(filename);
        let filename_ptr = filename.as_ptr() as usize;
        let mode = mode as usize;
        syscall3(SYS_FACCESSAT, dfd, filename_ptr, mode).map(drop)
    }
}

/// Predeclare an access pattern for file data.
pub fn fadvise64(fd: i32, offset: loff_t, len: size_t, advice: i32) -> Result<(), Errno> {
    unsafe {
        let fd = fd as usize;
        let offset = offset as usize;
        let len = len as usize;
        let advice = advice as usize;
        syscall4(SYS_FADVISE64, fd, offset, len, advice).map(drop)
    }
}

/// Manipulate file space.
pub fn fallocate(fd: i32, mode: i32, offset: loff_t, len: loff_t) -> Result<(), Errno> {
    unsafe {
        let fd = fd as usize;
        let mode = mode as usize;
        let offset = offset as usize;
        let len = len as usize;
        syscall4(SYS_FALLOCATE, fd, mode, offset, len).map(drop)
    }
}

/// Create and initialize fanotify group.
pub fn fanotify_init(flags: u32, event_f_flags: u32) -> Result<i32, Errno> {
    unsafe {
        let flags = flags as usize;
        let event_f_flags = event_f_flags as usize;
        syscall2(SYS_FANOTIFY_INIT, flags, event_f_flags).map(|ret| ret as i32)
    }
}

/// Add, remove, or modify an fanotify mark on a filesystem object
pub fn fanotify_mark(
    fanotify_fd: i32,
    flags: u32,
    mask: u64,
    fd: i32,
    filename: &str,
) -> Result<(), Errno> {
    unsafe {
        let fanotify_fd = fanotify_fd as usize;
        let flags = flags as usize;
        let mask = mask as usize;
        let fd = fd as usize;
        let filename = CString::new(filename);
        let filename_ptr = filename.as_ptr() as usize;
        syscall5(
            SYS_FANOTIFY_MARK,
            fanotify_fd,
            flags,
            mask,
            fd,
            filename_ptr,
        )
        .map(drop)
    }
}

/// Change working directory.
pub fn fchdir(fd: i32) -> Result<(), Errno> {
    unsafe {
        let fd = fd as usize;
        syscall1(SYS_FCHDIR, fd).map(drop)
    }
}

/// Change permissions of a file.
pub fn fchmod(fd: i32, mode: mode_t) -> Result<(), Errno> {
    unsafe {
        let fd = fd as usize;
        let mode = mode as usize;
        syscall2(SYS_FCHMOD, fd, mode).map(drop)
    }
}

/// Change permissions of a file.
pub fn fchmodat(dirfd: i32, filename: &str, mode: mode_t) -> Result<(), Errno> {
    unsafe {
        let dirfd = dirfd as usize;
        let filename = CString::new(filename);
        let filename_ptr = filename.as_ptr() as usize;
        let mode = mode as usize;
        syscall3(SYS_FCHMODAT, dirfd, filename_ptr, mode).map(drop)
    }
}

/// Change ownership of a file.
pub fn fchown(fd: i32, user: uid_t, group: gid_t) -> Result<(), Errno> {
    unsafe {
        let fd = fd as usize;
        let user = user as usize;
        let group = group as usize;
        syscall3(SYS_FCHOWN, fd, user, group).map(drop)
    }
}

/// Change ownership of a file.
pub fn fchownat(
    dirfd: i32,
    filename: &str,
    user: uid_t,
    group: gid_t,
    flag: i32,
) -> Result<(), Errno> {
    unsafe {
        let dirfd = dirfd as usize;
        let filename = CString::new(filename);
        let filename_ptr = filename.as_ptr() as usize;
        let user = user as usize;
        let group = group as usize;
        let flag = flag as usize;
        syscall5(SYS_FCHOWNAT, dirfd, filename_ptr, user, group, flag).map(drop)
    }
}

/// manipulate file descriptor.
pub fn fcntl(fd: i32, cmd: i32, arg: usize) -> Result<i32, Errno> {
    unsafe {
        let fd = fd as usize;
        let cmd = cmd as usize;
        syscall3(SYS_FCNTL, fd, cmd, arg).map(|ret| ret as i32)
    }
}

/// Manipulate file descriptor.
pub fn fcntl64(fd: i32, cmd: i32, arg: usize) -> Result<i32, Errno> {
    unsafe {
        let fd = fd as usize;
        let cmd = cmd as usize;
        syscall3(SYS_FCNTL64, fd, cmd, arg).map(|ret| ret as i32)
    }
}

/// Flush all modified in-core data (exclude metadata) refered by `fd` to disk.
pub fn fdatasync(fd: i32) -> Result<(), Errno> {
    unsafe {
        let fd = fd as usize;
        syscall1(SYS_FDATASYNC, fd).map(drop)
    }
}

/// Get extended attribute value.
pub fn fgetxattr(fd: i32, name: &str, value: usize, size: size_t) -> Result<ssize_t, Errno> {
    unsafe {
        let fd = fd as usize;
        let name_ptr = name.as_ptr() as usize;
        let size = size as usize;
        syscall4(SYS_FGETXATTR, fd, name_ptr, value, size).map(|ret| ret as ssize_t)
    }
}

/// Load a kernel module.
pub fn finit_module(fd: i32, param_values: &str, flags: i32) -> Result<(), Errno> {
    unsafe {
        let fd = fd as usize;
        let param_values = CString::new(param_values);
        let param_values_ptr = param_values.as_ptr() as usize;
        let flags = flags as usize;
        syscall3(SYS_FINIT_MODULE, fd, param_values_ptr, flags).map(drop)
    }
}

/// List extended attribute names.
pub fn flistxattr(fd: i32, list: &mut [u8]) -> Result<ssize_t, Errno> {
    unsafe {
        let fd = fd as usize;
        let list_ptr = list.as_mut_ptr() as usize;
        let len = list.len();
        syscall3(SYS_FLISTXATTR, fd, list_ptr, len).map(|ret| ret as ssize_t)
    }
}

/// Apply or remove an advisory lock on an open file.
pub fn flock(fd: i32, operation: i32) -> Result<(), Errno> {
    unsafe {
        let fd = fd as usize;
        let operation = operation as usize;
        syscall2(SYS_FLOCK, fd, operation).map(drop)
    }
}

/// Create a child process.
pub fn fork() -> Result<pid_t, Errno> {
    unsafe { syscall0(SYS_FORK).map(|ret| ret as pid_t) }
}

/// Remove an extended attribute.
pub fn fremovexattr(fd: i32, name: &str) -> Result<(), Errno> {
    unsafe {
        let fd = fd as usize;
        let name_ptr = name.as_ptr() as usize;
        syscall2(SYS_FREMOVEXATTR, fd, name_ptr).map(drop)
    }
}

/// Set parameters and trigger actions on a context.
pub fn fsconfig(fd: i32, cmd: u32, key: &str, value: &str, aux: i32) -> Result<(), Errno> {
    unsafe {
        let fd = fd as usize;
        let cmd = cmd as usize;
        let key = CString::new(key);
        let key_ptr = key.as_ptr() as usize;
        let value = CString::new(value);
        let value_ptr = value.as_ptr() as usize;
        let aux = aux as usize;
        syscall5(SYS_FSCONFIG, fd, cmd, key_ptr, value_ptr, aux).map(drop)
    }
}

/// Set extended attribute value.
pub fn fsetxattr(fd: i32, name: &str, value: usize, size: size_t) -> Result<ssize_t, Errno> {
    unsafe {
        let fd = fd as usize;
        let name_ptr = name.as_ptr() as usize;
        let size = size as usize;
        syscall4(SYS_FSETXATTR, fd, name_ptr, value, size).map(|ret| ret as ssize_t)
    }
}

/// Create a kernel mount representation for a new, prepared superblock.
pub fn fsmount(fs_fd: i32, flags: u32, attr_flags: u32) -> Result<i32, Errno> {
    unsafe {
        let fs_fd = fs_fd as usize;
        let flags = flags as usize;
        let attr_flags = attr_flags as usize;
        syscall3(SYS_FSMOUNT, fs_fd, flags, attr_flags).map(|ret| ret as i32)
    }
}

/// Open a filesystem by name so that it can be configured for mounting.
pub fn fsopen(fs_name: &str, flags: u32) -> Result<(), Errno> {
    unsafe {
        let fs_name = CString::new(fs_name);
        let fs_name_ptr = fs_name.as_ptr() as usize;
        let flags = flags as usize;
        syscall2(SYS_FSOPEN, fs_name_ptr, flags).map(drop)
    }
}

/// Pick a superblock into a context for reconfiguration.
pub fn fspick(dfd: i32, path: &str, flags: i32) -> Result<i32, Errno> {
    unsafe {
        let dfd = dfd as usize;
        let path = CString::new(path);
        let path_ptr = path.as_ptr() as usize;
        let flags = flags as usize;
        syscall3(SYS_FSPICK, dfd, path_ptr, flags).map(|ret| ret as i32)
    }
}

/// Get file status about a file descriptor.
pub fn fstat(fd: i32, statbuf: &mut stat_t) -> Result<(), Errno> {
    unsafe {
        let fd = fd as usize;
        let statbuf_ptr = statbuf as *mut stat_t as usize;
        syscall2(SYS_FSTAT, fd, statbuf_ptr).map(drop)
    }
}

/// Get file status.
pub fn fstat64(fd: i32, statbuf: &mut stat64_t) -> Result<(), Errno> {
    unsafe {
        let fd = fd as usize;
        let statbuf_ptr = statbuf as *mut stat64_t as usize;
        syscall2(SYS_FSTAT64, fd, statbuf_ptr).map(drop)
    }
}

/// Get file status.
pub fn fstatat64(dfd: i32, filename: &str, statbuf: &mut stat64_t, flag: i32) -> Result<(), Errno> {
    unsafe {
        let dfd = dfd as usize;
        let filename = CString::new(filename);
        let filename_ptr = filename.as_ptr() as usize;
        let statbuf_ptr = statbuf as *mut stat64_t as usize;
        let flag = flag as usize;
        syscall4(SYS_FSTATAT64, dfd, filename_ptr, statbuf_ptr, flag).map(drop)
    }
}

/// Get filesystem statistics.
pub fn fstatfs(fd: i32, buf: &mut statfs_t) -> Result<(), Errno> {
    unsafe {
        let fd = fd as usize;
        let buf_ptr = buf as *mut statfs_t as usize;
        syscall2(SYS_FSTATFS, fd, buf_ptr).map(drop)
    }
}

/// Get filesystem statistics.
pub fn fstatfs64(fd: i32, buf: &mut statfs64_t) -> Result<(), Errno> {
    unsafe {
        let fd = fd as usize;
        let buf_ptr = buf as *mut statfs64_t as usize;
        syscall2(SYS_FSTATFS64, fd, buf_ptr).map(drop)
    }
}

/// Flush all modified in-core data refered by `fd` to disk.
pub fn fsync(fd: i32) -> Result<(), Errno> {
    unsafe {
        let fd = fd as usize;
        syscall1(SYS_FSYNC, fd).map(drop)
    }
}

/// Return date and time.
/// DEPRECATED.
pub fn ftime() {
    core::unimplemented!();
    // syscall0(SYS_FTIME);
}

/// Truncate an opened file to a specified length.
pub fn ftruncate(fd: i32, length: off_t) -> Result<(), Errno> {
    unsafe {
        let fd = fd as usize;
        let length = length as usize;
        syscall2(SYS_FTRUNCATE, fd, length).map(drop)
    }
}

/// Truncate a file to a specific length.
pub fn ftruncate64(fd: i32, len: loff_t) -> Result<(), Errno> {
    unsafe {
        let fd = fd as usize;
        let len = len as usize;
        syscall2(SYS_FTRUNCATE64, fd, len).map(drop)
    }
}

/// Fast user-space locking.
pub fn futex(
    uaddr: &mut i32,
    futex_op: i32,
    val: u32,
    timeout: &mut timespec_t,
    uaddr2: &mut i32,
    val3: i32,
) -> Result<i32, Errno> {
    unsafe {
        let uaddr_ptr = uaddr as *mut i32 as usize;
        let futex_op = futex_op as usize;
        let val = val as usize;
        let timeout_ptr = timeout as *mut timespec_t as usize;
        let uaddr2_ptr = uaddr2 as *mut i32 as usize;
        let val3 = val3 as usize;
        syscall6(
            SYS_FUTEX,
            uaddr_ptr,
            futex_op,
            val,
            timeout_ptr,
            uaddr2_ptr,
            val3,
        )
        .map(|ret| ret as i32)
    }
}

pub fn futex_time64() {
    core::unimplemented!();
    // syscall0(SYS_FUTEX_TIME64);
}

/// Change timestamp of a file relative to a directory file discriptor.
pub fn futimesat(dirfd: i32, filename: &str, times: &[timeval_t; 2]) -> Result<(), Errno> {
    unsafe {
        let dirfd = dirfd as usize;
        let filename = CString::new(filename);
        let filename_ptr = filename.as_ptr() as usize;
        let times_ptr = times.as_ptr() as usize;
        syscall3(SYS_FUTIMESAT, dirfd, filename_ptr, times_ptr).map(drop)
    }
}

/// Determine CPU and NUMA node on which the calling thread is running.
pub fn getcpu(cpu: &mut u32, node: &mut u32, cache: &mut getcpu_cache_t) -> Result<(), Errno> {
    unsafe {
        let cpu_ptr = cpu as *mut u32 as usize;
        let node_ptr = node as *mut u32 as usize;
        let cache_ptr = cache as *mut getcpu_cache_t as usize;
        syscall3(SYS_GETCPU, cpu_ptr, node_ptr, cache_ptr).map(drop)
    }
}

/// Get current working directory.
// TODO(Shaohua): Convert path to string.
pub fn getcwd() -> Result<Vec<u8>, Errno> {
    unsafe {
        let buf_len = (PATH_MAX + 1) as usize;
        let buf = CString::with_capacity(buf_len);
        let buf_ptr = buf.as_ptr() as usize;
        syscall2(SYS_GETCWD, buf_ptr, buf_len).map(|_ret| buf.strim_into_bytes())
    }
}

/// Get directory entries.
/// Deprecated. Use `getdents64()` instead.
pub fn getdents() {
    core::unimplemented!();
    // syscall0(SYS_GETDENTS);
}

/// Get directory entries.
pub fn getdents64(fd: i32) -> Result<Vec<linux_dirent64_extern_t>, Errno> {
    const BUF_SIZE: usize = 4096;
    unsafe {
        let buf: Vec<u8> = vec![0; BUF_SIZE];
        let buf_box = buf.into_boxed_slice();
        let buf_box_ptr = alloc::boxed::Box::into_raw(buf_box) as *mut u8 as usize;
        let fd = fd as usize;
        let nread = syscall3(SYS_GETDENTS64, fd, buf_box_ptr, BUF_SIZE)?;
        let mut result: Vec<linux_dirent64_extern_t> = Vec::new();

        if nread == 0 {
            return Ok(result);
        }

        let mut bpos = 0;
        while bpos < nread {
            let d = (buf_box_ptr + bpos) as *mut linux_dirent64_t;
            let mut name_vec: Vec<u8> = vec![];
            for i in 0..PATH_MAX {
                let c = (*d).d_name[i as usize];
                if c == 0 {
                    break;
                }
                name_vec.push(c);
            }
            let name = String::from_utf8(name_vec).unwrap();
            result.push(linux_dirent64_extern_t {
                d_ino: (*d).d_ino,
                d_off: (*d).d_off,
                d_type: (*d).d_type,
                d_name: name,
            });
            bpos = bpos + (*d).d_reclen as usize;
        }
        return Ok(result);
    }
}

/// Get the effective group ID of the calling process.
pub fn getegid() -> gid_t {
    unsafe { syscall0(SYS_GETEGID).expect("getegid() failed") as gid_t }
}

/// Get the effective user ID of the calling process.
pub fn geteuid() -> uid_t {
    unsafe { syscall0(SYS_GETEUID).expect("geteuid() failed") as uid_t }
}

/// Get the real group ID of the calling process.
pub fn getgid() -> gid_t {
    unsafe { syscall0(SYS_GETGID).expect("getgid() failed") as gid_t }
}

/// Get list of supplementary group Ids.
pub fn getgroups(size: i32, group_list: &mut [gid_t]) -> Result<i32, Errno> {
    unsafe {
        let size = size as usize;
        let group_ptr = group_list.as_mut_ptr() as usize;
        syscall2(SYS_GETGROUPS, size, group_ptr).map(|ret| ret as i32)
    }
}

/// Get value of an interval timer.
pub fn getitimer(which: i32, curr_val: &mut itimerval_t) -> Result<(), Errno> {
    unsafe {
        let which = which as usize;
        let curr_val_ptr = curr_val as *mut itimerval_t as usize;
        syscall2(SYS_GETITIMER, which, curr_val_ptr).map(drop)
    }
}

/// Get name of connected peer socket.
pub fn getpeername(
    sockfd: i32,
    addr: &mut sockaddr_in_t,
    addrlen: &mut socklen_t,
) -> Result<(), Errno> {
    unsafe {
        let sockfd = sockfd as usize;
        let addr_ptr = addr as *mut sockaddr_in_t as usize;
        let addrlen_ptr = addrlen as *mut socklen_t as usize;
        syscall3(SYS_GETPEERNAME, sockfd, addr_ptr, addrlen_ptr).map(drop)
    }
}

/// Returns the PGID(process group ID) of the process specified by `pid`.
pub fn getpgid(pid: pid_t) -> Result<pid_t, Errno> {
    unsafe {
        let pid = pid as usize;
        syscall1(SYS_GETPGID, pid).map(|ret| ret as pid_t)
    }
}

/// Get the process group ID of the calling process.
pub fn getpgrp() -> pid_t {
    unsafe { syscall0(SYS_GETPGRP).expect("getpgrp() failed") as pid_t }
}

/// Get the process ID (PID) of the calling process.
pub fn getpid() -> pid_t {
    unsafe { syscall0(SYS_GETPID).expect("getpid() failed") as pid_t }
}

pub fn getpmsg() {
    core::unimplemented!();
    // syscall0(SYS_GETPMSG);
}

/// Get the process ID of the parent of the calling process.
pub fn getppid() -> pid_t {
    unsafe { syscall0(SYS_GETPPID).expect("getppid() failed") as pid_t }
}

/// Get program scheduling priority.
pub fn getpriority(which: i32, who: i32) -> Result<i32, Errno> {
    unsafe {
        let which = which as usize;
        let who = who as usize;
        syscall2(SYS_GETPRIORITY, which, who).map(|ret| {
            let ret = ret as i32;
            if ret > PRIO_MAX {
                return PRIO_MAX - ret;
            }
            ret
        })
    }
}

/// Obtain a series of random bytes.
pub fn getrandom(buf: &mut [u8], flags: u32) -> Result<ssize_t, Errno> {
    unsafe {
        let buf_ptr = buf.as_mut_ptr() as usize;
        let buf_len = buf.len();
        let flags = flags as usize;
        syscall3(SYS_GETRANDOM, buf_ptr, buf_len, flags).map(|ret| ret as ssize_t)
    }
}

/// Get real, effect and saved group ID.
pub fn getresgid(rgid: &mut gid_t, egid: &mut gid_t, sgid: &mut gid_t) -> Result<(), Errno> {
    unsafe {
        let rgid_ptr = rgid as *mut gid_t as usize;
        let egid_ptr = egid as *mut gid_t as usize;
        let sgid_ptr = sgid as *mut gid_t as usize;
        syscall3(SYS_GETRESGID, rgid_ptr, egid_ptr, sgid_ptr).map(drop)
    }
}

/// Get real, effect and saved user ID.
pub fn getresuid(ruid: &mut uid_t, euid: &mut uid_t, suid: &mut uid_t) -> Result<(), Errno> {
    unsafe {
        let ruid_ptr = ruid as *mut uid_t as usize;
        let euid_ptr = euid as *mut uid_t as usize;
        let suid_ptr = suid as *mut uid_t as usize;
        syscall3(SYS_GETRESUID, ruid_ptr, euid_ptr, suid_ptr).map(drop)
    }
}

/// Get resource limit.
pub fn getrlimit(resource: i32, rlim: &mut rlimit_t) -> Result<(), Errno> {
    unsafe {
        let resource = resource as usize;
        let rlim_ptr = rlim as *mut rlimit_t as usize;
        syscall2(SYS_GETRLIMIT, resource, rlim_ptr).map(drop)
    }
}

/// Get resource usage.
pub fn getrusage(who: i32, usage: &mut rusage_t) -> Result<(), Errno> {
    unsafe {
        let who = who as usize;
        let usage_ptr = usage as *mut rusage_t as usize;
        syscall2(SYS_GETRUSAGE, who, usage_ptr).map(drop)
    }
}

/// Get session Id.
pub fn getsid(pid: pid_t) -> pid_t {
    unsafe {
        let pid = pid as usize;
        syscall1(SYS_GETSID, pid).expect("getsid() failed") as pid_t
    }
}

/// Get current address to which the socket `sockfd` is bound.
pub fn getsockname(
    sockfd: i32,
    addr: &mut sockaddr_in_t,
    addrlen: &mut socklen_t,
) -> Result<(), Errno> {
    unsafe {
        let sockfd = sockfd as usize;
        let addr_ptr = addr as *mut sockaddr_in_t as usize;
        let addrlen_ptr = addrlen as *mut socklen_t as usize;
        syscall3(SYS_GETSOCKNAME, sockfd, addr_ptr, addrlen_ptr).map(drop)
    }
}

/// Get options on sockets
pub fn getsockopt(
    sockfd: i32,
    level: i32,
    optname: i32,
    optval: &mut usize,
    optlen: &mut socklen_t,
) -> Result<(), Errno> {
    unsafe {
        let sockfd = sockfd as usize;
        let level = level as usize;
        let optname = optname as usize;
        let optval_ptr = optval as *mut usize as usize;
        let optlen_ptr = optlen as *mut socklen_t as usize;
        syscall5(
            SYS_GETSOCKOPT,
            sockfd,
            level,
            optname,
            optval_ptr,
            optlen_ptr,
        )
        .map(drop)
    }
}

/// Get the caller's thread ID (TID).
pub fn gettid() -> pid_t {
    unsafe { syscall0(SYS_GETTID).expect("getpid() failed") as pid_t }
}

/// Get time.
pub fn gettimeofday(timeval: &mut timeval_t, tz: &mut timezone_t) -> Result<(), Errno> {
    unsafe {
        let timeval_ptr = timeval as *mut timeval_t as usize;
        let tz_ptr = tz as *mut timezone_t as usize;
        syscall2(SYS_GETTIMEOFDAY, timeval_ptr, tz_ptr).map(drop)
    }
}

/// Get the real user ID of the calling process.
pub fn getuid() -> uid_t {
    unsafe { syscall0(SYS_GETUID).expect("getuid() failed") as uid_t }
}

/// Get extended attribute value.
pub fn getxattr(filename: &str, name: &str, value: usize, size: size_t) -> Result<ssize_t, Errno> {
    unsafe {
        let filename = CString::new(filename);
        let filename_ptr = filename.as_ptr() as usize;
        let name = CString::new(name);
        let name_ptr = name.as_ptr() as usize;
        let size = size as usize;
        syscall4(SYS_GETXATTR, filename_ptr, name_ptr, value, size).map(|ret| ret as ssize_t)
    }
}

/// Retrieve exported kernel and module symbols.
/// Deprecated.
pub fn get_kernel_syms() {
    core::unimplemented!();
    // syscall0(SYS_GET_KERNEL_SYMS);
}

/// Retrieve NUMA memory policy for a thread
pub fn get_mempolicy(
    mode: &mut i32,
    nmask: &mut usize,
    maxnode: usize,
    addr: usize,
    flags: usize,
) -> Result<(), Errno> {
    unsafe {
        let mode_ptr = mode as *mut i32 as usize;
        let nmask_ptr = nmask as *mut usize as usize;
        syscall5(SYS_GET_MEMPOLICY, mode_ptr, nmask_ptr, maxnode, addr, flags).map(drop)
    }
}

/// Get list of robust futexes.
// TODO(Shaohua): Fix argument type.
pub fn get_robust_list(
    pid: pid_t,
    head_ptr: &mut usize,
    len_ptr: &mut size_t,
) -> Result<(), Errno> {
    unsafe {
        let pid = pid as usize;
        let head_ptr = head_ptr as *mut usize as usize;
        let len_ptr = len_ptr as *mut size_t as usize;
        syscall3(SYS_GET_ROBUST_LIST, pid, head_ptr, len_ptr).map(drop)
    }
}

pub fn gtty() {
    core::unimplemented!();
    // syscall0(SYS_GTTY);
}

/// Make process 0 idle.
/// Never returns for process 0, and already returns EPERM for a user process.
pub fn idle() -> Result<(), Errno> {
    unsafe { syscall0(SYS_IDLE).map(drop) }
}

/// Load a kernel module.
pub fn init_module(module_image: usize, len: usize, param_values: &str) -> Result<(), Errno> {
    unsafe {
        let param_values = CString::new(param_values);
        let param_values_ptr = param_values.as_ptr() as usize;
        syscall3(SYS_INIT_MODULE, module_image, len, param_values_ptr).map(drop)
    }
}

/// Add a watch to an initialized inotify instance.
pub fn inotify_add_watch(fd: i32, filename: &str, mask: u32) -> Result<i32, Errno> {
    unsafe {
        let fd = fd as usize;
        let filename = CString::new(filename);
        let filename_ptr = filename.as_ptr() as usize;
        let mask = mask as usize;
        syscall3(SYS_INOTIFY_ADD_WATCH, fd, filename_ptr, mask).map(|ret| ret as i32)
    }
}

/// Initialize an inotify instance.
pub fn inotify_init() -> Result<i32, Errno> {
    unsafe { syscall0(SYS_INOTIFY_INIT).map(|ret| ret as i32) }
}

/// Initialize an inotify instance.
pub fn inotify_init1(flags: i32) -> Result<i32, Errno> {
    unsafe {
        let flags = flags as usize;
        syscall1(SYS_INOTIFY_INIT1, flags).map(|ret| ret as i32)
    }
}

/// Remove an existing watch from an inotify instance.
pub fn inotify_rm_watch(fd: i32, wd: i32) -> Result<(), Errno> {
    unsafe {
        let fd = fd as usize;
        let wd = wd as usize;
        syscall2(SYS_INOTIFY_RM_WATCH, fd, wd).map(drop)
    }
}

/// Control device.
pub fn ioctl(fd: i32, cmd: i32, arg: usize) -> Result<(), Errno> {
    unsafe {
        let fd = fd as usize;
        let cmd = cmd as usize;
        syscall3(SYS_IOCTL, fd, cmd, arg).map(drop)
    }
}

/// Set port input/output permissions.
pub fn ioperm(from: usize, num: usize, turn_on: i32) -> Result<(), Errno> {
    unsafe {
        let turn_on = turn_on as usize;
        syscall3(SYS_IOPERM, from, num, turn_on).map(drop)
    }
}

/// Change I/O privilege level.
pub fn iopl(level: i32) -> Result<(), Errno> {
    unsafe {
        let level = level as usize;
        syscall1(SYS_IOPL, level).map(drop)
    }
}

/// Get I/O scheduling class and priority
pub fn ioprio_get(which: i32, who: i32) -> Result<i32, Errno> {
    unsafe {
        let which = which as usize;
        let who = who as usize;
        syscall2(SYS_IOPRIO_GET, which, who).map(|ret| ret as i32)
    }
}

/// Set I/O scheduling class and priority
pub fn ioprio_set(which: i32, who: i32, ioprio: i32) -> Result<(), Errno> {
    unsafe {
        let which = which as usize;
        let who = who as usize;
        let ioprio = ioprio as usize;
        syscall3(SYS_IOPRIO_SET, which, who, ioprio).map(drop)
    }
}

/// Attempts to cancel an iocb previously passed to io_submit.
/// Attempts to cancel an iocb previously passed to io_submit.  If
/// the operation is successfully cancelled, the resulting event is
/// copied into the memory pointed to by result without being placed
/// into the completion queue and 0 is returned.  May fail with
/// -EFAULT if any of the data structures pointed to are invalid.
/// May fail with -EINVAL if aio_context specified by ctx_id is
/// invalid.  May fail with -EAGAIN if the iocb specified was not
/// cancelled.  Will fail with -ENOSYS if not implemented.
pub fn io_cancel(
    ctx_id: aio_context_t,
    iocb: &mut iocb_t,
    result: &mut io_event_t,
) -> Result<(), Errno> {
    unsafe {
        let ctx_id = ctx_id as usize;
        let iocb_ptr = iocb as *mut iocb_t as usize;
        let result_ptr = result as *mut io_event_t as usize;
        syscall3(SYS_IO_CANCEL, ctx_id, iocb_ptr, result_ptr).map(drop)
    }
}

/// Destroy the aio_context specified.  May cancel any outstanding
/// AIOs and block on completion.  Will fail with -ENOSYS if not
/// implemented.  May fail with -EINVAL if the context pointed to is invalid.
pub fn io_destroy(ctx_id: aio_context_t) -> Result<(), Errno> {
    unsafe {
        let ctx_id = ctx_id as usize;
        syscall1(SYS_IO_DESTROY, ctx_id).map(drop)
    }
}

/// Attempts to read at least min_nr events and up to nr events from
/// the completion queue for the aio_context specified by ctx_id. If
/// it succeeds, the number of read events is returned. May fail with
/// -EINVAL if ctx_id is invalid, if min_nr is out of range, if nr is
/// out of range, if timeout is out of range.  May fail with -EFAULT
/// if any of the memory specified is invalid.  May return 0 or
/// < min_nr if the timeout specified by timeout has elapsed
/// before sufficient events are available, where timeout == NULL
/// specifies an infinite timeout. Note that the timeout pointed to by
/// timeout is relative.  Will fail with -ENOSYS if not implemented.
pub fn io_getevents(
    ctx_id: aio_context_t,
    min_nr: isize,
    nr: isize,
    events: &mut io_event_t,
    timeout: &mut timespec_t,
) -> Result<i32, Errno> {
    unsafe {
        let ctx_id = ctx_id as usize;
        let min_nr = min_nr as usize;
        let nr = nr as usize;
        let events_ptr = events as *mut io_event_t as usize;
        let timeout_ptr = timeout as *mut timespec_t as usize;
        syscall5(
            SYS_IO_GETEVENTS,
            ctx_id,
            min_nr,
            nr,
            events_ptr,
            timeout_ptr,
        )
        .map(|ret| ret as i32)
    }
}

/// read asynchronous I/O events from the completion queue
pub fn io_pgetevents(
    ctx_id: aio_context_t,
    min_nr: isize,
    nr: isize,
    events: &mut io_event_t,
    timeout: &mut timespec_t,
    usig: &aio_sigset_t,
) -> Result<i32, Errno> {
    unsafe {
        let ctx_id = ctx_id as usize;
        let min_nr = min_nr as usize;
        let nr = nr as usize;
        let events_ptr = events as *mut io_event_t as usize;
        let timeout_ptr = timeout as *mut timespec_t as usize;
        let usig_ptr = usig as *const aio_sigset_t as usize;
        syscall6(
            SYS_IO_PGETEVENTS,
            ctx_id,
            min_nr,
            nr,
            events_ptr,
            timeout_ptr,
            usig_ptr,
        )
        .map(|ret| ret as i32)
    }
}

pub fn io_pgetevents_time64() {
    core::unimplemented!();
    // syscall0(SYS_IO_PGETEVENTS_TIME64);
}

/// Create an asynchronous I/O context.
/// Create an aio_context capable of receiving at least nr_events.
/// ctxp must not point to an aio_context that already exists, and
/// must be initialized to 0 prior to the call.  On successful
/// creation of the aio_context, *ctxp is filled in with the resulting
/// handle.  May fail with -EINVAL if *ctxp is not initialized,
/// if the specified nr_events exceeds internal limits.  May fail
/// with -EAGAIN if the specified nr_events exceeds the user's limit
/// of available events.  May fail with -ENOMEM if insufficient kernel
/// resources are available.  May fail with -EFAULT if an invalid
/// pointer is passed for ctxp.  Will fail with -ENOSYS if not implemented.
pub fn io_setup(nr_events: u32, ctx_id: &mut aio_context_t) -> Result<(), Errno> {
    unsafe {
        let nr_events = nr_events as usize;
        let ctx_id_ptr = ctx_id as *mut aio_context_t as usize;
        syscall2(SYS_IO_SETUP, nr_events, ctx_id_ptr).map(drop)
    }
}

/// Queue the nr iocbs pointed to by iocbpp for processing.  Returns
/// the number of iocbs queued.  May return -EINVAL if the aio_context
/// specified by ctx_id is invalid, if nr is < 0, if the iocb at
/// *iocbpp[0] is not properly initialized, if the operation specified
/// is invalid for the file descriptor in the iocb.  May fail with
/// -EFAULT if any of the data structures point to invalid data.  May
/// fail with -EBADF if the file descriptor specified in the first
/// iocb is invalid.  May fail with -EAGAIN if insufficient resources
/// are available to queue any iocbs.  Will return 0 if nr is 0.  Will
/// fail with -ENOSYS if not implemented.
// TODO(Shaohua): type of iocbpp is struct iocb**
pub fn io_submit(ctx_id: aio_context_t, nr: isize, iocb: &mut iocb_t) -> Result<i32, Errno> {
    unsafe {
        let ctx_id = ctx_id as usize;
        let nr = nr as usize;
        let iocb_ptr = iocb as *mut iocb_t as usize;
        syscall3(SYS_IO_SUBMIT, ctx_id, nr, iocb_ptr).map(|ret| ret as i32)
    }
}

pub fn io_uring_enter(
    fd: i32,
    to_submit: u32,
    min_complete: u32,
    flags: u32,
    sig: &sigset_t,
    sigsetsize: size_t,
) -> Result<i32, Errno> {
    unsafe {
        let fd = fd as usize;
        let to_submit = to_submit as usize;
        let min_complete = min_complete as usize;
        let flags = flags as usize;
        let sig_ptr = sig as *const sigset_t as usize;
        let sigsetsize = sigsetsize as usize;
        syscall6(
            SYS_IO_URING_ENTER,
            fd,
            to_submit,
            min_complete,
            flags,
            sig_ptr,
            sigsetsize,
        )
        .map(|ret| ret as i32)
    }
}

pub fn io_uring_register(fd: i32, opcode: u32, arg: usize, nr_args: u32) -> Result<i32, Errno> {
    unsafe {
        let fd = fd as usize;
        let opcode = opcode as usize;
        let nr_args = nr_args as usize;
        syscall4(SYS_IO_URING_REGISTER, fd, opcode, arg, nr_args).map(|ret| ret as i32)
    }
}

pub fn io_uring_setup(entries: u32, params: &mut io_uring_params_t) -> Result<i32, Errno> {
    unsafe {
        let entries = entries as usize;
        let params_ptr = params as *mut io_uring_params_t as usize;
        syscall2(SYS_IO_URING_SETUP, entries, params_ptr).map(|ret| ret as i32)
    }
}

/// System V IPC system calls.
pub fn ipc(
    call: u32,
    first: i32,
    second: i32,
    third: i32,
    ptr: usize,
    fifth: isize,
) -> Result<(), Errno> {
    unsafe {
        let call = call as usize;
        let first = first as usize;
        let second = second as usize;
        let third = third as usize;
        let fifth = fifth as usize;
        syscall6(SYS_IPC, call, first, second, third, ptr, fifth).map(drop)
    }
}

/// Compare two processes to determine if they share a kernel resource.
pub fn kcmp(pid1: pid_t, pid2: pid_t, type_: i32, idx1: usize, idx2: usize) -> Result<i32, Errno> {
    unsafe {
        let pid1 = pid1 as usize;
        let pid2 = pid2 as usize;
        let type_ = type_ as usize;
        syscall5(SYS_KCMP, pid1, pid2, type_, idx1, idx2).map(|ret| ret as i32)
    }
}

/// Load a new kernel for later execution.
pub fn kexec_load(
    entry: usize,
    nr_segments: usize,
    segments: &mut kexec_segment_t,
    flags: usize,
) -> Result<(), Errno> {
    unsafe {
        let segments_ptr = segments as *mut kexec_segment_t as usize;
        syscall4(SYS_KEXEC_LOAD, entry, nr_segments, segments_ptr, flags).map(drop)
    }
}

/// Manipulate the kernel's key management facility.
pub fn keyctl(
    operation: i32,
    arg2: usize,
    arg3: usize,
    arg4: usize,
    arg5: usize,
) -> Result<usize, Errno> {
    unsafe {
        let operation = operation as usize;
        syscall5(SYS_KEYCTL, operation, arg2, arg3, arg4, arg5)
    }
}

/// Send signal to a process.
pub fn kill(pid: pid_t, signal: i32) -> Result<(), Errno> {
    unsafe {
        let pid = pid as usize;
        let signal = signal as usize;
        syscall2(SYS_KILL, pid, signal).map(drop)
    }
}

/// Change ownership of a file.
pub fn lchown(filename: &str, user: uid_t, group: gid_t) -> Result<(), Errno> {
    unsafe {
        let filename = CString::new(filename);
        let filename_ptr = filename.as_ptr() as usize;
        let user = user as usize;
        let group = group as usize;
        syscall3(SYS_LCHOWN, filename_ptr, user, group).map(drop)
    }
}

/// Get extended attribute value.
pub fn lgetxattr(filename: &str, name: &str, value: usize, size: size_t) -> Result<ssize_t, Errno> {
    unsafe {
        let filename = CString::new(filename);
        let filename_ptr = filename.as_ptr() as usize;
        let name = CString::new(name);
        let name_ptr = name.as_ptr() as usize;
        let size = size as usize;
        syscall4(SYS_LGETXATTR, filename_ptr, name_ptr, value, size).map(|ret| ret as ssize_t)
    }
}

/// Make a new name for a file.
pub fn link(oldfilename: &str, newfilename: &str) -> Result<(), Errno> {
    unsafe {
        let oldfilename = CString::new(oldfilename);
        let oldfilename_ptr = oldfilename.as_ptr() as usize;
        let newfilename = CString::new(newfilename);
        let newfilename_ptr = newfilename.as_ptr() as usize;
        syscall2(SYS_LINK, oldfilename_ptr, newfilename_ptr).map(drop)
    }
}

/// Make a new name for a file.
pub fn linkat(olddfd: i32, oldfilename: &str, newdfd: i32, newfilename: &str) -> Result<(), Errno> {
    unsafe {
        let olddfd = olddfd as usize;
        let oldfilename = CString::new(oldfilename);
        let oldfilename_ptr = oldfilename.as_ptr() as usize;
        let newdfd = newdfd as usize;
        let newfilename = CString::new(newfilename);
        let newfilename_ptr = newfilename.as_ptr() as usize;
        syscall4(SYS_LINKAT, olddfd, oldfilename_ptr, newdfd, newfilename_ptr).map(drop)
    }
}

/// Listen for connections on a socket.
pub fn listen(sockfd: i32, backlog: i32) -> Result<(), Errno> {
    unsafe {
        let sockfd = sockfd as usize;
        let backlog = backlog as usize;
        syscall2(SYS_LISTEN, sockfd, backlog).map(drop)
    }
}

/// List extended attribute names.
pub fn listxattr(filename: &str, list: &mut [u8]) -> Result<ssize_t, Errno> {
    unsafe {
        let filename = CString::new(filename);
        let filename_ptr = filename.as_ptr() as usize;
        let list_ptr = list.as_mut_ptr() as usize;
        let len = list.len();
        syscall3(SYS_LISTXATTR, filename_ptr, list_ptr, len).map(|ret| ret as ssize_t)
    }
}

/// List extended attribute names.
pub fn llistxattr(filename: &str, list: &mut [u8]) -> Result<ssize_t, Errno> {
    unsafe {
        let filename = CString::new(filename);
        let filename_ptr = filename.as_ptr() as usize;
        let list_ptr = list.as_mut_ptr() as usize;
        let len = list.len();
        syscall3(SYS_LLISTXATTR, filename_ptr, list_ptr, len).map(|ret| ret as ssize_t)
    }
}

pub fn lock() {
    core::unimplemented!();
    // syscall0(SYS_LOCK);
}

/// Return a directory entry's path.
// TODO(Shaohua): Returns a string.
pub fn lookup_dcookie(cookie: u64, buf: &mut [u8]) -> Result<i32, Errno> {
    unsafe {
        let cookie = cookie as usize;
        let buf_ptr = buf.as_mut_ptr() as usize;
        let buf_len = buf.len();
        syscall3(SYS_LOOKUP_DCOOKIE, cookie, buf_ptr, buf_len).map(|ret| ret as i32)
    }
}

/// Remove an extended attribute.
pub fn lremovexattr(filename: &str, name: &str) -> Result<(), Errno> {
    unsafe {
        let filename = CString::new(filename);
        let filename_ptr = filename.as_ptr() as usize;
        let name = CString::new(name);
        let name_ptr = name.as_ptr() as usize;
        syscall2(SYS_LREMOVEXATTR, filename_ptr, name_ptr).map(drop)
    }
}

/// Reposition file offset.
pub fn lseek(fd: i32, offset: off_t, whence: i32) -> Result<(), Errno> {
    unsafe {
        let fd = fd as usize;
        let offset = offset as usize;
        let whence = whence as usize;
        syscall3(SYS_LSEEK, fd, offset, whence).map(drop)
    }
}

/// Set extended attribute value.
pub fn lsetxattr(filename: &str, name: &str, value: usize, size: size_t) -> Result<ssize_t, Errno> {
    unsafe {
        let filename = CString::new(filename);
        let filename_ptr = filename.as_ptr() as usize;
        let name = CString::new(name);
        let name_ptr = name.as_ptr() as usize;
        let size = size as usize;
        syscall4(SYS_LSETXATTR, filename_ptr, name_ptr, value, size).map(|ret| ret as ssize_t)
    }
}

/// Get file status about a file, without following symbolic.
pub fn lstat(filename: &str, statbuf: &mut stat_t) -> Result<(), Errno> {
    unsafe {
        let filename = CString::new(filename);
        let filename_ptr = filename.as_ptr() as usize;
        let statbuf_ptr = statbuf as *mut stat_t as usize;
        syscall2(SYS_LSTAT, filename_ptr, statbuf_ptr).map(drop)
    }
}

/// Get file status about a file, without following symbolic.
pub fn lstat64(filename: &str, statbuf: &mut stat64_t) -> Result<(), Errno> {
    unsafe {
        let filename = CString::new(filename);
        let filename_ptr = filename.as_ptr() as usize;
        let statbuf_ptr = statbuf as *mut stat64_t as usize;
        syscall2(SYS_LSTAT64, filename_ptr, statbuf_ptr).map(drop)
    }
}

/// Give advice about use of memory.
pub fn madvise(addr: usize, len: size_t, advice: i32) -> Result<(), Errno> {
    unsafe {
        let len = len as usize;
        let advice = advice as usize;
        syscall3(SYS_MADVISE, addr, len, advice).map(drop)
    }
}

/// Set memory policy for a memory range.
pub fn mbind(
    start: usize,
    len: usize,
    mode: i32,
    nmask: *const usize,
    maxnode: usize,
    flags: i32,
) -> Result<(), Errno> {
    unsafe {
        let mode = mode as usize;
        let nmask = nmask as usize;
        let flags = flags as usize;
        syscall6(SYS_MBIND, start, len, mode, nmask, maxnode, flags).map(drop)
    }
}

/// sys_membarrier - issue memory barriers on a set of threads
/// @cmd:   Takes command values defined in enum membarrier_cmd.
/// @flags: Currently needs to be 0. For future extensions.
///
/// If this system call is not implemented, -ENOSYS is returned. If the
/// command specified does not exist, not available on the running
/// kernel, or if the command argument is invalid, this system call
/// returns -EINVAL. For a given command, with flags argument set to 0,
/// this system call is guaranteed to always return the same value until
/// reboot.
///
/// All memory accesses performed in program order from each targeted thread
/// is guaranteed to be ordered with respect to sys_membarrier(). If we use
/// the semantic "barrier()" to represent a compiler barrier forcing memory
/// accesses to be performed in program order across the barrier, and
/// smp_mb() to represent explicit memory barriers forcing full memory
/// ordering across the barrier, we have the following ordering table for
/// each pair of barrier(), sys_membarrier() and smp_mb():
///
/// The pair ordering is detailed as (O: ordered, X: not ordered):
///
/// ```text
///                        barrier()   smp_mb() sys_membarrier()
///        barrier()          X           X            O
///        smp_mb()           X           O            O
///        sys_membarrier()   O           O            O
/// ```
pub fn membarrier(cmd: i32, flags: i32) -> Result<i32, Errno> {
    unsafe {
        let cmd = cmd as usize;
        let flags = flags as usize;
        syscall2(SYS_MEMBARRIER, cmd, flags).map(|ret| ret as i32)
    }
}

/// Create an anonymous file.
pub fn memfd_create(name: &str, flags: u32) -> Result<i32, Errno> {
    unsafe {
        let name = CString::new(name);
        let name_ptr = name.as_ptr() as usize;
        let flags = flags as usize;
        syscall2(SYS_MEMFD_CREATE, name_ptr, flags).map(|ret| ret as i32)
    }
}

/// Move all pages in a process to another set of nodes
pub fn migrate_pages(
    pid: pid_t,
    maxnode: usize,
    old_nodes: *const usize,
    new_nodes: *const usize,
) -> Result<isize, Errno> {
    unsafe {
        let pid = pid as usize;
        let old_nodes = old_nodes as usize;
        let new_nodes = new_nodes as usize;
        syscall4(SYS_MIGRATE_PAGES, pid, maxnode, old_nodes, new_nodes).map(|ret| ret as isize)
    }
}

/// mincore() returns the memory residency status of the pages in the
/// current process's address space specified by [addr, addr + len).
/// The status is returned in a vector of bytes.  The least significant
/// bit of each byte is 1 if the referenced page is in memory, otherwise
/// it is zero.
///
/// Because the status of a page can change after mincore() checks it
/// but before it returns to the application, the returned vector may
/// contain stale information.  Only locked pages are guaranteed to
/// remain in memory.
///
/// return values:
///  zero    - success
///  -EFAULT - vec points to an illegal address
///  -EINVAL - addr is not a multiple of PAGE_SIZE
///  -ENOMEM - Addresses in the range [addr, addr + len] are
///  	invalid for the address space of this process, or
///  	specify one or more pages which are not currently
///  	mapped
///  -EAGAIN - A kernel resource was temporarily unavailable.
pub fn mincore(start: usize, len: size_t, vec: *const u8) -> Result<(), Errno> {
    unsafe {
        let len = len as usize;
        let vec_ptr = vec as usize;
        syscall3(SYS_MINCORE, start, len, vec_ptr).map(drop)
    }
}

/// Create a directory.
pub fn mkdir(filename: &str, mode: mode_t) -> Result<(), Errno> {
    unsafe {
        let filename = CString::new(filename);
        let filename_ptr = filename.as_ptr() as usize;
        let mode = mode as usize;
        syscall2(SYS_MKDIR, filename_ptr, mode).map(drop)
    }
}

/// Create a directory.
pub fn mkdirat(dirfd: i32, filename: &str, mode: mode_t) -> Result<(), Errno> {
    unsafe {
        let dirfd = dirfd as usize;
        let filename = CString::new(filename);
        let filename_ptr = filename.as_ptr() as usize;
        let mode = mode as usize;
        syscall3(SYS_MKDIRAT, dirfd, filename_ptr, mode).map(drop)
    }
}

/// Create a special or ordinary file.
pub fn mknod(filename: &str, mode: mode_t, dev: dev_t) -> Result<(), Errno> {
    unsafe {
        let filename = CString::new(filename);
        let filename_ptr = filename.as_ptr() as usize;
        let mode = mode as usize;
        let dev = dev as usize;
        syscall3(SYS_MKNOD, filename_ptr, mode, dev).map(drop)
    }
}

/// Create a special or ordinary file.
pub fn mknodat(dirfd: i32, filename: &str, mode: mode_t, dev: dev_t) -> Result<(), Errno> {
    unsafe {
        let dirfd = dirfd as usize;
        let filename = CString::new(filename);
        let filename_ptr = filename.as_ptr() as usize;
        let mode = mode as usize;
        let dev = dev as usize;
        syscall4(SYS_MKNODAT, dirfd, filename_ptr, mode, dev).map(drop)
    }
}

/// Lock memory.
pub fn mlock(addr: usize, len: size_t) -> Result<(), Errno> {
    unsafe {
        let len = len as usize;
        syscall2(SYS_MLOCK, addr, len).map(drop)
    }
}

/// Lock memory.
pub fn mlock2(addr: usize, len: size_t, flags: i32) -> Result<(), Errno> {
    unsafe {
        let len = len as usize;
        let flags = flags as usize;
        syscall3(SYS_MLOCK2, addr, len, flags).map(drop)
    }
}

/// Lock memory.
pub fn mlockall(flags: i32) -> Result<(), Errno> {
    unsafe {
        let flags = flags as usize;
        syscall1(SYS_MLOCKALL, flags).map(drop)
    }
}

/// Map files or devices into memory.
pub fn mmap(
    start: usize,
    len: size_t,
    prot: i32,
    flags: i32,
    fd: i32,
    offset: off_t,
) -> Result<usize, Errno> {
    unsafe {
        let len = len as usize;
        let prot = prot as usize;
        let flags = flags as usize;
        let fd = fd as usize;
        let offset = offset as usize;
        syscall6(SYS_MMAP, start, len, prot, flags, fd, offset)
    }
}

/// Map files or devices into memory.
pub fn mmap2(
    start: usize,
    len: size_t,
    prot: i32,
    flags: i32,
    fd: i32,
    offset: off_t,
) -> Result<usize, Errno> {
    unsafe {
        let len = len as usize;
        let prot = prot as usize;
        let flags = flags as usize;
        let fd = fd as usize;
        let offset = offset as usize;
        syscall6(SYS_MMAP2, start, len, prot, flags, fd, offset)
    }
}

// FIXME(Shaohua):
pub fn modify_ldt() {
    core::unimplemented!();
    // syscall0(SYS_MODIFY_LDT);
}

/// Mount filesystem.
pub fn mount(
    dev_name: &str,
    dir_name: &str,
    fs_type: &str,
    flags: usize,
    data: usize,
) -> Result<(), Errno> {
    unsafe {
        let dev_name_ptr = dev_name.as_ptr() as usize;
        let dir_name_ptr = dir_name.as_ptr() as usize;
        let fs_type_ptr = fs_type.as_ptr() as usize;
        syscall5(
            SYS_MOUNT,
            dev_name_ptr,
            dir_name_ptr,
            fs_type_ptr,
            flags,
            data,
        )
        .map(drop)
    }
}

/// Move a mount from one place to another. In combination with
/// fsopen()/fsmount() this is used to install a new mount and in combination
/// with open_tree(OPEN_TREE_CLONE [| AT_RECURSIVE]) it can be used to copy
/// a mount subtree.
///
/// Note the flags value is a combination of MOVE_MOUNT_* flags.
pub fn move_mount(
    from_dfd: i32,
    from_pathname: &str,
    to_dfd: i32,
    to_pathname: &str,
    flags: u32,
) -> Result<i32, Errno> {
    unsafe {
        let from_dfd = from_dfd as usize;
        let from_pathname = CString::new(from_pathname);
        let from_pathname_ptr = from_pathname.as_ptr() as usize;
        let to_dfd = to_dfd as usize;
        let to_pathname = CString::new(to_pathname);
        let to_pathname_ptr = to_pathname.as_ptr() as usize;
        let flags = flags as usize;
        syscall5(
            SYS_MOVE_MOUNT,
            from_dfd,
            from_pathname_ptr,
            to_dfd,
            to_pathname_ptr,
            flags,
        )
        .map(|ret| ret as i32)
    }
}

/// Move individual pages of a process to another node
pub fn move_pages(
    pid: pid_t,
    nr_pages: usize,
    pages: usize,
    nodes: *const i32,
    status: &mut i32,
    flags: i32,
) -> Result<(), Errno> {
    unsafe {
        let pid = pid as usize;
        let nodes_ptr = nodes as usize;
        let status = status as *mut i32 as usize;
        let flags = flags as usize;
        syscall6(
            SYS_MOVE_PAGES,
            pid,
            nr_pages,
            pages,
            nodes_ptr,
            status,
            flags,
        )
        .map(drop)
    }
}

/// Set protection on a region of memory.
pub fn mprotect(addr: usize, len: size_t, prot: i32) -> Result<(), Errno> {
    unsafe {
        let len = len as usize;
        let prot = prot as usize;
        syscall3(SYS_MPROTECT, addr, len, prot).map(drop)
    }
}

pub fn mpx() {
    core::unimplemented!();
    // syscall0(SYS_MPX);
}

/// Get/set message queue attributes
pub fn mq_getsetattr(
    mqdes: mqd_t,
    new_attr: &mut mq_attr_t,
    old_attr: &mut mq_attr_t,
) -> Result<mqd_t, Errno> {
    unsafe {
        let mqdes = mqdes as usize;
        let new_attr_ptr = new_attr as *mut mq_attr_t as usize;
        let old_attr_ptr = old_attr as *mut mq_attr_t as usize;
        syscall3(SYS_MQ_GETSETATTR, mqdes, new_attr_ptr, old_attr_ptr).map(|ret| ret as mqd_t)
    }
}

/// Register for notification when a message is available
pub fn mq_notify(mqdes: mqd_t, notification: &sigevent_t) -> Result<(), Errno> {
    unsafe {
        let mqdes = mqdes as usize;
        let notification_ptr = notification as *const sigevent_t as usize;
        syscall2(SYS_MQ_NOTIFY, mqdes, notification_ptr).map(drop)
    }
}

/// Open a message queue.
pub fn mq_open(
    name: &str,
    oflag: i32,
    mode: umode_t,
    attr: &mut mq_attr_t,
) -> Result<mqd_t, Errno> {
    unsafe {
        let name = CString::new(name);
        let name_ptr = name.as_ptr() as usize;
        let oflag = oflag as usize;
        let mode = mode as usize;
        let attr_ptr = attr as *mut mq_attr_t as usize;
        syscall4(SYS_MQ_OPEN, name_ptr, oflag, mode, attr_ptr).map(|ret| ret as mqd_t)
    }
}

/// Receive a message from a message queue
pub fn mq_timedreceive(
    mqdes: mqd_t,
    msg: &str,
    msg_prio: u32,
    abs_timeout: &timespec_t,
) -> Result<ssize_t, Errno> {
    unsafe {
        let mqdes = mqdes as usize;
        let msg = CString::new(msg);
        let msg_ptr = msg.as_ptr() as usize;
        let msg_len = msg.len();
        let msg_prio = msg_prio as usize;
        let abs_timeout_ptr = abs_timeout as *const timespec_t as usize;
        syscall5(
            SYS_MQ_TIMEDRECEIVE,
            mqdes,
            msg_ptr,
            msg_len,
            msg_prio,
            abs_timeout_ptr,
        )
        .map(|ret| ret as ssize_t)
    }
}

pub fn mq_timedreceive_time64() {
    core::unimplemented!();
    // syscall0(SYS_MQ_TIMEDRECEIVE_TIME64);
}

/// Send message to a message queue
pub fn mq_timedsend(
    mqdes: mqd_t,
    msg: &str,
    msg_prio: u32,
    abs_timeout: &timespec_t,
) -> Result<(), Errno> {
    unsafe {
        let mqdes = mqdes as usize;
        let msg = CString::new(msg);
        let msg_ptr = msg.as_ptr() as usize;
        let msg_len = msg.len();
        let msg_prio = msg_prio as usize;
        let abs_timeout_ptr = abs_timeout as *const timespec_t as usize;
        syscall5(
            SYS_MQ_TIMEDSEND,
            mqdes,
            msg_ptr,
            msg_len,
            msg_prio,
            abs_timeout_ptr,
        )
        .map(drop)
    }
}

pub fn mq_timedsend_time64() {
    core::unimplemented!();
    // syscall0(SYS_MQ_TIMEDSEND_TIME64);
}

/// Remove a message queue
pub fn mq_unlink(name: &str) -> Result<(), Errno> {
    unsafe {
        let name = CString::new(name);
        let name_ptr = name.as_ptr() as usize;
        syscall1(SYS_MQ_UNLINK, name_ptr).map(drop)
    }
}

/// Remap a virtual memory address
pub fn mremap(
    addr: usize,
    old_len: size_t,
    new_len: size_t,
    flags: usize,
    new_addr: usize,
) -> Result<usize, Errno> {
    unsafe {
        let old_len = old_len as usize;
        let new_len = new_len as usize;
        syscall5(SYS_MREMAP, addr, old_len, new_len, flags, new_addr)
    }
}

/// System V message control operations.
pub fn msgctl(msqid: i32, cmd: i32, buf: &mut msqid_ds_t) -> Result<i32, Errno> {
    unsafe {
        let msqid = msqid as usize;
        let cmd = cmd as usize;
        let buf_ptr = buf as *mut msqid_ds_t as usize;
        syscall3(SYS_MSGCTL, msqid, cmd, buf_ptr).map(|ret| ret as i32)
    }
}

/// Get a System V message queue identifier.
pub fn msgget(key: key_t, msgflg: i32) -> Result<i32, Errno> {
    unsafe {
        let key = key as usize;
        let msgflg = msgflg as usize;
        syscall2(SYS_MSGGET, key, msgflg).map(|ret| ret as i32)
    }
}

/// Receive messages from a System V message queue.
pub fn msgrcv(msqid: i32, msgq: usize, msgsz: size_t, msgtyp: isize) -> Result<ssize_t, Errno> {
    unsafe {
        let msqid = msqid as usize;
        let msgsz = msgsz as usize;
        let msgtyp = msgtyp as usize;
        syscall4(SYS_MSGRCV, msqid, msgq, msgsz, msgtyp).map(|ret| ret as ssize_t)
    }
}

/// Append the message to a System V message queue.
pub fn msgsnd(msqid: i32, msgq: usize, msgsz: size_t, msgflg: i32) -> Result<(), Errno> {
    unsafe {
        let msqid = msqid as usize;
        let msgsz = msgsz as usize;
        let msgflg = msgflg as usize;
        syscall4(SYS_MSGSND, msqid, msgq, msgsz, msgflg).map(drop)
    }
}

/// Synchronize a file with memory map.
pub fn msync(addr: usize, len: size_t, flags: i32) -> Result<(), Errno> {
    unsafe {
        let len = len as usize;
        let flags = flags as usize;
        syscall3(SYS_MSYNC, addr, len, flags).map(drop)
    }
}

/// Unlock memory.
pub fn munlock(addr: usize, len: size_t) -> Result<(), Errno> {
    unsafe {
        let len = len as usize;
        syscall2(SYS_MUNLOCK, addr, len).map(drop)
    }
}

/// Unlock memory.
pub fn munlockall() -> Result<(), Errno> {
    unsafe { syscall0(SYS_MUNLOCKALL).map(drop) }
}

/// Unmap files or devices from memory.
pub fn munmap(addr: usize, len: size_t) -> Result<(), Errno> {
    unsafe {
        let len = len as usize;
        syscall2(SYS_MUNMAP, addr, len).map(drop)
    }
}

/// Obtain handle for a filename
pub fn name_to_handle_at(
    dfd: i32,
    filename: &str,
    handle: &mut file_handle_t,
    mount_id: &mut i32,
    flags: i32,
) -> Result<(), Errno> {
    unsafe {
        let dfd = dfd as usize;
        let filename = CString::new(filename);
        let filename_ptr = filename.as_ptr() as usize;
        let handle_ptr = handle as *mut file_handle_t as usize;
        let mount_id_ptr = mount_id as *mut i32 as usize;
        let flags = flags as usize;
        syscall5(
            SYS_NAME_TO_HANDLE_AT,
            dfd,
            filename_ptr,
            handle_ptr,
            mount_id_ptr,
            flags,
        )
        .map(drop)
    }
}

/// High resolution sleep.
pub fn nanosleep(req: &timespec_t, rem: &mut timespec_t) -> Result<(), Errno> {
    unsafe {
        let req_ptr = req as *const timespec_t as usize;
        let rem_ptr = rem as *mut timespec_t as usize;
        syscall2(SYS_NANOSLEEP, req_ptr, rem_ptr).map(drop)
    }
}

/// Syscall interface to kernel nfs daemon.
/// Deprecated.
pub fn nfsservctl() {
    core::unimplemented!();
    // syscall0(SYS_NFSSERVCTL);
}

/// Change the priority of current process.
pub fn nice(increment: i32) -> Result<i32, Errno> {
    unsafe {
        let increment = increment as usize;
        syscall1(SYS_NICE, increment).map(|ret| ret as i32)
    }
}

/// Open and possibly create a file.
pub fn open(filename: &str, flags: i32, mode: mode_t) -> Result<i32, Errno> {
    unsafe {
        let filename = CString::new(filename);
        let filename_ptr = filename.as_ptr() as usize;
        let flags = flags as usize;
        let mode = mode as usize;
        syscall3(SYS_OPEN, filename_ptr, flags, mode).map(|ret| ret as i32)
    }
}

/// Open and possibly create a file within a directory.
pub fn openat(dirfd: i32, filename: &str, flags: i32, mode: mode_t) -> Result<i32, Errno> {
    unsafe {
        let dirfd = dirfd as usize;
        let filename = CString::new(filename);
        let filename_ptr = filename.as_ptr() as usize;
        let flags = flags as usize;
        let mode = mode as usize;
        syscall4(SYS_OPENAT, dirfd, filename_ptr, flags, mode).map(|ret| ret as i32)
    }
}

/// Obtain handle for an open file
pub fn open_by_handle_at(
    mount_fd: i32,
    handle: &mut file_handle_t,
    flags: i32,
) -> Result<(), Errno> {
    unsafe {
        let mount_fd = mount_fd as usize;
        let handle_ptr = handle as *mut file_handle_t as usize;
        let flags = flags as usize;
        syscall3(SYS_OPEN_BY_HANDLE_AT, mount_fd, handle_ptr, flags).map(drop)
    }
}

pub fn open_tree(dfd: i32, filename: &str, flags: u32) -> Result<i32, Errno> {
    unsafe {
        let dfd = dfd as usize;
        let filename = CString::new(filename);
        let filename_ptr = filename.as_ptr() as usize;
        let flags = flags as usize;
        syscall3(SYS_OPEN_TREE, dfd, filename_ptr, flags).map(|ret| ret as i32)
    }
}

// Pause the calling process to sleep until a signal is delivered.
pub fn pause() -> Result<(), Errno> {
    unsafe { syscall0(SYS_PAUSE).map(drop) }
}

/// Set up performance monitoring.
pub fn perf_event_open(
    attr: &mut perf_event_attr_t,
    pid: pid_t,
    cpu: i32,
    group_fd: i32,
    flags: usize,
) -> Result<i32, Errno> {
    unsafe {
        let attr_ptr = attr as *mut perf_event_attr_t as usize;
        let pid = pid as usize;
        let cpu = cpu as usize;
        let group_fd = group_fd as usize;
        syscall5(SYS_PERF_EVENT_OPEN, attr_ptr, pid, cpu, group_fd, flags).map(|ret| ret as i32)
    }
}

/// Set the process execution domain.
pub fn personality(persona: u32) -> Result<u32, Errno> {
    unsafe {
        let persona = persona as usize;
        syscall1(SYS_PERSONALITY, persona).map(|ret| ret as u32)
    }
}

/// sys_pidfd_send_signal - Signal a process through a pidfd
/// @pidfd:  file descriptor of the process
/// @sig:    signal to send
/// @info:   signal info
/// @flags:  future flags
///
/// The syscall currently only signals via PIDTYPE_PID which covers
/// kill(<positive-pid>, <signal>. It does not signal threads or process
/// groups.
/// In order to extend the syscall to threads and process groups the @flags
/// argument should be used. In essence, the @flags argument will determine
/// what is signaled and not the file descriptor itself. Put in other words,
/// grouping is a property of the flags argument not a property of the file
/// descriptor.
///
/// Return: 0 on success, negative errno on failure
pub fn pidfd_send_signal(
    pidfd: i32,
    sig: i32,
    info: &mut siginfo_t,
    flags: u32,
) -> Result<(), Errno> {
    unsafe {
        let pidfd = pidfd as usize;
        let sig = sig as usize;
        let info_ptr = info as *mut siginfo_t as usize;
        let flags = flags as usize;
        syscall4(SYS_PIDFD_SEND_SIGNAL, pidfd, sig, info_ptr, flags).map(drop)
    }
}

/// Create a pipe
pub fn pipe(pipefd: &mut [i32; 2]) -> Result<(), Errno> {
    unsafe {
        let pipefd_ptr = pipefd.as_mut_ptr() as usize;
        syscall1(SYS_PIPE, pipefd_ptr).map(drop)
    }
}

/// Create a pipe.
pub fn pipe2(pipefd: &mut [i32; 2], flags: i32) -> Result<(), Errno> {
    unsafe {
        let pipefd_ptr = pipefd.as_mut_ptr() as usize;
        let flags = flags as usize;
        syscall2(SYS_PIPE2, pipefd_ptr, flags).map(drop)
    }
}

/// Change the root filesystem.
pub fn pivot_root(new_root: &str, put_old: &str) -> Result<(), Errno> {
    unsafe {
        let new_root_ptr = new_root.as_ptr() as usize;
        let put_old_ptr = put_old.as_ptr() as usize;
        syscall2(SYS_PIVOT_ROOT, new_root_ptr, put_old_ptr).map(drop)
    }
}

/// Create a new protection key.
pub fn pkey_alloc(flags: usize, init_val: usize) -> Result<i32, Errno> {
    unsafe { syscall2(SYS_PKEY_ALLOC, flags, init_val).map(|ret| ret as i32) }
}

/// Free a protection key.
pub fn pkey_free(pkey: i32) -> Result<(), Errno> {
    unsafe {
        let pkey = pkey as usize;
        syscall1(SYS_PKEY_FREE, pkey).map(drop)
    }
}

/// Set protection on a region of memory.
pub fn pkey_mprotect(start: usize, len: size_t, prot: usize, pkey: i32) -> Result<(), Errno> {
    unsafe {
        let len = len as usize;
        let pkey = pkey as usize;
        syscall4(SYS_PKEY_MPROTECT, start, len, prot, pkey).map(drop)
    }
}

/// Wait for some event on file descriptors.
pub fn poll(fds: &mut [pollfd_t], timeout: i32) -> Result<(), Errno> {
    unsafe {
        let fds_ptr = fds.as_mut_ptr() as usize;
        let nfds = fds.len() as usize;
        let timeout = timeout as usize;
        syscall3(SYS_POLL, fds_ptr, nfds, timeout).map(drop)
    }
}

/// Wait for some event on a file descriptor.
pub fn ppoll(
    fds: &mut pollfd_t,
    nfds: i32,
    timeout: &timespec_t,
    sigmask: &sigset_t,
    sigsetsize: size_t,
) -> Result<i32, Errno> {
    unsafe {
        let fds_ptr = fds as *mut pollfd_t as usize;
        let nfds = nfds as usize;
        let timeout_ptr = timeout as *const timespec_t as usize;
        let sigmask_ptr = sigmask as *const sigset_t as usize;
        let sigsetsize = sigsetsize as usize;
        syscall5(
            SYS_PPOLL,
            fds_ptr,
            nfds,
            timeout_ptr,
            sigmask_ptr,
            sigsetsize,
        )
        .map(|ret| ret as i32)
    }
}

pub fn ppoll_time64() {
    core::unimplemented!();
    // syscall0(SYS_PPOLL_TIME64);
}

/// Operations on a process.
pub fn prctl(
    option: i32,
    arg2: usize,
    arg3: usize,
    arg4: usize,
    arg5: usize,
) -> Result<i32, Errno> {
    unsafe {
        let option = option as usize;
        let arg2 = arg2 as usize;
        let arg3 = arg3 as usize;
        let arg4 = arg4 as usize;
        let arg5 = arg5 as usize;
        syscall5(SYS_PRCTL, option, arg2, arg3, arg4, arg5).map(|ret| ret as i32)
    }
}

/// Read from a file descriptor without changing file offset.
pub fn pread64(fd: i32, buf: &mut [u8], offset: off_t) -> Result<ssize_t, Errno> {
    unsafe {
        let fd = fd as usize;
        let buf_ptr = buf.as_ptr() as usize;
        let len = buf.len() as usize;
        let offset = offset as usize;
        syscall4(SYS_PREAD64, fd, buf_ptr, len, offset).map(|ret| ret as ssize_t)
    }
}

/// Read from a file descriptor without changing file offset.
pub fn preadv(fd: i32, vec: &[iovec_t], pos_l: usize, pos_h: usize) -> Result<ssize_t, Errno> {
    unsafe {
        let fd = fd as usize;
        let vec_ptr = vec.as_ptr() as usize;
        let vec_len = vec.len();
        syscall5(SYS_PREADV, fd, vec_ptr, vec_len, pos_l, pos_h).map(|ret| ret as ssize_t)
    }
}

/// Read from a file descriptor without changing file offset.
pub fn preadv2(
    fd: i32,
    vec: &[iovec_t],
    pos_l: usize,
    pos_h: usize,
    flags: rwf_t,
) -> Result<ssize_t, Errno> {
    unsafe {
        let fd = fd as usize;
        let vec_ptr = vec.as_ptr() as usize;
        let vec_len = vec.len();
        let flags = flags as usize;
        syscall6(SYS_PREADV2, fd, vec_ptr, vec_len, pos_l, pos_h, flags).map(|ret| ret as ssize_t)
    }
}

/// Get/set the resource limits of an arbitary process.
pub fn prlimit64(
    pid: pid_t,
    resource: i32,
    new_limit: &rlimit_t,
    old_limit: &mut rlimit_t,
) -> Result<(), Errno> {
    unsafe {
        let pid = pid as usize;
        let resource = resource as usize;
        let new_limit_ptr = new_limit as *const rlimit_t as usize;
        let old_limit_ptr = old_limit as *mut rlimit_t as usize;
        syscall4(SYS_PRLIMIT64, pid, resource, new_limit_ptr, old_limit_ptr).map(drop)
    }
}

/// Transfer data between process address spaces
pub fn process_vm_readv(
    pid: pid_t,
    lvec: &[iovec_t],
    rvec: &[iovec_t],
    flags: i32,
) -> Result<ssize_t, Errno> {
    unsafe {
        let pid = pid as usize;
        let lvec_ptr = lvec.as_ptr() as usize;
        let lvec_len = lvec.len();
        let rvec_ptr = rvec.as_ptr() as usize;
        let rvec_len = rvec.len();
        let flags = flags as usize;
        syscall6(
            SYS_PROCESS_VM_READV,
            pid,
            lvec_ptr,
            lvec_len,
            rvec_ptr,
            rvec_len,
            flags,
        )
        .map(|ret| ret as ssize_t)
    }
}

/// Transfer data between process address spaces
pub fn process_vm_writev(
    pid: pid_t,
    lvec: &[iovec_t],
    rvec: &[iovec_t],
    flags: i32,
) -> Result<ssize_t, Errno> {
    unsafe {
        let pid = pid as usize;
        let lvec_ptr = lvec.as_ptr() as usize;
        let lvec_len = lvec.len();
        let rvec_ptr = rvec.as_ptr() as usize;
        let rvec_len = rvec.len();
        let flags = flags as usize;
        syscall6(
            SYS_PROCESS_VM_WRITEV,
            pid,
            lvec_ptr,
            lvec_len,
            rvec_ptr,
            rvec_len,
            flags,
        )
        .map(|ret| ret as ssize_t)
    }
}

pub fn prof() {
    core::unimplemented!();
    // syscall0(SYS_PROF);
}

pub fn profil() {
    core::unimplemented!();
    // syscall0(SYS_PROFIL);
}

/// Sychronous I/O multiplexing.
/// Most architectures can't handle 7-argument syscalls. So we provide a
/// 6-argument version where the sixth argument is a pointer to a structure
/// which has a pointer to the sigset_t itself followed by a size_t containing
/// the sigset size.
pub fn pselect6(
    nfds: i32,
    readfds: &mut fd_set_t,
    writefds: &mut fd_set_t,
    exceptfds: &mut fd_set_t,
    timeout: &timespec_t,
    sigmask: &sigset_t,
) -> Result<i32, Errno> {
    unsafe {
        let nfds = nfds as usize;
        let readfds_ptr = readfds as *mut fd_set_t as usize;
        let writefds_ptr = writefds as *mut fd_set_t as usize;
        let exceptfds_ptr = exceptfds as *mut fd_set_t as usize;
        let timeout_ptr = timeout as *const timespec_t as usize;
        let sigmask_ptr = sigmask as *const sigset_t as usize;
        syscall6(
            SYS_PSELECT6,
            nfds,
            readfds_ptr,
            writefds_ptr,
            exceptfds_ptr,
            timeout_ptr,
            sigmask_ptr,
        )
        .map(|ret| ret as i32)
    }
}

pub fn pselect6_time64() {
    core::unimplemented!();
    // syscall0(SYS_PSELECT6_TIME64);
}

/// Process trace.
pub fn ptrace(request: i32, pid: pid_t, addr: usize, data: usize) -> Result<isize, Errno> {
    unsafe {
        let request = request as usize;
        let pid = pid as usize;
        syscall4(SYS_PTRACE, request, pid, addr, data).map(|ret| ret as isize)
    }
}

pub fn putpmsg() {
    core::unimplemented!();
    // syscall0(SYS_PUTPMSG);
}

/// Write to a file descriptor without changing file offset.
pub fn pwrite64(fd: i32, buf: &[u8], offset: off_t) -> Result<ssize_t, Errno> {
    unsafe {
        let fd = fd as usize;
        let buf_ptr = buf.as_ptr() as usize;
        let len = buf.len() as usize;
        let offset = offset as usize;
        syscall4(SYS_PWRITE64, fd, buf_ptr, len, offset).map(|ret| ret as ssize_t)
    }
}

/// Write to a file descriptor without changing file offset.
pub fn pwritev(
    fd: i32,
    vec: &iovec_t,
    vlen: usize,
    pos_l: usize,
    pos_h: usize,
) -> Result<ssize_t, Errno> {
    unsafe {
        let fd = fd as usize;
        let vec_ptr = vec as *const iovec_t as usize;
        syscall5(SYS_PWRITEV, fd, vec_ptr, vlen, pos_l, pos_h).map(|ret| ret as ssize_t)
    }
}

/// Write to a file descriptor without changing file offset.
pub fn pwritev2(
    fd: i32,
    vec: &iovec_t,
    vlen: usize,
    pos_l: usize,
    pos_h: usize,
    flags: rwf_t,
) -> Result<ssize_t, Errno> {
    unsafe {
        let fd = fd as usize;
        let vec_ptr = vec as *const iovec_t as usize;
        let flags = flags as usize;
        syscall6(SYS_PWRITEV2, fd, vec_ptr, vlen, pos_l, pos_h, flags).map(|ret| ret as ssize_t)
    }
}

pub fn query_module() {
    core::unimplemented!();
    // syscall0(SYS_QUERY_MODULE);
}

/// Manipulate disk quotes.
pub fn quotactl(cmd: i32, special: &str, id: qid_t, addr: usize) -> Result<(), Errno> {
    unsafe {
        let cmd = cmd as usize;
        let special = CString::new(special);
        let special_ptr = special.as_ptr() as usize;
        let id = id as usize;
        syscall4(SYS_QUOTACTL, cmd, special_ptr, id, addr).map(drop)
    }
}

/// Read from a file descriptor.
pub fn read(fd: i32, buf: &mut [u8]) -> Result<ssize_t, Errno> {
    unsafe {
        let fd = fd as usize;
        let buf_ptr = buf.as_mut_ptr() as usize;
        let len = buf.len() as usize;
        syscall3(SYS_READ, fd, buf_ptr, len).map(|ret| ret as ssize_t)
    }
}

/// Initialize file head into page cache.
pub fn readahead(fd: i32, offset: off_t, count: size_t) -> Result<(), Errno> {
    unsafe {
        let fd = fd as usize;
        let offset = offset as usize;
        let count = count as usize;
        syscall3(SYS_READAHEAD, fd, offset, count).map(drop)
    }
}

pub fn readdir() {
    core::unimplemented!();
    // syscall0(SYS_READDIR);
}

/// Read value of a symbolic link.
pub fn readlink(filename: &str, buf: &mut [u8]) -> Result<ssize_t, Errno> {
    unsafe {
        let filename = CString::new(filename);
        let filename_ptr = filename.as_ptr() as usize;
        let buf_ptr = buf.as_mut_ptr() as usize;
        let buf_len = buf.len();
        syscall3(SYS_READLINK, filename_ptr, buf_ptr, buf_len).map(|ret| ret as ssize_t)
    }
}

/// Read value of a symbolic link.
pub fn readlinkat(dirfd: i32, filename: &str, buf: &mut [u8]) -> Result<ssize_t, Errno> {
    unsafe {
        let dirfd = dirfd as usize;
        let filename = CString::new(filename);
        let filename_ptr = filename.as_ptr() as usize;
        let buf_ptr = buf.as_mut_ptr() as usize;
        let buf_len = buf.len();
        syscall4(SYS_READLINKAT, dirfd, filename_ptr, buf_ptr, buf_len).map(|ret| ret as ssize_t)
    }
}

/// Read from a file descriptor into multiple buffers.
pub fn readv(fd: i32, iov: &mut [iovec_t]) -> Result<ssize_t, Errno> {
    unsafe {
        let fd = fd as usize;
        let iov_ptr = iov.as_mut_ptr() as usize;
        let len = iov.len() as usize;
        syscall3(SYS_READV, fd, iov_ptr, len).map(|ret| ret as ssize_t)
    }
}

/// Reboot or enable/disable Ctrl-Alt-Del.
pub fn reboot(magic: i32, magci2: i32, cmd: u32, arg: usize) -> Result<(), Errno> {
    unsafe {
        let magic = magic as usize;
        let magic2 = magci2 as usize;
        let cmd = cmd as usize;
        syscall4(SYS_REBOOT, magic, magic2, cmd, arg).map(drop)
    }
}

/// Receive a datagram from a socket.
pub fn recv(sockfd: i32, buf: &mut [u8], flags: i32) -> Result<ssize_t, Errno> {
    unsafe {
        let sockfd = sockfd as usize;
        let buf_ptr = buf.as_mut_ptr() as usize;
        let buflen = buf.len();
        let flags = flags as usize;
        syscall4(SYS_RECV, sockfd, buf_ptr, buflen, flags).map(|ret| ret as ssize_t)
    }
}

/// Receive a message from a socket.
pub fn recvfrom(
    sockfd: i32,
    buf: &mut [u8],
    flags: i32,
    src_addr: &mut sockaddr_in_t,
    addrlen: &mut socklen_t,
) -> Result<ssize_t, Errno> {
    unsafe {
        let sockfd = sockfd as usize;
        let buf_ptr = buf.as_mut_ptr() as usize;
        let buflen = buf.len();
        let flags = flags as usize;
        let src_addr_ptr = src_addr as *mut sockaddr_in_t as usize;
        let addrlen_ptr = addrlen as *mut socklen_t as usize;
        syscall6(
            SYS_RECVFROM,
            sockfd,
            buf_ptr,
            buflen,
            flags,
            src_addr_ptr,
            addrlen_ptr,
        )
        .map(|ret| ret as ssize_t)
    }
}

/// Receives multile messages on a socket
pub fn recvmmsg(
    sockfd: i32,
    msgvec: &mut [mmsghdr_t],
    flags: i32,
    timeout: &mut timespec_t,
) -> Result<i32, Errno> {
    unsafe {
        let sockfd = sockfd as usize;
        let msgvec_ptr = msgvec as *mut [mmsghdr_t] as *mut mmsghdr_t as usize;
        let vlen = msgvec.len() as usize;
        let flags = flags as usize;
        let timeout_ptr = timeout as *mut timespec_t as usize;
        syscall5(SYS_RECVMMSG, sockfd, msgvec_ptr, vlen, flags, timeout_ptr).map(|ret| ret as i32)
    }
}

pub fn recvmmsg_time64() {
    core::unimplemented!();
    // syscall0(SYS_RECVMMSG_TIME64);
}

/// Receive a msg from a socket.
pub fn recvmsg(sockfd: i32, msg: &mut msghdr_t, flags: i32) -> Result<ssize_t, Errno> {
    unsafe {
        let sockfd = sockfd as usize;
        let msg_ptr = msg as *mut msghdr_t as usize;
        let flags = flags as usize;
        syscall3(SYS_RECVMSG, sockfd, msg_ptr, flags).map(|ret| ret as ssize_t)
    }
}

/// Create a nonlinear file mapping.
/// Deprecated.
pub fn remap_file_pages(
    start: usize,
    size: size_t,
    prot: i32,
    pgoff: off_t,
    flags: i32,
) -> Result<(), Errno> {
    unsafe {
        let size = size as usize;
        let prot = prot as usize;
        let pgoff = pgoff as usize;
        let flags = flags as usize;
        syscall5(SYS_REMAP_FILE_PAGES, start, size, prot, pgoff, flags).map(drop)
    }
}

/// Remove an extended attribute.
pub fn removexattr(filename: &str, name: &str) -> Result<(), Errno> {
    unsafe {
        let filename = CString::new(filename);
        let filename_ptr = filename.as_ptr() as usize;
        let name_ptr = name.as_ptr() as usize;
        syscall2(SYS_REMOVEXATTR, filename_ptr, name_ptr).map(drop)
    }
}

/// Change name or location of a file.
pub fn rename(oldfilename: &str, newfilename: &str) -> Result<(), Errno> {
    unsafe {
        let oldfilename = CString::new(oldfilename);
        let oldfilename_ptr = oldfilename.as_ptr() as usize;
        let newfilename = CString::new(newfilename);
        let newfilename_ptr = newfilename.as_ptr() as usize;
        syscall2(SYS_RENAME, oldfilename_ptr, newfilename_ptr).map(drop)
    }
}

/// Change name or location of a file.
pub fn renameat(
    olddfd: i32,
    oldfilename: &str,
    newdfd: i32,
    newfilename: &str,
) -> Result<(), Errno> {
    unsafe {
        let olddfd = olddfd as usize;
        let oldfilename = CString::new(oldfilename);
        let oldfilename_ptr = oldfilename.as_ptr() as usize;
        let newdfd = newdfd as usize;
        let newfilename = CString::new(newfilename);
        let newfilename_ptr = newfilename.as_ptr() as usize;
        syscall4(
            SYS_RENAMEAT,
            olddfd,
            oldfilename_ptr,
            newdfd,
            newfilename_ptr,
        )
        .map(drop)
    }
}

/// Change name or location of a file.
pub fn renameat2(
    olddfd: i32,
    oldfilename: &str,
    newdfd: i32,
    newfilename: &str,
    flags: i32,
) -> Result<(), Errno> {
    unsafe {
        let olddfd = olddfd as usize;
        let oldfilename = CString::new(oldfilename);
        let oldfilename_ptr = oldfilename.as_ptr() as usize;
        let newdfd = newdfd as usize;
        let newfilename = CString::new(newfilename);
        let newfilename_ptr = newfilename.as_ptr() as usize;
        let flags = flags as usize;
        syscall5(
            SYS_RENAMEAT2,
            olddfd,
            oldfilename_ptr,
            newdfd,
            newfilename_ptr,
            flags,
        )
        .map(drop)
    }
}

/// Request a key from kernel's key management facility.
pub fn request_key(
    type_: &str,
    description: &str,
    callout_info: &str,
    dest_keyring: key_serial_t,
) -> Result<key_serial_t, Errno> {
    unsafe {
        let type_ = CString::new(type_);
        let type_ptr = type_.as_ptr() as usize;
        let description = CString::new(description);
        let description_ptr = description.as_ptr() as usize;
        let callout_info = CString::new(callout_info);
        let callout_info_ptr = callout_info.as_ptr() as usize;
        let dest_keyring = dest_keyring as usize;
        syscall4(
            SYS_REQUEST_KEY,
            type_ptr,
            description_ptr,
            callout_info_ptr,
            dest_keyring,
        )
        .map(|ret| ret as key_serial_t)
    }
}

pub fn reserved221() {
    core::unimplemented!();
    // syscall0(SYS_RESERVED221);
}

pub fn reserved82() {
    core::unimplemented!();
    // syscall0(SYS_RESERVED82);
}

/// Restart a system call after interruption by a stop signal.
pub fn restart_syscall() -> Result<i32, Errno> {
    unsafe { syscall0(SYS_RESTART_SYSCALL).map(|ret| ret as i32) }
}

/// Delete a directory.
pub fn rmdir(filename: &str) -> Result<(), Errno> {
    unsafe {
        let filename = CString::new(filename);
        let filename_ptr = filename.as_ptr() as usize;
        syscall1(SYS_RMDIR, filename_ptr).map(drop)
    }
}

/// Setup restartable sequences for caller thread.
pub fn rseq(rseq: &mut [rseq_t], flags: i32, sig: u32) -> Result<i32, Errno> {
    unsafe {
        let rseq_ptr = rseq.as_mut_ptr() as usize;
        let rseq_len = rseq.len();
        let flags = flags as usize;
        let sig = sig as usize;
        syscall4(SYS_RSEQ, rseq_ptr, rseq_len, flags, sig).map(|ret| ret as i32)
    }
}

/// Examine and change a signal action.
pub fn rt_sigaction(
    sig: i32,
    act: &sigaction_t,
    old_act: &mut sigaction_t,
    sigsetsize: size_t,
) -> Result<(), Errno> {
    unsafe {
        let sig = sig as usize;
        let act_ptr = act as *const sigaction_t as usize;
        let old_act_ptr = old_act as *mut sigaction_t as usize;
        let sigsetsize = sigsetsize as usize;
        syscall4(SYS_RT_SIGACTION, sig, act_ptr, old_act_ptr, sigsetsize).map(drop)
    }
}

/// Examine pending signals.
pub fn rt_sigpending(set: &mut [sigset_t]) -> Result<(), Errno> {
    unsafe {
        let set_ptr = set.as_mut_ptr() as usize;
        syscall1(SYS_RT_SIGPENDING, set_ptr).map(drop)
    }
}

/// Change the list of currently blocked signals.
pub fn rt_sigprocmask(how: i32, set: &sigset_t, oldset: &mut sigset_t) -> Result<(), Errno> {
    unsafe {
        let how = how as usize;
        let set_ptr = set as *const sigset_t as usize;
        let oldset_ptr = oldset as *mut sigset_t as usize;
        syscall3(SYS_RT_SIGPROCMASK, how, set_ptr, oldset_ptr).map(drop)
    }
}

/// Queue a signal and data.
pub fn rt_sigqueueinfo(pid: pid_t, sig: i32, uinfo: &mut siginfo_t) -> Result<(), Errno> {
    unsafe {
        let pid = pid as usize;
        let sig = sig as usize;
        let uinfo_ptr = uinfo as *mut siginfo_t as usize;
        syscall3(SYS_RT_SIGQUEUEINFO, pid, sig, uinfo_ptr).map(drop)
    }
}

/// Return from signal handler and cleanup stack frame.
/// Never returns.
pub fn rt_sigreturn() {
    unsafe {
        let _ = syscall0(SYS_RT_SIGRETURN);
    }
}

/// Wait for a signal.
/// Always returns Errno, normally EINTR.
pub fn rt_sigsuspend(set: &mut sigset_t, sigsetsize: size_t) -> Result<(), Errno> {
    unsafe {
        let set_ptr = set as *mut sigset_t as usize;
        let sigsetsize = sigsetsize as usize;
        syscall2(SYS_RT_SIGSUSPEND, set_ptr, sigsetsize).map(drop)
    }
}

/// Synchronously wait for queued signals.
pub fn rt_sigtimedwait(
    uthese: &sigset_t,
    uinfo: &mut siginfo_t,
    uts: &timespec_t,
    sigsetsize: size_t,
) -> Result<i32, Errno> {
    unsafe {
        let uthese_ptr = uthese as *const sigset_t as usize;
        let uinfo_ptr = uinfo as *mut siginfo_t as usize;
        let uts_ptr = uts as *const timespec_t as usize;
        let sigsetsize = sigsetsize as usize;
        syscall4(
            SYS_RT_SIGTIMEDWAIT,
            uthese_ptr,
            uinfo_ptr,
            uts_ptr,
            sigsetsize,
        )
        .map(|ret| ret as i32)
    }
}

pub fn rt_sigtimedwait_time64() {
    core::unimplemented!();
    // syscall0(SYS_RT_SIGTIMEDWAIT_TIME64);
}

/// Queue a signal and data.
pub fn rt_tgsigqueueinfo(
    tgid: pid_t,
    tid: pid_t,
    sig: i32,
    uinfo: &mut siginfo_t,
) -> Result<(), Errno> {
    unsafe {
        let tgid = tgid as usize;
        let tid = tid as usize;
        let sig = sig as usize;
        let uinfo_ptr = uinfo as *mut siginfo_t as usize;
        syscall4(SYS_RT_TGSIGQUEUEINFO, tgid, tid, sig, uinfo_ptr).map(drop)
    }
}

/// Get a thread's CPU affinity mask.
pub fn sched_getaffinity(pid: pid_t, len: u32, user_mask: &mut usize) -> Result<(), Errno> {
    unsafe {
        let pid = pid as usize;
        let len = len as usize;
        let user_mask_ptr = user_mask as *mut usize as usize;
        syscall3(SYS_SCHED_GETAFFINITY, pid, len, user_mask_ptr).map(drop)
    }
}

/// Get scheduling policy and attributes
pub fn sched_getattr(
    pid: pid_t,
    attr: &mut sched_attr_t,
    size: u32,
    flags: u32,
) -> Result<(), Errno> {
    unsafe {
        let pid = pid as usize;
        let attr_ptr = attr as *mut sched_attr_t as usize;
        let size = size as usize;
        let flags = flags as usize;
        syscall4(SYS_SCHED_GETATTR, pid, attr_ptr, size, flags).map(drop)
    }
}

/// Get scheduling paramters.
pub fn sched_getparam(pid: pid_t, param: &mut sched_param_t) -> Result<(), Errno> {
    unsafe {
        let pid = pid as usize;
        let param_ptr = param as *mut sched_param_t as usize;
        syscall2(SYS_SCHED_GETPARAM, pid, param_ptr).map(drop)
    }
}

/// Get scheduling parameter.
pub fn sched_getscheduler(pid: pid_t) -> Result<i32, Errno> {
    unsafe {
        let pid = pid as usize;
        syscall1(SYS_SCHED_GETSCHEDULER, pid).map(|ret| ret as i32)
    }
}

/// Get static priority max value.
pub fn sched_get_priority_max(policy: i32) -> Result<i32, Errno> {
    unsafe {
        let policy = policy as usize;
        syscall1(SYS_SCHED_GET_PRIORITY_MAX, policy).map(|ret| ret as i32)
    }
}

/// Get static priority min value.
pub fn sched_get_priority_min(policy: i32) -> Result<i32, Errno> {
    unsafe {
        let policy = policy as usize;
        syscall1(SYS_SCHED_GET_PRIORITY_MIN, policy).map(|ret| ret as i32)
    }
}

/// Get the SCHED_RR interval for the named process.
pub fn sched_rr_get_interval(pid: pid_t, interval: &mut timespec_t) -> Result<(), Errno> {
    unsafe {
        let pid = pid as usize;
        let interval_ptr = interval as *mut timespec_t as usize;
        syscall2(SYS_SCHED_RR_GET_INTERVAL, pid, interval_ptr).map(drop)
    }
}

pub fn sched_rr_get_interval_time64() {
    core::unimplemented!();
    // syscall0(SYS_SCHED_RR_GET_INTERVAL_TIME64);
}

/// Set a thread's CPU affinity mask.
pub fn sched_setaffinity(pid: pid_t, len: u32, user_mask: &mut usize) -> Result<(), Errno> {
    unsafe {
        let pid = pid as usize;
        let len = len as usize;
        let user_mask_ptr = user_mask as *mut usize as usize;
        syscall3(SYS_SCHED_SETAFFINITY, pid, len, user_mask_ptr).map(drop)
    }
}

/// Set the RT priority of a thread.
pub fn sched_setattr(pid: pid_t, attr: &mut sched_attr_t, flags: u32) -> Result<(), Errno> {
    unsafe {
        let pid = pid as usize;
        let attr_ptr = attr as *mut sched_attr_t as usize;
        let flags = flags as usize;
        syscall3(SYS_SCHED_SETATTR, pid, attr_ptr, flags).map(drop)
    }
}

/// Set scheduling paramters.
pub fn sched_setparam(pid: pid_t, param: &sched_param_t) -> Result<(), Errno> {
    unsafe {
        let pid = pid as usize;
        let param_ptr = param as *const sched_param_t as usize;
        syscall2(SYS_SCHED_SETPARAM, pid, param_ptr).map(drop)
    }
}

/// Set scheduling parameter.
pub fn sched_setscheduler(pid: pid_t, policy: i32, param: &sched_param_t) -> Result<(), Errno> {
    unsafe {
        let pid = pid as usize;
        let policy = policy as usize;
        let param_ptr = param as *const sched_param_t as usize;
        syscall3(SYS_SCHED_SETSCHEDULER, pid, policy, param_ptr).map(drop)
    }
}

/// Yield the processor.
pub fn sched_yield() -> Result<(), Errno> {
    unsafe { syscall0(SYS_SCHED_YIELD).map(drop) }
}

/// Operate on Secure Computing state of the process.
pub fn seccomp(operation: u32, flags: u32, args: usize) -> Result<(), Errno> {
    unsafe {
        let operation = operation as usize;
        let flags = flags as usize;
        syscall3(SYS_SECCOMP, operation, flags, args).map(drop)
    }
}

/// System V semaphore control operations
pub fn semctl(semid: i32, semnum: i32, cmd: i32, arg: usize) -> Result<i32, Errno> {
    unsafe {
        let semid = semid as usize;
        let semnum = semnum as usize;
        let cmd = cmd as usize;
        syscall4(SYS_SEMCTL, semid, semnum, cmd, arg).map(|ret| ret as i32)
    }
}

/// Get a System V semphore set identifier.
pub fn semget(key: key_t, nsems: i32, semflg: i32) -> Result<i32, Errno> {
    unsafe {
        let key = key as usize;
        let nsems = nsems as usize;
        let semflg = semflg as usize;
        syscall3(SYS_SEMGET, key, nsems, semflg).map(|ret| ret as i32)
    }
}

pub fn semtimedop_time64() {
    core::unimplemented!();
    // syscall0(SYS_SEMTIMEDOP_TIME64);
}

/// Send a message on a socket.
pub fn send(sockfd: i32, buf: &[u8], len: size_t, flags: i32) -> Result<ssize_t, Errno> {
    unsafe {
        let sockfd = sockfd as usize;
        let buf_ptr = buf.as_ptr() as usize;
        let len = len as usize;
        let flags = flags as usize;
        syscall4(SYS_SEND, sockfd, buf_ptr, len, flags).map(|ret| ret as ssize_t)
    }
}

/// Transfer data between two file descriptors.
pub fn sendfile(
    out_fd: i32,
    in_fd: i32,
    offset: &mut off_t,
    count: size_t,
) -> Result<ssize_t, Errno> {
    unsafe {
        let out_fd = out_fd as usize;
        let in_fd = in_fd as usize;
        let offset_ptr = offset as *mut off_t as usize;
        let count = count as usize;
        syscall4(SYS_SENDFILE, out_fd, in_fd, offset_ptr, count).map(|ret| ret as ssize_t)
    }
}

/// Transfer data between file descriptors.
pub fn sendfile64(
    out_fd: i32,
    in_fd: i32,
    offset: loff_t,
    count: size_t,
) -> Result<ssize_t, Errno> {
    unsafe {
        let out_fd = out_fd as usize;
        let in_fd = in_fd as usize;
        let offset = offset as usize;
        let count = count as usize;
        syscall4(SYS_SENDFILE64, out_fd, in_fd, offset, count).map(|ret| ret as ssize_t)
    }
}

/// Send multiple messages on a socket
pub fn sendmmsg(sockfd: i32, msgvec: &mut [mmsghdr_t], flags: i32) -> Result<i32, Errno> {
    unsafe {
        let sockfd = sockfd as usize;
        let msgvec_ptr = msgvec as *mut [mmsghdr_t] as *mut mmsghdr_t as usize;
        let vlen = msgvec.len() as usize;
        let flags = flags as usize;
        syscall4(SYS_SENDMMSG, sockfd, msgvec_ptr, vlen, flags).map(|ret| ret as i32)
    }
}

/// Send a message on a socket. Allow sending ancillary data.
pub fn sendmsg(sockfd: i32, msg: &msghdr_t, flags: i32) -> Result<ssize_t, Errno> {
    unsafe {
        let sockfd = sockfd as usize;
        let msg_ptr = msg as *const msghdr_t as usize;
        let flags = flags as usize;
        syscall3(SYS_SENDMSG, sockfd, msg_ptr, flags).map(|ret| ret as ssize_t)
    }
}

/// Send a message on a socket.
pub fn sendto(
    sockfd: i32,
    buf: &[u8],
    len: size_t,
    flags: i32,
    dest_addr: &sockaddr_in_t,
    addrlen: socklen_t,
) -> Result<ssize_t, Errno> {
    unsafe {
        let sockfd = sockfd as usize;
        let buf_ptr = buf.as_ptr() as usize;
        let len = len as usize;
        let flags = flags as usize;
        let dest_addr_ptr = dest_addr as *const sockaddr_in_t as usize;
        let addrlen = addrlen as usize;
        syscall6(
            SYS_SENDTO,
            sockfd,
            buf_ptr,
            len,
            flags,
            dest_addr_ptr,
            addrlen,
        )
        .map(|ret| ret as ssize_t)
    }
}

/// Set NIS domain name.
pub fn setdomainname(name: &str) -> Result<(), Errno> {
    unsafe {
        let name = CString::new(name);
        let name_ptr = name.as_ptr() as usize;
        let name_len = name.len() as usize;
        syscall2(SYS_SETDOMAINNAME, name_ptr, name_len).map(drop)
    }
}

/// Set group identify used for filesystem checkes.
pub fn setfsgid(fsgid: gid_t) -> Result<gid_t, Errno> {
    unsafe {
        let fsgid = fsgid as usize;
        syscall1(SYS_SETFSGID, fsgid).map(|ret| ret as gid_t)
    }
}

/// Set user identify used for filesystem checkes.
pub fn setfsuid(fsuid: uid_t) -> Result<uid_t, Errno> {
    unsafe {
        let fsuid = fsuid as usize;
        syscall1(SYS_SETFSUID, fsuid).map(|ret| ret as uid_t)
    }
}

/// Set the group ID of the calling process to `gid`.
pub fn setgid(gid: gid_t) -> Result<(), Errno> {
    unsafe {
        let gid = gid as usize;
        syscall1(SYS_SETGID, gid).map(drop)
    }
}

/// Set list of supplementary group Ids.
pub fn setgroups(group_list: &[gid_t]) -> Result<(), Errno> {
    unsafe {
        let group_ptr = group_list.as_ptr() as usize;
        let group_len = group_list.len();
        syscall2(SYS_SETGROUPS, group_ptr, group_len).map(drop)
    }
}

/// Set hostname
pub fn sethostname(name: &str) -> Result<(), Errno> {
    unsafe {
        let name_ptr = name.as_ptr() as usize;
        let name_len = name.len();
        syscall2(SYS_SETHOSTNAME, name_ptr, name_len).map(drop)
    }
}

/// Set value of an interval timer.
pub fn setitimer(
    which: i32,
    new_val: &itimerval_t,
    old_val: &mut itimerval_t,
) -> Result<(), Errno> {
    unsafe {
        let which = which as usize;
        let new_val_ptr = new_val as *const itimerval_t as usize;
        let old_val_ptr = old_val as *mut itimerval_t as usize;
        syscall3(SYS_SETITIMER, which, new_val_ptr, old_val_ptr).map(drop)
    }
}

/// Reassociate thread with a namespace.
pub fn setns(fd: i32, nstype: i32) -> Result<(), Errno> {
    unsafe {
        let fd = fd as usize;
        let nstype = nstype as usize;
        syscall2(SYS_SETNS, fd, nstype).map(drop)
    }
}

/// Set the process group ID (PGID) of the process specified by `pid` to `pgid`.
pub fn setpgid(pid: pid_t, pgid: pid_t) -> Result<(), Errno> {
    unsafe {
        let pid = pid as usize;
        let pgid = pgid as usize;
        syscall2(SYS_SETPGID, pid, pgid).map(drop)
    }
}

/// Set program scheduling priority.
pub fn setpriority(which: i32, who: i32, prio: i32) -> Result<(), Errno> {
    unsafe {
        let which = which as usize;
        let who = who as usize;
        let prio = prio as usize;
        syscall3(SYS_SETPRIORITY, which, who, prio).map(drop)
    }
}

/// Set real and effective group IDs of the calling process.
pub fn setregid(rgid: gid_t, egid: gid_t) -> Result<(), Errno> {
    unsafe {
        let rgid = rgid as usize;
        let egid = egid as usize;
        syscall2(SYS_SETREGID, rgid, egid).map(drop)
    }
}

/// Set real, effective and saved group Ids of the calling process.
pub fn setresgid(rgid: gid_t, egid: gid_t, sgid: gid_t) -> Result<(), Errno> {
    unsafe {
        let rgid = rgid as usize;
        let egid = egid as usize;
        let sgid = sgid as usize;
        syscall3(SYS_SETRESGID, rgid, egid, sgid).map(drop)
    }
}

/// Set real, effective and saved user Ids of the calling process.
pub fn setresuid(ruid: uid_t, euid: uid_t, suid: uid_t) -> Result<(), Errno> {
    unsafe {
        let ruid = ruid as usize;
        let euid = euid as usize;
        let suid = suid as usize;
        syscall3(SYS_SETRESUID, ruid, euid, suid).map(drop)
    }
}

/// Set real and effective user IDs of the calling process.
pub fn setreuid(ruid: uid_t, euid: uid_t) -> Result<(), Errno> {
    unsafe {
        let ruid = ruid as usize;
        let euid = euid as usize;
        syscall2(SYS_SETREUID, ruid, euid).map(drop)
    }
}

/// Set resource limit
pub fn setrlimit(resource: u32, rlimit: &rlimit_t) -> Result<(), Errno> {
    unsafe {
        let resource = resource as usize;
        let rlimit_ptr = rlimit as *const rlimit_t as usize;
        syscall2(SYS_SETRLIMIT, resource, rlimit_ptr).map(drop)
    }
}

/// Create a new session if the calling process is not a process group leader.
pub fn setsid() -> Result<pid_t, Errno> {
    unsafe { syscall0(SYS_SETSID).map(|ret| ret as pid_t) }
}

/// Set options on sockets.
pub fn setsockopt(
    sockfd: i32,
    level: i32,
    optname: i32,
    optval: usize,
    optlen: socklen_t,
) -> Result<(), Errno> {
    unsafe {
        let sockfd = sockfd as usize;
        let level = level as usize;
        let optname = optname as usize;
        let optlen = optlen as usize;
        syscall5(SYS_SETSOCKOPT, sockfd, level, optname, optval, optlen).map(drop)
    }
}

/// Set system time and timezone.
pub fn settimeofday(timeval: &timeval_t, tz: &timezone_t) -> Result<(), Errno> {
    unsafe {
        let timeval_ptr = timeval as *const timeval_t as usize;
        let tz_ptr = tz as *const timezone_t as usize;
        syscall2(SYS_SETTIMEOFDAY, timeval_ptr, tz_ptr).map(drop)
    }
}

/// Set the effective user ID of the calling process to `uid`.
pub fn setuid(uid: uid_t) -> Result<(), Errno> {
    unsafe {
        let uid = uid as usize;
        syscall1(SYS_SETUID, uid).map(drop)
    }
}

/// Set extended attribute value.
pub fn setxattr(filename: &str, name: &str, value: usize, size: size_t) -> Result<ssize_t, Errno> {
    unsafe {
        let filename = CString::new(filename);
        let filename_ptr = filename.as_ptr() as usize;
        let name_ptr = name.as_ptr() as usize;
        let size = size as usize;
        syscall4(SYS_SETXATTR, filename_ptr, name_ptr, value, size).map(|ret| ret as ssize_t)
    }
}

/// Set default NUMA memory policy for a thread and its children
pub fn set_mempolicy(mode: i32, nmask: *const usize, maxnode: usize) -> Result<(), Errno> {
    unsafe {
        let mode = mode as usize;
        let nmask = nmask as usize;
        syscall3(SYS_SET_MEMPOLICY, mode, nmask, maxnode).map(drop)
    }
}

/// Set the robust-futex list head of a task.
pub fn set_robust_list(heads: &mut [robust_list_head_t]) -> Result<(), Errno> {
    unsafe {
        let heads_ptr = heads.as_mut_ptr() as usize;
        let len = heads.len();
        syscall2(SYS_SET_ROBUST_LIST, heads_ptr, len).map(drop)
    }
}

/// Set thread-local storage information.
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub fn set_thread_area(user_desc: &mut user_desc_t) -> Result<(), Errno> {
    unsafe {
        let user_desc_ptr = user_desc as *mut user_desc_t as usize;
        syscall1(SYS_SET_THREAD_AREA, user_desc_ptr).map(drop)
    }
}

#[cfg(any(target_arch = "mips", target_arch = "mips64"))]
/// Set thread-local storage information.
pub fn set_thread_area(addr: usize) -> Result<(), Errno> {
    unsafe { syscall1(SYS_SET_THREAD_AREA, addr).map(drop) }
}

/// Set pointer to thread ID.
pub fn set_tid_address(tid: &mut i32) -> Result<isize, Errno> {
    unsafe {
        let tid_ptr = tid as *mut i32 as usize;
        syscall1(SYS_SET_TID_ADDRESS, tid_ptr).map(|ret| ret as isize)
    }
}

/// Manipulation of signal mask.
/// Depercated. Use `sigprocmask` instead.
pub fn sgetmask() {
    core::unimplemented!();
    // syscall0(SYS_SGETMASK);
}

/// Attach the System V shared memory segment.
pub fn shmat(shmid: i32, shmaddr: usize, shmflg: i32) -> Result<usize, Errno> {
    unsafe {
        let shmid = shmid as usize;
        let shmflg = shmflg as usize;
        syscall3(SYS_SHMAT, shmid, shmaddr, shmflg)
    }
}

/// System V shared memory control.
pub fn shmctl(shmid: i32, cmd: i32, buf: &mut shmid_ds_t) -> Result<i32, Errno> {
    unsafe {
        let shmid = shmid as usize;
        let cmd = cmd as usize;
        let buf_ptr = buf as *mut shmid_ds_t as usize;
        syscall3(SYS_SHMCTL, shmid, cmd, buf_ptr).map(|ret| ret as i32)
    }
}

/// Detach the System V shared memory segment.
pub fn shmdt(shmaddr: usize) -> Result<(), Errno> {
    unsafe { syscall1(SYS_SHMDT, shmaddr).map(drop) }
}

/// Allocates a System V shared memory segment.
pub fn shmget(key: key_t, size: size_t, shmflg: i32) -> Result<(), Errno> {
    unsafe {
        let key = key as usize;
        let size = size as usize;
        let shmflg = shmflg as usize;
        syscall3(SYS_SHMGET, key, size, shmflg).map(drop)
    }
}

/// Shutdown part of a full-duplex connection.
pub fn shutdown(sockfd: i32, how: i32) -> Result<(), Errno> {
    unsafe {
        let sockfd = sockfd as usize;
        let how = how as usize;
        syscall2(SYS_SHUTDOWN, sockfd, how).map(drop)
    }
}

/// Examine and change a signal action.
pub fn sigaction(sig: i32, act: &sigaction_t, old_act: &mut sigaction_t) -> Result<(), Errno> {
    unsafe {
        let sig = sig as usize;
        let act_ptr = act as *const sigaction_t as usize;
        let old_act_ptr = old_act as *mut sigaction_t as usize;
        syscall3(SYS_SIGACTION, sig, act_ptr, old_act_ptr).map(drop)
    }
}

/// Get/set signal stack context.
pub fn sigaltstack(uss: &sigaltstack_t, uoss: &mut sigaltstack_t) -> Result<(), Errno> {
    unsafe {
        let uss_ptr = uss as *const sigaltstack_t as usize;
        let uoss_ptr = uoss as *mut sigaltstack_t as usize;
        syscall2(SYS_SIGALTSTACK, uss_ptr, uoss_ptr).map(drop)
    }
}

/// Signal handling.
/// Deprecated. Use sigaction() instead.
pub fn signal(sig: i32, handler: sighandler_t) -> Result<sighandler_t, Errno> {
    unsafe {
        let sig = sig as usize;
        let handler = handler as usize;
        syscall2(SYS_SIGNAL, sig, handler).map(|ret| ret as sighandler_t)
    }
}

/// Create a file descriptor to accept signals.
pub fn signalfd(fd: i32, mask: &[sigset_t]) -> Result<i32, Errno> {
    unsafe {
        let fd = fd as usize;
        let mask_ptr = mask.as_ptr() as usize;
        let mask_len = mask.len() as usize;
        syscall3(SYS_SIGNALFD, fd, mask_ptr, mask_len).map(|ret| ret as i32)
    }
}

/// Create a file descriptor to accept signals.
pub fn signalfd4(fd: i32, mask: &[sigset_t], flags: i32) -> Result<i32, Errno> {
    unsafe {
        let fd = fd as usize;
        let mask_ptr = mask.as_ptr() as usize;
        let mask_len = mask.len() as usize;
        let flags = flags as usize;
        syscall4(SYS_SIGNALFD4, fd, mask_ptr, mask_len, flags).map(|ret| ret as i32)
    }
}

/// Examine pending signals.
pub fn sigpending(set: &mut sigset_t) -> Result<(), Errno> {
    unsafe {
        let set_ptr = set as *mut sigset_t as usize;
        syscall1(SYS_SIGPENDING, set_ptr).map(drop)
    }
}

/// Examine and change blocked signals.
pub fn sigprocmask(how: i32, newset: &mut sigset_t, oldset: &mut sigset_t) -> Result<(), Errno> {
    unsafe {
        let how = how as usize;
        let newset_ptr = newset as *mut sigset_t as usize;
        let oldset_ptr = oldset as *mut sigset_t as usize;
        syscall3(SYS_SIGPROCMASK, how, newset_ptr, oldset_ptr).map(drop)
    }
}

/// Return from signal handler and cleanup stack frame.
/// Never returns.
pub fn sigreturn() {
    unsafe {
        let _ = syscall0(SYS_SIGRETURN);
    }
}

/// Wait for a signal.
pub fn sigsuspend(mask: &old_sigset_t) -> Result<(), Errno> {
    unsafe {
        let mask_ptr = mask as *const old_sigset_t as usize;
        syscall1(SYS_SIGSUSPEND, mask_ptr).map(drop)
    }
}

/// Create an endpoint for communication.
pub fn socket(domain: i32, sock_type: i32, protocol: i32) -> Result<i32, Errno> {
    unsafe {
        let domain = domain as usize;
        let sock_type = sock_type as usize;
        let protocol = protocol as usize;
        syscall3(SYS_SOCKET, domain, sock_type, protocol).map(|ret| ret as i32)
    }
}

//// System call vectors.
///
/// Argument checking cleaned up. Saved 20% in size.
/// This function doesn't need to set the kernel lock because
/// it is set by the callees.
// TODO(Shaohua): Check args type and return type
pub fn socketcall(call: i32, args: &mut usize) -> Result<usize, Errno> {
    unsafe {
        let call = call as usize;
        let args_ptr = args as *mut usize as usize;
        syscall2(SYS_SOCKETCALL, call, args_ptr)
    }
}

/// Create a pair of connected socket.
pub fn socketpair(domain: i32, type_: i32, protocol: i32, sv: [i32; 2]) -> Result<(), Errno> {
    unsafe {
        let domain = domain as usize;
        let type_ = type_ as usize;
        let protocol = protocol as usize;
        let sv_ptr = sv.as_ptr() as usize;
        syscall4(SYS_SOCKETPAIR, domain, type_, protocol, sv_ptr).map(drop)
    }
}

/// Splice data to/from pipe.
pub fn splice(
    fd_in: i32,
    off_in: &mut loff_t,
    fd_out: i32,
    off_out: &mut loff_t,
    len: size_t,
    flags: u32,
) -> Result<ssize_t, Errno> {
    unsafe {
        let fd_in = fd_in as usize;
        let off_in_ptr = off_in as *mut loff_t as usize;
        let fd_out = fd_out as usize;
        let off_out_ptr = off_out as *mut loff_t as usize;
        let len = len as usize;
        let flags = flags as usize;
        syscall6(
            SYS_SPLICE,
            fd_in,
            off_in_ptr,
            fd_out,
            off_out_ptr,
            len,
            flags,
        )
        .map(|ret| ret as ssize_t)
    }
}

/// Manipulation of signal mask.
/// Deprecated. Use `sigprocmask` instead.
pub fn ssetmask() {
    core::unimplemented!();
    // syscall0(SYS_SSETMASK);
}

/// Get file status about a file.
pub fn stat(filename: &str, statbuf: &mut stat_t) -> Result<(), Errno> {
    unsafe {
        let filename = CString::new(filename);
        let filename_ptr = filename.as_ptr() as usize;
        let statbuf_ptr = statbuf as *mut stat_t as usize;
        syscall2(SYS_STAT, filename_ptr, statbuf_ptr).map(drop)
    }
}

/// Get file status about a file.
pub fn stat64(filename: &str, statbuf: &mut stat64_t) -> Result<(), Errno> {
    unsafe {
        let filename = CString::new(filename);
        let filename_ptr = filename.as_ptr() as usize;
        let statbuf_ptr = statbuf as *mut stat64_t as usize;
        syscall2(SYS_STAT64, filename_ptr, statbuf_ptr).map(drop)
    }
}

/// Get filesystem statistics.
pub fn statfs(filename: &str, buf: &mut statfs_t) -> Result<(), Errno> {
    unsafe {
        let filename = CString::new(filename);
        let filename_ptr = filename.as_ptr() as usize;
        let buf_ptr = buf as *mut statfs_t as usize;
        syscall2(SYS_STATFS, filename_ptr, buf_ptr).map(drop)
    }
}

/// Get filesystem statistics.
pub fn statfs64(filename: &str, buf: &mut statfs64_t) -> Result<(), Errno> {
    unsafe {
        let filename = CString::new(filename);
        let filename_ptr = filename.as_ptr() as usize;
        let buf_ptr = buf as *mut statfs64_t as usize;
        syscall2(SYS_STATFS64, filename_ptr, buf_ptr).map(drop)
    }
}

/// Get file status about a file (extended).
pub fn statx(
    dirfd: i32,
    filename: &str,
    flags: i32,
    mask: u32,
    buf: &mut statx_t,
) -> Result<(), Errno> {
    unsafe {
        let dirfd = dirfd as usize;
        let filename = CString::new(filename);
        let filename_ptr = filename.as_ptr() as usize;
        let flags = flags as usize;
        let mask = mask as usize;
        let buf_ptr = buf as *mut statx_t as usize;
        syscall5(SYS_STATX, dirfd, filename_ptr, flags, mask, buf_ptr).map(drop)
    }
}

/// Set time.
pub fn stime(t: &time_t) -> Result<(), Errno> {
    unsafe {
        let t_ptr = t as *const time_t as usize;
        syscall1(SYS_STIME, t_ptr).map(drop)
    }
}

pub fn stty() {
    core::unimplemented!();
    // syscall0(SYS_STTY);
}

/// Stop swapping to file/device.
pub fn swapoff(filename: &str) -> Result<(), Errno> {
    unsafe {
        let filename = CString::new(filename);
        let filename_ptr = filename.as_ptr() as usize;
        syscall1(SYS_SWAPOFF, filename_ptr).map(drop)
    }
}

/// Start swapping to file/device.
pub fn swapon(filename: &str, flags: i32) -> Result<(), Errno> {
    unsafe {
        let filename = CString::new(filename);
        let filename_ptr = filename.as_ptr() as usize;
        let flags = flags as usize;
        syscall2(SYS_SWAPON, filename_ptr, flags).map(drop)
    }
}

/// Make a new name for a file.
pub fn symlink(oldname: &str, newname: &str) -> Result<(), Errno> {
    unsafe {
        let oldname_ptr = oldname.as_ptr() as usize;
        let newname_ptr = newname.as_ptr() as usize;
        syscall2(SYS_SYMLINK, oldname_ptr, newname_ptr).map(drop)
    }
}

/// Make a new name for a file.
pub fn symlinkat(oldname: &str, newfd: i32, newname: &str) -> Result<(), Errno> {
    unsafe {
        let oldname_ptr = oldname.as_ptr() as usize;
        let newfd = newfd as usize;
        let newname_ptr = newname.as_ptr() as usize;
        syscall3(SYS_SYMLINKAT, oldname_ptr, newfd, newname_ptr).map(drop)
    }
}

/// Commit filesystem caches to disk.
pub fn sync() {
    unsafe {
        let _ret = syscall0(SYS_SYNC);
    }
}

/// Commit filesystem cache related to `fd` to disk.
pub fn syncfs(fd: i32) -> Result<(), Errno> {
    unsafe {
        let fd = fd as usize;
        syscall1(SYS_SYNCFS, fd).map(drop)
    }
}

/// Sync a file segment to disk
pub fn sync_file_range(fd: i32, offset: off_t, nbytes: off_t, flags: i32) -> Result<(), Errno> {
    unsafe {
        let fd = fd as usize;
        let offset = offset as usize;
        let nbytes = nbytes as usize;
        let flags = flags as usize;
        syscall4(SYS_SYNC_FILE_RANGE, fd, offset, nbytes, flags).map(drop)
    }
}

pub fn syscall() {
    core::unimplemented!();
    // syscall0(SYS_SYSCALL);
}

/// Get filesystem type information.
pub fn sysfs(option: i32, arg1: usize, arg2: usize) -> Result<i32, Errno> {
    unsafe {
        let option = option as usize;
        let arg1 = arg1 as usize;
        let arg2 = arg2 as usize;
        syscall3(SYS_SYSFS, option, arg1, arg2).map(|ret| ret as i32)
    }
}

/// Return system information.
pub fn sysinfo(info: &mut sysinfo_t) -> Result<(), Errno> {
    unsafe {
        let info_ptr = info as *mut sysinfo_t as usize;
        syscall1(SYS_SYSINFO, info_ptr).map(drop)
    }
}

/// Read and/or clear kernel message ring buffer; set console_loglevel
pub fn syslog(action: i32, buf: &mut [u8]) -> Result<i32, Errno> {
    unsafe {
        let action = action as usize;
        let buf_ptr = buf.as_mut_ptr() as usize;
        let buf_len = buf.len();
        syscall3(SYS_SYSLOG, action, buf_ptr, buf_len).map(|ret| ret as i32)
    }
}

pub fn sysmips() {
    core::unimplemented!();
    // syscall0(SYS_SYSMIPS);
}

/// Duplicate pipe content.
pub fn tee(fd_in: i32, fd_out: i32, len: size_t, flags: u32) -> Result<ssize_t, Errno> {
    unsafe {
        let fd_in = fd_in as usize;
        let fd_out = fd_out as usize;
        let len = len as usize;
        let flags = flags as usize;
        syscall4(SYS_TEE, fd_in, fd_out, len, flags).map(|ret| ret as ssize_t)
    }
}

/// Send a signal to a thread.
pub fn tgkill(tgid: i32, tid: i32, sig: i32) -> Result<(), Errno> {
    unsafe {
        let tgid = tgid as usize;
        let tid = tid as usize;
        let sig = sig as usize;
        syscall3(SYS_TGKILL, tgid, tid, sig).map(drop)
    }
}

/// Get time in seconds.
pub fn time() -> Result<time_t, Errno> {
    unsafe { syscall0(SYS_TIME).map(|ret| ret as time_t) }
}

pub fn timerfd() {
    core::unimplemented!();
    // syscall0(SYS_TIMERFD);
}

/// Create a timer that notifies via a file descriptor.
pub fn timerfd_create(clockid: i32, flags: i32) -> Result<i32, Errno> {
    unsafe {
        let clockid = clockid as usize;
        let flags = flags as usize;
        syscall2(SYS_TIMERFD_CREATE, clockid, flags).map(|ret| ret as i32)
    }
}

/// Get current timer via a file descriptor.
pub fn timerfd_gettime(ufd: i32, otmr: &mut itimerval_t) -> Result<(), Errno> {
    unsafe {
        let ufd = ufd as usize;
        let otmr_ptr = otmr as *mut itimerval_t as usize;
        syscall2(SYS_TIMERFD_GETTIME, ufd, otmr_ptr).map(drop)
    }
}

pub fn timerfd_gettime64() {
    core::unimplemented!();
    // syscall0(SYS_TIMERFD_GETTIME64);
}

/// Set current timer via a file descriptor.
pub fn timerfd_settime(
    ufd: i32,
    flags: i32,
    utmr: &itimerval_t,
    otmr: &mut itimerval_t,
) -> Result<(), Errno> {
    unsafe {
        let ufd = ufd as usize;
        let flags = flags as usize;
        let utmr_ptr = utmr as *const itimerval_t as usize;
        let otmr_ptr = otmr as *mut itimerval_t as usize;
        syscall4(SYS_TIMERFD_SETTIME, ufd, flags, utmr_ptr, otmr_ptr).map(drop)
    }
}

pub fn timerfd_settime64() {
    core::unimplemented!();
    // syscall0(SYS_TIMERFD_SETTIME64);
}

/// Create a per-process timer
pub fn timer_create(
    clock: clockid_t,
    event: &mut sigevent_t,
    timer_id: &mut timer_t,
) -> Result<(), Errno> {
    unsafe {
        let clock = clock as usize;
        let event_ptr = event as *mut sigevent_t as usize;
        let timer_id_ptr = timer_id as *mut timer_t as usize;
        syscall3(SYS_TIMER_CREATE, clock, event_ptr, timer_id_ptr).map(drop)
    }
}

/// Delete a per-process timer
pub fn timer_delete(timer_id: timer_t) -> Result<(), Errno> {
    unsafe {
        let timer_id = timer_id as usize;
        syscall1(SYS_TIMER_DELETE, timer_id).map(drop)
    }
}

/// Get overrun count for a per-process timer
pub fn timer_getoverrun(timer_id: timer_t) -> Result<(), Errno> {
    unsafe {
        let timer_id = timer_id as usize;
        syscall1(SYS_TIMER_GETOVERRUN, timer_id).map(drop)
    }
}

/// Fetch state of per-process timer
pub fn timer_gettime(timer_id: timer_t, curr: &mut itimerspec_t) -> Result<(), Errno> {
    unsafe {
        let timer_id = timer_id as usize;
        let curr_ptr = curr as *mut itimerspec_t as usize;
        syscall2(SYS_TIMER_GETTIME, timer_id, curr_ptr).map(drop)
    }
}

pub fn timer_gettime64() {
    core::unimplemented!();
    // syscall0(SYS_TIMER_GETTIME64);
}

/// Arm/disarm state of per-process timer
pub fn timer_settime(
    timer_id: timer_t,
    flags: i32,
    new_value: &itimerspec_t,
    old_value: &mut itimerspec_t,
) -> Result<(), Errno> {
    unsafe {
        let timer_id = timer_id as usize;
        let flags = flags as usize;
        let new_value_ptr = new_value as *const itimerspec_t as usize;
        let old_value_ptr = old_value as *mut itimerspec_t as usize;
        syscall4(
            SYS_TIMER_SETTIME,
            timer_id,
            flags,
            new_value_ptr,
            old_value_ptr,
        )
        .map(drop)
    }
}

pub fn timer_settime64() {
    core::unimplemented!();
    // syscall0(SYS_TIMER_SETTIME64);
}

/// Get process times.
pub fn times(buf: &mut tms_t) -> Result<clock_t, Errno> {
    unsafe {
        let buf_ptr = buf as *mut tms_t as usize;
        syscall1(SYS_TIMES, buf_ptr).map(|ret| ret as clock_t)
    }
}

/// Send a signal to a thread (obsolete).
pub fn tkill(tid: i32, sig: i32) -> Result<(), Errno> {
    unsafe {
        let tid = tid as usize;
        let sig = sig as usize;
        syscall2(SYS_TKILL, tid, sig).map(drop)
    }
}

/// Truncate a file to a specified length.
pub fn truncate(filename: &str, length: off_t) -> Result<(), Errno> {
    unsafe {
        let filename = CString::new(filename);
        let filename_ptr = filename.as_ptr() as usize;
        let length = length as usize;
        syscall2(SYS_TRUNCATE, filename_ptr, length).map(drop)
    }
}

/// Truncate a file to a specific length.
pub fn truncate64(path: &str, len: loff_t) -> Result<(), Errno> {
    unsafe {
        let path = CString::new(path);
        let path_ptr = path.as_ptr() as usize;
        let len = len as usize;
        syscall2(SYS_TRUNCATE64, path_ptr, len).map(drop)
    }
}

/// Deprecated.
pub fn ulimit() {
    core::unimplemented!();
    // syscall0(SYS_ULIMIT);
}

/// Set file mode creation mask.
pub fn umask(mode: mode_t) -> Result<mode_t, Errno> {
    unsafe {
        let mode = mode as usize;
        syscall1(SYS_UMASK, mode).map(|ret| ret as mode_t)
    }
}

/// Umount filesystem.
pub fn umount(name: &str, flags: i32) -> Result<(), Errno> {
    unsafe {
        let name = CString::new(name);
        let name_ptr = name.as_ptr() as usize;
        let flags = flags as usize;
        syscall2(SYS_UMOUNT, name_ptr, flags).map(drop)
    }
}

/// Umount filesystem.
pub fn umount2(name: &str, flags: i32) -> Result<(), Errno> {
    unsafe {
        let name_ptr = name.as_ptr() as usize;
        let flags = flags as usize;
        syscall2(SYS_UMOUNT2, name_ptr, flags).map(drop)
    }
}

/// Get name and information about current kernel.
pub fn uname(buf: &mut utsname_t) -> Result<(), Errno> {
    unsafe {
        let buf_ptr = buf as *mut utsname_t as usize;
        syscall1(SYS_UNAME, buf_ptr).map(drop)
    }
}

/// Delete a name and possibly the file it refers to.
pub fn unlink(filename: &str) -> Result<(), Errno> {
    unsafe {
        let filename = CString::new(filename);
        let filename_ptr = filename.as_ptr() as usize;
        syscall1(SYS_UNLINK, filename_ptr).map(drop)
    }
}

/// Delete a name and possibly the file it refers to.
pub fn unlinkat(dfd: i32, filename: &str, flag: i32) -> Result<(), Errno> {
    unsafe {
        let dfd = dfd as usize;
        let filename = CString::new(filename);
        let filename_ptr = filename.as_ptr() as usize;
        let flag = flag as usize;
        syscall3(SYS_UNLINKAT, dfd, filename_ptr, flag).map(drop)
    }
}

/// Disassociate parts of the process execution context
pub fn unshare(flags: i32) -> Result<(), Errno> {
    unsafe {
        let flags = flags as usize;
        syscall1(SYS_UNSHARE, flags).map(drop)
    }
}

pub fn unused109() {
    core::unimplemented!();
    // syscall0(SYS_UNUSED109);
}

pub fn unused150() {
    core::unimplemented!();
    // syscall0(SYS_UNUSED150);
}

pub fn unused18() {
    core::unimplemented!();
    // syscall0(SYS_UNUSED18);
}

pub fn unused28() {
    core::unimplemented!();
    // syscall0(SYS_UNUSED28);
}

pub fn unused59() {
    core::unimplemented!();
    // syscall0(SYS_UNUSED59);
}

pub fn unused84() {
    core::unimplemented!();
    // syscall0(SYS_UNUSED84);
}

/// Load shared library.
pub fn uselib(library: &str) -> Result<(), Errno> {
    unsafe {
        let library_ptr = library.as_ptr() as usize;
        syscall1(SYS_USELIB, library_ptr).map(drop)
    }
}

/// Create a file descriptor to handle page faults in user space.
pub fn userfaultfd(flags: i32) -> Result<i32, Errno> {
    unsafe {
        let flags = flags as usize;
        syscall1(SYS_USERFAULTFD, flags).map(|ret| ret as i32)
    }
}

/// Get filesystem statistics
pub fn ustat(dev: dev_t, ubuf: &mut ustat_t) -> Result<(), Errno> {
    unsafe {
        let dev = dev as usize;
        let ubuf_ptr = ubuf as *mut ustat_t as usize;
        syscall2(SYS_USTAT, dev, ubuf_ptr).map(drop)
    }
}

/// Change file last access and modification time.
pub fn utime(filename: &str, times: &utimbuf_t) -> Result<(), Errno> {
    unsafe {
        let filename = CString::new(filename);
        let filename_ptr = filename.as_ptr() as usize;
        let times_ptr = times as *const utimbuf_t as usize;
        syscall2(SYS_UTIME, filename_ptr, times_ptr).map(drop)
    }
}

/// Change time timestamps with nanosecond precision.
pub fn utimensat(
    dirfd: i32,
    filename: &str,
    times: &[timespec_t; 2],
    flags: i32,
) -> Result<(), Errno> {
    unsafe {
        let dirfd = dirfd as usize;
        let filename = CString::new(filename);
        let filename_ptr = filename.as_ptr() as usize;
        let times_ptr = times.as_ptr() as usize;
        let flags = flags as usize;
        syscall4(SYS_UTIMENSAT, dirfd, filename_ptr, times_ptr, flags).map(drop)
    }
}

pub fn utimensat_time64() {
    core::unimplemented!();
    // syscall0(SYS_UTIMENSAT_TIME64);
}

/// Change file last access and modification time.
pub fn utimes(filename: &str, times: &[timeval_t; 2]) -> Result<(), Errno> {
    unsafe {
        let filename = CString::new(filename);
        let filename_ptr = filename.as_ptr() as usize;
        let times_ptr = times.as_ptr() as usize;
        syscall2(SYS_UTIMES, filename_ptr, times_ptr).map(drop)
    }
}

/// Virtually hang up the current terminal.
pub fn vhangup() -> Result<(), Errno> {
    unsafe { syscall0(SYS_VHANGUP).map(drop) }
}

pub fn vm86() {
    core::unimplemented!();
    // syscall0(SYS_VM86);
}

/// Splice user page into a pipe.
pub fn vmsplice(fd: i32, iov: &iovec_t, nr_segs: usize, flags: u32) -> Result<ssize_t, Errno> {
    unsafe {
        let fd = fd as usize;
        let iov_ptr = iov as *const iovec_t as usize;
        let flags = flags as usize;
        syscall4(SYS_VMSPLICE, fd, iov_ptr, nr_segs, flags).map(|ret| ret as ssize_t)
    }
}

pub fn vserver() {
    core::unimplemented!();
    // syscall0(SYS_VSERVER);
}

/// Wait for process to change state.
pub fn wait4(
    pid: pid_t,
    wstatus: &mut i32,
    options: i32,
    rusage: &mut rusage_t,
) -> Result<(), Errno> {
    unsafe {
        let pid = pid as usize;
        let wstatus_ptr = wstatus as *mut i32 as usize;
        let options = options as usize;
        let rusage_ptr = rusage as *mut rusage_t as usize;
        syscall4(SYS_WAIT4, pid, wstatus_ptr, options, rusage_ptr).map(drop)
    }
}

/// Wait for process to change state
pub fn waitid(
    which: i32,
    pid: pid_t,
    info: &mut siginfo_t,
    options: i32,
    ru: &mut rusage_t,
) -> Result<(), Errno> {
    unsafe {
        let which = which as usize;
        let pid = pid as usize;
        let info_ptr = info as *mut siginfo_t as usize;
        let options = options as usize;
        let ru_ptr = ru as *mut rusage_t as usize;
        syscall5(SYS_WAITID, which, pid, info_ptr, options, ru_ptr).map(drop)
    }
}

/// Wait for process to chane state.
pub fn waitpid(pid: pid_t, status: &mut i32, options: i32) -> Result<pid_t, Errno> {
    unsafe {
        let pid = pid as usize;
        let status_ptr = status as *mut i32 as usize;
        let options = options as usize;
        syscall3(SYS_WAITPID, pid, status_ptr, options).map(|ret| ret as pid_t)
    }
}

/// Write to a file descriptor.
pub fn write(fd: i32, buf: &[u8]) -> Result<ssize_t, Errno> {
    unsafe {
        let fd = fd as usize;
        let buf_ptr = buf.as_ptr() as usize;
        let len = buf.len() as usize;
        syscall3(SYS_WRITE, fd, buf_ptr, len).map(|ret| ret as ssize_t)
    }
}

/// Write to a file descriptor from multiple buffers.
pub fn writev(fd: i32, iov: &[iovec_t]) -> Result<ssize_t, Errno> {
    unsafe {
        let fd = fd as usize;
        let iov_ptr = iov.as_ptr() as usize;
        let len = iov.len() as usize;
        syscall3(SYS_WRITEV, fd, iov_ptr, len).map(|ret| ret as ssize_t)
    }
}

/// Reposition read/write file offset.
pub fn _llseek(
    fd: i32,
    offset_high: usize,
    offset_low: usize,
    result: &mut loff_t,
    whence: i32,
) -> Result<(), Errno> {
    unsafe {
        let fd = fd as usize;
        let result_ptr = result as *mut loff_t as usize;
        let whence = whence as usize;
        syscall5(SYS__LLSEEK, fd, offset_high, offset_low, result_ptr, whence).map(drop)
    }
}

pub fn _newselect() {
    core::unimplemented!();
    // syscall0(SYS__NEWSELECT);
}

/// Read/write system parameters.
pub fn _sysctl(args: &mut sysctl_args_t) -> Result<(), Errno> {
    unsafe {
        let args_ptr = args as *mut sysctl_args_t as usize;
        syscall1(SYS__SYSCTL, args_ptr).map(drop)
    }
}
