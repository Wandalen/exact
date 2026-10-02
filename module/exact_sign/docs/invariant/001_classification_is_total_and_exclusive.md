# Invariant: Classification Is Total And Mutually Exclusive

### Scope

- **Purpose**: State that every representable backing value classifies into exactly one sign, so every crate that gates behavior on sign (non-negativity enforcement, saturation clamp direction) can rely on the classification never being ambiguous or absent.
- **Responsibility**: `sign_of`, `is_negative`, `is_zero`, `sign_neg_allowed`.
- **In Scope**: The three-way `Sign` classification over the full `Backing` range.
- **Out of Scope**: Where non-negativity is actually enforced (→ `exact_kind`'s own construction-time check, which calls `sign_neg_allowed` rather than re-deriving it); which direction a saturating add clamps toward (→ [`exact_add`'s own `invariant/`](../../../exact_add/docs/invariant/readme.md), which consumes `is_negative` rather than re-deriving sign logic).

### Statement

`sign_of` classifies every `Backing` (`i64`) value into exactly one of
`Sign::Neg`, `Sign::Zero`, or `Sign::Pos` — the three match arms are
`value < 0`, `value == 0`, and the implicit remaining case, which together
cover `i64`'s entire range with no gap and no overlap. `is_negative`/
`is_zero` are each a direct `matches!` projection of that same classification,
so they can never disagree with `sign_of` or with each other about a given
value. `sign_neg_allowed` is a total function of its two inputs — it always
returns, for every `(bool, Backing)` pair, never panics, and never needs a
default case.

### Rationale

Two guarantees depend on this classification never being wrong or
incomplete. `sign_neg_allowed` decides whether a kind may hold a given value
— no kind calls it yet (`exact_kind` checks `Qty` directly at construction),
but a classification that missed a value (treated it as neither negative nor
non-negative) would leave that check with nothing to decide against. `exact_add`'s saturating
clamp reads `is_negative` to pick which boundary a failed checked add clamps
toward (see its own invariant doc) — a classification that could disagree
with itself between two call sites would make that clamp direction
unreliable in exactly the cases it exists to get right. Keeping the
three-way split exhaustive and keeping every derived predicate a direct
projection of it, rather than a second independent comparison, is what makes
both of those downstream guarantees able to cite this crate instead of
re-deriving sign logic themselves.

### Sources

| File | Relationship |
|------|--------------|
| `src/lib.rs:28-44` | `sign_of` — the exhaustive three-way classification |
| `src/lib.rs:46-51` | `is_negative` — a direct projection of `sign_of` |
| `src/lib.rs:53-58` | `is_zero` — the same, for the zero case |
| `src/lib.rs:60-72` | `sign_neg_allowed` — the policy gate built on top, called once per kind at construction |

### Tests

| File | Relationship |
|------|--------------|
| `tests/sign_classification_test.rs` | Classification coverage across the backing range, including both signed extremes and zero |
