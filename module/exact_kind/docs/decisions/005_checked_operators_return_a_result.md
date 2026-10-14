# ADR-005: `+` And `-` Return A `Result`

**Date**: 2026-10-13
**Status**: Proposed
**Deciders**: ihortry, for wandalen's review

## Context

Adding money takes a named call: `money_add( a, b )?` or
`a.checked_add( b )?`, and a sum of three is
`money_add( money_add( a, b )?, c )?`. The family's crate proposal says
checked add and sub are "named explicitly rather than left to operator
overloading" ([`exact_add`'s crate page](../../../../docs/crate/006_exact_add.md)),
because the usual `+` returns a plain value and must hide an overflow — panic
or wrap — where a sum past the ceiling is a normal case the caller has to
handle.

`core::ops::Add` does not require a plain value: its `Output` can be any type,
including a `Result`. So `+` can keep the checked behaviour and still read
like arithmetic.

## Decision

- **`a + b` and `a - b` on `Decimal`, `Qty` and `PriceAt`** are that kind's
  `a.checked_add( b )` and `a.checked_sub( b )`: the `Output` is
  `Result< Kind< SCALE >, KindError >`. Nothing panics and nothing wraps. A
  sum past the ceiling is `Err( ExceedsCeiling )`, and a quantity below zero
  is `Err( Negative )` — exactly what the checked methods return.
- **A chain needs one `?`.** `a + b` is a `Result`, so `+` and `-` are also
  implemented with a `Result< Kind< SCALE >, KindError >` on the left: an
  error there is passed on untouched (`self?`), otherwise the right side is
  added or subtracted. `( a + b + c )?` works left to right and stops at the
  first error, where `( ( a + b )? + c )?` was needed otherwise.
- **Same kind, same scale only.** Both sides are `Kind< SCALE >`, so
  `Decimal< 2 >` plus `Decimal< 8 >`, or a price plus a quantity, does not
  compile — the same rules as `checked_add`.
- **Written once, for every kind.** Four generic impls on `Amount< K, SCALE >`
  — the one struct `Decimal`, `Qty` and `PriceAt` are aliases of — each
  calling the kind's own `checked_add` or `checked_sub`
  (→ [ADR-006](006_one_amount_struct_generic_over_kind.md)). A fix to one impl
  reaches all three kinds at once.
- `money_add`/`qty_add`/`price_add`, their `_sub` twins and the `checked_*`
  methods stay. The operators add a spelling; they replace nothing.

## Alternatives Considered

### Option 1: Keep named calls only

What the crate proposal chose. Rejected: its reason — an operator must hide
the overflow — does not hold once `Output` is a `Result`, and the nested
`money_add( money_add( a, b )?, c )?` is the shape the named calls force on
every sum of more than two.

### Option 2: `+` returns a plain value and panics past the ceiling

How the standard integers behave in debug builds. Rejected: a panic in the
middle of a settlement is the failure `exact_add`'s
[ADR-002](../../../exact_add/docs/decisions/002_no_panicking_variant_yet.md)
rejects, and a sum past the ceiling is not a bug in the caller.

### Option 3: `+` clamps to the ceiling

`money_saturating_add` already does this, by name. Rejected for `+`: a
clamped sum is a different amount, changed silently.

### Option 4: Only `Kind + Kind`, no impl on `Result`

Smaller, but every chain then needs a `?` per step:
`( ( a + b )? + c )?`. The impls on `Result` are what make a chain read as
arithmetic.

### Option 5: `Decimal` only

Smaller: four impls on one kind. Rejected: quantities are summed too — the fills
of one order — and `-` on `Qty` is where a checked operator matters most,
since a quantity below zero is refused as `Negative`. One rule for all three
kinds keeps `+` from meaning something on money that it does not mean on a
quantity.

### Option 6: Twelve impls, four per kind

With three separate structs, `+` and `-` take four impls each — written out,
or written by a macro from one template. Rejected: twelve blocks that differ
in one word, where a fix made to one kind's blocks is easy to miss in
another's. One struct with the kind as a type parameter needs four
([ADR-006](006_one_amount_struct_generic_over_kind.md)).

### Option 7: `*` and `/`

Not done: they need a rounding mode, and an operator has no place for one.
`money_mul_ratio( a, r, mode )` and `money_div_round( a, n, mode )` stay.

## Consequences

**Positive:**
- `( a + b + c )?` and `( fill_1 + fill_2 + fill_3 )?` read like arithmetic
  and keep every check `checked_add` and `checked_sub` make.
- A sum past the ceiling, or a quantity below zero, is still impossible to
  ignore: the `Result` must be handled before the value can be used.
- The demo lane reads as its scenes do: `( a + b - total )` for scene 002,
  `held - taken` for a refused withdrawal, and `| a, b | a + b` to recombine
  a split.

**Negative:**
- `a + b` is a `Result`, not a number, which surprises a reader expecting the
  usual `+`. `( a + b )?` is the price.
- `a + ( b + c )` does not compile: only left-to-right chains have an impl.
- The impls are generic over the kind, so finding what `+` does for one kind
  means following its private hook to that kind's own `checked_add`.
- `docs/crate/006_exact_add.md` says arithmetic is not left to operator
  overloading. Its text is the original proposal and is not edited; its
  Design status line records this decision.

**Neutral:**
- No new dependency, no new error variant: the operators only call the
  existing checked methods. `exact_arith` re-exports the three types, and the
  impls come with them.

## Related

- [ADR-006](006_one_amount_struct_generic_over_kind.md) — how the impls are written once, as four generic impls on `Amount`: [`+`](../item/implementation/008_add_for_amount.md), [`+` on a `Result`](../item/implementation/009_add_for_result.md), [`-`](../item/implementation/010_sub_for_amount.md), [`-` on a `Result`](../item/implementation/011_sub_for_result.md)
- [`exact_add`'s ADR-002](../../../exact_add/docs/decisions/002_no_panicking_variant_yet.md) — no panicking arithmetic; these operators do not panic
