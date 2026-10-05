# Manual Testing — exact_bytes

What a person checks by hand for `Wire`, the fixed-width byte encoding for a
conserved value, and its to/from conversions per kind.

`exact_bytes` is pure, deterministic logic — no threads, no IO, no timing —
so almost everything about it is decidable by the automated suite. The one
thing a test suite doesn't surface on its own is whether the crate's rustdoc
actually has runnable examples.

## Plan

### M1 — Documented examples compile and are worth reading

```bash
cargo test -p exact_bytes --doc
```

Expected: 1 doctest runs and passes — the crate-level `# Examples` block in
`src/lib.rs`'s module doc. The check exists so a future doc example silently
failing to run (for example a `` ```rust,ignore `` typo) would be caught by
comparing against this record.

## Run Record

| Date | By | Result | Notes |
|------|-----|--------|-------|
| 2026-10-02 | claude | M1 pass | 0 doctests ran, matching the 0 found in this crate's rustdoc — confirmed via the workspace-wide `cargo test --doc --workspace` baseline. |
| 2026-10-05 | claude | M1 pass | 1 doctest ran and passed (the module doc's `# Examples` block); corrects the 0 recorded above, which predates that block. |
