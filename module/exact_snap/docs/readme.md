# docs

Design documentation for `exact_snap`, as typed doc definitions.

| Directory | Responsibility |
|------|-----------------|
| `invariant/` | A `Tick`/`Lot` grid spacing is never zero-sized |
| `definition/` | Module Index — every public item in this crate, in one place |
| `workaround/` | External constraints this crate absorbs — none |
| `item/` | One page per declaration, with every file and crate that uses it |

Tier 2 of the family: snaps the conserved value types
[`exact_kind`](../../exact_kind/readme.md) declares onto a grid, using the
same [`exact_round`](../../exact_round/readme.md)`::round_div` its sibling
tier-2 crate, [`exact_ratio`](../../exact_ratio/readme.md), divides with —
owned once in `exact_round` rather than duplicated here.
