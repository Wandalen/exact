# docs

Design documentation for `exact_arith`, as typed doc definitions.

| Directory | Responsibility |
|------|-----------------|
| `feature/` | Version scope — what this facade's re-export surface commits to, and which of the family's original exit criteria now hold |
| `invariant/` | The facade's own restraint: it declares nothing of its own |
| `definition/` | Module Index — every one of the 98 publicly re-exported items, in one place |
| `workaround/` | External constraints this crate absorbs — none |

Tier 4 of the family: the single dependency every consumer takes instead of
the other 14 crates individually. This crate's own contract — re-export
everything, add nothing — is covered here; the rationale for any one
re-exported item's own behaviour lives in the leaf crate that actually
declares it, cross-referenced from
[`docs/definition/readme.md`](definition/readme.md) rather than restated.
