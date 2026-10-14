# Invariant: Every Scale Is At Most 18

### Scope

- **Purpose**: State that no `Decimal`, `Qty` or `PriceAt` can exist at a scale whose whole unit does not fit the backing integer, so every scale a program holds is one `pow10` can compute.
- **Responsibility**: The private `REPRESENTABLE` check on `Decimal< SCALE >`, and every constructor that evaluates it.
- **In Scope**: `from_minor`, the `ZERO`/`EPSILON`/`MAX`/`MIN` constants, and every path built on them — `rescale`, `Qty`/`PriceAt`, the wire decoders.
- **Out of Scope**: Why the limit is 18 (→ `exact_scale`'s [representable range](../../../exact_scale/docs/non_functional_requirement/001_representable_range_and_headroom.md)); the ceiling a value must stay under at a valid scale (→ [Checked Operations Total](002_checked_operations_total.md)).

### Statement

One whole unit at scale `SCALE` is `10^SCALE` minor units, and an `i64` holds
`10^18` but not `10^19`. So scales 0 to 18 exist, and 19 and above do not.

`exact_scale::pow10` already refuses a power above 18. `Decimal` asks it about
its own scale on every construction:

```rust
const REPRESENTABLE : () = assert!( Self::ONE_MINOR > 0 );   // ONE_MINOR = pow10( SCALE )
```

`from_minor` evaluates it on its first line, and each of `ZERO`, `EPSILON`,
`MAX` and `MIN` evaluates it before building its value. Because it is a
`const`, the check runs while compiling: `Decimal::< 20 >::from_minor( 1 )`,
`Decimal::< 19 >::MAX` and `one.rescale::< 20 >( … )` do not build. At a valid
scale it costs nothing at run time.

Every other way to make a value goes through these: `Qty` and `PriceAt` wrap a
`Decimal` built by `from_minor`, `rescale` ends with
`Decimal::< TO >::from_minor`, and the `*_from_wire` decoders call
`from_minor`. `parse`, `from_int` and `Display` already used `ONE_MINOR`.

### Rationale

Before the check, only the paths that happened to use `ONE_MINOR` refused a
scale past 18. `rescale` did not: `Decimal::< 0 >::from_minor( 1 )?.rescale::< 19 >`
panicked at run time inside `pow10( 19 )`, and
`Decimal::< 6 >::from_minor( 1 )?.rescale::< 20 >` asked `pow10` only about the
difference, `14`, and returned a `Decimal< 20 >` that `money_to_wire` would
write with scale byte `20` (review of #6, P6-2). A scale is fixed in the code,
so a mistake in it is the programmer's, visible before anything runs — a build
error, not a run-time `KindError`, and no new error variant. The limit itself
stays in one place, `pow10`.

### Sources

| File | Relationship |
|------|--------------|
| `src/lib.rs:278-281` | `REPRESENTABLE` — the check |
| `src/lib.rs:283-297` | `ZERO`, `EPSILON`, `MAX`, `MIN` — each evaluates it |
| `src/lib.rs:304-307` | `from_minor` — evaluates it first |
| `../../../exact_scale/src/lib.rs:64-67` | `pow10` — the limit itself |

### Tests

| File | Relationship |
|------|--------------|
| `src/lib.rs` (`rescale`'s doc) | A `compile_fail` example: `rescale::< 20 >` does not build |
