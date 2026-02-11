# dir-path-2-tree

A Rust program that reads dot-separated paths from a file and builds a hierarchical tree structure, displaying it with indentation to visualize the hierarchy.

## Project Structure

- `src/lib.rs` - Core library module containing the tree data structure and path insertion logic (unit-testable)
- `src/main.rs` - Application entry point that reads from a file and displays the tree

## Building the Project

Ensure you have Rust installed. If not, install it from [rustup.rs](https://rustup.rs/).

To build the project in release mode:

```bash
cargo build --release
```

The compiled binary will be available at `target/release/dir-path-2-tree.exe` (on Windows) or `target/release/dir-path-2-tree` (on Unix).

## Running the Program

Place your input paths in a file named `file.txt` in the project directory. Each line should contain dot-separated path components.

Example `file.txt`:

```text
var.text.bla.bla
usr.bin.bash
usr.bin.c
home.me.and.you
var.you.dont.know
home.me.can.dont
```

Run the program:

```bash
cargo run --release
```

Output:

```text
home
 me
  and
   you
  can
   dont
usr
 bin
  bash
  c
var
 text
  bla
   bla
 you
  dont
   know
```

The tree is displayed with each element on its own line, indented by depth level to show the hierarchy. Note that entries are sorted alphabetically by BTreeMap.

## Running Unit Tests

Run all unit tests:

```bash
cargo test --lib
```

Run tests with output displayed:

```bash
cargo test --lib -- --nocapture
```

Run a specific test:

```bash
cargo test --lib test_insert_single_path
```

### Available Tests

The library includes 7 comprehensive unit tests:

- `test_insert_single_path` - Validates insertion of a simple path
- `test_insert_multiple_paths` - Tests multiple paths with shared prefixes
- `test_insert_single_element` - Edge case for single-node paths
- `test_insert_empty_parts` - Ensures empty paths are handled gracefully
- `test_tree_structure_is_sorted` - Verifies BTreeMap maintains alphabetical order
- `test_complex_tree_structure` - Tests a complex multi-level hierarchy
- `test_insert_overlapping_paths` - Validates handling of overlapping paths

All tests should show:

```bash
test result: ok. 7 passed; 0 failed; 0 ignored
```

## Code Coverage

To generate a code coverage report, first install `cargo-tarpaulin`:

```bash
cargo install cargo-tarpaulin
```

Generate an HTML coverage report:

```bash
cargo tarpaulin --out Html --output-dir coverage
```

The report will be generated at `coverage/tarpaulin-report.html`. To view it, start a local HTTP server:

```bash
python -m http.server 8888
```

Then open `http://localhost:8888/tarpaulin-report.html` in your browser.

## Architecture

### Tree Structure

The `Tree` struct wraps a `BTreeMap` to maintain sorted, hierarchical paths:

```rust
pub struct Tree(pub BTreeMap<String, Box<Tree>>);
```

### Key Functions

- `insert_path(tree, parts)` - Recursively inserts dot-separated path components into the tree
- `print_tree(tree, depth)` - Recursively displays the tree with depth-based indentation

The implementation uses Rust's ownership and recursive patterns to elegantly handle nested tree structures without manual memory management.
