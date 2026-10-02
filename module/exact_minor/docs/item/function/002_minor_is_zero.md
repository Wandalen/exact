# 002: minor_is_zero

## Representation

Whether a count of minor units is exactly zero.

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_minor/src/lib.rs:62`

```rust
pub const fn minor_is_zero( m : Backing ) -> bool
{
  m == 0
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 62 | Declaration |
| `exact_arith/src/lib.rs:69` | — | Facade re-export only |

Not called anywhere — the same honest-empty finding as
[minor_zero](001_minor_zero.md): not in either of this crate's own test
files, not by `exact_sign` or `exact_kind`, not by `exact_arith`'s tests.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_minor` | `(defining crate)` | Declared; unexercised by its own tests |
| `exact_arith` | `src/lib.rs` | Re-export only |

## Caller Tree

No caller anywhere, intra-crate or external — an honest empty tree.

## Callee Tree

- None — a pure equality comparison.
