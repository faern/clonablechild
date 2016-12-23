# ClonableChild

Extends and wraps `std::process::Child` to make it clonable. Thus eliminating the problem that
`libstd` does not have a cross platfrom and simple way to kill a child while also waiting for it.

## Getting Started

Add the dependency to your Cargo.toml:

```toml
[dependencies]
clonablechild = "0.1"
```

## Example

Use it in your program to kill a sleep process before it terminates naturally:

```rust
extern crate clonablechild;

use clonablechild::ChildExt;

use std::process::Command;
use std::thread;
use std::time::Duration;

fn main() {
    // This command is specific to unix systems. See tests for Windows examples.
    let child = Command::new("sleep").arg("10").spawn().unwrap();
    let clonable_child = child.into_clonable();
    let child_kill_handle = clonable_child.clone();

    thread::spawn(move || {
        thread::sleep(Duration::new(1, 0));
        child_kill_handle.kill().expect("Expected to be able to kill subprocess");
    });
    
    let exit_status = clonable_child.wait().unwrap();
    // Assert child was killed by a signal and did not exit cleanly
    assert_eq!(None, exit_status.code());
    assert!(!exit_status.success());
}
```
