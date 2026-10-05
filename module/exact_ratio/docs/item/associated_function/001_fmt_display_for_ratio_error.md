# 001: Display::fmt for RatioError

## Representation

Formats the human-readable message for whichever `RatioError` variant `self`
holds.

## Kind

Associated Function/Method (§ Item Kind Taxonomy : Associated Item Kinds #1)

## Definition

`module/exact_ratio/src/lib.rs:67`

```rust
fn fmt( &self, f : &mut core::fmt::Formatter< '_ > ) -> core::fmt::Result
{
  match self
  {
    Self::DivZero => write!( f, "a zero denominator was supplied" ),
    Self::Overflow => write!( f, "left the representable or declared range" ),
    Self::Negative { minor } => write!( f, "{minor} minor units is below zero, which this kind cannot hold" ),
  }
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 67-75 | Declaration |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_ratio` | `(defining crate)` | Declared, never triggered |

## Caller Tree

No caller anywhere in the workspace — an honest empty tree. Verified via
grep for any `Display`-triggering use of a `RatioError` value
(`to_string`/`format!`/`{}`/`println!`/`eprintln!`/`write!`) across every
`.rs` file in `module/`: the only match is this impl's own
declaration. `RatioError` values are matched by variant in tests and call
sites, never rendered. This mirrors `exact_kind::KindError`'s own `Display`,
which is likewise never rendered anywhere in the current workspace.

## Callee Tree

- **External:** `core::write!` macro expansion (×3, one per match arm)
