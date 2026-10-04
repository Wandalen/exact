# 002: Report

## Representation

The outcome of auditing a log: how many postings were folded, and their
signed net total in minor units. Carried forward from `exact_audit`
unchanged.

## Kind

Struct (§ Item Kind Taxonomy : Stable Item Kinds #6)

## Definition

`module/exact_conserve/src/lib.rs:140-147`

```rust
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
pub struct Report
{
  pub entries : usize,
  pub net_minor : i128,
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 141-147 | Declaration |
| `src/lib.rs` | 214 | Constructed by `verify` as its return value |
| `tests/conservation_test.rs` | throughout | Every test inspects a `Report` returned by `verify` |
| `exact_arith/src/lib.rs:129` | — | Facade re-export |

No production call site anywhere constructs a `Report` directly — only
`verify` does (its sole constructor). `exchange_core` and `cluster_economy`
both consume `verify`'s return value but never build a `Report` by hand.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_conserve` | `(defining crate)` | `verify`'s return type; exercised throughout its own test suite |
| `exact_arith` | `src/lib.rs` | Re-export only |
