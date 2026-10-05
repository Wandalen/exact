# Manual Testing — exact_cmp

What a person checks by hand for comparison, equality, and min/max per kind.

`exact_cmp` is pure, deterministic logic — no threads, no IO, no timing — so
almost everything about it is decidable by the automated suite. The one claim
worth checking by eye across the whole family, not just this crate, is
determinism itself: a float silently breaks equality (`0.1 + 0.2 != 0.3` at
the bit level), and comparison is exactly where that bug would first surface
if a float had crept in anywhere upstream.

## Plan

### M1 — No floating-point type anywhere in the family except one disclosed control arm

```bash
grep -rn "f32\|f64" module/*/src
```

Expected: every hit is confined to
`module/smoke_exact_market_split/src/lib.rs`, inside its deliberately
disclosed `f64` control arm — a lane whose own doc comment says outright
that "`f64` appears in this file and in no other file of the family," and
which exists specifically so the smoke test can assert the float and exact
paths *disagree*. Zero hits in `exact_cmp` itself, or in any of the other 14
library crates. Confirmed 2026-10-02.

### M2 — Documented examples compile and are worth reading

```bash
cargo test -p exact_cmp --doc
```

Expected: 1 doctest runs and passes — the crate-level `# Examples` block in
`src/lib.rs`'s module doc. The check exists so a future doc example silently
failing to run (for example a `` ```rust,ignore `` typo) would be caught by
comparing against this record.

## Run Record

| Date | By | Result | Notes |
|------|-----|--------|-------|
| 2026-10-02 | claude | M1 pass, M2 pass | M1: all `f32`/`f64` hits confined to `smoke_exact_market_split`'s disclosed control arm; zero hits in `exact_cmp` or the other 14 library crates. M2: 0 doctests ran, matching the 0 found in this crate's rustdoc — confirmed via the workspace-wide `cargo test --doc --workspace` baseline. |
| 2026-10-05 | claude | M2 pass | 1 doctest ran and passed (the module doc's `# Examples` block); corrects the 0 recorded above, which predates that block. |
