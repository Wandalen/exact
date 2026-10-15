# 003: TickAt inherent impl

## Representation

`TickAt`'s own constructor and field accessor, at every scale `S`: refuse a
zero-sized grid at construction and store a negative one as its magnitude,
then expose it back out as a plain `PriceAt< S >`.

## Kind

Implementation (§ Item Kind Taxonomy : Stable Item Kinds #12)

## Definition

`module/exact_snap/src/lib.rs:81-109`

```rust
impl< const S : u32 > TickAt< S >
{
  pub const fn new( price : PriceAt< S > ) -> Result< Self, SnapError >
  {
    if price.minor() == 0
    {
      return Err( SnapError::ZeroTick );
    }
    // The price range is symmetric about zero, so the magnitude always fits.
    match PriceAt::< S >::from_minor( price.minor().abs() )
    {
      Ok( size ) => Ok( Self( size ) ),
      Err( _ ) => Err( SnapError::Overflow ),
    }
  }

  #[ must_use ]
  pub const fn price( self ) -> PriceAt< S >
  {
    self.0
  }
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 81-109 | Declaration |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_snap` | `(defining crate)` | `TickAt`'s only constructor and accessor; see [new](../associated_function/002_new_tick.md) and [price](../associated_function/003_price_tick.md) for method-level usage evidence |
