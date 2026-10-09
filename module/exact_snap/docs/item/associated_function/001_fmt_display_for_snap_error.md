# 001: Display::fmt for SnapError

## Representation

Writes one of the 3 pre-written sentences for `SnapError`.

## Kind

Associated Function/Method (§ Item Kind Taxonomy : Associated Item Kinds #1)

## Definition

`module/exact_snap/src/lib.rs:43-52`

```rust
fn fmt( &self, f : &mut core::fmt::Formatter< '_ > ) -> core::fmt::Result
{
  match self
  {
    Self::ZeroTick => write!( f, "a zero-sized tick was supplied" ),
    Self::ZeroLot => write!( f, "a zero-sized lot was supplied" ),
    Self::Overflow => write!( f, "the snapped result left the representable or declared range" ),
    Self::OffGrid => write!( f, "the value is not on the grid and Rounding::Exact was requested" ),
  }
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 43-52 | Declaration |

No file anywhere formats a `SnapError` value (`{}`, `to_string()`, etc.) —
verified against `tests/snap_test.rs`, which checks variants via `assert_eq!`
equality, never by rendering them. An honest empty finding.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_snap` | `(defining crate)` | The `Display` method; never actually invoked |

## Caller Tree

No caller anywhere, intra-crate or external — an honest empty tree. Called
only by the `core::fmt` machinery if a `SnapError` were ever formatted, which
no code in this workspace currently does.

## Callee Tree

- **External:** `core::fmt::Formatter::write_str` (via the `write!` macro expansion, 3 arms)
