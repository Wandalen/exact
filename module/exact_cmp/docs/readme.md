# docs

Design documentation for `exact_cmp`, as typed doc definitions.

| Directory | Responsibility |
|------|-----------------|
| `invariant/` | Totality and consistency of the ordering every comparison function provides |
| `decisions/` | Why comparison here is infallible rather than guarded by a scale check |
| `definition/` | Module Index — every public item in this crate, in one place |
| `workaround/` | External constraints this crate absorbs — none |
| `item/` | One page per declaration, with every file and crate that uses it |

Tier 2 of the family: every function dispatches directly to the `Ord`/
`PartialEq` [`exact_kind`](../../exact_kind/readme.md) already derives —
this crate adds no comparison logic of its own, only the preferred design's
free-function names.
