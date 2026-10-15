# 001: Kind

## Representation

One kind of amount: money, a quantity or a price. The bound on `Amount`'s
`K` wherever an `impl` needs to know the kind — its `Debug` and its `+` and
`-`. `Kind` has no items of its own; everything it carries comes from its
supertraits: `Copy`, `Ord`, `Hash` and `Debug`, and `sealed::Sealed`, which
lives in a private module
(→ [ADR-006](../../decisions/006_one_amount_struct_generic_over_kind.md)).

The four standard supertraits are there for `Amount`'s derives: a derive on
`Amount< K, SCALE >` requires `K` to have the trait it derives, so code
generic over `K : Kind` — `items.iter().copied().max()`, a `HashSet` of
amounts — compiles only because `Kind` promises them. Every kind derives all
four. The private `Sealed` carries the rest:

| Private `Sealed` item | What it is for |
|---|---|
| `const NAME : &'static str` | The name `Debug` prints: `"Decimal"`, `"Qty"` or `"PriceAt"` |
| `fn checked_add< const SCALE : u32 >( a, b )` | The kind's own inherent `checked_add`, handed to the generic `+` |
| `fn checked_sub< const SCALE : u32 >( a, b )` | The kind's own inherent `checked_sub`, handed to the generic `-` |

`Sealed : Sized` because the hooks name `Amount< Self, SCALE >`, and a type
parameter — `Amount`'s `K` — is `Sized` unless it says otherwise.

Sealed, so the set of kinds is closed. Another crate cannot name `Sealed`,
so it can neither implement `Kind` for a type of its own — a fourth kind
whose `checked_add` skips the ceiling or the non-negativity check — nor call
`MoneyKind::checked_add` as a second public way to add. Two `compile_fail`
doctests pin both.

The three implementations, each empty — the whole impl is its one line, so
they have no pages of their own:

| Implementation | Line | Kind |
|---|---|---|
| `impl Kind for MoneyKind {}` | `src/lib.rs:193` | [MoneyKind](../struct/002_money_kind.md), behind `Decimal` |
| `impl Kind for QtyKind {}` | `src/lib.rs:194` | [QtyKind](../struct/003_qty_kind.md), behind `Qty` |
| `impl Kind for PriceKind {}` | `src/lib.rs:195` | [PriceKind](../struct/004_price_kind.md), behind `PriceAt` |

## Kind

Trait (§ Item Kind Taxonomy : Stable Item Kinds #10)

## Definition

`module/exact_kind/src/lib.rs:179`

```rust
pub trait Kind : sealed::Sealed + Copy + Ord + core::hash::Hash + fmt::Debug {}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 158-179 | Declaration |
| `src/lib.rs` | 165-170 | `compile_fail` doctest: a type outside this crate cannot implement `Kind` |
| `src/lib.rs` | 175-178 | `compile_fail` doctest: `MoneyKind::checked_add` cannot be called outside this crate |
| `src/lib.rs` | 193-195 | The three implementations |
| `tests/amount_test.rs` | 6, 65-85 | Code generic over `K : Kind` copies, orders, hashes and prints amounts of any kind |
| `src/lib.rs` | 231 | The bound on `Debug` for `Amount` |
| `src/lib.rs` | 1037,1048,1059,1070 | The bound on each of the four `+`/`-` impls |
| `exact_arith/src/lib.rs:88` | — | Facade re-export |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | The bound on `Amount`'s `Debug` and operators |
| `exact_arith` | `src/lib.rs` | Re-export |
