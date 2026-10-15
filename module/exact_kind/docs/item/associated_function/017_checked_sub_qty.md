# 017: Qty::checked_sub

## Representation

Subtract, refusing to go below zero. The one operation where `Qty`'s
behaviour genuinely diverges from `Decimal`'s: `rhs` exceeding `self`
produces `KindError::Negative`, not a successful signed result.

## Kind

Associated Function/Method (§ Item Kind Taxonomy : Associated Item Kinds #1)

## Definition

`module/exact_kind/src/lib.rs:767`

```rust
pub const fn checked_sub( self, rhs : Self ) -> Result< Self, KindError >
{
  match self.value().checked_sub( rhs.value() )
  {
    Ok( value ) => Self::from_decimal( value ),
    Err( e ) => Err( e ),
  }
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 767,996 | Declaration; `QtyKind`'s hook, which `-` on `Qty` and on a `Result< Qty, KindError >` calls |
| `tests/non_negative_test.rs` | throughout | The primary invariant test — `rhs > self` refusal, exact-zero success |
| `exact_add/src/lib.rs:85` | — | `qty_sub`'s entire body |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | The crate's signature invariant, and the body of the `-` operator on `Qty`; exercised extensively by its own tests |
| `exact_add` | `src/lib.rs` | **Production** — the entire implementation of `qty_sub` |

## Caller Tree

- **External:** `exact_add::qty_sub` (`exact_add/src/lib.rs:85`)

- **Intra-crate:** `QtyKind`'s `checked_sub` hook (`src/lib.rs:996`), through which [`-` on `Amount`](028_sub_for_amount.md) and [`-` on a `Result`](029_sub_for_result.md) reach it for `Qty` — a quantity below zero comes back as `Negative`

## Callee Tree

- [Decimal::checked_sub](006_checked_sub_decimal.md) (`src/lib.rs:769`)
- [Qty::from_decimal](010_from_decimal_qty.md) (`src/lib.rs:771`)
