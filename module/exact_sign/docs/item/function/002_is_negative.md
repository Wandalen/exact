# 002: is_negative

## Representation

Whether a backing value is strictly negative.

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_sign/src/lib.rs:48`

```rust
pub const fn is_negative( value : Backing ) -> bool
{
  matches!( sign_of( value ), Sign::Neg )
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 48 | Declaration |
| `tests/sign_classification_test.rs:16-18` | — | Boundary agreement with `sign_of` |
| `exact_add/src/lib.rs:115` | — | `money_saturating_add`'s clamp-direction decision |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_sign` | `(defining crate)` | Exercised by its own boundary test |
| `exact_add` | `src/lib.rs` | **Production** — the crate's one load-bearing external call: decides whether `money_saturating_add` clamps to `Money::MIN` or `Money::MAX` on overflow |

## Caller Tree

- **External:** `exact_add::money_saturating_add` (`exact_add/src/lib.rs:115`)

No intra-crate caller.

## Callee Tree

- [sign_of](001_sign_of.md) (`src/lib.rs:50`)
- **External:** `core::matches!` macro expansion against `Sign::Neg`
