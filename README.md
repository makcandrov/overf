# overf

[<img alt="github" src="https://img.shields.io/badge/github-source-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="22">](https://github.com/makcandrov/overf)
[<img alt="crates.io" src="https://img.shields.io/crates/v/overf.svg?style=for-the-badge&color=fc8d62&logo=rust" height="22">](https://crates.io/crates/overf)
[<img alt="docs.rs" src="https://img.shields.io/docsrs/overf/latest?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K" height="22">](https://docs.rs/overf)


Define the overflow behavior of mathematical operations within blocks of code. This library provides the macros `checked!`, `overflowing!`, `saturating!`, `propagating!` and `default!`, enabling you to easily manage overflow in nested contexts.

# Features

- **Customizable Overflow Behavior**: Choose between checked, overflowing, and saturating operations for any block of code.
- **Nested Blocks**: Define different overflow behaviors in nested blocks for more granular control.
- **Reset Behavior**: Use the `default!` macro to reset the overflow behavior back to the default.

# Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
overf = "0.1"
```

# Usage

```rust
use overf::{checked, overflowing, saturating, default};

fn main() {
    checked! {
        let result = 1usize + 1usize;
        let sum = default! { 2usize + 2usize }; // Use default behavior
        let difference = 5usize - 3usize;

        // Nested blocks
        saturating! {
            let total = 100u8 - 200u8;
        }
    }

    overflowing! {
        let result = 1usize + usize::MAX;
    }
}
```
