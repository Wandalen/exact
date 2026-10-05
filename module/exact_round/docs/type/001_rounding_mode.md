# Type: Rounding Mode

### Scope

- **Purpose**: Define what each `Rounding` variant means and what `rounding_name` guarantees, so a caller chooses a mode by its denotation rather than by reading `round_div`'s implementation.
- **Responsibility**: The `Rounding` enum and its stable-name accessor, `rounding_name`.
- **In Scope**: The three variants' denotational meaning, and the shape (`Copy`, comparable) that makes `Rounding` a plain policy value.
- **Out of Scope**: Which variant applies when a caller chooses none (→ [Half-Even As The Default](../decisions/001_half_even_as_the_unbiased_default.md)); the division arithmetic that applies a chosen mode (→ [Rounding Division](../algorithm/001_rounding_division.md)).

### Definition

`Rounding` is a three-variant enum — `Down`, `Up`, `HalfEven` — naming how a
value that falls between two representable grid points is placed onto one of
them. `Down` rounds toward negative infinity (the floor); `Up` rounds toward
positive infinity (the ceiling); `HalfEven` rounds to the nearest grid point,
and on an exact tie rounds to whichever neighbour has an even last digit.

`Rounding` derives `Debug, Clone, Copy, PartialEq, Eq, Hash` — a plain,
comparable, copyable policy value, not a resource, the same shape
[`exact_sign`'s `Sign`](../../../exact_sign/docs/type/001_sign_classification.md)
takes for the same reason.

`rounding_name` maps each variant to a stable, lowercase, snake_case string
(`"down"`, `"up"`, `"half_even"`) for diagnostics and logs. It is one-way by
design — there is no `rounding_from_name`, because nothing in this family
parses a rounding mode back out of a log line.

### Sources

| File | Relationship |
|------|--------------|
| `src/lib.rs:35-54` | The `Rounding` enum, its doc comment, and its three variants |
| `src/lib.rs:69-82` | `rounding_name` — the stable-name accessor |

### Tests

| File | Relationship |
|------|--------------|
| `tests/rounding_mode_test.rs` | `every_rounding_mode_has_a_stable_name`, `rounding_is_copy_and_comparable` |
