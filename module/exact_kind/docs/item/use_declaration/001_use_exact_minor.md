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

`module/exact_kind/src/lib.rs:43`

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
| `src/lib.rs` | 90,119,276,304,321,333,378,420,492,496,503,686,700,722,741,782,907,918,1094 | `Backing`: every constructor/operation's raw-integer parameter and return, the `KindError` fields, and `fraction_digits`'s input |
| `src/lib.rs` | 260 | `Minor`: the type of `Decimal`'s stored count |
| `src/lib.rs` | 264,268 | `MinorError`: turned into `KindError::Overflow` by `kind_overflow` |
| `src/lib.rs` | 392,406,440 | `minor_checked_add`/`_sub`/`_neg`: `Decimal`'s `checked_add`/`checked_sub`/`checked_neg` |
| `src/lib.rs` | 284,287,294,297,311,335,394,408,442 | `minor_zero`, `minor_from_i64`, `minor_to_i64`: in and out of the stored `Minor` |

No external file references this `use` declaration directly — it is private
to this module; other crates reach these names by depending on `exact_minor`
themselves, not through `exact_kind`.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | `Decimal`'s stored `Minor`, its checked add/sub/neg, and the raw `Backing` integer at the API boundary |
