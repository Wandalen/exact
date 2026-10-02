# 001: minor_zero

## Representation

Zero, in minor units.

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_minor/src/lib.rs:76`

```rust
pub const fn minor_zero() -> Backing
{
  0
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 76 | Declaration |
| `tests/zero_test.rs` | throughout | Equality with `0`, and `minor_is_zero` on it |
| `exact_arith/src/lib.rs:72` | — | Facade re-export only |

No production caller — not `exact_sign`, `exact_kind`, or `exact_arith`'s
own tests. Exercised only by this crate's own `tests/zero_test.rs`.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_minor` | `(defining crate)` | Exercised by its own `tests/zero_test.rs` |
| `exact_arith` | `src/lib.rs` | Re-export only |

## Caller Tree

No caller anywhere, intra-crate or external — an honest empty tree, not an
omission. Only this crate's own `tests/zero_test.rs` calls it.

## Callee Tree

- None — a literal.
