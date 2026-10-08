# 003: Ratio::d

## Representation

The denominator — always strictly positive, by `ratio_new`'s own
normalization invariant.

## Kind

Associated Function/Method (§ Item Kind Taxonomy : Associated Item Kinds #1)

## Definition

`module/exact_ratio/src/lib.rs:109`

```rust
#[ must_use ]
pub const fn d( self ) -> i64
{
  self.d
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 109 | Declaration |
| `tests/ratio_and_div_round_test.rs:25` | — | Asserts the normalized (positive) denominator after a negative-denominator `ratio_new` call |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_ratio` | `(defining crate)` | Exercised by its own test |

## Caller Tree

No caller anywhere in the Defining Crate's production code — an honest empty
tree. `mul_ratio_minor` (the one place `Ratio`'s denominator is read in
production) reads the private field `r.d` directly rather than calling this
accessor, since both are defined in the same module. The one real call site
is in `exact_ratio`'s own tests, out of scope for this tree. No other
workspace crate depends on `exact_ratio` today.

## Callee Tree

- None — a pure field access.
