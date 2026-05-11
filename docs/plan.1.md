# Plan: Declarative Macro for flood-tide (Zero-Dependency)

## Objective
Implement a declarative macro (`macro_rules!`) to automate the generation of boilerplate code required by `flood-tide`. The goal is to provide a "middle ground" for small to medium projects that avoids manual table management without introducing heavy procedural macro dependencies.

## Key Features
- **Zero Dependencies:** Uses only built-in `macro_rules!` and `const fn`.
- **Automatic Generation:** Generates the `enum` for IDs, the configuration `struct`, sorted `Opt` arrays, and the `match` parsing logic.
- **Compile-time Validation/Sorting:** Utilizes `const fn` to ensure that the internal tables used for binary search are correctly sorted at compile time.

## Proposed Syntax
```rust
flood_tide::argparse! {
    pub struct MyConfig {
        #[option(short = 'v', long = "verbose", desc = "Enable verbose output")]
        verbose: bool,
        #[option(short = 'c', long = "count", has_arg = Arg::Yes, desc = "Set count")]
        count: String,
    }
}
```

## Implementation Steps

### Phase 1: Compile-time Sorting Utility
1. Create a `const fn` that can sort an array of `Opt` structures. Since `flood-tide` relies on binary search, the `OPT_ARY` (sorted by long name) and `OPT_ARY_SHO_IDX` (sorted by short name) must be perfectly ordered.
2. Implement a validation check using `const` assertions to ensure the macro-generated data is valid.

### Phase 2: Macro Definition (`macro_rules!`)
1. Define the `argparse!` macro to:
    - Parse the custom struct definition and attributes.
    - Generate a `CmdOP` enum automatically.
    - Generate the `Opt` array.
    - Generate a helper function or trait implementation for `parse_match`.

### Phase 3: Refactoring Examples
1. Update `examples/gnu-cat.rs` or create a new example using the macro to demonstrate the drastic reduction in lines of code (target: >50% reduction in boilerplate).

## Verification & Testing
1. **Performance:** Compare the binary size and execution speed of the macro-generated parser against the manual implementation.
2. **Compatibility:** Ensure it works in `no_std` environments.
3. **Safety:** Replace `unsafe` `transmute_copy` with safe, macro-generated `match` statements for enum conversion.

---
Review Date: 2026-05-11
Reviewer: Gemini CLI Agent
