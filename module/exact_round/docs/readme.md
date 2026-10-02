# docs

Design documentation for `exact_round`, as typed doc definitions.

| Directory | Responsibility |
|------|-----------------|
| `type/` | `Rounding`'s three variants and its stable-name accessor |
| `invariant/` | No float ever appears, and the result never diverges from the true quotient by more than one unit |
| `algorithm/` | The sign-normalized, mode-driven division `round_div` performs |
| `decisions/` | Why `HalfEven` departs from the preferred design's planned default, and why `round_div` lives here |
| `definition/` | Module Index — every public item in this crate, in one place |
| `workaround/` | External constraints this crate absorbs — none |

Tier 0 of the family — a dependency-free root alongside
[`exact_minor`](../../exact_minor/readme.md) and
[`exact_scale`](../../exact_scale/readme.md) — consumed by
[`exact_ratio`](../../exact_ratio/readme.md) and
[`exact_snap`](../../exact_snap/readme.md) for both `Rounding` and
`round_div`. Net-new: the family's prior shape never offered more than one
implicit rounding behaviour, so nothing here ports old code — every item is
written fresh against the current source.
