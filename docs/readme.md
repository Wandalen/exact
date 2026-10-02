# docs

Family-level material for workstream 006 that belongs to no single crate below.

## Responsibility Table

| File | Responsibility |
|------|-----------------|
| [`codename_vm_ecs_overlap_20260930_0040.md`](codename_vm_ecs_overlap_20260930_0040.md) | Archived external architecture-overlap conversation; touches this family only in passing ("keep, no overlap") — not this family's own design source |
| [`codename_vm_ecs_export_1.md`](../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md) | Archived design conversation: the preferred/target 15-crate decomposition for this workstream, since implemented in full — source for the 6 collections below. Relocated outside this repository to `codename_space_sandbox/docs/` (2026-10-01) — a pasted chat transcript has no business under version control in a Rust crate family; every citing instance's own `Sources` table was updated to the new relative path |
| [`feature/`](feature/readme.md) | Capabilities the target decomposition specifies (22), from `codename_vm_ecs_export_1.md`'s "Features" list |
| [`hard_problem/`](hard_problem/readme.md) | The workstream's "Hard problems" (14) the target decomposition exists to solve |
| [`crate/`](crate/readme.md) | Per-crate dependency/boundary specs for the 15 proposed crates, from `codename_vm_ecs_export_1.md`'s "List crates" answer (15) |
| [`type/`](type/readme.md) | Per-crate struct/enum/trait/function/error surface for the 15 proposed crates (15) |
| [`wall_smoke/`](wall_smoke/readme.md) | The one proposed demo's full contract — name, preconditions, coverage claim, golden print, pass criteria — from `codename_vm_ecs_export_1.md`'s "Wall smoke" label (1) |
| [`scene/`](scene/readme.md) | Steps of the proposed `smoke_exact_market_split` demo, from `codename_vm_ecs_export_1.md`'s "Scene" list (10) |
| [`guide/`](guide/readme.md) | Fast onboarding: each of the 16 crates' real current purpose and the real dependency tree, not the proposal (1) |
| [`research/`](research/readme.md) | Dated, sourced investigations into questions this family's design raises, starting with existing-crate alternatives (1) |

## Workstream Charter

`../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:42-48` opens workstream 006 with a charter that
predates, and sits above, the `feature/`/`hard_problem/`/`crate/`/`type/`
breakdowns above — no existing collection's scope covers it, so it is
recorded here instead of split across them:

> **Workstream 006 — Exact arithmetic.** Responsibility: Money and quantity
> as integer minor units (or fixed-scale decimals), with an explicit
> rounding policy and conservation checks. No binary floats. Plain types for
> 002 and 010. Depends on: nothing in the catalog. Feeds: 002 (prices,
> quantities, fees), 010 (wallets), 011 (stocks).

**Design status**: "Depends on nothing" holds exactly — `exact_minor`,
`exact_scale`, and `exact_round` are the family's only zero-dependency
crates, and nothing outside `module/` is a Cargo dependency of any
of the 15. "Feeds 002" and "feeds 010" both hold: `exchange_core` and
`cluster_economy` (workstream 010, both in the `codename_space_sandbox`
monorepo) each depend on `exact_arith` directly — a path dependency
reaching this repository's `module/exact_arith` from their own location,
depth varying per consumer since this family was extracted to its own
repository — and call its re-exported `Entry`/`verify`/`Money`/`Quantity`
in their own production paths. **"Feeds 011" does not hold** — workstream
011 (`module/division/011_macro_simulation`, `cluster_macro`) has no
dependency on `exact_arith` or any `exact_*` crate; its own `Tick`/`Lot`
naming is an unrelated simulation-tick/session-lot vocabulary collision,
not a consumer of `exact_snap`'s types of the same name (confirmed via
`Cargo.toml` inspection, not just a name grep).

**Earlier mention**: a separate, broader VM/ECS design conversation (external
to this repo — `docs/reference/grok_vm_ecs_design_conversation.md` in the
`codename_space_sandbox` monorepo) first named this workstream as a bare
placeholder — three crate stubs, `exact_decimal`/`exact_qty`/`exact_audit` —
before the focused redesign captured in `codename_vm_ecs_export_1.md` above
superseded it. Not archived here: it adds no design content beyond names
already fully superseded by the 15-crate decomposition this directory
documents.

## Dependency Tree

`../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:386-410` closes the "List crates" answer with
the proposed dependency tree, its roots, and a recommended build subset —
distributed knowledge that is implicit across all 15 `crate/*.md` instances'
own `Dependencies` sections but never assembled as one artifact until now:

```text
exact_minor
exact_scale
exact_round
exact_sign            → exact_minor
exact_kind            → exact_minor, exact_scale
exact_add             → exact_kind, exact_sign
exact_ratio           → exact_kind, exact_round
exact_dust            → exact_kind, exact_ratio
exact_parse           → exact_kind, exact_scale
exact_fmt             → exact_kind
exact_bytes           → exact_kind, exact_scale
exact_snap            → exact_kind, exact_round
exact_conserve        → exact_add, exact_kind
exact_cmp             → exact_kind
exact_arith           → kind, add, ratio, dust, parse, fmt, bytes, snap, conserve, cmp
```

Roots: `exact_minor`, `exact_scale`, `exact_round`. Proposed consumer edge:
"002 depends on `exact_arith` (or directly on `exact_kind`, `exact_cmp`,
`exact_snap`, `exact_conserve` if you want a narrower edge)." Proposed MVP
build subset: minor, scale, kind, sign, round, add, ratio, dust, cmp,
conserve, arith — parse/fmt/bytes/snap deferred as not yet needed for a
first working order-book integration.

**Design status**: the real `exact_arith` facade diverges from this tree —
it depends directly on, and re-exports, all 14 leaves (including the 3 roots
"reached transitively" here, plus `exact_sign`, absent from this proposed
edge list entirely) rather than 10 direct dependencies with 3 transitive
ones; see [`crate/015_exact_arith.md`](crate/015_exact_arith.md)'s own
Design status for the full account. The proposed consumer-narrowing edge
("002... directly on `exact_kind`, `exact_cmp`, `exact_snap`,
`exact_conserve`") was not taken — `exchange_core` depends on `exact_arith`
alone (confirmed via its `Cargo.toml`), matching the simpler option the
proposal itself offered as an alternative. The MVP build-order subset was
not followed as a staged rollout — all 15 crates (16 with the demo lane)
were built together in one migration rather than parse/fmt/bytes/snap
trailing behind the rest.

Per-crate design documentation (type, invariant, decisions, algorithm, ...) for
the 16 crates that actually exist lives in each crate's own `docs/`, per
Leaf-Proximate Placement — see
[`exact_kind/docs/readme.md`](../module/exact_kind/docs/readme.md) for a
representative example of that pattern.

The `feature/`, `hard_problem/`, `crate/`, `type/`, `wall_smoke/`, and `scene/`
collections below document `codename_vm_ecs_export_1.md`'s proposed 15-crate split
(`exact_minor`, `exact_scale`, `exact_kind`, `exact_sign`, `exact_round`,
`exact_add`, `exact_ratio`, `exact_dust`, `exact_parse`, `exact_fmt`,
`exact_bytes`, `exact_snap`, `exact_conserve`, `exact_cmp`, `exact_arith`) as it
was originally proposed, sourced from an archived conversation. That proposal
has since been built in full (plus a sixteenth crate, the
`smoke_exact_market_split` demo lane, covered by `wall_smoke/` and `scene/`) — the 5-crate
structure this directory used to describe as "the current implementation"
(`exact_decimal`, `exact_qty`, `exact_audit`, `exact_arithmetic`,
`smoke_exact_arithmetic`) no longer exists; it was deleted at the migration's
cutover, and is retrievable only via `git show HEAD:<old_crate>/<path>`
from before that commit. This directory's role has shifted accordingly: it is
now a historical record of the original proposal, not a living specification.
Every instance's **Design status** line states which real crate now
implements it and whether the real implementation matched the proposal or
deviated from it (with a pointer to that crate's own `decisions/` entry where
it deviated) — in place of the old "preferred, not yet implemented" note.
Every instance still cites `codename_vm_ecs_export_1.md` line ranges as its
own Sources, unchanged, since that citation is about the proposal's origin,
not its implementation status — the original proposed content (structs,
functions, procedures, statements) is likewise left as written, a record of
what was proposed rather than a restatement of what was built.

Collection names track the source document's own headings as closely as
possible (`crate/` for its "List crates" prompt, `wall_smoke/` for its "Wall
smoke" demo label, `scene/` for its "Scene" demo steps) rather than
`doc_des.rulebook.md`'s closest catalog names, per explicit request. The source's "Boundaries" section (In/Out/Must-not) is deliberately
not represented as its own collection — its "Must-not" items restated existing
`hard_problem/` entries, its "In" items restated existing `crate/`/`type/`
entries, and its "Out" items named other workstreams' territory, so none of it
added content this family's own docs didn't already carry.
