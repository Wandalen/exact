# 017: Qty::checked_sub

## Representation

Subtract, refusing to go below zero. The one operation where `Qty`'s
behaviour genuinely diverges from `Decimal`'s: `rhs` exceeding `self`
produces `KindError::Negative`, not a successful signed result.

## Kind

Associated Function/Method (§ Item Kind Taxonomy : Associated Item Kinds #1)

## Definition

`module/exact_kind/src/lib.rs:525`

```rust
pub const fn checked_sub( self, rhs : Self ) -> Result< Self, KindError >
{
  match self.value.checked_sub( rhs.value )
  {
    Ok( value ) => Self::from_decimal( value ),
    Err( e ) => Err( e ),
  }
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 525 | Declaration |
| `tests/non_negative_test.rs` | throughout | The primary invariant test — `rhs > self` refusal, exact-zero success |
| `exact_add/src/lib.rs:85` | — | `qty_sub`'s entire body |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | The crate's signature invariant; exercised extensively by its own tests |
| `exact_add` | `src/lib.rs` | **Production** — the entire implementation of `qty_sub` |

## Caller Tree

- **External:** `exact_add::qty_sub` (`exact_add/src/lib.rs:85`)

No intra-crate caller.

## Callee Tree

- [Decimal::checked_sub](006_checked_sub_decimal.md) (`src/lib.rs:527`)
- [Qty::from_decimal](010_from_decimal_qty.md) (`src/lib.rs:529`)
