# 002: MoneyKind

## Representation

The kind of [`Decimal`](../type_alias/004_decimal.md): money, which may be negative. A unit
struct with no fields, used only as `Amount`'s `K` — no value of it is
ever built. `Decimal< SCALE >` is `Amount< MoneyKind, SCALE >`.

It implements [`Kind`](../trait/001_kind.md) (`src/lib.rs:193`) and the
private `Sealed` (`src/lib.rs:985-990`), whose `NAME` is "Decimal" — the
name `Debug` prints — and whose `checked_add`/`checked_sub` hooks
(`src/lib.rs:988-989`) hand Decimal's own inherent `checked_add` and
`checked_sub` to the generic `+` and `-`. The `impl Kind` is empty and the
`Sealed` impl is private, so neither has a page of its own.

## Kind

Struct (§ Item Kind Taxonomy : Stable Item Kinds #6)

## Definition

`module/exact_kind/src/lib.rs:183`

```rust
#[ derive( Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash ) ]
pub struct MoneyKind;
```

The derives are there for `Amount`'s own: `#[ derive ]` on
`Amount< K, SCALE >` requires `K` to implement each trait it derives, and
[`Kind`](../trait/001_kind.md) requires `Copy`, `Ord`, `Hash` and `Debug`.

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 181-183 | Declaration |
| `src/lib.rs` | 160,176,177,183,193,260,985 | `Kind`'s doc; `impl Kind`; the `Decimal` alias; `impl Sealed` |
| `tests/amount_test.rs` | 5,25 | Imported; `Decimal` spelled out as `Amount< MoneyKind, 6 >` |
| `exact_arith/src/lib.rs:88` | — | Facade re-export |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | The kind behind `Decimal` |
| `exact_arith` | `src/lib.rs` | Re-export |
