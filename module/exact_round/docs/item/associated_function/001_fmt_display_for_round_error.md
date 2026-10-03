# 001: Display::fmt for RoundError

## Representation

Renders each `RoundError` variant's message. See
[impl Display for RoundError](../implementation/001_display_for_round_error.md)
for the full body.

## Kind

Associated Function/Method (§ Item Kind Taxonomy : Associated Item Kinds #1)

## Definition

`module/exact_round/src/lib.rs:95`

```rust
fn fmt( &self, f : &mut core::fmt::Formatter< '_ > ) -> core::fmt::Result
{
  match self
  {
    Self::DivZero => write!( f, "a zero divisor was supplied" ),
    Self::Overflow => write!( f, "normalizing a negative divisor overflowed" ),
  }
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 95-102 | Declaration |

No file anywhere calls this method — an honest empty finding (confirmed via
grep), matching [RoundError](../enum/002_round_error.md)'s own File Usage
note.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_round` | `(defining crate)` | Declared here; never invoked |

## Caller Tree

No caller anywhere, intra-crate or external — an honest empty tree.

## Callee Tree

- **External:** `fmt::Formatter::write_fmt` (×2, one per variant, via the
  `write!` macro)
