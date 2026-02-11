use std::fs::File;
use std::io::{BufRead, BufReader};
use dir_path_2_tree::{Tree, insert_path, print_tree};

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
