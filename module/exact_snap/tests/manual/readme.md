# Manual Testing — exact_snap

What a person checks by hand for snapping a price or a quantity to the
nearest point on a grid — a tick size for price, a lot size for quantity.

`exact_snap` is pure, deterministic logic — no threads, no IO, no timing — so
almost everything about it is decidable by the automated suite. The one thing
a test suite doesn't surface on its own is whether the crate's rustdoc
actually has runnable examples.

## Plan

### M1 — Documented examples compile and are worth reading

```bash
cargo test -p exact_snap --doc
```

Expected: 0 doctests run today — this crate's rustdoc has no `# Examples`
section yet, so there is nothing for `cargo test --doc` to execute. That is
the honest baseline, not a failure; the check exists so a future doc example
silently failing to run (for example a `` ```rust,ignore `` typo) would be
caught by comparing against this record.

## Run Record

| Date | By | Result | Notes |
|------|-----|--------|-------|
| 2026-10-02 | claude | M1 pass | 0 doctests ran, matching the 0 found in this crate's rustdoc — confirmed via the workspace-wide `cargo test --doc --workspace` baseline. |
