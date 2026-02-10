use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::BTreeMap;

#[derive(Default)]
struct Tree(BTreeMap<String, Box<Tree>>);

fn insert_path(tree: &mut BTreeMap<String, Box<Tree>>, parts: &[&str]) {
    if let Some((key, rest)) = parts.split_first() {
        let subtree = tree
            .entry((*key).to_string())
            .or_insert_with(|| Box::new(Tree::default()));
        if !rest.is_empty() {
            insert_path(&mut subtree.0, rest);
        }
    }
}

fn print_tree(tree: &BTreeMap<String, Box<Tree>>, depth: usize) {
    for (key, subtree) in tree {
        println!("{}{}", " ".repeat(depth), key);
        print_tree(&subtree.0, depth + 1);
    }
}

fn main() -> std::io::Result<()> {
    let file = File::open("file.txt")?;
    let reader = BufReader::new(file);

    let mut tree = Tree::default();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split('.').collect();
        insert_path(&mut tree.0, &parts);
    }

    print_tree(&tree.0, 0);
    Ok(())
}
