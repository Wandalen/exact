# Algorithm: Weighted-Parts Dust Split

### Scope

- **Purpose**: Define how a money value is divided in proportion to whole-number weights without ever creating or destroying a unit.
- **Responsibility**: `weighted_share`'s per-slot quotient, `money_dust_split_weighted`'s leftover, and `slot_minor`'s distribution of it.
- **In Scope**: Splitting `Money` by a `&[ u32 ]` weight list, the effect of the `Rounding` mode on each share, and the three `DustTo` destinations.
- **Out of Scope**: The equal-count split (→ [Equal-Parts Dust Split](001_equal_parts_dust_split.md)); `round_div_wide`'s own sign and tie rules (→ `../../../exact_round/src/lib.rs`).

### Algorithm

**Inputs.** A total (read at its raw minor-unit count), a weight list `weights: &[u32]`, a `Rounding` mode, and a `DustTo` destination.

1. **Sum the weights**, as `u64`, checked. A sum of `0` — no weights, or only zeros — is `DustError::EmptyParts`: there is nothing to split by.
2. **Each slot's share**: `total × weight / sum`, widened to `i128` so the product cannot overflow, divided through `exact_round::round_div_wide` under `mode`, and narrowed back to `i64` — a share never exceeds the total. Under `Rounding::Exact` a share that is not whole is `DustError::Remainder`.
3. **The leftover**: `total − Σ shares`, by checked subtraction — the cents the rounding of step 2 lost (or, for a mode that rounds away from zero, gained, as a negative leftover).
4. **Distribute** with `slot_minor`, unchanged from the equal split: `First` adds the leftover to slot 0; `Sink` leaves it out of every slot; `Reject` refuses a nonzero leftover with `DustError::Remainder`.
5. **Build** each slot with `Decimal::< S >::from_minor`, which refuses a count past the ceiling as `DustError::Overflow`.

Each share is computed once: built straight into the returned `Vec` in step 2 and summed on the way, so step 4 only rebuilds slot 0 and the returned `Vec` is the only allocation.

**Example.** `$10.01` at weights `[ 70, 20, 10 ]`, `Rounding::Down`, `DustTo::First`:

| Step | Value |
|------|-------|
| 1. sum | `100` |
| 2. shares | `1001 × 70 / 100 = 700.7 → 700`; `200.2 → 200`; `100.1 → 100` |
| 3. leftover | `1001 − 1000 = 1` |
| 4. `First` | `[ 701, 200, 100 ]` |
| 5. result | `[ 7.01, 2.00, 1.00 ]`, summing to `10.01` |

### Property — The Accounted Total Never Moves

As for the equal split: with `First` the parts sum to the total exactly; with `Sink` the parts plus the held-back leftover do; `Reject` returns no parts at all. Equal weights reproduce `money_dust_split` exactly, under every mode.

### Decisions

| File | Relationship |
|------|--------------|
| [004_weighted_split_as_a_named_operation.md](../decisions/004_weighted_split_as_a_named_operation.md) | Why this is a separate function, why `u32` weights, and the edge-case rules |
| [002_equal_count_split_surface.md](../decisions/002_equal_count_split_surface.md) | The equal-count surface, and the rule that a weighted split be its own operation |

### Sources

| File | Relationship |
|------|--------------|
| `src/lib.rs:272-280` | `weighted_share` — step 2 |
| `src/lib.rs:317-339` | `money_dust_split_weighted` — steps 1, 3 and 5 |
| `src/lib.rs:138-153` | `slot_minor` — step 4, shared with the equal split |
| `../../../exact_round/src/lib.rs:171` | `round_div_wide` — the rounded `i128` division of step 2 |

### Tests

| File | Relationship |
|------|--------------|
| `tests/weighted_split_test.rs` | Exact percentages; each `DustTo`; `Exact`; `Reject` on a remainder, not a rounded share; empty and zero weights; a zero weight; signs under `TowardZero`; equal weights against the equal split; the total conserved over 70 035 cases, under `First` and `Sink`; no overflow at `Money::MAX` |
