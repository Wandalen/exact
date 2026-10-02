# Manual Testing — exact_scale

What a person checks by hand for scale-factor math and the declared ceiling:
powers of ten and the headroom and ceiling constants derived from them.

`exact_scale` is pure, deterministic logic — no threads, no IO, no timing —
so almost everything about it is decidable by the automated suite. The one
thing a test suite doesn't surface on its own is whether the crate's rustdoc
actually has runnable examples.

## Plan

### M1 — Documented examples compile and are worth reading

```bash
cargo test -p exact_scale --doc
```

Expected: 1 doc example runs and passes — confirmed 2026-10-02 via the
workspace-wide `cargo test --doc --workspace` baseline. Then read the
rendered docs and check the example shows a real ceiling/headroom
derivation, not just the constant's syntax:

```bash
cargo doc -p exact_scale --no-deps --open
```

## Run Record

| Date | By | Result | Notes |
|------|-----|--------|-------|
| 2026-10-02 | claude | M1 pass | 1 doctest ran and passed, matching the workspace-wide baseline. |
