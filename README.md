<!-- cargo-rdme start -->

Compile-time checked non-zero integers with type inference and first-class `const` support.

```rust
const UNSIGNED: NonZeroUsize = nonzero!(1);
const SIGNED: NonZeroI8 = nonzero!(-1);
           // ^ correctly infers return type

const MY_CONST: usize = 20;
const MY_NONZERO_CONST: NonZeroUsize = nonzero!(MY_CONST - 19);
              // refer to other constant values ^
```

```rust
let runtime = nonzero!(0); // eager evaluation
```

# Comparison with other libraries
- [`nonzero`](https://docs.rs/nonzero/latest/nonzero/) uses a proc-macro
  that parses the expression passed to it, precluding
  - Type inference.
  - Referencing constants.

<!-- cargo-rdme end -->
