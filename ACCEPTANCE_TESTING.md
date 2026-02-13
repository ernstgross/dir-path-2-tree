# Acceptance Testing with Cucumber/Gherkin

## Summary

A Gherkin-based acceptance test framework has been added to the `dir-path-2-tree` project using the `cucumber` crate for Rust.

## What Was Added

### 1. **Cargo.toml Updates**

- Added `cucumber = "0.20"` as a dev dependency
- Added `tokio` with full features for async runtime support

### 2. **Gherkin Feature File**

- **Location**: `tests/features/tree_building.feature`
- **Contains**: 3 acceptance test scenarios
  - **Scenario 1**: Build tree from multiple dot-separated paths (uses `file.txt` input with expected output)
  - **Scenario 2**: Build tree from single nested path (`a.b.c`)
  - **Scenario 3**: Build tree from paths with multiple branches (`a.b.c`, `a.b.d`, `a.e`)

### 3. **Test Implementation**

- **Location**: `tests/features.rs`
- **Test World**: TreeWorld struct that holds:
  - `paths`: List of dot-separated path strings from input
  - `output`: The formatted tree output
- **Step Definitions** (3 step functions):
  - `#[given]` - Reads paths from Gherkin table
  - `#[when]` - Builds the tree structure
  - `#[then]` - Compares output against expected docstring

### 4. **Integration Tests**

- **Location**: `tests/integration_test.rs`
- Verifies that tree building functions work correctly independent of the Gherkin framework

## Running the Tests

### Run all tests

```bash
cargo test
```

### Run only Cucumber acceptance tests

```bash
cargo test --test features
```

### Run only integration tests

```bash
cargo test --test integration_test
```

### Run with verbose output

```bash
cargo test -- --nocapture --test-threads=1
```

## Test Results

All tests pass successfully

```bash
✓ 3 Gherkin scenarios (9 steps) - Acceptance tests
✓ 2 Unit tests - Integration tests
✓ 3 Doc tests - Library documentation examples
```

## BDD/Gherkin Framework Benefits

1. **Readable Business Language**: Feature files are written in English, understandable to non-technical stakeholders
2. **Automated Execution**: Tests run automatically via `cargo test` with standard Rust tooling
3. **Living Documentation**: Feature files serve as executable specifications
4. **Easy to Extend**: Add more scenarios without changing test code
5. **Reusable Steps**: Step implementations are reusable across multiple scenarios

## Framework Details

### Cucumber-rs v0.20

- **Lightweight BDD framework for Rust**
- Supports Gherkin syntax with `Given-When-Then` structure
- Async/await support via Tokio
- Automatic test discovery and reporting

### Adding More Tests

To add a new scenario, simply add it to `tree_building.feature`:

```gherkin
Scenario: Your new test
  Given I have a file with dot-separated paths:
    | path.to.test |
  When I build the tree
  Then the output should match:
    """
    path
     to
      test
    """
```

The step definitions in `tests/features.rs` will automatically execute the test when the feature file is parsed.
