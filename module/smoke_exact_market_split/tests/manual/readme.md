# Manual Testing — smoke_exact_market_split

What a person checks by hand for the smoke lane that runs this family's
slice end to end, with a control arm, through `exact_arith` — the 14-crate
facade — alone.

`smoke_exact_market_split` is pure, deterministic logic — no threads, no IO,
no timing — so almost everything about it is decidable by the automated
suite. The one thing a test suite doesn't surface on its own is whether the
crate's rustdoc actually has runnable examples.

## Plan

### M1 — Documented examples compile and are worth reading

```bash
cargo test -p smoke_exact_market_split --doc
```

Expected: 0 doctests run today — this crate's rustdoc has no `# Examples`
section yet, so there is nothing for `cargo test --doc` to execute. That is
the honest baseline, not a failure; the check exists so a future doc example
silently failing to run (for example a `` ```rust,ignore `` typo) would be
caught by comparing against this record. This crate's real demonstration is
the runnable binary itself — `cargo run -p smoke_exact_market_split` —
covered separately by the root readme's and guide's quickstart sections, not
by a doctest.

## Run Record

| Date | By | Result | Notes |
|------|-----|--------|-------|
| 2026-10-02 | claude | M1 pass | 0 doctests ran, matching the 0 found in this crate's rustdoc — confirmed via the workspace-wide `cargo test --doc --workspace` baseline. |
