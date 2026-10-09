# Type: Rounding Mode

### Scope

- **Purpose**: Define what each `Rounding` variant means and what `rounding_name` guarantees, so a caller chooses a mode by its denotation rather than by reading `round_div`'s implementation.
- **Responsibility**: The `Rounding` enum and its stable-name accessor, `rounding_name`.
- **In Scope**: The eight variants' denotational meaning, and the shape (`Copy`, comparable) that makes `Rounding` a plain policy value.
- **Out of Scope**: Which variant applies when a caller chooses none (→ [Half-Even As The Default](../decisions/001_half_even_as_the_unbiased_default.md)); the division arithmetic that applies a chosen mode (→ [Rounding Division](../algorithm/001_rounding_division.md)).

### Definition

`Rounding` is an eight-variant enum — `Down`, `Up`, `HalfEven`, `TowardZero`,
`AwayFromZero`, `HalfUp`, `HalfDown`, `Exact` — naming how a value that falls
between two representable grid points is placed onto one of them, or, for
`Exact`, that it must not fall between them at all. `Down` rounds toward
negative infinity (the floor); `Up` rounds toward positive infinity (the
ceiling); `TowardZero` truncates toward zero; `AwayFromZero` rounds away from
zero. The three `Half*` variants round to the nearest grid point and differ
only on an exact tie: `HalfEven` takes whichever neighbour has an even last
digit, `HalfUp` the one farther from zero, and `HalfDown` the one nearer to
zero — so `HalfUp` takes `-2.5` to `-3`, down, despite its name. `Exact`
rounds nothing: a division with a remainder is refused as
`RoundError::Inexact`, for an amount that must never be rounded, such as a
settlement value (→ [An Exact Mode That Refuses A Remainder](../decisions/004_exact_refuses_a_remainder.md)).

`Rounding` derives `Debug, Clone, Copy, PartialEq, Eq, Hash` — a plain,
comparable, copyable policy value, not a resource, the same shape
[`exact_sign`'s `Sign`](../../../exact_sign/docs/type/001_sign_classification.md)
takes for the same reason.

`rounding_name` maps each variant to a stable, lowercase, snake_case string
(`"down"`, `"up"`, `"half_even"`, `"toward_zero"`, `"away_from_zero"`,
`"half_up"`, `"half_down"`, `"exact"`) for diagnostics and logs. It is one-way by
design — there is no `rounding_from_name`, because nothing in this family
parses a rounding mode back out of a log line.

### Sources

| File | Relationship |
|------|--------------|
| `src/lib.rs:35-73` | The `Rounding` enum, its doc comment, and its eight variants |
| `src/lib.rs:88-106` | `rounding_name` — the stable-name accessor |

### Tests

| File | Relationship |
|------|--------------|
| `tests/rounding_mode_test.rs` | `every_rounding_mode_has_a_stable_name`, `rounding_is_copy_and_comparable` |
