use limits_rs::{get_own_limits, get_pid_limits, Limits};

#[cfg(target_os = "linux")]
pub fn own_limit() {
    let limits = get_own_limits().unwrap();
    let max = limits.max_open_files.soft;
}
