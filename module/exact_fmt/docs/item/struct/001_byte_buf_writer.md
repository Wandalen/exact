# 001: ByteBufWriter

## Representation

A `core::fmt::Write` sink over a caller-supplied `&mut [u8]`, tracking how
many bytes have been written so far. Exists solely to give `fmt_into` an
allocation-free target to render into via the standard `write!` macro.

**Private, not `pub` — cataloged anyway.** This is the one private top-level
Item in the crate. `item_des.rulebook.md`'s only explicit visibility
carve-out (§ Instance Documentation : Caller Tree Content) names private
*functions* specifically as exempt from their own Item Instance; nothing in
the rulebook narrows the Item Entity's own "every Rust Item... defined in
this crate's own source tree" scope, or § Item Kind Taxonomy : Stable Item
Kinds' "every top-level Rust Item," by visibility for any other Kind. Read
together, a private struct is still a top-level Item and still gets its own
instance — only a private *function* collapses to an inline citation. Flagged
here explicitly since it's a judgment call, not a rule stated in so many
words.

## Kind

Struct (§ Item Kind Taxonomy : Stable Item Kinds #6)

## Definition

`module/exact_fmt/src/lib.rs:50-54`

```rust
struct ByteBufWriter< 'a >
{
  buf : &'a mut [ u8 ],
  len : usize,
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 50-54 | Declaration |
| `src/lib.rs` | 56-69 | `impl core::fmt::Write for ByteBufWriter< '_ >` |
| `src/lib.rs` | 84 | Constructed inside `fmt_into` |

Private to `exact_fmt` — cannot appear in any other crate's source by
construction (not exported, no `pub` on the struct or its fields).

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_fmt` | `src/lib.rs` | The sole write target `fmt_into` renders through |
