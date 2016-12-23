extern crate clonablechild;

use clonablechild::{ChildExt, ClonableChild};

use platform_helper::*;

use std::io;
use std::process::{Command, Stdio, ExitStatus};
use std::thread::{self, JoinHandle};
use std::time::Duration;

#[test]
fn wait_for_child_twice() {
    let testee = create_testee(INSTANT_EXIT_COMMAND);

    for _ in 0..2 {
        let exit_status = testee.wait().expect("Expected To be able to wait for child");
        assert!(clean_exit(&exit_status));
    }
}

#[test]
fn killing_dead_child() {
    let testee = create_testee(INSTANT_EXIT_COMMAND);
    testee.wait().expect("Expected To be able to wait for child");

    assert!(testee.kill().is_err());
}

#[test]
fn killing_running_without_wait() {
    let testee = create_testee(LONG_RUNNING_COMMAND);

    assert!(testee.kill().is_ok());
    let exit_status = testee.wait().expect("Expected To be able to wait for child");
    assert!(was_killed(&exit_status));
}

#[test]
fn killing_dead_without_wait() {
    let testee = create_testee(INSTANT_EXIT_COMMAND);
    sleep_one_sec();

    let kill_result = testee.kill();
    #[cfg(unix)]
    assert!(kill_result.is_ok());
    #[cfg(windows)]
    assert!(kill_result.is_err());

    let exit_status = testee.wait().expect("Expected To be able to wait for child");
    assert!(!was_killed(&exit_status));
}

#[test]
fn multiple_wait_long_running_child() {
    let (_, threads) = create_testee_where_many_threads_wait(LONG_RUNNING_COMMAND);

    for thread in threads {
        let thread_result = thread.join().expect("Expected to be able to join thread");
        let exit_status = thread_result.expect("Expected thread to have an Result::Ok");
        assert!(clean_exit(&exit_status));
    }
}

#[test]
fn multiple_wait_and_kill_long_running_child() {
    let (testee, threads) = create_testee_where_many_threads_wait(LONG_RUNNING_COMMAND);
    sleep_one_sec();
    testee.kill().expect("Expected kill Result to be Ok");

    for thread in threads {
        let thread_result = thread.join().expect("Expected to be able to join thread");
        let exit_status = thread_result.expect("Expected thread to have a Result::Ok");
        assert!(was_killed(&exit_status));
    }
}


fn create_testee(command: (&str, &[&str])) -> ClonableChild {
    let child = Command::new(command.0)
        .args(command.1)
        .stdout(Stdio::null())
        .spawn()
        .expect(&format!("Expected to be able to spawn {}", command.0));
    child.into_clonable()
}

fn create_testee_where_many_threads_wait
    (command: (&str, &[&str]))
     -> (ClonableChild, Vec<JoinHandle<io::Result<ExitStatus>>>) {
    let testee = create_testee(command);
    let mut threads = vec![];
    for _ in 0..10 {
        let testee_clone = testee.clone();
        threads.push(thread::spawn(move || testee_clone.wait()));
    }
    (testee, threads)
}

fn sleep_one_sec() {
    thread::sleep(Duration::new(1, 0));
}

fn clean_exit(exit_status: &ExitStatus) -> bool {
    exit_status.success() && (exit_status.code() == Some(0))
}

#[cfg(unix)]
pub mod platform_helper {
    use std::process::ExitStatus;

    pub const LONG_RUNNING_COMMAND: (&'static str, &'static [&'static str]) = ("sleep", &["3"]);

    pub const INSTANT_EXIT_COMMAND: (&'static str, &'static [&'static str]) = ("echo", &[]);

    pub fn was_killed(exit_status: &ExitStatus) -> bool {
        (!exit_status.success()) && (exit_status.code().is_none())
    }
}

#[cfg(windows)]
pub mod platform_helper {
    use std::process::ExitStatus;

    pub const LONG_RUNNING_COMMAND: (&'static str, &'static [&'static str]) =
        ("ping", &["127.0.0.1", "-n", "4"]);

    pub const INSTANT_EXIT_COMMAND: (&'static str, &'static [&'static str]) = ("echo", &[]);

    pub fn was_killed(exit_status: &ExitStatus) -> bool {
        (!exit_status.success()) && (exit_status.code() == Some(1))
    }
}
