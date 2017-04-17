# ClonableChild

**DEPRECATED**: *This crate has been deprecated and yanked from crates.io. The problem it solves is solved in a better way by the [`shared_child`](https://crates.io/crates/shared_child) crate. See [this issue](https://github.com/oconnor663/shared_child.rs/issues/9) for more information.*

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
fn main() {
    // This command is specific to unix systems. See tests for Windows examples.
    let child = Command::new("sleep").arg("10").spawn().unwrap();
    let clonable_child = child.into_clonable();

    kill_async(clonable_child.clone());
    let exit_status = clonable_child.wait().unwrap();

    // Assert child was killed by a signal and did not exit cleanly
    assert_eq!(None, exit_status.code());
    assert!(!exit_status.success());
}

fn kill_async(child: ClonableChild) {
    thread::spawn(move || {
        thread::sleep(Duration::new(1, 0));
        child.kill().expect("Expected to be able to kill subprocess");
    });
}
```
