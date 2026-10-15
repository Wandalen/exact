# docs

Design documentation for `exact_conserve`, as typed doc definitions.

| Directory | Responsibility |
|------|-----------------|
| `type/` | The posting record, the audit outcome, and the error shared by both layers |
| `invariant/` | The sum-to-zero guarantee every audit path in this crate enforces exactly, no tolerance |
| `algorithm/` | The conservation fold — plain-log and typed-layer, both exact-zero with no tolerance |
| `decisions/` | Why this crate is no longer zero-dependency, what of the original `exact_audit` contract still holds, and why conservation is checked per asset |
| `definition/` | Module Index — every definition in this crate, in one place |
| `workaround/` | External constraints this crate absorbs — none |
| `item/` | One page per declaration, with every file and crate that uses it |

This crate audits whether a set of postings sums to zero; [`exact_dust`](../../exact_dust/docs/readme.md) is the sibling whose split procedure this audit would catch a conservation failure of, as an aggregate imbalance rather than at its source.
