# 004: Lot inherent impl

## Representation

`Lot`'s own constructor and field accessor — the `Quantity` counterpart to
[Tick's inherent impl](003_tick_inherent_impl.md), same shape: refuse a
zero-sized grid at construction, then expose it back out as a plain
`Quantity`.

## Kind

Implementation (§ Item Kind Taxonomy : Stable Item Kinds #12)

## Definition

`module/exact_snap/src/lib.rs:110-132`

```rust
impl Lot
{
  pub const fn new( qty : Quantity ) -> Result< Self, SnapError >
  {
    if qty.minor() == 0
    {
      return Err( SnapError::ZeroLot );
    }
    Ok( Self( qty ) )
  }

  #[ must_use ]
  pub const fn qty( self ) -> Quantity
  {
    self.0
  }
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 110-132 | Declaration |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_snap` | `(defining crate)` | `Lot`'s only constructor and accessor; see [new](../associated_function/004_new_lot.md) and [qty](../associated_function/005_qty_lot.md) for method-level usage evidence |
