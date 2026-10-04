# 015: Qty::whole

## Representation

The whole-unit part, truncated, delegating to the wrapped decimal.

## Kind

Associated Function/Method (§ Item Kind Taxonomy : Associated Item Kinds #1)

## Definition

`module/exact_kind/src/lib.rs:491`

```rust
pub const fn whole( self ) -> Backing
{
  self.value.whole()
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 491 | Declaration |
| `tests/non_negative_test.rs:49,97,98,135` | — | Truncation checks, including after `checked_add`/`checked_mul_int` and on `Quantity::MAX` |

No file outside `exact_kind` calls `Qty::whole` — an honest empty finding
(contrast [Decimal::whole](004_whole_decimal.md), which `exact_kind`'s own
`parse_render_test.rs` also exercises directly; neither has any external
caller).

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | Exercised by its own truncation tests |

## Caller Tree

No caller anywhere, intra-crate or external — an honest empty tree.

## Callee Tree

- [Decimal::whole](004_whole_decimal.md) (`src/lib.rs:493`)
