# docs

Design documentation for `exact_ratio`, as typed doc definitions.

| Directory | Responsibility |
|------|-----------------|
| `type/` | `Ratio`'s shape, normalization, and construction |
| `algorithm/` | The widened multiply every `*_mul_ratio` function shares |
| `invariant/` | Why the widened intermediate product can never silently overflow |
| `decisions/` | Why `RatioError` departs from the preferred design's own listing |
| `definition/` | Module Index — every public item in this crate, in one place |
| `workaround/` | External constraints this crate absorbs — none |
| `item/` | One page per declaration, with every file and crate that uses it |

Tier 2 of the family: multiplies and divides the conserved value types
[`exact_kind`](../../exact_kind/readme.md) declares, using the rounding
modes [`exact_round`](../../exact_round/readme.md) declares. Its sibling
tier-2 crate, [`exact_snap`](../../exact_snap/readme.md), shares
`exact_round::round_div` rather than duplicating it here.

This crate owns part of [Hard Problem 12](../../../docs/hard_problem/012_hot_path_performance.md)
(hot-path performance) — every function here is a straight-line widen/divide/
narrow with no loop or heap type, but no benchmark harness exists anywhere in
this family to measure that claim against `f64`; see the hard-problem doc for
the full disclosure.
