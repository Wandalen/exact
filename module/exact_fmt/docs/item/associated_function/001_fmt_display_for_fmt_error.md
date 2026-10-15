# 001: Display::fmt for FmtError

## Representation

Renders `FmtError::BufFull` as the sentence "the buffer was too small to
hold the rendered text."

## Kind

Associated Function/Method (§ Item Kind Taxonomy : Associated Item Kinds #1)

## Definition

`module/exact_fmt/src/lib.rs:53-59`

```rust
fn fmt( &self, f : &mut core::fmt::Formatter< '_ > ) -> core::fmt::Result
{
  match self
  {
    Self::BufFull => write!( f, "the buffer was too small to hold the rendered text" ),
  }
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 53-59 | Declaration |

No file anywhere formats a `FmtError` value — not this crate's own
`tests/fmt_test.rs` (which asserts `Err( FmtError::BufFull )` by equality,
never by rendered string), not `exact_arith`'s re-export. An honest empty
finding: the impl exists to satisfy `core::error::Error`'s supertrait bound,
not because any caller renders the message today.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_fmt` | `(defining crate)` | Declared only; never invoked by this crate's own tests |

## Caller Tree

No caller anywhere, intra-crate or external — an honest empty tree.

## Callee Tree

- **External:** `core::write!` macro expansion over the given `Formatter`
