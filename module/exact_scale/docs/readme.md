# docs

Design documentation for `exact_scale`, as typed doc definitions.

| Directory | Responsibility |
|------|-----------------|
| `non_functional_requirement/` | The declared ceiling's headroom over the backing width, and `pow10`'s own boundary |
| `invariant/` | The compile-time-enforced margin between the ceiling and the backing width |
| `decisions/` | Why scale stays a compile-time const-generic fact, not a runtime `Scale` type |
| `definition/` | Module Index — every definition in this crate, in one place |
| `workaround/` | External constraints this crate absorbs — none |

This is Tier 0 of the family, alongside `exact_minor` — a sibling root with no
edge to it, by design: scale and the backing width are two independent
concerns until `exact_kind` depends on both and brings them together under
one type. Scale here stays a compile-time fact carried in a type's own const
generic parameter, not a runtime value, so this crate stays the constants and
the power-of-ten table behind that mechanism rather than a runtime `Scale`
type — see
[`exact_kind`'s own `docs/`](../../exact_kind/docs/readme.md) for where that
mechanism is actually realized.
