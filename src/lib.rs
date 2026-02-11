use std::collections::BTreeMap;

#[derive(Default, Debug, PartialEq, Clone)]
pub struct Tree(pub BTreeMap<String, Box<Tree>>);

/// Inserts a path (as dot-separated parts) into the tree structure
/// 
/// # Arguments
/// * `tree` - The tree node to insert into
/// * `parts` - Slice of path parts to insert
/// 
/// # Example
/// ```
/// use dir_path_2_tree::*;
/// let mut tree = Tree::default();
/// insert_path(&mut tree.0, &["a", "b", "c"]);
/// ```
pub fn insert_path(tree: &mut BTreeMap<String, Box<Tree>>, parts: &[&str]) {
    if let Some((key, rest)) = parts.split_first() {
        let subtree = tree
            .entry((*key).to_string())
            .or_insert_with(|| Box::new(Tree::default()));
        if !rest.is_empty() {
            insert_path(&mut subtree.0, rest);
        }
    }
}

/// Prints the tree structure with indentation based on depth
/// 
/// # Arguments
/// * `tree` - The tree to print
/// * `depth` - Current depth level for indentation
pub fn print_tree(tree: &BTreeMap<String, Box<Tree>>, depth: usize) {
    for (key, subtree) in tree {
        println!("{}{}", " ".repeat(depth), key);
        print_tree(&subtree.0, depth + 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_single_path() {
        let mut tree = Tree::default();
        insert_path(&mut tree.0, &["a", "b", "c"]);
        
        assert!(tree.0.contains_key("a"));
        assert!(tree.0["a"].0.contains_key("b"));
        assert!(tree.0["a"].0["b"].0.contains_key("c"));
        assert!(tree.0["a"].0["b"].0["c"].0.is_empty());
    }

    #[test]
    fn test_insert_multiple_paths() {
        let mut tree = Tree::default();
        insert_path(&mut tree.0, &["a", "b", "c"]);
        insert_path(&mut tree.0, &["a", "b", "d"]);
        insert_path(&mut tree.0, &["a", "e"]);
        
        assert!(tree.0.contains_key("a"));
        let a_subtree = &tree.0["a"].0;
        
        assert!(a_subtree.contains_key("b"));
        assert!(a_subtree.contains_key("e"));
        
        let b_subtree = &a_subtree["b"].0;
        assert!(b_subtree.contains_key("c"));
        assert!(b_subtree.contains_key("d"));
    }

    #[test]
    fn test_insert_single_element() {
        let mut tree = Tree::default();
        insert_path(&mut tree.0, &["single"]);
        
        assert!(tree.0.contains_key("single"));
        assert!(tree.0["single"].0.is_empty());
    }

    #[test]
    fn test_insert_empty_parts() {
        let mut tree = Tree::default();
        let original = tree.clone();
        insert_path(&mut tree.0, &[]);
        
        assert_eq!(tree, original);
    }

    #[test]
    fn test_tree_structure_is_sorted() {
        let mut tree = Tree::default();
        insert_path(&mut tree.0, &["z"]);
        insert_path(&mut tree.0, &["a"]);
        insert_path(&mut tree.0, &["m"]);
        
        let keys: Vec<_> = tree.0.keys().collect();
        assert_eq!(keys, vec![&"a".to_string(), &"m".to_string(), &"z".to_string()]);
    }

    #[test]
    fn test_complex_tree_structure() {
        let mut tree = Tree::default();
        insert_path(&mut tree.0, &["config", "database", "host"]);
        insert_path(&mut tree.0, &["config", "database", "port"]);
        insert_path(&mut tree.0, &["config", "app", "name"]);
        insert_path(&mut tree.0, &["logs", "error"]);
        
        assert_eq!(tree.0.len(), 2);
        assert!(tree.0.contains_key("config"));
        assert!(tree.0.contains_key("logs"));
        
        let config = &tree.0["config"].0;
        assert_eq!(config.len(), 2);
        assert!(config.contains_key("database"));
        assert!(config.contains_key("app"));
        
        let database = &config["database"].0;
        assert_eq!(database.len(), 2);
    }

    #[test]
    fn test_insert_overlapping_paths() {
        let mut tree = Tree::default();
        insert_path(&mut tree.0, &["a", "b"]);
        insert_path(&mut tree.0, &["a", "b", "c"]);
        
        assert!(tree.0["a"].0["b"].0.contains_key("c"));
    }
}
