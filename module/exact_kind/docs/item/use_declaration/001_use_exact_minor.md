# 001: use exact_minor::Backing

## Representation

Brings in the backing integer alias from the Tier-0 crate this one depends
on. `Decimal`/`Qty` store their minor-unit count as `Backing` rather than a
bare `i64`, so a width change at `exact_minor` propagates here without an
edit.

## Kind

Use Declaration (§ Item Kind Taxonomy : Stable Item Kinds #3)

## Definition

`module/exact_kind/src/lib.rs:52`

```rust
use exact_minor::Backing;
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 52,90,119,153,155,179,184,186,200,202,212,219,231,233,246,248,256,263,265,273,281,283,336,340,347,352,397,418,420,427,433,447,462,469,476 | `Backing` used as the field type of `Decimal::minor`, every constructor/operation's parameter and return path, and `Qty`'s forwarding methods |

No external file references this `use` declaration directly — it is private
to this module; other crates reach `Backing` by depending on `exact_minor`
themselves, not through `exact_kind`.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | Names the backing integer type throughout `Decimal`/`Qty`'s representation and API |
