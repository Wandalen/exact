# docs

Design documentation for `exact_dust`, as typed doc definitions.

| Directory | Responsibility |
|------|-----------------|
| `algorithm/` | The equal-parts split procedure — per-share division, leftover computation, and its three destinations |
| `invariant/` | Why a split's total is always fully accounted for, whichever `DustTo` destination is chosen |
| `decisions/` | Three deviations from the preferred design's own crate listing, and why each is correct for what this crate actually needs |
| `definition/` | Module Index — every definition in this crate, in one place |
| `workaround/` | External constraints this crate absorbs — none |
| `item/` | One page per declaration, with every file and crate that uses it |

This crate splits a conserved value into equal shares; [`exact_conserve`](../../exact_conserve/docs/readme.md) is the sibling that audits whether a set of postings — a split's output among them — actually sums to zero.
