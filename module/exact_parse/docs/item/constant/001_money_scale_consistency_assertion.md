# 001: const _ : () = assert!( Money::ONE_MINOR == exact_scale::pow10( exact_scale::MONEY_SCALE ) )

## Representation

A cross-crate consistency guard between `exact_kind` and `exact_scale`:
`Money`'s scale and `exact_scale::MONEY_SCALE` are declared in two different
crates now, connected only by a shared numeric literal at each definition
site. This assertion fails the build the moment they drift apart, rather
than waiting for a parse to silently use the wrong scale (module comment,
`src/lib.rs:40-44`). An anonymous (`_`-named) compile-time assertion, same
pattern as `exact_scale`'s own
[range budget assertion](../../../../exact_scale/docs/item/constant/005_range_budget_assertion.md).

## Kind

Constant (§ Item Kind Taxonomy : Stable Item Kinds #9)

## Definition

`module/exact_parse/src/lib.rs:45`

```rust
const _ : () = assert!( Money::ONE_MINOR == exact_scale::pow10( exact_scale::MONEY_SCALE ) );
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 45 | Declaration — evaluated once, at compile time; has no runtime call sites by construction |

Not referenceable from any other file — it is anonymous (`_`) and private,
existing solely for its compile-time side effect. Grep-confirmed: no other
file in the workspace restates this exact assertion (unlike `exact_scale`'s
own assertion, which `exact_kind`'s test suite independently re-states).

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_parse` | `(defining crate)` | The compile-time proof that `exact_kind::Money`'s scale and `exact_scale::MONEY_SCALE` are mutually consistent |
