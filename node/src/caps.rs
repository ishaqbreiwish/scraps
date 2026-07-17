#[cfg(target_os = "linux")]
use std::fs;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum CapsError {
    #[cfg(target_os = "linux")]
    #[error("failed to read /proc/meminfo: {0}")]
    MemInfoUnreadable(#[from] std::io::Error),
    #[cfg(target_os = "linux")]
    #[error("MemTotal line not found or malformed in /proc/meminfo")]
    MemTotalMissing,
    #[cfg(target_os = "linux")]
    #[error("failed to parse MemTotal value: {0}")]
    MemTotalParse(#[from] std::num::ParseIntError),
    #[cfg(target_os = "macos")]
    #[error("sysctlbyname(hw.memsize) failed with errno {0}")]
    SysctlFailed(i32),
}

#[derive(Debug)]
pub struct NodeCapabilities {
    pub total_ram_bytes: u64,
    pub arch: String,
    pub kvm_available: bool,
}

pub fn detect() -> Result<NodeCapabilities, CapsError> {
    Ok(NodeCapabilities {
        total_ram_bytes: total_ram_bytes()?,
        arch: std::env::consts::ARCH.to_string(),
        kvm_available: kvm_available(),
    })
}

#[cfg(target_os = "linux")]
fn total_ram_bytes() -> Result<u64, CapsError> {
    let contents = fs::read_to_string("/proc/meminfo")?;
    let kb: u64 = contents
        .lines()
        .find(|line| line.starts_with("MemTotal:"))
        .and_then(|line| line.split_whitespace().nth(1))
        .ok_or(CapsError::MemTotalMissing)?
        .parse()?;
    Ok(kb * 1024)
}

// sysctlbyname's oldp/oldlenp pair is a classic "caller allocates, kernel fills
// in and reports how much it wrote" C API — we hand it a u64-sized buffer and
// check the reported length matches what we expect.
#[cfg(target_os = "macos")]
fn total_ram_bytes() -> Result<u64, CapsError> {
    use std::mem;

    let mut mem_size: u64 = 0;
    let mut size = mem::size_of::<u64>();

    let ret = unsafe {
        libc::sysctlbyname(
            c"hw.memsize".as_ptr(),
            &mut mem_size as *mut u64 as *mut libc::c_void,
            &mut size,
            std::ptr::null_mut(),
            0,
        )
    };

    if ret != 0 {
        let errno = std::io::Error::last_os_error()
            .raw_os_error()
            .unwrap_or(-1);
        return Err(CapsError::SysctlFailed(errno));
    }

    Ok(mem_size)
}

#[cfg(target_os = "linux")]
fn kvm_available() -> bool {
    fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open("/dev/kvm")
        .is_ok()
}

#[cfg(target_os = "macos")]
fn kvm_available() -> bool {
    false
}
