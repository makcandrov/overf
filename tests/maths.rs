#![allow(unused)]

use overf::{checked, default, overflowing, propagating, saturating};

#[test]
fn test_checked_arithmetic() {
    checked! {
        let a = 10usize + 5usize;
        let b = 20usize - 10usize;
        let c = 3usize * 7usize;
        let d = 21usize / 3usize;
        let e = 10usize % 3usize;
        let f = 1usize << 3; // 8
        let g = 8usize >> 2; // 2

        // Nested
        default! {
            let h = a + b; // Should not panic
            let i = b - d; // Should not panic
        }

        let k = -1i32;
    }
}

#[test]
fn test_overflowing_arithmetic() {
    overflowing! {
        let a = 10usize + 5usize;
        let b = 20usize - 10usize;
        let c = 3usize * 7usize;
        let d = 21usize / 3usize;
        let e = 10usize % 3usize;
        let f = 1usize << 3;
        let g = 8usize >> 2;

        // Nested
        default! {
            let h = a + b; // Should not panic
            let i = b - d; // Should not panic
        }

        let k = -g;
    }
}

#[test]
fn test_saturating_arithmetic() {
    saturating! {
        let a = 10usize + 5usize;
        let b = 20usize - 10usize;
        let c = 3usize * 7usize;
        let d = 21usize / 3usize;

        // These operators don't have a saturating version.
        // It should fallback to the default behaviour.
        let e = 10usize % 3usize;
        let f = 1usize << 3; // 8
        let g = 8usize >> 2; // 2

        // Nested
        checked! {
            let h = a + b; // Should not panic
            let i = b - d; // Should not panic
        }
    }
}

#[test]
fn test_propagating_arithmetic() {
    propagating! {
        fn inner() -> Option<()> {
            let a = 10usize + 5usize;
            let b = 20usize - 10usize;
            let c = 3usize * 7usize;
            let d = 21usize / 3usize;
            let e = 10usize % 3usize;
            let f = 1usize << 3; // 8
            let g = 8usize >> 2; // 2

            // Nested
            default! {
                let h = a + b; // Should not panic
                let i = b - d; // Should not panic
            }

            let k = -g;

            Some(())
        }

    }
}
