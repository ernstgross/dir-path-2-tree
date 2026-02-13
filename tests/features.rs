use cucumber::{gherkin::Step, given, when, then, World};
use dir_path_2_tree::{insert_path, Tree};

#[derive(World, Debug, Default, Clone)]
pub struct TreeWorld {
    paths: Vec<String>,
    output: String,
}

#[given(expr = "I have a file with dot-separated paths:")]
fn step_given_paths(world: &mut TreeWorld, step: &Step) {
    world.paths.clear();
    if let Some(table) = step.table() {
        // Cucumber tables with vertical bar syntax all rows are data (no header row in this case)
        for row in table.rows.iter() {
            if let Some(path) = row.get(0) {
                world.paths.push(path.to_string());
            }
        }
    }
}

#[when(expr = "I build the tree")]
fn step_when_build_tree(world: &mut TreeWorld) {
    let mut tree = Tree::default();

    for path_str in &world.paths {
        let parts: Vec<&str> = path_str.split('.').collect();
        insert_path(&mut tree.0, &parts);
    }

    world.output = format_tree(&tree.0, 0);
}

#[then(expr = "the output should match:")]
fn step_then_verify_output(world: &mut TreeWorld, step: &Step) {
    let expected = step.docstring()
        .map(|s| s.to_string())
        .unwrap_or_default();
    
    assert_eq!(
        world.output.trim(),
        expected.trim(),
        "Output mismatch!\nExpected:\n{}\n\nActual:\n{}",
        expected,
        world.output
    );
}

/// Helper function to format the tree as a string (matches the print_tree behavior)
fn format_tree(tree: &std::collections::BTreeMap<String, Box<Tree>>, depth: usize) -> String {
    let mut result = String::new();
    for (key, subtree) in tree {
        result.push_str(&" ".repeat(depth));
        result.push_str(key);
        result.push('\n');
        result.push_str(&format_tree(&subtree.0, depth + 1));
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn cucumber_tests() {
        TreeWorld::run("tests/features").await;
    }
}


