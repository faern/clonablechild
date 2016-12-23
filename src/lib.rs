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


pub trait ChildExt {
    fn into_clonable(self) -> ClonableChild;
}

impl ChildExt for Child {
    fn into_clonable(self) -> ClonableChild {
        ClonableChild::new(self)
    }
}

#[derive(Clone)]
pub struct ClonableChild {
    id: u32,
    child: Arc<Mutex<Child>>,
    imp_child: imp::ClonableChild,
}

impl ClonableChild {
    pub fn new(child: Child) -> Self {
        let imp_child = imp::ClonableChild::new(&child);
        ClonableChild {
            id: child.id(),
            child: Arc::new(Mutex::new(child)),
            imp_child: imp_child,
        }
    }

    pub fn kill(&self) -> io::Result<()> {
        let child = self.child.try_lock();
        match child {
            Ok(mut child) => child.kill(),
            Err(..) => self.imp_child.kill(),
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn wait(&self) -> io::Result<ExitStatus> {
        let mut child = self.child.lock().unwrap();
        child.wait()
    }
}
