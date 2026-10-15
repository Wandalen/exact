# Feature: Bench Note Vs F64

### Scope

- **Purpose**: Give workstream 002 documented evidence that the exact path is cheap enough to adopt on the hot path.
- **Responsibility**: A benchmark comparing this family's arithmetic against `f64`.
- **In Scope**: A bench living in `exact_arith`'s tests or a dedicated `exact_bench` spike.
- **Out of Scope**: A permanent `module/` crate for the bench — the source is explicit this doesn't warrant one until 002's hot path needs the number.

**Design status**: built 2026-10-02, matching this feature's own scoped shape — "a bench living in `exact_arith`'s tests," not a permanent `module/` crate: [`exact_arith/tests/bench_vs_f64.rs`](../../module/exact_arith/tests/bench_vs_f64.rs), a `#[test]` (no `criterion` dependency) timing add/compare/ratio against `f64` over 10,000,000 iterations each, with a loose non-gating sanity bound rather than a strict pass/fail performance assertion. Results recorded in [hard problem 12](../hard_problem/012_hot_path_performance.md)'s own Design status. The family's other `f64` usage remains the unrelated correctness "control arm" in `smoke_exact_market_split` that demonstrates `0.1 × 10 != 1.0` in `f64` — a different claim (correctness, not timing) than this feature's own.

### Statement

A documented bench versus `f64` gives workstream 002 the evidence it needs to adopt exact arithmetic on the matching hot path, addressing hard problem 12 (hot path) with a number instead of an assertion.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:208` | Feature 22 in the source's numbered Features list |
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:140-146` | Hard problem 12, "Hot path," this feature substantiates |
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:384` | "A bench lives in exact_arith tests or a exact_bench spike, not in module/ until 002's hot path needs the number" |
