# 004: PriceKind

## Representation

The kind of [`PriceAt`](../type_alias/006_price_at.md): a price, which may be negative — a discount. A unit
struct with no fields, used only as `Amount`'s `K` — no value of it is
ever built. `PriceAt< SCALE >` is `Amount< PriceKind, SCALE >`.

It implements [`Kind`](../trait/001_kind.md) (`src/lib.rs:195`) and the
private `Sealed` (`src/lib.rs:999-1004`), whose `NAME` is "PriceAt" — the
name `Debug` prints — and whose `checked_add`/`checked_sub` hooks
(`src/lib.rs:1002-1003`) hand PriceAt's own inherent `checked_add` and
`checked_sub` to the generic `+` and `-`. The `impl Kind` is empty and the
`Sealed` impl is private, so neither has a page of its own.

## Kind

Struct (§ Item Kind Taxonomy : Stable Item Kinds #6)

## Definition

`module/exact_kind/src/lib.rs:191`

```rust
#[ derive( Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash ) ]
pub struct PriceKind;
```

The derives are there for `Amount`'s own: `#[ derive ]` on
`Amount< K, SCALE >` requires `K` to implement each trait it derives.

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 189-191 | Declaration |
| `src/lib.rs` | 160,191,195,892,999 | `Kind`'s doc; `impl Kind`; the `PriceAt` alias; `impl Sealed` |
| `tests/amount_test.rs` | 5,27 | Imported; `PriceAt` spelled out as `Amount< PriceKind, 6 >` |
| `exact_arith/src/lib.rs:88` | — | Facade re-export |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | The kind behind `PriceAt` |
| `exact_arith` | `src/lib.rs` | Re-export |
