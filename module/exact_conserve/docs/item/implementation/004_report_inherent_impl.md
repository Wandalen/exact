# 004: Report inherent impl

## Representation

The hand-written impl on `Report`, providing its two query methods —
`is_balanced` (a yes/no verdict) and `discrepancy_minor` (the signed amount
behind that verdict). Deliberately does not compute per-account totals; the
module doc comment explains why (`src/lib.rs:18-22`): an account's non-zero
balance is normal, so reporting it alongside the one non-zero that actually
matters would bury the real signal.

## Kind

Implementation (§ Item Kind Taxonomy : Stable Item Kinds #12)

## Definition

`module/exact_conserve/src/lib.rs:149-173`

```rust
impl Report
{
  pub const fn is_balanced( &self ) -> bool
  {
    self.net_minor == 0
  }

  pub const fn discrepancy_minor( &self ) -> i128
  {
    self.net_minor
  }
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 149-173 | Declaration |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_conserve` | `(defining crate)` | Both methods are this crate's own public read surface on `Report` |
