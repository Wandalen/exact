# 001: Sign

## Representation

The sign of a backing value: `Neg` (strictly less than zero), `Zero`
(exactly zero), `Pos` (strictly greater than zero). Net-new — the family's
prior shape encoded "can this go negative" as a type-level choice (`Decimal`
signed, `Qty` never) rather than a runtime classification, so there is no
real-code precedent to port (module doc comment, `src/lib.rs:6-10`).

## Kind

Enum (§ Item Kind Taxonomy : Stable Item Kinds #7)

## Definition

`module/exact_sign/src/lib.rs:16`

```rust
pub enum Sign
{
  Neg,
  Zero,
  Pos,
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 16,30,34,38,42,50,57 | Return type of `sign_of`; matched in `is_negative`/`is_zero` |
| `tests/sign_classification_test.rs` | throughout | All 3 variants checked at the boundary |
| `exact_arith/src/lib.rs:79` | — | Facade re-export |
| `exact_arith/tests/facade_test.rs:57` | — | `sign_of( -5 )` via the re-exported path |

No production (non-test) file outside `exact_sign` constructs or matches on
`Sign` directly — `exact_add`, the one crate that calls into this crate in
production, uses [`is_negative`](../function/002_is_negative.md) instead,
which hides the enum entirely behind a boolean question.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_sign` | `(defining crate)` | Exercised by its own boundary tests |
| `exact_arith` | `src/lib.rs`, `tests/facade_test.rs` | Re-export; test-only direct use |
