# ADR-004: A Requested Display Precision Rounds Half To Even

**Date**: 2026-10-11
**Status**: Proposed
**Deciders**: ihortry, for wandalen's review

## Context

`Display` for `Decimal`, `Qty` and `PriceAt` ignored a requested precision:
`format!( "{:.2}", price )` printed `0.9` for `0.90`, and `8024.7` for an
amount in cents. Every other number type in Rust honours `{:.N}`, so a caller
printing money had to patch the text by hand — the exchange's
`two_decimals()` does it with three `String` allocations per price (E5b).

`Qty` and `PriceAt` had a second gap: each rendered its inner decimal with
`write!( f, "{}", self.value )`, which starts a fresh format with no
precision, so even a `Decimal` that honoured `{:.2}` would not have received
it through them.

## Decision

- **No precision** (`{}`): unchanged — exact, trailing zeros trimmed.
- **A precision of N places** (`{:.N}`): exactly N digits after the point.
  - **More places than the scale holds** are zeros: `0.9` at `{:.4}` is `0.9000`.
  - **Fewer places** round the dropped digits **half to even**, through
    `exact_round::round_div`: `1.005` at `{:.2}` is `1.00`, `1.015` is `1.02`.
  - The sign is taken after rounding, so `-0.004` at `{:.2}` is `0.00`, never `-0.00`.
- **Width, fill, alignment, `+` and `0`** are honoured the way they are for
  integers, with or without a precision: `{:>10.2}` of `1.5` is `      1.50`,
  `{:+.2}` is `+1.50`, `{:08.2}` of `-1.5` is `-0001.50`. Right-aligned by
  default, as for every number; `0` puts the zeros after the sign.
- **One renderer**, a private, non-generic `render`, serves both `{}` and
  `{:.N}`: it works out the digits after the point — trimmed, or exactly N —
  and then pads the text as `Formatter::pad_integral` pads an integer. It
  writes straight into the formatter, so it needs no buffer and has no limit
  on the precision.
- `Qty` and `PriceAt` forward the formatter itself —
  `fmt::Display::fmt( &self.value, f )` — so every flag reaches the decimal.
- Only the text is rounded; the value is never changed. Nothing is allocated:
  digits and zeros are written straight into the formatter, checked by
  `exact_arith/tests/no_alloc_test.rs`.

## Alternatives Considered

### Option 1: Round half up

What many people expect on paper. Rejected: half-up has a directional bias,
and it disagrees with both the family's own default
(`exact_round::rounding_default()` is `HalfEven`) and Rust's own `{:.N}` for
`f64`, which rounds a tie to even (`format!( "{:.1}", 0.25 )` is `"0.2"`).

### Option 2: Truncate the dropped digits

The simplest. Rejected: truncation is biased toward zero, and it silently
prints `1.01` for `1.019`.

### Option 3: Ignore the precision when it is smaller than the scale

Print every digit, padding only. Rejected: `{:.2}` would then print three or
more digits for some values, which is exactly the inconsistency E5b reports.

### Option 4: A precision argument on `exact_fmt`'s functions instead

Rejected: `{:.N}` is the standard way to ask, and `Display` lives on
`exact_kind`'s types (`exact_fmt` cannot implement it, by Rust's orphan rule —
feature 016's Design status), so the precision belongs there.

### Option 5: Render into a stack buffer, then call `Formatter::pad_integral`

Suggested in review (#7), and `pad_integral` gives the integer padding rules
for free. Rejected for the streaming renderer: the text must fit one buffer,
and a precision has no upper bound — `{:.40}` of a large value passes any
fixed size, and growing the buffer allocates. `render` writes the same
padding `pad_integral` would, straight into the formatter;
`flags_match_f64_formatting` checks it against `f64` for every flag.

## Consequences

**Positive:**
- `format!( "{:.2}", amount )` prints money as money; the exchange's
  `two_decimals()` can go for prices on a cent tick. It is not a one-for-one
  swap for a finer price: `two_decimals()` only pads, so `100.125` stays
  `100.125`, while `{:.2}` rounds it to `100.12`.
- Rendering at a precision allocates nothing.

**Negative:**
- A printed value can differ from the stored one when fewer places are asked
  for — by design, as for every number type in Rust.
- Printing past the scale does not round-trip: `{:.4}` of a `Decimal< 2 >`
  `1.5` is `1.5000`, which `parse` at scale 2 refuses as `ExcessPrecision`.
  Print at most the scale's places for text that will be parsed back.

**Neutral:**
- The flags other than precision were added in review (#7): E5b asked only
  for precision, but the first real use of `{:.2}` is tables of amounts,
  where `{:>10.2}` printing `1.50` unpadded misaligns every column. A value
  formatted with a width but no other flag now pads, where it used to print
  bare.
- `exact_kind` already depended on `exact_round` for `rescale`
  (→ [ADR-002](002_explicit_rescale_between_scales.md)); no new dependency.

## Related

- [impl Display for Decimal](../item/implementation/004_display_for_decimal.md) — the impl this changes
- [Display::fmt for Qty](../item/associated_function/022_fmt_display_for_qty.md) — the forwarding fix
- [Feature 016](../../../../docs/feature/016_display_with_scale.md) — display with scale; its Design status records this
