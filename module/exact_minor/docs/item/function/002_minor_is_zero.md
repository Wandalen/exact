# 002: minor_is_zero

## Representation

Whether a count of minor units is exactly zero.

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_minor/src/lib.rs:227`

```rust
pub const fn minor_is_zero( m : Minor ) -> bool
{
  m.0 == 0
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 227-230 | Declaration |
| `tests/zero_test.rs` | throughout | True for zero; false for `±1` and both backing extremes |
| `exact_arith/src/lib.rs:75` | — | Facade re-export only |

No production caller — the same finding as
[minor_zero](001_minor_zero.md): not `exact_sign`, `exact_kind`, or
`exact_arith`'s tests. Exercised only by this crate's own `tests/zero_test.rs`.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_minor` | `(defining crate)` | Exercised by its own `tests/zero_test.rs` |
| `exact_arith` | `src/lib.rs` | Re-export only |

## Caller Tree

No caller anywhere, intra-crate or external — an honest empty tree.

## Callee Tree

- None — a pure equality comparison.
