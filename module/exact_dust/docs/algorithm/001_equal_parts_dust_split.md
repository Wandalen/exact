# Algorithm: Equal-Parts Dust Split

### Scope

- **Purpose**: Define how a conserved value is divided into equal integer shares without ever creating or destroying a unit, so a division is never a hole in the conservation argument a logged split later has to pass.
- **Responsibility**: `split_minor`'s per-share quotient and leftover, and `fill_minor`'s distribution of that leftover across the output slots.
- **In Scope**: Equal-count splitting (`parts: usize`) of `Money`/`Quantity`, the effect of the chosen `Rounding` mode on the per-share quotient, and the three `DustTo` destinations for whatever the chosen mode leaves over.
- **Out of Scope**: `round_div`'s own sign-normalization and tie-breaking rules — that is `exact_round`'s own responsibility (see `../../../exact_round/src/lib.rs`); detecting, after the fact, that a logged split failed to conserve (→ [Conservation Verification Fold](../../../exact_conserve/docs/algorithm/001_conservation_verification_fold.md)); weighted (non-equal-share) splitting, which this crate does not implement (→ [Equal-Count Split Surface](../decisions/002_equal_count_split_surface.md)).

### Algorithm

**Inputs.** A total `total` (a `Money` or `Quantity`, read at its raw minor-unit count), a part count `parts: usize`, a `Rounding` mode, and a `DustTo` destination.

1. **Reject zero parts.** `parts == 0` returns [`DustError::EmptyParts`] immediately — there is no share size for an empty split, and silently returning an empty `Vec` would leave `total` unaccounted at the call site.
2. **Compute the per-share quotient.** `share = round_div(total_minor, parts, mode)` — `exact_round`'s own sign-normalizing, tie-breaking division, the same one `exact_ratio` and `exact_snap` already share. Which way `share` leans relative to the exact rational quotient depends entirely on `mode`: `Down` floors it, `Up` ceils it, `HalfEven` rounds to the nearest representable share and breaks an exact tie toward the even one.
3. **Compute the leftover by subtraction, not by a second rounding.** `allocated = share.checked_mul(parts)`, then `leftover = total_minor.checked_sub(allocated)`. Both steps are checked — an overflow in either returns [`DustError::Overflow`] rather than wrapping. The leftover is derived once, by subtracting the allocated amount from the total, never accumulated from per-share error terms — so there is no rounding of a rounding to hide a defect behind.
   - Under `Down`, `share` never exceeds the exact quotient, so `leftover >= 0`.
   - Under `Up`, `share` never falls short of the exact quotient, so `leftover <= 0` — the per-share claims collectively over-cover `total`, and `leftover` is the (non-positive) correction still owed back.
   - Under `HalfEven`, `leftover` can land on either side of zero, since the rounded share can lean either way relative to the plain truncating quotient.

   The identity that actually matters, and that holds unconditionally regardless of which way `mode` leans: `share * parts + leftover == total_minor`, exactly, because `leftover` is defined as that difference and nothing else — there is no path through this function where the two sides can disagree.
4. **Distribute the leftover per `DustTo`** (`fill_minor`):
   - **`Reject`**: a nonzero `leftover` is refused outright as [`DustError::Remainder`] before any slot is built. A zero leftover falls through to the same output every other destination would produce.
   - **`Sink`**: every one of the `parts` slots gets exactly `share`; `leftover` is never applied to any slot. It stays queryable on its own via [`money_dust_remainder`]/[`qty_dust_remainder`], so it is held back rather than dropped — the caller that asked for `Sink` owns deciding what becomes of it.
   - **`First`**: slot `0` gets `share.checked_add(leftover)`; every other slot gets plain `share`. Because `leftover` can be negative (the `Up` case above), this "add" is sometimes effectively a subtraction — which is exactly why it is done at this raw-minor stage rather than through either kind's own checked arithmetic (→ [Leftover Correction via Raw Minor-Unit Reconstruction](../decisions/003_leftover_via_raw_minor_reconstruction.md)).
5. **Reconstruct the typed output.** Every raw minor count — slot 0's possibly-adjusted value included — goes back through `Money::from_minor`/`Quantity::from_minor`, which is where each kind's own range and (for `Quantity`) non-negativity check actually runs. A `Quantity` split whose slot 0 would need to go negative under `First` is refused here, as [`DustError::Overflow`] — see `qty_dust_split_refuses_a_first_slot_that_would_go_negative_under_up_rounding` in this crate's own tests.

### Property — The Accounted Total Never Moves

For `DustTo::Sink` and a successful `DustTo::Reject`, every output slot sums to `share * parts`, and `leftover` — zero for `Reject`, otherwise queryable via the `_remainder` functions — accounts for the rest; nothing is silently dropped. For `DustTo::First`, the output slots themselves sum to `share * parts + leftover == total_minor` exactly, by step 3's identity. Either way the total `total` represents is fully accounted for after the call, whether every unit of it landed in an output slot or part of it was deliberately held back for the caller to place.

This is a narrower guarantee than a weighted, multi-share split would need: there is only ever one `share` value here — every slot before distribution is identical — so there is no selection step and nothing for an iteration order or a tie-break rule to make nondeterministic. The only slot that can ever receive the leftover is slot `0`, by construction of `DustTo::First`; `Sink` and `Reject` never place it at all.

### Decisions

| File | Relationship |
|------|--------------|
| [001_direct_exact_round_dependency.md](../decisions/001_direct_exact_round_dependency.md) | Why step 2 calls `exact_round::round_div` directly instead of reaching it through `exact_ratio` |
| [002_equal_count_split_surface.md](../decisions/002_equal_count_split_surface.md) | Why `parts` is a plain count and why there is no `price_dust_split` |
| [003_leftover_via_raw_minor_reconstruction.md](../decisions/003_leftover_via_raw_minor_reconstruction.md) | Why step 4's `First` correction happens at the raw-minor stage, and why that is also why `DustError` has no variant dedicated to the negative-slot case |

### Sources

| File | Relationship |
|------|--------------|
| `src/lib.rs:105-116` | `split_minor` — the per-share quotient and the subtraction-derived leftover (steps 2-3) |
| `src/lib.rs:121-141` | `fill_minor` — leftover distribution per `DustTo` (step 4) |
| `src/lib.rs:143-158` | `money_dust_split` — reconstructs the typed output via `Money::from_minor` (step 5) |
| `src/lib.rs:196-203` | `qty_dust_split` — the `Quantity` counterpart, where step 5's refusal is actually reachable |
| `src/lib.rs:65-73` | `DustError` — the three failure modes steps 1, 3, and 4 return |
| `../../../exact_round/src/lib.rs:109-187` | `round_div` — the per-share division this procedure drives directly (step 2) |

### Tests

| File | Relationship |
|------|--------------|
| `tests/dust_split_test.rs` | Even splits with zero remainder; `Down`/`Up`/`HalfEven` leftover under all three `DustTo` destinations; the `Quantity`-only negative-slot refusal under `Up` |
