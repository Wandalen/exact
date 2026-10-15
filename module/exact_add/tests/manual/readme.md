# Manual Testing — exact_add

What a person checks by hand for checked and saturating addition and
subtraction, dispatched per kind.

`exact_add` is pure, deterministic logic — no threads, no IO, no timing — so
almost everything about it is decidable by the automated suite. The one thing
a test doesn't surface on its own is cost: every call in this crate sits on
`exact_arith`'s hot path, so a person reads the source by eye to confirm
nothing crept in that doesn't belong there.

## Plan

### M1 — Every function stays straight-line and allocation-free

```bash
grep -c "pub const fn" src/lib.rs
grep -n "for .* in \|while \|Vec<\|Box<\|String" src/lib.rs | grep -v "^[0-9]*:\s*//"
```

Expected: the first command counts 9 — every public function
(`money_add`, `money_sub`, `qty_add`, `qty_sub`, `price_add`, `price_sub`,
`money_checked_neg`, `money_saturating_add`, `qty_saturating_add`) is a
`const fn`, meaning the compiler itself guarantees no hidden runtime cost.
The second command (code lines only, and matching the `for X in Y` loop
idiom specifically rather than a bare "for" — which would also match an
`impl Trait for Type` line and the English word inside doc-comment prose)
produces no output — no loop construct and no heap-backed type appears
anywhere in the crate's actual code. Confirmed 2026-10-02: every body is a
single delegating call or a short match into `exact_kind`'s or `exact_sign`'s
own checked methods.

### M2 — Documented examples compile and are worth reading

```bash
cargo test -p exact_add --doc
```

Expected: 1 doctest runs and passes — the crate-level `# Examples` block in
`src/lib.rs`'s module doc. The check exists so a future doc example silently
failing to run (for example a `` ```rust,ignore `` typo) would be caught by
comparing against this record.

## Run Record

| Date | By | Result | Notes |
|------|-----|--------|-------|
| 2026-10-02 | claude | M1 pass, M2 pass | M1: 9/9 public functions are `const fn`; grep for loop/heap constructs returned no matches. M2: 0 doctests ran, matching the 0 found in this crate's rustdoc — confirmed via the workspace-wide `cargo test --doc --workspace` baseline. |
| 2026-10-05 | claude | M2 pass | 1 doctest ran and passed (the module doc's `# Examples` block); corrects the 0 recorded above, which predates that block. |
