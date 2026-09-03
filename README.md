# RTIC-RW

Crate to showcase the 1-1 relationship between the RTIC-RW resource handling and the Rust aliasing invariant:

At any time a (shared) resource `R` (location) should be:

- accessible through an arbitrary number of immutable references (`& R`), or
- accessible through a single mutable reference (`&mut R`)
