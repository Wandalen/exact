# Manual Testing — exact_arith

What a person checks by hand for the facade over this family's 14-crate value
substrate — `Money`, commodity quantities, and every other conserved value,
exact and checked instead of floating-point.

`exact_arith` is pure, deterministic logic — no threads, no IO, no timing —
so almost everything about it is decidable by the automated suite. The one
thing a test suite doesn't surface on its own is whether the crate's rustdoc
actually has runnable examples.

## Plan

### M1 — Documented examples compile and are worth reading

```bash
cargo test -p exact_arith --doc
```

Expected: 1 doc example runs and passes — confirmed 2026-10-02 via the
workspace-wide `cargo test --doc --workspace` baseline. Then read the
rendered docs and check the example shows the facade's point (one import,
one conserved-value call) rather than its syntax:

```bash
cargo doc -p exact_arith --no-deps --open
```

## Run Record

| Date | By | Result | Notes |
|------|-----|--------|-------|
| 2026-10-02 | claude | M1 pass | 1 doctest ran and passed, matching the workspace-wide baseline. |
