# 009: minor_to_i64

## Representation

The raw `i64` a count of minor units holds — the one way out.

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_minor/src/lib.rs:62`

```rust
pub const fn minor_to_i64( m : Minor ) -> i64
{
  m.0
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 62-65 | Declaration |
| `tests/` | throughout | Round trip in `tests/conversion_test.rs`; `tests/zero_test.rs` |
| `exact_kind/src/lib.rs:229,250,264,298` | — | **Production** — `Decimal::minor`, and the results of `checked_add`/`checked_sub`/`checked_neg` |
| `exact_arith/src/lib.rs:78` | — | Facade re-export |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_minor` | `(defining crate)` | Exercised by its own tests |
| `exact_kind` | `src/lib.rs` | **Production** — the raw integer every `Decimal` hands out |
| `exact_arith` | `src/lib.rs` | Re-export only |

## Caller Tree

- **External:** `exact_kind::Decimal::minor`, `checked_add`, `checked_sub`, `checked_neg` (`exact_kind/src/lib.rs:229,250,264,298`)

## Callee Tree

- None.
