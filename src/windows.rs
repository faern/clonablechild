use std::io;
use std::os::raw::{c_void, c_uint, c_int};
use std::os::windows::io::AsRawHandle;
use std::process::Child;

#[derive(Clone)]
struct RawHandle(*mut c_void);

unsafe impl Send for RawHandle {}
unsafe impl Sync for RawHandle {}

extern "system" {
    fn TerminateProcess(hProcess: *mut c_void, uExitCode: c_uint) -> c_int;
}

#[derive(Clone)]
pub struct ClonableChild {
    handle: RawHandle,
}

impl ClonableChild {
    pub fn new(child: &Child) -> Self {
        ClonableChild { handle: RawHandle(child.as_raw_handle()) }
    }

    pub fn kill(&self) -> io::Result<()> {
        if unsafe { TerminateProcess(self.handle.0, 1) } == 0 {
            Err(io::Error::last_os_error())
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
pub mod testing {
    use std::process::ExitStatus;

    pub const LONG_RUNNING_COMMAND: (&'static str, &'static [&'static str]) =
        ("ping", &["127.0.0.1", "-n", "4"]);

    pub const INSTANT_EXIT_COMMAND: (&'static str, &'static [&'static str]) = ("echo", &[]);

    pub fn was_killed(exit_status: &ExitStatus) -> bool {
        (!exit_status.success()) && (exit_status.code() == Some(1))
    }
}
