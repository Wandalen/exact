# 001: Display::fmt for MinorError

## Representation

Renders each of the two `MinorError` variants as a sentence: `Overflow` as
`"{operation} rose above the representable range"`, `Underflow` as
`"{operation} fell below the representable range"`. See
[impl Display for MinorError](../implementation/001_display_for_minor_error.md)
for the full body.

## Kind

Associated Function/Method (§ Item Kind Taxonomy : Associated Item Kinds #1)

## Definition

`module/exact_minor/src/lib.rs:206`

```rust
fn fmt( &self, f : &mut fmt::Formatter< '_ > ) -> fmt::Result
{
  match self
  {
    Self::Overflow { operation } => write!( f, "{operation} rose above the representable range" ),
    Self::Underflow { operation } => write!( f, "{operation} fell below the representable range" ),
  }
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 206-213 | Declaration |
| `tests/checked_arithmetic_test.rs` | `overflow_error_names_the_failed_operation` | Via `.to_string()` |

No production code anywhere in the workspace calls this method explicitly or
via `.to_string()`/format interpolation on a `MinorError`-typed value —
confirmed by grep across every `.rs` file in `module/`. Only `exact_minor`'s
own `tests/checked_arithmetic_test.rs` renders one.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_minor` | `(defining crate)` | Exercised by its own `tests/checked_arithmetic_test.rs` |

## Caller Tree

No caller anywhere, intra-crate or external — an honest empty tree.

## Callee Tree

- **External:** `fmt::Formatter::write_fmt` — via the `write!` macro
