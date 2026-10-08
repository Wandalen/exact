# 002: Ratio::n

## Representation

The numerator.

## Kind

Associated Function/Method (§ Item Kind Taxonomy : Associated Item Kinds #1)

## Definition

`module/exact_ratio/src/lib.rs:102`

```rust
#[ must_use ]
pub const fn n( self ) -> i64
{
  self.n
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 102 | Declaration |
| `tests/ratio_and_div_round_test.rs:24` | — | Asserts the normalized numerator after a negative-denominator `ratio_new` call |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_ratio` | `(defining crate)` | Exercised by its own test |

## Caller Tree

No caller anywhere in the Defining Crate's production code — an honest empty
tree. `mul_ratio_minor` (the one place `Ratio`'s numerator is read in
production) reads the private field `r.n` directly rather than calling this
accessor, since both are defined in the same module. The one real call site
is in `exact_ratio`'s own tests, out of scope for this tree. No other
workspace crate depends on `exact_ratio` today.

## Callee Tree

- None — a pure field access.
