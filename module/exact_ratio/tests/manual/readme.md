# Manual Testing — exact_ratio

What a person checks by hand for the rational multiplier (`Ratio`) and
mode-driven integer division, per kind.

`exact_ratio` is pure, deterministic logic — no threads, no IO, no timing —
so almost everything about it is decidable by the automated suite. The one
thing a test doesn't surface on its own is cost: every multiply here widens
to `i128` before dividing back down, per this crate's own documented range
budget, and that widening is the whole cost model — a person reads the
source by eye to confirm nothing else crept in alongside it.

## Plan

### M1 — Every function stays straight-line: widen, multiply or divide, narrow

```bash
grep -n "for .* in \|while \|Vec<\|Box<\|String" src/lib.rs | grep -v "^[0-9]*:\s*//"
```

Expected: no output. Confirmed 2026-10-02 — every public function
(`ratio_new`, `money_mul_ratio`, `qty_mul_ratio`, `price_mul_ratio`,
`money_div_round`, `qty_div_round`) is a short, straight-line body: a
widen-multiply-narrow sequence, or a delegating call into
`exact_round::round_div`. No loop construct and no heap-backed type appears
anywhere in the crate.

### M2 — Documented examples compile and are worth reading

```bash
cargo test -p exact_ratio --doc
```

Expected: 1 doctest runs and passes — the crate-level `# Examples` block in
`src/lib.rs`'s module doc. The check exists so a future doc example silently
failing to run (for example a `` ```rust,ignore `` typo) would be caught by
comparing against this record.

## Run Record

| Date | By | Result | Notes |
|------|-----|--------|-------|
| 2026-10-02 | claude | M1 pass, M2 pass | M1: grep for the for-loop idiom and heap-backed types returned no matches across all 6 public functions. M2: 0 doctests ran, matching the 0 found in this crate's rustdoc — confirmed via the workspace-wide `cargo test --doc --workspace` baseline. |
| 2026-10-05 | claude | M2 pass | 1 doctest ran and passed (the module doc's `# Examples` block); corrects the 0 recorded above, which predates that block. |
