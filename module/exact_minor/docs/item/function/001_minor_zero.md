# 001: minor_zero

## Representation

Zero, in minor units.

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_minor/src/lib.rs:55`

```rust
pub const fn minor_zero() -> Backing
{
  0
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 55 | Declaration |
| `exact_arith/src/lib.rs:69` | — | Facade re-export only |

Not called anywhere — not in `exact_minor`'s own test suite (neither
`checked_arithmetic_test.rs` nor `saturating_arithmetic_test.rs` references
it), not by `exact_sign` or `exact_kind`, and not exercised by
`exact_arith`'s own tests either.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_minor` | `(defining crate)` | Declared; unexercised by its own tests |
| `exact_arith` | `src/lib.rs` | Re-export only |

## Caller Tree

No caller anywhere, intra-crate or external — an honest empty tree, not an
omission. Both of this crate's own test files were checked directly; neither
calls it.

## Callee Tree

- None — a literal.
