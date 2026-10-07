//! What recursion does with a deep tree, and what prevents it.
//!
//! The input is one path of depth 5 000 (`tests/data/deep_path.txt`), the stack that of a small thread: 64 KiB, as
//! a thread on an embedded target might get. The recursive variants overflow it; the iterative ones and the depth
//! limit do not.
//!
//! A stack overflow cannot be caught: Rust aborts the whole process. Each crashing case therefore runs in a child
//! process -- this test binary, started again with only that test and the variable `STACK_OVERFLOW_CHILD` -- and the
//! parent checks that the child died with Rust's message. Run with `-- --nocapture` to see what the children wrote.

use std::collections::BTreeMap;
use std::io;
use std::process::Command;

use dir_path_2_tree::*;

const STACK: usize = 64 * 1024;
const CHILD: &str = "STACK_OVERFLOW_CHILD";

fn deep_path() -> Vec<String> {
    include_str!("data/deep_path.txt").trim().split('.').map(String::from).collect()
}

/// Runs `work` on a thread with a stack of `STACK` bytes and waits for it.
fn on_small_stack(work: impl FnOnce() + Send + 'static) {
    std::thread::Builder::new()
        .stack_size(STACK)
        .spawn(work)
        .expect("thread starts")
        .join()
        .expect("the thread returns");
}

/// In the child, runs the crashing case; in the parent, starts the child and checks that it overflowed.
fn crashes_in_a_child(test: &str, case: impl FnOnce() + Send + 'static) {
    if std::env::var_os(CHILD).is_some() {
        on_small_stack(case);
        return;
    }
    let child = Command::new(std::env::current_exe().expect("the test binary"))
        .args([test, "--exact", "--nocapture", "--test-threads=1"])
        .env(CHILD, "1")
        .output()
        .expect("the child starts");
    let stderr = String::from_utf8_lossy(&child.stderr);
    println!("{test}: the child ended with {}", child.status);
    for line in stderr.lines().filter(|l| l.contains("overflow")) {
        println!("    {line}");
    }
    assert!(!child.status.success(), "{test}: the child did not crash");
    assert!(stderr.contains("has overflowed its stack"), "{test}: no stack overflow in\n{stderr}");
}

/// A copy of the tree as it was at first: the derived drop recurses once per level.
#[derive(Default)]
struct NaiveTree(BTreeMap<String, Box<NaiveTree>>);

// ---------- what crashes ----------

#[test]
fn recursive_insert_overflows_the_stack() {
    crashes_in_a_child("recursive_insert_overflows_the_stack", || {
        let path = deep_path();
        let parts: Vec<&str> = path.iter().map(String::as_str).collect();
        let mut tree = Tree::default();
        insert_path(&mut tree.0, &parts); // one call per level
    });
}

#[test]
fn recursive_write_overflows_the_stack() {
    crashes_in_a_child("recursive_write_overflows_the_stack", || {
        let path = deep_path();
        let parts: Vec<&str> = path.iter().map(String::as_str).collect();
        let mut tree = Tree::default();
        insert_path_iterative(&mut tree.0, &parts);
        write_tree(&mut io::sink(), &tree.0, 0).expect("the sink takes everything"); // one call per level
    });
}

#[test]
fn recursive_drop_overflows_the_stack() {
    crashes_in_a_child("recursive_drop_overflows_the_stack", || {
        let mut tree = NaiveTree::default();
        let mut level = &mut tree.0;
        for key in deep_path() {
            level = &mut level.entry(key).or_insert_with(Default::default).0;
        }
        drop(tree); // the derived drop: one call per level
    });
}

// ---------- what prevents it ----------

#[test]
fn iterative_insert_write_and_drop_need_no_call_stack() {
    on_small_stack(|| {
        let path = deep_path();
        let parts: Vec<&str> = path.iter().map(String::as_str).collect();
        let mut tree = Tree::default();
        insert_path_iterative(&mut tree.0, &parts);
        write_tree_iterative(&mut io::sink(), &tree.0, 0).expect("the sink takes everything");
        drop(tree); // Tree drops its levels one after the other
    });
}

#[test]
fn the_depth_limit_turns_the_crash_into_an_error() {
    on_small_stack(|| {
        let path = deep_path();
        let parts: Vec<&str> = path.iter().map(String::as_str).collect();
        let mut tree = Tree::default();
        insert_path_iterative(&mut tree.0, &parts);
        // The limit has to fit the stack: 30 levels of write_tree_safe stay well inside 64 KiB, also in a debug
        // build, whose frames are larger; 100 levels do not.
        let err = write_tree_safe(&mut io::sink(), &tree.0, 0, 30).expect_err("depth 5 000 exceeds the limit");
        println!("the_depth_limit_turns_the_crash_into_an_error: {err}");
        assert_eq!(err.kind(), io::ErrorKind::InvalidInput);
    });
}
