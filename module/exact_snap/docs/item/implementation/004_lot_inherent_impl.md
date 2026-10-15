# 004: LotAt inherent impl

## Representation

`LotAt`'s own constructor and field accessor, at every scale `S` — the
quantity counterpart to [TickAt's inherent impl](003_tick_inherent_impl.md),
same shape: refuse a zero-sized grid at construction, then expose it back out
as a plain `Qty< S >`.

## Kind

Implementation (§ Item Kind Taxonomy : Stable Item Kinds #12)

## Definition

`module/exact_snap/src/lib.rs:119-141`

```rust
impl< const S : u32 > LotAt< S >
{
  pub const fn new( qty : Qty< S > ) -> Result< Self, SnapError >
  {
    if qty.minor() == 0
    {
      return Err( SnapError::ZeroLot );
    }
    Ok( Self( qty ) )
  }

  #[ must_use ]
  pub const fn qty( self ) -> Qty< S >
  {
    self.0
  }
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 119-141 | Declaration |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_snap` | `(defining crate)` | `LotAt`'s only constructor and accessor; see [new](../associated_function/004_new_lot.md) and [qty](../associated_function/005_qty_lot.md) for method-level usage evidence |
