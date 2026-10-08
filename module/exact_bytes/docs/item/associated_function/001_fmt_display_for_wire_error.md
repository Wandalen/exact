# 001: Display::fmt for WireError

## Representation

Writes the matched variant's sentence into the formatter.

## Kind

Associated Function/Method (§ Item Kind Taxonomy : Associated Item Kinds #1)

## Definition

`module/exact_bytes/src/lib.rs:77-86`

```rust
fn fmt( &self, f : &mut core::fmt::Formatter< '_ > ) -> core::fmt::Result
{
  match self
  {
    Self::BadKind => write!( f, "the wire's kind byte did not match the kind being decoded into" ),
    Self::BadScale => write!( f, "the wire's scale byte did not match the kind's expected scale" ),
    Self::Truncated => write!( f, "the byte slice was shorter than the wire encoding's fixed length" ),
    Self::Overflow => write!( f, "the decoded value left the representable or declared range" ),
    Self::Negative { minor } => write!( f, "{minor} minor units is below zero, which this kind cannot hold" ),
  }
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 77-86 | Declaration |

No file anywhere formats a `WireError` value (`{}`, `.to_string()`, `println!`,
etc.) — every real use of `WireError` is construction or `assert_eq!`/
`matches!` pattern matching (see [WireError](../enum/001_wire_error.md)'s own
File Usage). An honest empty finding, mirroring the identical
`KindError`/`RatioError` Display-never-rendered finding already recorded in
`exact_kind` and `exact_ratio`.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_bytes` | `(defining crate)` | The `Display` impl's sole method; never invoked |

## Caller Tree

No caller anywhere, intra-crate or external — an honest empty tree.

## Callee Tree

- **External:** `core::write!` macro expansion against the given `Formatter`
