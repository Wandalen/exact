# Non-Functional Requirement: Representable Range And Headroom

### Scope

- **Purpose**: State the numeric budget this crate holds itself to, so a future change to the ceiling or the backing width is checked against a concrete margin instead of guesswork.
- **Responsibility**: `CEILING_WHOLE_UNITS`, `CEILING_MINOR_UNITS`, `HEADROOM_FACTOR`, `MONEY_SCALE`, `pow10`, and the compile-time assertion linking them.
- **In Scope**: The declared ceiling and its relationship to `i64::MAX`, and `pow10`'s own boundary.
- **Out of Scope**: Which backing width or per-kind scale a conserved value actually uses (→ [`exact_kind`'s own `type/001_conserved_value_type_family.md`](../../../exact_kind/docs/type/001_conserved_value_type_family.md), which consumes these constants rather than declaring its own).

### Requirement

The declared ceiling — `CEILING_WHOLE_UNITS = 9_000_000_000` whole units,
equivalently `CEILING_MINOR_UNITS = 9_000_000_000_000_000` minor units at
`MONEY_SCALE = 6` — must stay at least `HEADROOM_FACTOR` (1000×) below
`i64::MAX` (≈9.22×10¹⁸). This is checked at compile time, not merely
documented:

```rust
const _ : () = assert!( CEILING_MINOR_UNITS <= i64::MAX / HEADROOM_FACTOR );
```

A build fails outright if a future change to either the ceiling or the
headroom factor violates the relationship. Verified directly:

```sh
# run from anywhere — no crate root dependency, pure arithmetic
python3 -c 'print( 9_000_000_000 * 10**6 <= ( 2**63 - 1 ) // 1000 )'
# True
```

### Why 1000×, Not Some Other Margin

A factor of 1000 is roughly ten bits, sized as a magnitude allowance for
intermediates rather than as a generic safety margin: it makes accumulating a
thousand ceiling-sized amounts safe without overflowing `i64`
(`CEILING_MINOR_UNITS.checked_mul(HEADROOM_FACTOR)` stays in range — see
Sources below), which is the shape of a streaming fold over many
ceiling-sized entries. It does **not** make multiply-before-divide safe on its
own — a rate multiplying by a larger factor before dividing can still exceed
this allowance, which is why this requirement budgets accumulation headroom
specifically and not every possible intermediate.

### `pow10`'s Own Boundary

`pow10(n)` returns `10ⁿ` as a `Backing`-shaped value for any `n` up to 18 —
the largest power of ten an `i64` holds (`10¹⁸ ≈ 1.0×10¹⁸ < i64::MAX`; `10¹⁹`
overflows). Past that, it panics rather than wrapping or returning a
truncated value. The panic is deliberate. In a `const` position
(`CEILING_MINOR_UNITS`'s own declaration, and every `Decimal<SCALE>::ONE_MINOR`
in `exact_kind`) it becomes a compile error, so a `SCALE` past 18 fails to
build. A runtime call past 18 panics at runtime instead, as this crate's own
test shows. `exact_kind` does call `pow10` at runtime — in `Decimal::parse`
(`pow10( SCALE - supplied )`) and twice in `Decimal`'s `Display` (trimming
trailing zeros) — but each exponent is at most `SCALE`, so none of them can
pass 18 while `SCALE` itself does not.

### Sources

| File | Relationship |
|------|--------------|
| `src/lib.rs:22` | `HEADROOM_FACTOR = 1000` and its doc comment |
| `src/lib.rs:29` | `CEILING_WHOLE_UNITS = 9_000_000_000` and its doc comment |
| `src/lib.rs:37` | `MONEY_SCALE = 6` |
| `src/lib.rs:45` | `CEILING_MINOR_UNITS`, derived from the whole-unit ceiling at `MONEY_SCALE` |
| `src/lib.rs:49` | The compile-time assertion enforcing the 1000× margin |
| `src/lib.rs:64-75` | `pow10` — the power-of-ten table and its `n <= 18` boundary |

### Tests

| File | Relationship |
|------|--------------|
| `tests/scale_factor_test.rs` | Re-derives the ceiling relation and the `pow10` boundary, including the panic past `n = 18` |
