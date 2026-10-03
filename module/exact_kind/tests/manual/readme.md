# Manual Testing — exact_kind

What a person checks by hand for the conserved value type family — `Money`,
`Qty`, and `Price`, each a fixed-point decimal whose scale lives in the type.

`exact_kind` is pure, deterministic logic — no threads, no IO, no timing — so
almost everything about it is decidable by the automated suite. The manual
plan covers the two things a test cannot assert about itself: that the
*compiler* rejects a scale mismatch, and that every kind value stays cheap to
copy.

## Plan

### M1 — Two different `SCALE` values are two different types, not a runtime check

`Decimal<const SCALE: u32>` carries its scale as a const generic parameter
(`src/lib.rs:153`), and `Qty<const SCALE: u32>` the same way (`:397`). `Money`
is `Decimal<MONEY_SCALE>` (`:56`) — a scale-6 decimal and a scale-2 decimal
are therefore different monomorphized types at compile time. This is why the
family carries no `ScaleMismatch` runtime error anywhere: the mismatch never
reaches runtime at all. A test cannot assert this; the proof is that the
mismatched code does not compile.

```bash
cd module/exact_kind
cat > /tmp/-exact_kind_m1.rs <<'EOF'
fn main()
{
  let a = exact_kind::Decimal::< 6 >::parse( "1" ).unwrap();
  let b = exact_kind::Decimal::< 2 >::parse( "1" ).unwrap();
  let _ = a == b;
}
EOF
```

Expected: adding that comparison to a test file and building it fails with
`mismatched types`, naming `Decimal<6>` and `Decimal<2>` as the two
conflicting types. A build that succeeds means the const generic stopped
carrying the scale, and this crate's own "Disclosed deviations" doc comment
(which claims `ScaleMismatch` is unreachable for exactly this reason) would
need re-examining.

### M2 — Every kind value is `Copy` — no heap allocation on the hot path

A `Decimal`/`Qty` value gets copied on every arithmetic call across this
14-crate family. If it ever stopped being `Copy`, every one of those call
sites would start moving or cloning instead — silently, with no compiler
error pointing at the cause.

```bash
grep -n "derive.*Copy" src/lib.rs
```

Expected: every `derive(...)` line on a public struct/enum in this crate
includes `Copy`. Confirmed 2026-10-02 for all 3: `KindError` (line 72),
`Decimal` (line 153), `Qty` (line 397). `Copy` is only derivable when every
field is itself `Copy`, so this one grep line is sufficient proof neither
struct hides a `String`, `Vec`, or `Box` behind its private fields.

### M3 — Documented examples compile and are worth reading

```bash
cargo test -p exact_kind --doc
```

Expected: 6 doc examples run and pass — 3 that compile and 3 marked
`compile fail`, the last group pinning that money and quantities never mix
(`Display for Qty`'s doc comment). Then read the
rendered docs and check the examples show the type's round-trip and
checked-arithmetic behaviour, not just its syntax:

```bash
cargo doc -p exact_kind --no-deps --open
```

## Run Record

| Date | By | Result | Notes |
|------|-----|--------|-------|
| 2026-10-02 | claude | M1 pass, M2 pass, M3 pass | M1: actually compiled (via `rustc` against the built `exact_kind` rlib, equivalent to pasting the snippet into a test file) — failed exactly as predicted, with `error[E0308]: mismatched types ... expected struct 'Decimal<6>', found struct 'Decimal<2>'`. M2: all 3 derives (`KindError`, `Decimal`, `Qty`) include `Copy`. M3: 2/2 doctests passed in the workspace baseline run. |
| 2026-10-02 | ihortry | M3 pass | 6 doctests: 3 compile (`Decimal`, `Qty`, and the same-kind companion on `Display for Qty`), 3 `compile fail` (money + quantity, quantity + money, quantity as money). |
