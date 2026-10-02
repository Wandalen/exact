# 005: const _ : () = assert!( CEILING_MINOR_UNITS <= i64::MAX / HEADROOM_FACTOR )

## Representation

The range budget's item 1, checked where the constant is declared rather
than only in a test that would have to sample its way to the top. An
anonymous (`_`-named) compile-time assertion: if the headroom relation ever
breaks, the crate fails to compile rather than silently shipping an unsafe
ceiling.

## Kind

Constant (§ Item Kind Taxonomy : Stable Item Kinds #9)

## Definition

`module/exact_scale/src/lib.rs:49`

```rust
const _ : () = assert!( CEILING_MINOR_UNITS <= i64::MAX / HEADROOM_FACTOR );
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 49 | Declaration — evaluated once, at compile time; has no runtime call sites by construction |

Not referenceable from any other file — it is anonymous (`_`) and private,
existing solely for its compile-time side effect. `exact_kind`'s own test
suite (`checked_arithmetic_test.rs:180`) re-states the identical assertion
independently rather than referencing this one, since a private anonymous
const cannot be imported.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_scale` | `(defining crate)` | The compile-time proof that `CEILING_MINOR_UNITS` and `HEADROOM_FACTOR` are mutually consistent |
