# ADR-001: Every Operation Is Generic Over The Scale

**Date**: 2026-10-11
**Status**: Proposed
**Deciders**: ihortry, for wandalen's review

## Context

The value types carry their scale as a parameter — `Decimal< S >`, `Qty< S >`,
and since `exact_kind`'s [ADR-003](../../../exact_kind/docs/decisions/003_price_generic_over_scale.md)
`PriceAt< S >` — but every free function in the family named the standard
aliases: `money_add( a : Money, … )`, `price_cmp( a : Price, … )`,
`qty_to_wire( v : Quantity )`. A value at any other scale could be built, and
then not passed to a single operation.

Rescaling to six places first works only for a value with six places or
fewer. A quantity of `0.12345678` BTC has eight: `Rounding::Exact` refuses it,
and any other mode loses value — at $65 000, rounding the quantity to
`0.123457` prices the trade at $8 024.71 instead of $8 024.69.

This decision spans nine crates. It is recorded here, in the facade that
re-exports all of them, rather than nine times.

## Decision

- Every function over a conserved value takes `< const S : u32 >`, and in its
  signature `Money` becomes `Decimal< S >`, `Quantity` becomes `Qty< S >`,
  `Price` becomes `PriceAt< S >` — 46 functions in `exact_add`, `exact_cmp`,
  `exact_fmt`, `exact_parse`, `exact_bytes`, `exact_ratio`, `exact_snap`,
  `exact_dust`, `exact_conserve`. Bodies are unchanged; a name such as
  `Money::from_minor` inside one becomes `Decimal::< S >::from_minor`.
- Both operands of a two-value function share one `S`, so mixing scales stays a
  compile error.
- Two functions need more than this, each recorded in its own crate:
  `price_mul_qty` takes a price and a quantity at two different scales
  (`exact_ratio`'s [ADR-002](../../../exact_ratio/docs/decisions/002_price_mul_qty_across_two_scales.md)),
  and the wire functions write and check the type's own scale
  (`exact_bytes`'s [ADR-002](../../../exact_bytes/docs/decisions/002_scale_byte_from_the_type.md)).
  `Tick`/`Lot` follow the same pattern as `Price`
  (`exact_snap`'s [ADR-001](../../../exact_snap/docs/decisions/001_tick_and_lot_generic_over_scale.md)).
- `const fn`s stay `const fn`.

## Alternatives Considered

### Option 1: Keep every function at the standard scale; rescale first

Rejected: it works for coarser values (cents rescale to millionths exactly)
and fails for finer ones — the quantities that most need it, at eight or
eighteen places.

### Option 2: Make generic only the functions the exchange needs now

A smaller change, and the one a reviewer may prefer under YAGNI. Not chosen:
the family's surface would then work at any scale for some operations and at
six places for others, a split every caller would have to discover function by
function. Each change is one signature line, so doing all of them costs little
more than doing a few.

### Option 3: Leave the six parsing and decoding functions at the standard scale

`money_from_str`, `qty_from_str`, `price_from_str`, `money_from_wire`,
`qty_from_wire` and `price_from_wire` take no value of the kind, so their
scale appears only in the return type. Rejected for the wire functions: a
record written at another scale has no other way to be read. Rejected for the
parsing functions for consistency: `Decimal::< S >::parse` already parses at
any scale, and a generic `money_from_str` is that same call under the
family's free-function name.

## Consequences

**Positive:**
- Every operation works at every scale, with no rounding forced on a value.
- Code using `Money`, `Quantity`, `Price`, `Tick` or `Lot` compiles unchanged:
  Rust infers `S` from the arguments.

**Negative:**
- Breaking for the six functions above, wherever nothing else fixes the scale:
  `money_from_str( "1.23" ).unwrap().minor()` no longer compiles and needs
  `money_from_str::< MONEY_SCALE >( … )` or a typed binding. Inside this
  workspace that is `exact_parse`'s tests and doctest and `exact_bytes`'s
  tests; the exchange calls none of the six. `money_sum_assert_zero( &[] )`
  likewise needs its empty slice typed.
- Each scale a program uses compiles its own copy of a function. A program
  uses a handful of scales, so the size cost is small.

**Neutral:**
- No function changed behaviour at the standard scale; the workspace's existing
  tests pass unchanged apart from the type annotations above.

## Related

- [`exact_kind`'s ADR-002](../../../exact_kind/docs/decisions/002_explicit_rescale_between_scales.md) — `rescale`, the explicit crossing between scales
- [`exact_kind`'s ADR-003](../../../exact_kind/docs/decisions/003_price_generic_over_scale.md) — `PriceAt< S >`
- [`tests/any_scale_test.rs`](../../tests/any_scale_test.rs) — the operations exercised in cents and satoshis through this facade
