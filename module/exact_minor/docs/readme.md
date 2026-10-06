# docs

Design documentation for `exact_minor`, as typed doc definitions.

| Directory | Responsibility |
|------|-----------------|
| `invariant/` | No float anywhere, and every checked operation total (saturating operations clamp instead) |
| `definition/` | Module Index — every definition in this crate, in one place |
| `workaround/` | External constraints this crate absorbs — none |
| `item/` | One page per declaration, with every file and crate that uses it |

This is Tier 0 of the family, alongside `exact_scale`: the backing integer
width (`Backing = i64`) is declared exactly once, here, and every other crate
in the family re-exports or layers over it rather than restating it. It holds
the same root role `exact_decimal` held before this family was split along
its preferred fifteen-crate boundaries — see
[`exact_kind`'s own `docs/`](../../exact_kind/docs/readme.md) for where the
conserved-value type built on top of this crate now lives.
