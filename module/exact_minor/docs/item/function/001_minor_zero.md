# 001: minor_zero

## Representation

Zero, in minor units.

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_minor/src/lib.rs:220`

```rust
pub const fn minor_zero() -> Minor
{
  Minor( 0 )
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 220-223 | Declaration |
| `tests/zero_test.rs` | throughout | Equality with `0`, and `minor_is_zero` on it |
| `exact_kind/src/lib.rs:179` | — | `Decimal::ZERO` |
| `exact_arith/src/lib.rs:78` | — | Facade re-export |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_minor` | `(defining crate)` | Exercised by its own `tests/zero_test.rs` |
| `exact_kind` | `src/lib.rs` | **Production** — `Decimal::ZERO`'s stored count |
| `exact_arith` | `src/lib.rs` | Re-export only |

## Caller Tree

- **External:** `exact_kind::Decimal::ZERO` (`exact_kind/src/lib.rs:179`)

## Callee Tree

- None — a literal.
