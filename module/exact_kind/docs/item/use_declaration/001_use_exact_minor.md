# 001: use exact_minor::{ Backing, Minor, … }

## Representation

Brings in what `Decimal` is built on from the Tier-0 crate this one depends
on: the backing integer alias (`Backing`), the `Minor` count that `Decimal`
stores, `exact_minor`'s checked arithmetic on it, and its error. `Decimal`
adds, subtracts and negates through `exact_minor`'s functions instead of
repeating that logic here; a width change at `exact_minor` propagates here
without an edit.

## Kind

Use Declaration (§ Item Kind Taxonomy : Stable Item Kinds #3)

## Definition

`module/exact_kind/src/lib.rs:42`

```rust
use exact_minor::
{
  Backing,
  Minor,
  MinorError,
  minor_checked_add,
  minor_checked_neg,
  minor_checked_sub,
  minor_from_i64,
  minor_to_i64,
  minor_zero,
};
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 85,114,176,199,215,227,234,276,348,352,359,448,462,484,491,532,650,661 | `Backing`: every constructor/operation's raw-integer parameter and return, and the `KindError` fields |
| `src/lib.rs` | 159 | `Minor`: the type of `Decimal`'s stored count |
| `src/lib.rs` | 164,168 | `MinorError`: turned into `KindError::Overflow` by `kind_overflow` |
| `src/lib.rs` | 248,262,296 | `minor_checked_add`/`_sub`/`_neg`: `Decimal`'s `checked_add`/`checked_sub`/`checked_neg` |
| `src/lib.rs` | 179,182,189,192,205,229,250,264,298 | `minor_zero`, `minor_from_i64`, `minor_to_i64`: in and out of the stored `Minor` |

No external file references this `use` declaration directly — it is private
to this module; other crates reach these names by depending on `exact_minor`
themselves, not through `exact_kind`.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | `Decimal`'s stored `Minor`, its checked add/sub/neg, and the raw `Backing` integer at the API boundary |
