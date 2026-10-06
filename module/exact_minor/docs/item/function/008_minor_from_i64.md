# 008: minor_from_i64

## Representation

Wrap a raw `i64` as a count of minor units — the one way in.

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_minor/src/lib.rs:55`

```rust
pub const fn minor_from_i64( v : i64 ) -> Minor
{
  Minor( v )
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 55-58 | Declaration |
| `tests/` | throughout | Every test builds its inputs with it; round trip in `tests/conversion_test.rs` |
| `exact_kind/src/lib.rs:182,189,192,205` | — | **Production** — `Decimal`'s `EPSILON`, `MAX`, `MIN` and `from_minor` build their stored `Minor` |
| `exact_arith/src/lib.rs:74` | — | Facade re-export |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_minor` | `(defining crate)` | Exercised by its own tests |
| `exact_kind` | `src/lib.rs` | **Production** — every `Decimal` is built through it |
| `exact_arith` | `src/lib.rs` | Re-export only |

## Caller Tree

- **External:** `exact_kind::Decimal::from_minor` and `Decimal`'s `EPSILON`/`MAX`/`MIN` (`exact_kind/src/lib.rs:182,189,192,205`)

## Callee Tree

- None.
