# dir-path-2-tree

A Rust program that reads dot-separated paths from a file and builds a hierarchical tree structure, displaying it with indentation to visualize the hierarchy.

## Project Structure

- `src/lib.rs` - Core library module containing the tree data structure and path insertion logic (unit-testable)
- `src/main.rs` - Application entry point that reads from a file and displays the tree
- `tests/` - Integration, equivalence and acceptance tests (Cucumber), and the stack overflow demonstration with its
  input `tests/data/deep_path.txt`
- `benches/variants.rs` - Criterion benchmarks of the variants
- `tools/build_pages.py`, `.github/workflows/pages.yml` - The project page on GitHub Pages

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

### Further Tests

- `tests/equivalence.rs` - Every insert variant builds the same tree, every write variant writes the same lines, the
  iterative one in pre-order, the safe one stops at its limit
- `tests/stack_overflow.rs` - Where recursion overflows the stack, and what prevents it (section *Recursion, Iteration
  and the Stack*)
- `tests/features.rs` with `tests/features/tree_building.feature` - Acceptance tests in Gherkin (`ACCEPTANCE_TESTING.md`)
- `tests/integration_test.rs` - The public API from outside the crate

Run them all with `cargo test`.

## Code Coverage

Coverage is measured with [cargo-llvm-cov](https://github.com/taiki-e/cargo-llvm-cov), source-based through LLVM:

```
rustup component add llvm-tools-preview
cargo install cargo-llvm-cov --locked
cargo llvm-cov --html
```

The report is written to `target/llvm-cov/html/index.html`; `cargo llvm-cov` alone prints the summary. The project
page shows the coverage of every push to `main`, file by file and line by line.

What the numbers leave out: `src/main.rs` runs only with `cargo run`; the `print_` functions, thin wrappers that
write to the console, run in doc tests, which cargo-llvm-cov does not count by default; and a child process of the
stack overflow demonstration aborts before it can write its profile - the routines it crashes in are covered by the
other tests.

## Architecture

### Tree Structure

The `Tree` struct wraps a `BTreeMap` to maintain sorted, hierarchical paths:

```rust
pub struct Tree(pub BTreeMap<String, Box<Tree>>);
```

### Key Functions

The same task in several forms, on purpose:

| Function | Form | Call stack per level of the tree |
| --- | --- | --- |
| `insert_path` | recursive | one frame |
| `insert_path_iterative` | a loop that walks down with a mutable reference | none |
| `insert_path_iterative_with_stack` | the recursion translated mechanically into an explicit stack; needs `unsafe` to satisfy the borrow checker | none |
| `write_tree` / `print_tree` | recursive, pre-order | one frame |
| `write_tree_iterative` / `print_tree_iterative` | an explicit stack of iterators, same order | none |
| `write_tree_safe` / `print_tree_safe` | recursive with a depth limit: an error instead of a crash | one frame, at most `max_depth` |
| `Drop for Tree` | takes the levels out one by one | none |

The `write_` functions write to any `std::io::Write` - a `Vec<u8>` in the tests, `io::sink()` in the benchmarks; the
`print_` functions write to the console.

The implementation uses Rust's ownership and recursive patterns to elegantly handle nested tree structures without manual memory management.

## Recursion, Iteration and the Stack

Recursion is the natural form for a tree, and the right one while the depth is small and known. When the depth comes
from outside - a file, a message, a user - every level costs a stack frame, and a deep enough input overflows the
stack. Rust cannot catch that: the process aborts. On a desktop the main thread has megabytes of stack; a thread on
an embedded target often has a few kilobytes.

`tests/stack_overflow.rs` shows it with one path of depth 5 000 (`tests/data/deep_path.txt`, 29 KB) on a thread with
64 KiB of stack. Each crashing case runs in a child process - the test binary started again with only that test - and
the test checks that the child died with Rust's message:

```text
$ cargo test --test stack_overflow -- --nocapture --test-threads=1
running 5 tests
test iterative_insert_write_and_drop_need_no_call_stack ... ok
test recursive_drop_overflows_the_stack ... recursive_drop_overflows_the_stack: the child ended with signal: 6 (SIGABRT) (core dumped)
    thread '<unknown>' (438428) has overflowed its stack
    fatal runtime error: stack overflow, aborting
ok
test recursive_insert_overflows_the_stack ... recursive_insert_overflows_the_stack: the child ended with signal: 6 (SIGABRT) (core dumped)
    thread '<unknown>' (438432) has overflowed its stack
    fatal runtime error: stack overflow, aborting
ok
test recursive_write_overflows_the_stack ... recursive_write_overflows_the_stack: the child ended with signal: 6 (SIGABRT) (core dumped)
    thread '<unknown>' (438436) has overflowed its stack
    fatal runtime error: stack overflow, aborting
ok
test the_depth_limit_turns_the_crash_into_an_error ... the_depth_limit_turns_the_crash_into_an_error: Maximum tree depth 30 exceeded (current depth: 31)
ok
test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.07s
```

| Case | Result on 64 KiB | What it shows |
| --- | --- | --- |
| `insert_path`, recursive | stack overflow | one frame per level of the input |
| `write_tree`, recursive | stack overflow | the same when printing |
| a copy of the tree with the derived `Drop` | stack overflow | even a tree built and printed iteratively crashes when it goes out of scope: the derived drop recurses |
| `insert_path_iterative`, `write_tree_iterative`, `Drop for Tree` | passes | the depth costs heap, not stack |
| `write_tree_safe` with a limit of 30 | error `Maximum tree depth 30 exceeded` | the crash becomes an error the caller can handle |

The limit has to fit the stack: 30 levels of `write_tree_safe` fit into 64 KiB in a debug build, 100 levels do not,
because unoptimised frames are larger.

Where each form fits:

- **Recursive** - host tools, configuration and file trees, trusted input of known, small depth; the code mirrors
  the data and is easy to review.
- **A loop instead of recursion** - when the recursion follows a single path, as `insert_path` does; constant stack,
  no `unsafe`, as readable as the recursion. Rust does not guarantee tail-call elimination, so the loop is written by
  hand.
- **An explicit stack** - for a traversal that branches, as printing does, over input of unknown depth: threads with
  a small fixed stack, interrupt contexts, rules that forbid recursion (MISRA C rule 17.2, the JPL "Power of 10").
- **Recursion with a depth limit** - when the recursive form should stay but the input is not trusted: parsers at a
  trust boundary, protection against uncontrolled recursion (CWE-674).

## Benchmarks

`benches/variants.rs` measures the variants with Criterion on a wide tree (10 000 paths of depth 6) and a deep one
(one path of depth 1 000), inserting and writing. Writing goes to `io::sink()`, so that the traversal is measured and
not the console; each insert builds one tree, dropped outside the timing.

```text
cargo bench --bench variants
```

The report is written to `target/criterion/report/index.html`.

| Benchmark | Variant | Median of the means | Means of the runs | Against recursive |
| --- | --- | --- | --- | --- |
| insert wide | recursive | 8.22 ms | 8.22 ms, 8.27 ms, 7.95 ms | — |
| insert wide | iterative | 5.62 ms | 6.25 ms, 5.54 ms, 5.62 ms | -32 % |
| insert wide | explicit stack | 5.85 ms | 5.78 ms, 5.85 ms, 5.86 ms | -29 % |
| insert deep | recursive | 62 µs | 60 µs, 62 µs, 62 µs | — |
| insert deep | iterative | 54 µs | 54 µs, 56 µs, 54 µs | -12 % |
| insert deep | explicit stack | 58 µs | 59 µs, 57 µs, 58 µs | -6 % |
| write wide | recursive | 3.21 ms | 3.41 ms, 3.21 ms, 2.88 ms | — |
| write wide | iterative | 3.30 ms | 3.29 ms, 3.30 ms, 3.31 ms | +3 % |
| write wide | safe | 2.97 ms | 3.06 ms, 2.97 ms, 2.85 ms | -7 % |
| write deep | recursive | 56 µs | 56 µs, 57 µs, 55 µs | — |
| write deep | iterative | 60 µs | 60 µs, 61 µs, 59 µs | +7 % |
| write deep | safe | 58 µs | 56 µs, 58 µs, 65 µs | +4 % |

Measured on an Intel Core i7-7700K under WSL2 (Ubuntu 24.04), rustc 1.98.1, Criterion 0.5: each run pinned to one
core (`taskset -c 3`), 10 s per benchmark, three runs. The table gives the median of the three means, the mean of
each run, and the median against the recursive variant.

What the numbers say:

- **The depth limit costs nothing measurable.** `safe` against `recursive`: -7 % on the wide tree, +4 % on the deep
  one - inside a spread of up to ±10 % between runs of the same code. On this machine, differences below about
  10 % cannot be told apart.
- **Writing with an explicit stack costs a little.** The stack of iterators is 3 to 7 % slower than the recursion; on
  the deep tree with the same sign in every run.
- **Inserting is dominated by allocation, not by the call stack.** Every new key allocates a `String` and a map
  node. Within a group, the variant measured first is the slowest: in the table that is the recursive one. Measured
  alone, each in a process of its own and alternating, the three variants lie within about 10 % of each other on the
  wide tree - iterative 7.6 and 7.0 ms, recursive 8.5 and 7.6 ms, explicit stack 6.9 and 6.8 ms - with overlapping
  confidence intervals. The order of the measurements and the state of the allocator move the result more than the
  form of the code.
- **A benchmark can mislead by itself.** A first version collected many trees and dropped them together
  (`iter_with_large_drop`); from one run to the next the order of the variants turned over - recursive 15.0 ms
  against iterative 6.2 ms, then 5.6 ms against 16.4 ms. Each tree is now dropped alone, outside the timing.
- **The spread is part of the result.** A desktop under WSL2 shares its cores with a virtual machine and everything
  else that runs, and a benchmark that allocates several MB per iteration measures the allocator too. A quiet native
  Linux machine with a fixed CPU frequency narrows the spread; the conclusions above should hold there, the absolute
  numbers will not.

## Project Page

The project page: <https://ernstgross.github.io/dir-path-2-tree/>

`.github/workflows/pages.yml` builds it on every push to `main` (`tools/build_pages.py`): this README, the test
results with the stack overflow demonstration of that run, the code coverage with its report, the benchmarks with
the Criterion report of that run, and the API documentation (`cargo doc`). The benchmark numbers above come from a dedicated machine; those of the page come from a
shared GitHub runner and are indicative only.

## Credits

Written by Ernst Gross. The tests of the stack overflow, the benchmarks, the project page and parts of this README
were written with the help of Claude (Anthropic); the commits concerned carry the trailer `Assisted-by: Claude
(Anthropic)`.
