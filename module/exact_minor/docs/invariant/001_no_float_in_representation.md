# Invariant: No Float In Representation

### Scope

- **Purpose**: State that this crate never represents a count of minor units with a float, so every crate built on top of it inherits an exact backing integer rather than one that may already have lost precision.
- **Responsibility**: The absence of `f32`/`f64` from `Backing` and from every public function's inputs and outputs.
- **In Scope**: `Backing` itself, and every function signature in this crate.
- **Out of Scope**: Float-freedom at the decimal/kind level — construction, parsing, and rendering of `Decimal`/`Qty` (→ [`exact_kind`'s own instance](../../../exact_kind/docs/invariant/001_no_float_in_the_public_constructor_surface.md), which is this crate's own leaf-level claim carried one tier up against a different, larger public surface).

### Statement

No float appears in an input or an output position anywhere in this crate.
`Backing` is `i64`, declared once and re-exported by every other crate in the
family rather than restated. Every function here — `minor_zero`,
`minor_is_zero`, `minor_checked_add`, `minor_checked_sub`, `minor_checked_neg`,
`minor_saturating_add`, `minor_saturating_sub` — takes and returns `Backing`
or a `bool`/`Result` built from it. Nothing here constructs a `Backing` from a
float or renders one through a float intermediate.

### Rationale

This is Tier 0 of the family: the backing integer width is named exactly once,
here, and the float ban holds at the narrowest possible surface before any
other crate has a chance to widen it. A float introduced at this level would
be invisible to every type built on top — `exact_kind`'s `Decimal<SCALE>`
stores exactly one `Backing` field, so a float smuggled in here is a float
smuggled into every conserved value the family can express. Holding the line
at the root means no downstream crate has to re-derive or re-check it for the
backing type itself.

### Sources

| File | Relationship |
|------|--------------|
| `src/lib.rs:41` | `pub type Backing = i64;` — the only integer type ever stored or passed |
| `src/lib.rs:76-86` | `minor_zero`, `minor_is_zero` — the two constructors/predicates with no range-checking of their own |
| `src/lib.rs:93-149` | Every arithmetic function's signature: `Backing` in, `Backing`/`Result<Backing, MinorError>` out |

### Tests

| File | Relationship |
|------|--------------|
| `tests/checked_arithmetic_test.rs` | Exercises every checked function at its range boundary without ever introducing a float |
| `tests/saturating_arithmetic_test.rs` | Exercises every saturating function the same way |
| `tests/zero_test.rs` | Exercises `minor_zero`/`minor_is_zero` — integer zero in, `bool` out |
