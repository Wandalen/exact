# 001: Display::fmt for DustError

## Representation

Renders `DustError`'s 3 variants as their respective sentences.

## Kind

Associated Function/Method (§ Item Kind Taxonomy : Associated Item Kinds #1)

## Definition

`module/exact_dust/src/lib.rs:90-98`

```rust
fn fmt( &self, f : &mut core::fmt::Formatter< '_ > ) -> core::fmt::Result
{
  match self
  {
    Self::EmptyParts => write!( f, "cannot split into zero parts" ),
    Self::Remainder => write!( f, "the split left a remainder and DustTo::Reject was requested" ),
    Self::Overflow => write!( f, "left the representable or declared range" ),
  }
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 90-98 | Declaration |

No file anywhere formats a `DustError` value — confirmed via a workspace-wide
search for `to_string`/`format!`/`Display` near `DustError`, which returns
nothing beyond the impl declaration itself. An honest empty finding: every
real consumer compares `DustError` by `PartialEq` or discards it via
`.expect(...)` (`smoke_exact_market_split/src/lib.rs:142`), never by
rendering the message — the same pattern already recorded for
`KindError`/`RatioError`/`SnapError`/`FmtError` across this family.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_dust` | `(defining crate)` | Declared only; never invoked anywhere in the workspace |

## Caller Tree

No caller anywhere, intra-crate or external — an honest empty tree.

## Callee Tree

- **External:** `core::write!` macro expansion over the given `Formatter`
