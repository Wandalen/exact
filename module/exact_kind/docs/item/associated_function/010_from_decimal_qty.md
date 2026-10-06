# 010: Qty::from_decimal

## Representation

Wrap a decimal, refusing a negative one. The one place the non-negativity
invariant is actually enforced — every other `Qty` constructor routes through
this.

## Kind

Associated Function/Method (§ Item Kind Taxonomy : Associated Item Kinds #1)

## Definition

`module/exact_kind/src/lib.rs:441`

```rust
pub const fn from_decimal( value : Decimal< SCALE > ) -> Result< Self, KindError >
{
  if value.minor() < 0
  {
    return Err( KindError::Negative { minor : value.minor() } );
  }
  Ok( Self { value } )
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 441,460,474,556 | Declaration; `from_minor`, `from_int`, `parse`'s delegation |
| `tests/non_negative_test.rs` | throughout | Direct negativity-refusal checks |

No file outside `exact_kind` calls `Qty::from_decimal` directly — an honest
gap. Every downstream crate reaches a `Quantity` through
[from_minor](011_from_minor_qty.md) or [parse](019_parse_qty.md) instead.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | The sole enforcement point of `Qty`'s non-negativity invariant; exercised directly by its own tests |

## Caller Tree

- [Qty::from_minor](011_from_minor_qty.md) (`src/lib.rs:460`)
- [Qty::from_int](012_from_int_qty.md) (`src/lib.rs:474`)
- [Qty::checked_add](016_checked_add_qty.md) (`src/lib.rs:515`)
- [Qty::checked_sub](017_checked_sub_qty.md) (`src/lib.rs:529`)
- [Qty::checked_mul_int](018_checked_mul_int_qty.md) (`src/lib.rs:544`)
- [Qty::parse](019_parse_qty.md) (`src/lib.rs:556`)

No external caller anywhere in the workspace — every public `Qty` entry
point funnels through this, making it (alongside
[Decimal::from_minor](001_from_minor_decimal.md)) one of the two true choke
points of the crate.

## Callee Tree

- [Decimal::minor](003_minor_decimal.md) (`src/lib.rs:443`, the negativity check)
