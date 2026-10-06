# 002: MinorWide

## Representation

A count of minor units at `i128` width, for magnitudes past `i64`. Exists only
with `--features i128`. A `Minor` widens into it with `MinorWide::from`
(always succeeds) and comes back with `Minor::try_from` (refused with
`Overflow`/`Underflow`, operation `"narrow"`, when it does not fit). It has the
same arithmetic as `Minor`, prefixed `minor_wide_` — listed in the
[module index](../../definition/readme.md). Unlike `Minor`'s, its field is
public (`MinorWide( pub i128 )`). Nothing in the family uses it yet; it exists
because [feature 001](../../../../../docs/feature/001_minor_as_i64_with_i128_feature.md)
requires the `i128` width behind a feature flag.

## Kind

Struct (§ Item Kind Taxonomy : Stable Item Kinds #6)

## Definition

`module/exact_minor/src/lib.rs:73`

```rust
pub struct MinorWide( pub i128 );
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | throughout the `i128` section | Conversions and the `minor_wide_*` functions |
| `tests/wide_test.rs` | throughout | Widening, narrowing, zero, checked and saturating arithmetic |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_minor` | `(defining crate)` | Only with `--features i128` |
