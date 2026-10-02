# Invariant: Ceiling Stays Within Headroom

### Scope

- **Purpose**: State that the declared ceiling can never silently drift to within overflow range of the backing width, so every crate built on top of these constants can treat accumulation up to the headroom factor as safe without re-deriving the margin itself.
- **Responsibility**: The compile-time relationship between `CEILING_MINOR_UNITS`, `HEADROOM_FACTOR`, and `i64::MAX`.
- **In Scope**: The constants this crate declares and the assertion linking them.
- **Out of Scope**: Which backing width or per-kind scale a conserved value actually uses (→ [`exact_kind`'s own `type/001_conserved_value_type_family.md`](../../../exact_kind/docs/type/001_conserved_value_type_family.md)); `pow10`'s own panic boundary past `n = 18` (→ `../non_functional_requirement/001_representable_range_and_headroom.md`'s "`pow10`'s Own Boundary" section, a related but distinct guarantee).

### Statement

`CEILING_MINOR_UNITS` (`9_000_000_000_000_000`, the declared ceiling at
`MONEY_SCALE = 6`) is guaranteed to stay at or below `i64::MAX / HEADROOM_FACTOR`
(`HEADROOM_FACTOR = 1000`). This is not merely documented — it is a
compile-time assertion:

```rust
const _ : () = assert!( CEILING_MINOR_UNITS <= i64::MAX / HEADROOM_FACTOR );
```

A build fails outright if any future edit to the ceiling, the headroom
factor, or the scale changes this relationship. A consumer of these
constants never needs to re-derive or re-check the margin itself — if the
crate compiled, the margin holds.

### Rationale

Every crate that accumulates ceiling-sized values — most directly
`exact_conserve`'s own `i128`-widened fold, which sums postings that may
individually approach the ceiling — relies on there being room between "one
ceiling-sized value" and "the backing width's own limit" for an accumulation
to proceed safely before any widening happens. A silent erosion of that
margin (an unexamined ceiling bump, say) would not fail loudly at the call
site that actually accumulates values; it would fail unpredictably, under
whichever specific input first exhausted the now-thinner margin. Enforcing
the relationship as a compile-time assertion here, once, moves that failure
to the earliest possible point — a build failure in this crate — rather than
a runtime surprise in a crate three tiers away that merely assumed the
margin was still there.

### Sources

| File | Relationship |
|------|--------------|
| `src/lib.rs:22` | `HEADROOM_FACTOR = 1000` |
| `src/lib.rs:29` | `CEILING_WHOLE_UNITS = 9_000_000_000` |
| `src/lib.rs:37` | `MONEY_SCALE = 6` |
| `src/lib.rs:45` | `CEILING_MINOR_UNITS`, derived from the whole-unit ceiling at `MONEY_SCALE` |
| `src/lib.rs:49` | The compile-time assertion enforcing the relationship |
| `../non_functional_requirement/001_representable_range_and_headroom.md` | The full numeric budget, including why 1000× specifically and `pow10`'s own separate boundary |

### Tests

| File | Relationship |
|------|--------------|
| `tests/scale_factor_test.rs` | Re-derives the ceiling relation in a runtime test, as a second confirmation alongside the compile-time assertion itself |
