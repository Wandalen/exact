# docs

Design documentation for `exact_add`, as typed doc definitions.

| Directory | Responsibility |
|------|-----------------|
| `invariant/` | Why a saturating add's clamp direction is always the mathematically correct one |
| `decisions/` | Why this crate reuses `exact_kind::KindError` directly, and why no panicking variant exists yet |
| `definition/` | Module Index — every definition in this crate, in one place |
| `workaround/` | External constraints this crate absorbs — none |

This is Tier 2 of the family, depending on `exact_kind` for the conserved
value types and `exact_sign` for classifying which direction a saturating
operation clamps toward (`money_saturating_add` clamps to `Money::MIN` when
the failed sum's sign, read via `exact_sign::is_negative`, is negative, and to
`Money::MAX` otherwise — see the function's own doc comment for the full
correctness argument, which this crate's `docs/` does not restate). Every
function here is a thin dispatch over `exact_kind`'s own methods; see
[`exact_kind`'s own `docs/`](../../exact_kind/docs/readme.md) for the
arithmetic this crate dispatches to.

This crate owns part of [Hard Problem 12](../../../docs/hard_problem/012_hot_path_performance.md)
(hot-path performance) — every function here is a `const fn` thin dispatch
with no loop or heap type, but no benchmark harness exists anywhere in this
family to measure that claim against `f64`; see the hard-problem doc for the
full disclosure.
