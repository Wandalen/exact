# 003: impl Wire

## Representation

`Wire`'s own inherent impl: one associated constant (`ENCODED_LEN`) and six
methods covering construction, field access, and the two byte-level
conversions.

## Kind

Implementation (§ Item Kind Taxonomy : Stable Item Kinds #12)

## Definition

`module/exact_bytes/src/lib.rs:127-192`

```rust
impl Wire
{
  pub const ENCODED_LEN : usize = 10;

  pub const fn new( minor : i64, scale : u8, kind : u8 ) -> Self { /* ... */ }
  pub const fn minor( self ) -> i64 { /* ... */ }
  pub const fn scale( self ) -> u8 { /* ... */ }
  pub const fn kind( self ) -> u8 { /* ... */ }
  pub fn to_bytes( self ) -> [ u8; Self::ENCODED_LEN ] { /* ... */ }
  pub fn from_bytes( bytes : &[ u8 ] ) -> Result< Self, WireError > { /* ... */ }
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 127-192 | Declaration |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_bytes` | `(defining crate)` | `Wire`'s entire public surface beyond its own field types; see each member's own Item Instance for its individual usage evidence |
| `exact_arith` | `src/lib.rs` | Re-exports `Wire` itself, which carries this impl's associated items along with it — no separate per-method re-export line exists or is needed |
