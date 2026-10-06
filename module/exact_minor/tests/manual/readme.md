# Manual Testing — exact_minor

What a person checks by hand for the raw subunit integer: checked and
saturating arithmetic over the backing width, with no scale or kind attached.

`exact_minor` is pure, deterministic logic — no threads, no IO, no timing —
so almost everything about it is decidable by the automated suite. The one
thing a test suite doesn't surface on its own is whether the crate's rustdoc
actually has runnable examples.

## Plan

### M1 — Documented examples compile and are worth reading

```bash
cargo test -p exact_minor --doc
```

Expected: 2 doctests run and pass — the `Minor` type's `compile_fail` example
(a bare `i64` is refused) and the crate-level `# Examples` block, which adds two counts with
`minor_checked_add`.
The check exists so a doc example silently failing to run (for example a
`` ```rust,ignore `` typo) would be caught by comparing against this record.

## Run Record

| Date | By | Result | Notes |
|------|-----|--------|-------|
| 2026-10-02 | claude | M1 pass | 0 doctests ran, matching the 0 found in this crate's rustdoc — confirmed via the workspace-wide `cargo test --doc --workspace` baseline. |
| 2026-10-02 | ihortry | M1 pass | 1 doctest ran and passed (`src/lib.rs - (line 28)`). Corrects the row above: the crate-level example already existed, so its "0 doctests" count was wrong. |
| 2026-10-06 | ihortry | M1 pass | 2 doctests ran and passed: `src/lib.rs - (line 26)` and `src/lib.rs - Minor (line 47) - compile fail` (rustdoc reports the `compile_fail` one in its own result line). Corrects the row above, which counted only the first. |
