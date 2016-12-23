//! Extends and wraps `std::process::Child` to make it clonable. Thus eliminating the problem that
//! `libstd` does not have a cross platfrom and simple way to kill a child while also waiting for
//! it.
//!
//! ## Getting Started
//!
//! Add the dependency to your Cargo.toml:
//!
//! ```toml
//! [dependencies]
//! clonablechild = "0.1"
//! ```
//!
//! ## Example
//!
//! Use it in your program to kill a sleep process before it terminates naturally:
//!
//! ```rust
//! extern crate clonablechild;
//!
//! use clonablechild::ChildExt;
//!
//! use std::process::Command;
//! use std::thread;
//! use std::time::Duration;
//!
//! fn main() {
//!     // This command is specific to unix systems. See tests for Windows examples.
//!     let child = Command::new("sleep").arg("10").spawn().unwrap();
//!     let clonable_child = child.into_clonable();
//!     let child_kill_handle = clonable_child.clone();
//!
//!     thread::spawn(move || {
//!         thread::sleep(Duration::new(1, 0));
//!         child_kill_handle.kill().expect("Expected to be able to kill subprocess");
//!     });
//!
//!     let exit_status = clonable_child.wait().unwrap();
//!     // Assert child was killed by a signal and did not exit cleanly
//!     assert_eq!(None, exit_status.code());
//!     assert!(!exit_status.success());
//! }
//! ```

#![deny(missing_docs)]

extern crate libc;

use std::io;
use std::process::{Child, ExitStatus};
use std::sync::{Arc, Mutex};

#[cfg(unix)]
#[path = "unix.rs"]
mod imp;

#[cfg(windows)]
#[path = "windows.rs"]
mod imp;


/// A trait that extends `Child` with a method to convert it into a `ClonableChild`.
pub trait ChildExt {
    /// Consumes the child and wraps it in a `ClonableChild`. Results in an object with a similar
    /// API, but that can be cloned.
    fn into_clonable(self) -> ClonableChild;
}

impl ChildExt for Child {
    fn into_clonable(self) -> ClonableChild {
        ClonableChild::new(self)
    }
}

/// Representation of a clonable `std::process::Child`.
#[derive(Clone)]
pub struct ClonableChild {
    id: u32,
    child: Arc<Mutex<Child>>,
    imp_child: imp::ClonableChild,
}

impl ClonableChild {
    /// Creates a new `ClonableChild` by consuming and wrapping the given `Child`.
    pub fn new(child: Child) -> Self {
        let imp_child = imp::ClonableChild::new(&child);
        ClonableChild {
            id: child.id(),
            child: Arc::new(Mutex::new(child)),
            imp_child: imp_child,
        }
    }

    /// Forces the child to exit. This is equivalent to sending a SIGKILL on unix platforms and
    /// calling TerminateProcess on Windows.
    ///
    /// This method first tries to use the ordinary `Child::kill()`, but if that is blocked by
    /// another thread waiting for the child it will kill it itself in the same way `Child::kill()`
    /// would have done.
    pub fn kill(&self) -> io::Result<()> {
        let child = self.child.try_lock();
        match child {
            Ok(mut child) => child.kill(),
            Err(..) => self.imp_child.kill(),
        }
    }

    /// Returns the OS-assigned process identifier associated with this child. This value is
    /// obtained from `Child::id()` in `ClonableChild::new()` and then that value is returned every
    /// time.
    pub fn id(&self) -> u32 {
        self.id
    }

    /// Behaves just like `Child::wait()`, see documentation for that method.
    pub fn wait(&self) -> io::Result<ExitStatus> {
        let mut child = self.child.lock().unwrap();
        child.wait()
    }
}
