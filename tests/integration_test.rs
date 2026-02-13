use dir_path_2_tree::{insert_path, Tree};

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

#[test]
fn test_single_path() {
    let mut tree = Tree::default();
    let parts: Vec<&str> = vec!["a", "b", "c"];
    insert_path(&mut tree.0, &parts);
    
    let output = format_tree(&tree.0, 0);
    let expected = "a\n b\n  c\n";
    
    println!("Output:\n{:?}", output);
    println!("Expected:\n{:?}", expected);
    assert_eq!(output, expected, "Output should match");
}

#[test]
fn test_multiple_paths() {
    let mut tree = Tree::default();
    let paths = vec![
        vec!["var", "text", "bla", "bla"],
        vec!["usr", "bin", "bash"],
        vec!["usr", "bin", "c"],
        vec!["home", "me", "and", "you"],
        vec!["var", "you", "dont", "know"],
        vec!["home", "me", "can", "dont"],
    ];
    
    for path in paths {
        insert_path(&mut tree.0, &path);
    }
    
    let output = format_tree(&tree.0, 0);
    println!("Output:\n{}", output);
    
    // Verify var branch has both text and you
    assert!(output.contains("var\n text"), "Should have var > text");
    assert!(output.contains("var\n") , "Should have var");
    assert!(output.contains("\n you\n"), "Should have you under var");
}
