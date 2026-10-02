# 012: Qty::from_int

## Representation

Build from a whole number of units, refusing a negative one.

## Kind

Associated Function/Method (§ Item Kind Taxonomy : Associated Item Kinds #1)

## Definition

`module/exact_kind/src/lib.rs:447`

```rust
pub const fn from_int( whole : Backing ) -> Result< Self, KindError >
{
  match Decimal::from_int( whole )
  {
    Ok( value ) => Self::from_decimal( value ),
    Err( e ) => Err( e ),
  }
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 447,392 | Declaration; struct-doc-comment doctest |
| `tests/non_negative_test.rs` | throughout | The primary constructor this test file uses |

No file outside `exact_kind` calls `Qty::from_int` directly — an honest gap.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | Exercised extensively by its own tests and its own struct-doc doctest |

## Caller Tree

No caller anywhere, intra-crate or external — an honest empty tree. `Qty`'s
downstream consumers all construct through
[from_minor](011_from_minor_qty.md) or [parse](019_parse_qty.md); this
constructor is exercised only by `exact_kind`'s own tests and doctest.

## Callee Tree

- [Decimal::from_int](002_from_int_decimal.md) (`src/lib.rs:449`)
- [Qty::from_decimal](010_from_decimal_qty.md) (`src/lib.rs:451`)
