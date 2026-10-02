# 004: const _ : () = assert!( exact_scale::MONEY_SCALE <= u8::MAX as u32 )

## Representation

A cross-crate compile-time guard: `Wire` stores `scale` as a single `u8`
(`src/lib.rs:90`), so this assertion fails the build the moment
`exact_scale::MONEY_SCALE` would no longer fit in that byte, rather than
letting `to_bytes`/`from_bytes` silently truncate or misinterpret it. An
anonymous (`_`-named) compile-time assertion, the same pattern
`exact_scale/docs/item/constant/005_range_budget_assertion.md` and
`exact_parse`'s own cross-crate consistency assertion already use.

## Kind

Constant (§ Item Kind Taxonomy : Stable Item Kinds #9)

## Definition

`module/exact_bytes/src/lib.rs:35`

```rust
const _ : () = assert!( exact_scale::MONEY_SCALE <= u8::MAX as u32 );
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 35 | Declaration — evaluated once, at compile time; has no runtime call sites by construction |

Not referenceable from any other file — anonymous (`_`) and private, existing
solely for its compile-time side effect.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_bytes` | `(defining crate)` | The compile-time proof that `exact_scale::MONEY_SCALE` fits the one-byte `scale` field every `Wire` carries |
