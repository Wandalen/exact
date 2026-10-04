# 004: sign_neg_allowed

## Representation

Whether `value` is admissible under a kind's own negative-value policy. A
policy function rather than a bare per-kind constant, so the decision reads
at the call site as a question about the *value* being checked (doc comment,
`src/lib.rs:71-76`).

**Doc comment vs. actual wiring — a verified discrepancy.** This function's
own doc comment states "`exact_kind` calls this once per kind, at
construction, per the family's decision to enforce non-negativity where a
`Qty` is built." That is not what the shipped code does: `exact_kind/Cargo.toml`
has no dependency on `exact_sign` at all, and `exact_kind/src/lib.rs` contains
no reference to this crate anywhere — its non-negativity refusal is enforced
directly, via `Qty::from_decimal`'s own `value.minor() < 0` check and
`KindError::Negative`, with no call through this function. `exact_kind`'s own
design-rationale doc already discloses this accurately: "`exact_sign`'s
negative-admission policy decision ... not yet wired to this crate's own
enforcement" (`exact_kind/docs/decisions/001_non_negativity_enforced_at_construction.md`,
Related section). This file's Representation states the function's own
documented intent; the File/Crate Usage and Caller Tree below report what is
actually, verifiably true today — zero production callers.

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_sign/src/lib.rs:78`

```rust
pub const fn sign_neg_allowed( neg_allowed : bool, value : Backing ) -> bool
{
  neg_allowed || !sign_is_negative( value )
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 78 | Declaration |
| `tests/sign_classification_test.rs:25-38` | — | Both policy states (allowed/disallowed) at all 3 signs |
| `exact_arith/src/lib.rs:85` | — | Facade re-export |

No production or test file in any of `exact_sign`'s 3 dependents
(`exact_add`, `exact_arith`, and transitively nothing else) calls
`sign_neg_allowed` — confirmed via grep across every `.rs` file in
`module/`. An honest empty finding, and the sharpest one in this
crate: the function exists, is tested, and is re-exported, but is not yet
wired into the one place (`exact_kind`) its own doc comment says it is.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_sign` | `(defining crate)` | Exercised by its own policy-state tests |
| `exact_arith` | `src/lib.rs` | Re-export only |

## Caller Tree

No caller anywhere, intra-crate or external — an honest empty tree, not an
omission. See the discrepancy note above.

## Callee Tree

- [sign_is_negative](002_is_negative.md) (`src/lib.rs:80`)
