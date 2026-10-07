//! All variants build the same tree and write the same lines.

use std::collections::BTreeMap;
use std::io;

use dir_path_2_tree::*;

type Insert = fn(&mut BTreeMap<String, Box<Tree>>, &[&str]);

const INSERTS: [(&str, Insert); 3] = [
    ("recursive", insert_path),
    ("iterative", insert_path_iterative),
    ("explicit stack", insert_path_iterative_with_stack),
];

fn paths() -> Vec<Vec<String>> {
    let mut paths: Vec<Vec<String>> = ["var.text.bla.bla", "usr.bin.bash", "usr.bin.c", "home.me.and.you",
        "var.you.dont.know", "home.me.can.dont", "a.b.c", "a.e", "a.b"]
        .iter()
        .map(|p| p.split('.').map(String::from).collect())
        .collect();
    paths.extend((0..500u32).map(|i| (0..4).map(|level| format!("n{}", (i / 3u32.pow(level)) % 3)).collect()));
    paths
}

fn build(paths: &[Vec<String>], insert: Insert) -> Tree {
    let mut tree = Tree::default();
    for path in paths {
        let parts: Vec<&str> = path.iter().map(String::as_str).collect();
        insert(&mut tree.0, &parts);
    }
    tree
}

fn written(write: impl Fn(&mut Vec<u8>) -> io::Result<()>) -> String {
    let mut out = Vec::new();
    write(&mut out).expect("writing to memory does not fail");
    String::from_utf8(out).expect("the keys are UTF-8")
}

#[test]
fn every_insert_variant_builds_the_same_tree() {
    let reference = build(&paths(), insert_path);
    for (name, insert) in INSERTS {
        assert_eq!(build(&paths(), insert), reference, "{name} builds another tree");
    }
}

#[test]
fn every_write_variant_writes_the_same_lines() {
    let tree = build(&paths(), insert_path);
    let reference = written(|out| write_tree(out, &tree.0, 0));
    assert_eq!(written(|out| write_tree_iterative(out, &tree.0, 0)), reference, "iterative");
    assert_eq!(written(|out| write_tree_safe(out, &tree.0, 0, 1000)), reference, "safe");
}

#[test]
fn the_iterative_variant_keeps_the_pre_order() {
    let tree = build(&[vec!["a".into(), "b".into(), "c".into()], vec!["a".into(), "e".into()]], insert_path);
    assert_eq!(written(|out| write_tree_iterative(out, &tree.0, 0)), "a\n b\n  c\n e\n");
}

#[test]
fn the_safe_variant_stops_at_its_limit() {
    let deep: Vec<String> = (0..10).map(|i| format!("d{i}")).collect();
    let tree = build(&[deep], insert_path);
    let mut out = Vec::new();
    let err = write_tree_safe(&mut out, &tree.0, 0, 3).expect_err("depth 10 exceeds the limit of 3");
    assert_eq!(err.kind(), io::ErrorKind::InvalidInput);
    assert_eq!(String::from_utf8(out).unwrap().lines().count(), 4, "the lines up to the limit are written");
}

