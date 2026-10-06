# 002: Decimal::from_int

## Representation

Build from a whole number of units, scaling by `ONE_MINOR` before the range
gate.

## Kind

Associated Function/Method (§ Item Kind Taxonomy : Associated Item Kinds #1)

## Definition

`module/exact_kind/src/lib.rs:215`

```rust
pub const fn from_int( whole : Backing ) -> Result< Self, KindError >
{
  let Some( minor ) = whole.checked_mul( Self::ONE_MINOR )
  else
  {
    return Err( KindError::Overflow { operation : "from_int" } );
  };
  Self::from_minor( minor )
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 215,472 | Declaration; `Qty::from_int`'s delegation |
| `tests/checked_arithmetic_test.rs` | throughout | Direct construction, overflow boundary |

No file outside `exact_kind` calls `Decimal::from_int` directly — an honest
gap. Every downstream crate that needs a whole-unit `Money`/`Price`
constructs one via [parse](009_parse_decimal.md) or
[from_minor](001_from_minor_decimal.md) instead.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | Backs `Qty::from_int`; exercised by its own tests |

## Caller Tree

- [Qty::from_int](012_from_int_qty.md) (`src/lib.rs:472`)

No external caller anywhere in the workspace — an honest empty leaf, not an
omission (confirmed via grep across all 10 downstream crates' `src/lib.rs`).

## Callee Tree

- **External:** `i64::checked_mul` — `whole.checked_mul( Self::ONE_MINOR )`
- [Decimal::from_minor](001_from_minor_decimal.md) (`src/lib.rs:222`)
