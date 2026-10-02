# Manual Testing — exact_conserve

What a person checks by hand for conservation auditing: does a set of
postings sum to zero?

`exact_conserve` is pure, deterministic logic — no threads, no IO, no timing —
so almost everything about it is decidable by the automated suite. The one
thing a test suite doesn't surface on its own is whether the crate's rustdoc
actually has runnable examples.

## Plan

### M1 — Documented examples compile and are worth reading

```bash
cargo test -p exact_conserve --doc
```

Expected: 1 doc example runs and passes — confirmed 2026-10-02 via the
workspace-wide `cargo test --doc --workspace` baseline. Then read the
rendered docs and check the example shows a real posting set summing to
zero, not just the auditor's call syntax:

```bash
cargo doc -p exact_conserve --no-deps --open
```

## Run Record

| Date | By | Result | Notes |
|------|-----|--------|-------|
| 2026-10-02 | claude | M1 pass | 1 doctest ran and passed, matching the workspace-wide baseline. |
