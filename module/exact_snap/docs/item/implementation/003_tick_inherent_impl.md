# 003: Tick inherent impl

## Representation

`Tick`'s own constructor and field accessor: refuse a zero-sized grid at
construction and store a negative one as its magnitude, then expose it back
out as a plain `Price`.

## Kind

Implementation (§ Item Kind Taxonomy : Stable Item Kinds #12)

## Definition

`module/exact_snap/src/lib.rs:76-104`

```rust
impl Tick
{
  pub const fn new( price : Price ) -> Result< Self, SnapError >
  {
    if price.minor() == 0
    {
      return Err( SnapError::ZeroTick );
    }
    // The price range is symmetric about zero, so the magnitude always fits.
    match Price::from_minor( price.minor().abs() )
    {
      Ok( size ) => Ok( Self( size ) ),
      Err( _ ) => Err( SnapError::Overflow ),
    }
  }

  #[ must_use ]
  pub const fn price( self ) -> Price
  {
    self.0
  }
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 76-104 | Declaration |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_snap` | `(defining crate)` | `Tick`'s only constructor and accessor; see [new](../associated_function/002_new_tick.md) and [price](../associated_function/003_price_tick.md) for method-level usage evidence |
