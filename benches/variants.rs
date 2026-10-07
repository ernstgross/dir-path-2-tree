//! The variants against each other: inserting paths and writing the tree, on a wide and on a deep tree.
//! Writing goes to `io::sink()`, so that the traversal is measured and not the console.
//!
//! Run: `cargo bench --bench variants`; the report is in `target/criterion/report/index.html`.

use std::collections::BTreeMap;
use std::io;

use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use dir_path_2_tree::*;

type Insert = fn(&mut BTreeMap<String, Box<Tree>>, &[&str]);

/// 10 000 paths of depth 6 over 10 names per level: wide and shallow, like a file or configuration tree.
fn wide() -> Vec<Vec<String>> {
    (0..10_000u32).map(|i| (0..6).map(|level| format!("n{}", (i / 10u32.pow(level)) % 10)).collect()).collect()
}

/// One path of depth 1000: deep and narrow, where the call stack matters.
fn deep() -> Vec<Vec<String>> {
    vec![(0..1000).map(|i| format!("d{i}")).collect()]
}

fn build(paths: &[Vec<&str>], insert: Insert) -> Tree {
    let mut tree = Tree::default();
    for parts in paths {
        insert(&mut tree.0, parts);
    }
    tree
}

fn shapes() -> Vec<(&'static str, Vec<Vec<String>>)> {
    vec![("wide", wide()), ("deep", deep())]
}

fn insert(c: &mut Criterion) {
    for (shape, paths) in shapes() {
        let parts: Vec<Vec<&str>> = paths.iter().map(|p| p.iter().map(String::as_str).collect()).collect();
        let mut group = c.benchmark_group(format!("insert {shape}"));
        for (name, f) in [("recursive", insert_path as Insert), ("iterative", insert_path_iterative),
                          ("explicit stack", insert_path_iterative_with_stack)] {
            // One tree per measurement, dropped outside the timing. Collecting many trees of several MB and dropping
            // them together (iter_with_large_drop) let the allocator decide the result: the order of the variants
            // turned over from one run to the next.
            group.bench_function(name, |b| {
                b.iter_batched(|| (), |()| build(black_box(&parts), f), BatchSize::PerIteration)
            });
        }
        group.finish();
    }
}

fn write(c: &mut Criterion) {
    for (shape, paths) in shapes() {
        let parts: Vec<Vec<&str>> = paths.iter().map(|p| p.iter().map(String::as_str).collect()).collect();
        let tree = build(&parts, insert_path_iterative);
        let mut group = c.benchmark_group(format!("write {shape}"));
        group.bench_function("recursive", |b| b.iter(|| write_tree(&mut io::sink(), black_box(&tree.0), 0)));
        group.bench_function("iterative", |b| b.iter(|| write_tree_iterative(&mut io::sink(), black_box(&tree.0), 0)));
        group.bench_function("safe", |b| b.iter(|| write_tree_safe(&mut io::sink(), black_box(&tree.0), 0, 5000)));
        group.finish();
    }
}

criterion_group!(benches, insert, write);
criterion_main!(benches);
