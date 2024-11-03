# overf

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
