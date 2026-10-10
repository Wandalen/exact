# Invariant: Leftover Is Never Silently Dropped

### Scope

- **Purpose**: State that a split's total is always fully accounted for — in its output shares, in an explicitly queryable remainder, or by outright refusal — so a caller never has to wonder whether a fraction of the original value quietly disappeared.
- **Responsibility**: `split_minor`'s leftover computation and how each `DustTo` destination disposes of it.
- **In Scope**: The relationship between `total`, the output shares, and the leftover, under all three `DustTo` variants.
- **Out of Scope**: Whether a set of split outputs actually sums to zero across a whole ledger — a different, aggregate-level check (→ [`exact_conserve`](../../../exact_conserve/docs/readme.md)); the per-share division and rounding procedure itself (→ `../algorithm/001_equal_parts_dust_split.md`).

### Statement

For every split, `total_minor == share * parts + leftover` holds by
construction (`split_minor`, `src/lib.rs:122-133`: `leftover` is computed as
`total_minor.checked_sub(allocated)`, never estimated). What differs per
`DustTo` is only where `leftover` ends up, never whether it is preserved:

- **`DustTo::First`**: `leftover` is folded into slot 0, so the output
  vector's own shares sum to exactly `total_minor`.
- **`DustTo::Sink`**: the output vector's shares sum to `total_minor - leftover`,
  *not* the full total — but `leftover` is never lost, only left out of the
  output slots. It remains independently queryable via
  `money_dust_remainder`/`qty_dust_remainder`, which recompute the identical
  `split_minor` and return it directly. A caller that calls a `_split`
  function with `DustTo::Sink` and discards the return value without also
  querying the remainder has chosen to ignore it — the crate itself never
  discards it internally.
- **`DustTo::Reject`**: a nonzero `leftover` fails the whole operation
  (`DustError::Remainder`) before any output is produced at all — there is no
  partial result to account for.

In no case does a nonzero fraction of `total_minor` vanish without appearing
in exactly one of: an output share, the queryable remainder, or a refused
operation.

### Rationale

A split is the one place in this family where a single conserved value is
deliberately divided into several — the opposite operation from
`exact_conserve`'s audit, which checks that many values already sum to one.
If this crate's own accounting were inexact, no aggregate audit downstream
could ever recover the lost unit, because the loss would have happened before
any `Entry`/`Report` was ever built from the output. Guaranteeing
`share * parts + leftover == total_minor` unconditionally, before `DustTo` is
even consulted, means the three destinations are a policy choice about
*where* the leftover goes, never a chance for it to go missing. `DustTo::Sink`
in particular reads, at first glance, like data is being discarded; stating
this invariant explicitly is what lets a reader confirm it is being withheld,
not destroyed.

### Sources

| File | Relationship |
|------|--------------|
| `src/lib.rs:122-133` | `split_minor` — `leftover = total_minor - allocated`, computed via `checked_sub`, never estimated |
| `src/lib.rs:135-197` | `slot_minor`, used by `split_with`/`split_into_with` — where `DustTo::First` folds `leftover` into slot 0, and `DustTo::Reject` refuses a nonzero one before producing any output |
| `src/lib.rs:223-235` | `money_dust_remainder` — the same `split_minor` computation, exposed directly so `DustTo::Sink`'s leftover is always independently recoverable |

### Tests

| File | Relationship |
|------|--------------|
| `tests/dust_split_test.rs` | `qty_dust_split_refuses_a_first_slot_that_would_go_negative_under_up_rounding` — the one case where folding the leftover into slot 0 cannot be honored, and the crate refuses rather than producing a wrong total |
