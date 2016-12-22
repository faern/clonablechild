use libc;
use std::io;
use std::process::Child;

#[derive(Clone)]
pub struct ClonableChild {
    pid: i32,
}

impl ClonableChild {
    pub fn new(child: &Child) -> Self {
        ClonableChild { pid: child.id() as i32 }
    }

    pub fn kill(&self) -> io::Result<()> {
        if unsafe { libc::kill(self.pid, libc::SIGKILL) } == -1 {
            Err(io::Error::last_os_error())
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
pub mod testing {
    use std::process::ExitStatus;

    pub const LONG_RUNNING_COMMAND: (&'static str, &'static [&'static str]) = ("sleep", &["3"]);

    pub const INSTANT_EXIT_COMMAND: (&'static str, &'static [&'static str]) = ("echo", &[]);

    pub fn was_killed(exit_status: &ExitStatus) -> bool {
        (!exit_status.success()) && (exit_status.code().is_none())
    }
}
