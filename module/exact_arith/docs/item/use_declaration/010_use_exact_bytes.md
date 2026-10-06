# 010: pub use exact_bytes::{ ... }

## Representation

Re-exports `exact_bytes`'s 3 kind discriminators, the `Wire` type, its error
enum, and all 6 to/from-wire conversion functions.

## Kind

Use Declaration (§ Item Kind Taxonomy : Stable Item Kinds #3)

## Definition

`module/exact_arith/src/lib.rs:120-133`

```rust
pub use exact_bytes::
{
  KIND_MONEY,
  KIND_PRICE,
  KIND_QTY,
  Wire,
  WireError,
  money_from_wire,
  money_to_wire,
  price_from_wire,
  price_to_wire,
  qty_from_wire,
  qty_to_wire,
};
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 120-133 | Declaration |
| `tests/facade_test.rs:9,41-42` | — | `money_to_wire`/`money_from_wire` round-tripped |

**Only 2 of the 11 re-exported names are exercised anywhere through this
facade**: `money_to_wire`/`money_from_wire`. The remaining 9
(`KIND_MONEY`/`KIND_PRICE`/`KIND_QTY`, `Wire`, `WireError`,
`price_from_wire`/`price_to_wire`, `qty_from_wire`/`qty_to_wire`) have no
confirmed caller anywhere outside `exact_bytes`'s own tests — a sharper,
mostly-unused subset within one re-export block, distinct from the
all-or-nothing pattern seen in most other blocks in this facade.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_arith` | `tests/facade_test.rs` | **Partially exercised** — `money_to_wire`/`money_from_wire` only |
