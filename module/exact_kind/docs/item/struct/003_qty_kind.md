# 003: QtyKind

## Representation

The kind of [`Qty`](../type_alias/005_qty.md): a quantity, never below zero. A unit
struct with no fields, used only as `Amount`'s `K` — no value of it is
ever built. `Qty< SCALE >` is `Amount< QtyKind, SCALE >`.

It implements [`Kind`](../trait/001_kind.md) (`src/lib.rs:194`) and the
private `Sealed` (`src/lib.rs:992-997`), whose `NAME` is "Qty" — the
name `Debug` prints — and whose `checked_add`/`checked_sub` hooks
(`src/lib.rs:995-996`) hand Qty's own inherent `checked_add` and
`checked_sub` to the generic `+` and `-`. The `impl Kind` is empty and the
`Sealed` impl is private, so neither has a page of its own.

## Kind

Struct (§ Item Kind Taxonomy : Stable Item Kinds #6)

## Definition

`module/exact_kind/src/lib.rs:187`

```rust
#[ derive( Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash ) ]
pub struct QtyKind;
```

The derives are there for `Amount`'s own: `#[ derive ]` on
`Amount< K, SCALE >` requires `K` to implement each trait it derives, and
[`Kind`](../trait/001_kind.md) requires `Copy`, `Ord`, `Hash` and `Debug`.

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 185-187 | Declaration |
| `src/lib.rs` | 160,187,194,653,992 | `Kind`'s doc; `impl Kind`; the `Qty` alias; `impl Sealed` |
| `tests/amount_test.rs` | 5,26 | Imported; `Qty` spelled out as `Amount< QtyKind, 6 >` |
| `exact_arith/src/lib.rs:88` | — | Facade re-export |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | The kind behind `Qty` |
| `exact_arith` | `src/lib.rs` | Re-export |
