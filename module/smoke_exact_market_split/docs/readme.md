# docs

Design documentation for `smoke_exact_market_split`, as typed doc definitions.

| Directory | Responsibility |
|------|-----------------|
| `decisions/` | Why the control arm must disagree, and why the lane is a library rather than a bare `main.rs` |
| `pitfall/` | Bug-fix documentation for defects found and fixed in this lane |
| `definition/` | Module Index — every definition in this crate, in one place |
| `workaround/` | External constraints this crate absorbs — none |

This crate is the family's smoke lane: all 14 leaves, exercised in one
process through `exact_arith` alone, with an `f64` control arm required to
disagree with the exact path. It replaces `smoke_exact_arithmetic` at the
Tier 5 cutover, carrying that crate's `decisions/` and `pitfall/` content
forward against the real current, 14-crate-wide source rather than restating
it verbatim. It has no `type/`, `invariant/`, `algorithm/`, or
`non_functional_requirement/` docs of its own — it composes properties
already owned and documented by the leaf crates (→
[`exact_arith/docs/`](../../exact_arith/docs/readme.md)) rather than defining
new ones.
