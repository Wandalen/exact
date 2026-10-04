# 020: Display::fmt for KindError

## Representation

Renders each `KindError` variant as a specific, investigable sentence. See
[impl Display for KindError](../implementation/001_display_for_kind_error.md)
for the full body.

## Kind

Associated Function/Method (§ Item Kind Taxonomy : Associated Item Kinds #1)

## Definition

`module/exact_kind/src/lib.rs:120`

```rust
fn fmt( &self, f : &mut fmt::Formatter< '_ > ) -> fmt::Result
{
  match self
  {
    Self::Overflow { operation } => write!( f, "{operation} left the representable range" ),
    Self::ExceedsCeiling { minor } => write!( f, "{minor} minor units exceeds the declared ceiling {CEILING_MINOR_UNITS}" ),
    Self::ExcessPrecision { digits, scale } => write!( f, "{digits} fractional digits into a type of scale {scale}" ),
    Self::Malformed { reason } => write!( f, "malformed decimal: {reason}" ),
    Self::Negative { minor } => write!( f, "{minor} minor units is below zero, which this kind cannot hold" ),
  }
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 120-130 | Declaration |

No file anywhere in the workspace calls this method explicitly or via
`.to_string()`/format interpolation on a `KindError`-typed value — confirmed
by grep across every `.rs` file in `module/`. An honest empty
finding, not an omission: `exact_bytes` and `exact_ratio` both map
`KindError` variants into their own local error types via explicit `match`
reconstruction (see [KindError](../enum/001_kind_error.md)'s Crate Usage),
never by rendering the message.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | Declared here; not exercised by this crate's own test suite either |

## Caller Tree

No caller anywhere, intra-crate or external — an honest empty tree.

## Callee Tree

- **External:** `fmt::Formatter::write_fmt` (×5, one per variant, via the
  `write!` macro)
