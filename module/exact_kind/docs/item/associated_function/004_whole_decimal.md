# 004: Decimal::whole

## Representation

The whole-unit part, truncated toward zero.

## Kind

Associated Function/Method (§ Item Kind Taxonomy : Associated Item Kinds #1)

## Definition

`module/exact_kind/src/lib.rs:234`

```rust
pub const fn whole( self ) -> Backing
{
  self.minor() / Self::ONE_MINOR
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 234,501 | Declaration; `Qty::whole`'s delegation |
| `tests/parse_render_test.rs:112-116` | — | Truncation-toward-zero checks on `Money` values, both signs |

No file outside `exact_kind` calls `Decimal::whole` — an honest empty finding
across both production and test code in all 10 downstream crates (confirmed
via grep).

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | Backs `Qty::whole`'s delegation; exercised directly by its own truncation tests |

## Caller Tree

- [Qty::whole](015_whole_qty.md) (`src/lib.rs:501`)

No external caller anywhere in the workspace.

## Callee Tree

- None — integer division against the const `ONE_MINOR`, no function call.
