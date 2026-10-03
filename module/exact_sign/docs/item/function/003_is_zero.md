# 003: sign_is_zero

## Representation

Whether a backing value is exactly zero.

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_sign/src/lib.rs:64`

```rust
pub const fn sign_is_zero( value : Backing ) -> bool
{
  matches!( sign_of( value ), Sign::Zero )
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 64 | Declaration |
| `tests/sign_classification_test.rs:19-20` | — | Boundary agreement with `sign_of` |

No file outside `exact_sign` calls `sign_is_zero` — an honest empty finding.
`exact_minor` has its own, independent `minor_is_zero` rather than depending
on this crate for the same question (it is Tier 0, with no edges to
`exact_sign`, a sibling Tier 0 crate).

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_sign` | `(defining crate)` | Exercised by its own boundary test |

## Caller Tree

No caller anywhere, intra-crate or external — an honest empty tree.

## Callee Tree

- [sign_of](001_sign_of.md) (`src/lib.rs:66`)
- **External:** `core::matches!` macro expansion against `Sign::Zero`
