//! A deep tree on a small stack: built, written and dropped without recursion.

use std::io;

use dir_path_2_tree::*;

#[test]
fn a_deep_tree_needs_no_call_stack() {
    // 64 KiB of stack, as a thread on an embedded target might get, and one path of depth 20 000.
    std::thread::Builder::new()
        .stack_size(64 * 1024)
        .spawn(|| {
            let deep: Vec<String> = (0..20_000).map(|i| format!("d{i}")).collect();
            let parts: Vec<&str> = deep.iter().map(String::as_str).collect();
            let mut tree = Tree::default();
            insert_path_iterative(&mut tree.0, &parts);
            write_tree_iterative(&mut io::sink(), &tree.0, 0).expect("the sink takes everything");
            drop(tree); // iterative as well: the derived Drop would recurse once per level
        })
        .expect("thread starts")
        .join()
        .expect("no stack overflow");
}
