# 004: pub use exact_sign::{ ... }

## Representation

Re-exports `exact_sign`'s full surface, though no *other* re-exported leaf's
public signature names `Sign` — included anyway per the module doc comment's
disclosed deviation (`src/lib.rs:57-63`): this facade exposes the whole
value substrate through one dependency, not only the slice other leaves
happen to reference.

## Kind

Use Declaration (§ Item Kind Taxonomy : Stable Item Kinds #3)

## Definition

`module/exact_arith/src/lib.rs:85`

```rust
pub use exact_sign::{ Sign, sign_is_negative, sign_is_zero, sign_neg_allowed, sign_of };
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 85 | Declaration |
| `tests/facade_test.rs:10,57` | — | `Sign`, `sign_of` asserted |

No real downstream consumer (outside `module/` itself) imports
anything from this block — confirmed via a full-workspace grep for `Sign`/
`sign_is_negative`/`sign_is_zero`/`sign_neg_allowed`/`sign_of` reached through
`exact_arith`. `sign_is_negative`, `sign_is_zero`, and `sign_neg_allowed` are not even
touched by this crate's own test suite — only `Sign` and `sign_of` are.
`sign_neg_allowed` in particular carries forward the same
doc-comment-vs-reality gap already found in `exact_sign`'s own catalog (its
doc comment claims `exact_kind` calls it; nothing does).

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_arith` | `tests/facade_test.rs` | **Partially exercised** — `Sign`/`sign_of` only, of the 5 re-exported names |
