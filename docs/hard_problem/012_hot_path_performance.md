# Hard Problem: Hot Path Performance

### Scope

- **Purpose**: State why add/compare/ratio must be cheap enough for the matching hot path.
- **Responsibility**: The requirement that this family not force workstream 002 back to floats for performance.
- **In Scope**: Add, compare, and ratio operations called per fill.
- **Out of Scope**: The actual bench proving this (→ `../feature/022_bench_note_vs_f64.md`).

**Design status**: resolved, with a real measurement. The arithmetic this hard problem is about is built — [`exact_add`](../../module/exact_add/readme.md), [`exact_ratio`](../../module/exact_ratio/readme.md), and [`exact_cmp`](../../module/exact_cmp/readme.md) are thin functions over integer operations with no heap allocation — and [feature 022's documented benchmark against `f64`](../feature/022_bench_note_vs_f64.md), previously never built, now exists at [`exact_arith/tests/bench_vs_f64.rs`](../../module/exact_arith/tests/bench_vs_f64.rs). Measured 2026-10-02 (10,000,000 iterations each, this machine): add 26.4ns vs. `f64`'s 17.7ns (1.5x), compare 19.2ns vs. 21.6ns (0.9x — Ordering on a plain `i64` beats `f64`'s NaN-aware `partial_cmp` here), ratio-multiply 54.4ns vs. 15.7ns (3.5x, the checked-overflow and rounding logic costing the most of the three). All three land in the tens of nanoseconds — tens of millions of ops/sec even in the slowest case — comfortably cheap enough for a matching hot path in absolute terms, despite the 1.5-3.5x constant-factor overhead against raw, unchecked `f64`. Re-run via `cargo test -p exact_arith --test bench_vs_f64 -- --nocapture`; numbers are environment-dependent, so treat the ratios as the stable finding and the absolute ns as this machine's own snapshot.

### Statement

- **Purpose**: add, compare, and ratio are cheap enough for matching.
- **Why**: 002 calls this per fill.
- **If missing**: the book falls back to floats.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:140-146` | Hard problem 12, verbatim Purpose/Why/If-missing |
