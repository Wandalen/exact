# Invariant: Grid Spacing Is Never Zero

### Scope

- **Purpose**: State that a `Tick` or `Lot` can never carry a zero-sized grid spacing, so every division this crate drives through `exact_round::round_div` is guaranteed a nonzero divisor before it runs.
- **Responsibility**: `Tick::new` and `Lot::new`'s zero refusal.
- **In Scope**: Construction of `Tick` and `Lot`.
- **Out of Scope**: A negative grid spacing, which is accepted — `round_div`'s own divisor normalization already handles it correctly, and neither the preferred design nor this crate names an error for it; the division itself (→ [`exact_round`](../../../exact_round/readme.md)).

### Statement

Neither `Tick::new` nor `Lot::new` can produce a value whose underlying
price or quantity is exactly zero. Both constructors check `.minor() == 0`
and refuse it — `SnapError::ZeroTick` or `SnapError::ZeroLot` — before a
`Tick` or `Lot` value ever exists. There is no second construction path:
both fields are private, so a `Tick` or `Lot` reaching `price_snap_tick` or
`qty_snap_lot` already passed this check.

### Rationale

`price_snap_tick` and `qty_snap_lot` both divide by the grid spacing
(`tick.0.minor()`, `lot.0.minor()`) to find which grid point a value is
nearest to. A zero divisor reaching `exact_round::round_div` would return
`RoundError::DivZero` deep inside the snap, reported as `SnapError::Overflow`
through this crate's defensive-but-unreachable error mapping — a confusing
result for a caller who passed a value `round_div` itself would have refused
outright. Refusing at construction instead means the failure is reported
once, at the moment the grid is defined, in the shape the caller actually
caused: a zero-sized grid, not an overflow.

A negative spacing is deliberately not refused the same way: `round_div`
already normalizes a negative divisor correctly, so there is no analogous
confusing-failure case to guard against, and neither the preferred design
nor this crate's own `SnapError` names a variant for it.

### Sources

| File | Relationship |
|------|--------------|
| `src/lib.rs:62-74` | `Tick::new` — the zero check and refusal |
| `src/lib.rs:90-102` | `Lot::new` — the same check and refusal |
| `src/lib.rs:42-54` | `round_error_to_snap_error` — the comment on why `RoundError::DivZero` is unreachable through this crate's public API once this invariant holds |

### Tests

| File | Relationship |
|------|--------------|
| `tests/snap_test.rs` | `zero_sized_tick_and_lot_are_refused_at_construction` |
