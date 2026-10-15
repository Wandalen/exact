# ADR-006: One `Amount< K, SCALE >` Struct, Generic Over The Kind

**Date**: 2026-10-14
**Status**: Proposed
**Deciders**: ihortry, for wandalen's review

## Context

`Decimal< SCALE >`, `Qty< SCALE >` and `PriceAt< SCALE >` were three structs
with one shape: one count of minor units at one scale. `Qty` and `PriceAt`
each held a `Decimal` as a `value` field
([ADR-001](001_non_negativity_enforced_at_construction.md),
[ADR-003](003_price_generic_over_scale.md)).

A generic `impl` cannot cover three different structs, so `+` and `-`
([ADR-005](005_checked_operators_return_a_result.md)) on them would take
twelve `impl` blocks, four per kind, that differ only in the kind's name —
written out, or by a macro from one template. Rust has no
`impl< T > Add for T` — the orphan rule (E0210) refuses a blanket impl of a
foreign trait on any `T` — but it does accept a blanket impl over every
`K` of one local generic struct, `impl< K > Add for Amount< K, SCALE >`.

## Decision

- **One struct, the kind as a type parameter.**
  `pub struct Amount< K, const SCALE : u32 > { minor : Minor, kind : PhantomData< K > }`.
  `PhantomData` is zero-sized, so every amount is still one `i64`.
- **Three kinds, three zero-sized markers**: `MoneyKind`, `QtyKind`,
  `PriceKind`.
- **The old names stay, as aliases.**
  `pub type Decimal< const SCALE : u32 > = Amount< MoneyKind, SCALE >;`, and
  `Qty`/`PriceAt` the same with `QtyKind`/`PriceKind`. `Money`, `Quantity` and
  `Price` alias them as before. Every caller's code compiles unchanged.
- **Each kind keeps its own `impl` block.** `impl< const SCALE : u32 > Qty< SCALE >`
  is an inherent block on `Amount< QtyKind, SCALE >` alone, so `Qty` still has
  no `MIN` or `checked_neg`, `PriceAt` still has no `from_int` or `whole`, and
  money still has no `as_decimal`. Every method keeps its body. Where `Qty` and
  `PriceAt` read `self.value` they now call a private `value()`, and where
  they wrapped `Self { value }` they call a private `from_value( value )`.
- **`Qty::from_decimal` stays the choke point.** `from_value` is private and
  unchecked, so only this crate can build a `Qty` that has not passed
  `from_decimal`, and every `Qty` constructor still goes through it. ADR-001's
  rule — refused at construction, reported as `KindError::Negative` — holds as
  written. Only its shape changes: `Qty` is no longer "a distinct struct
  wrapping one `Decimal`", but `Amount` of its own kind.
- **`+` and `-`: four generic impls, not twelve.**
  `impl< K : Kind, const SCALE : u32 > Add for Amount< K, SCALE >`, its `Sub`
  twin, and the two impls with a `Result` on the left. Each calls
  `K::checked_add` or `K::checked_sub`: a per-kind hook that calls that
  kind's own inherent `checked_add`/`checked_sub`, so `-` on a `Qty` still
  refuses a value below zero. ADR-005's operators, `Output` types and chains
  are exactly as it decides them.
- **`Kind` is sealed.** `pub trait Kind : sealed::Sealed {}`, with `Sealed` in
  a private module. Outside this crate no one can add a fourth kind — one that
  brings its own `checked_add`, past the ceiling and the non-negativity
  check — and no one can call the per-kind hooks directly. `Sealed : Sized`
  because `Amount< Self, SCALE >` in the hooks needs `Self : Sized`, the bound
  every type parameter has by default. Two `compile_fail` doctests pin both.
- **`Kind` implies `Copy + Ord + Hash + Debug`.** `Amount`'s derives require
  `K` to have each trait they derive, so without these supertraits code
  generic over `K : Kind` could not copy, order or hash an `Amount< K, S >`.
  `Kind` is sealed, so the bound can change without breaking any caller.
- **`Debug` is written by hand.** A derived `Debug` would print
  `Amount { minor: …, kind: PhantomData<exact_kind::QtyKind> }`. The
  hand-written one prints the kind's name, from the hook's `NAME`, and its one
  count. That is `Decimal`'s output unchanged, but a new shape for the two
  kinds that were wrappers: `Qty { value: Decimal { minor: Minor(…) } }`
  becomes `Qty { minor: Minor(…) }`, and the same for `PriceAt`.
- `exact_arith` re-exports `Amount`, `Kind` and the three kinds with the rest
  of `exact_kind`'s public items.

## Alternatives Considered

### Option 1: Keep three structs, with a macro writing the twelve impls

Rejected: the twelve impls exist only because three structs repeat one
shape. A reader has to expand a template to see them, and a fourth operator
means a fourth block in the template, not one generic impl.

### Option 2: Three structs plus a trait with a blanket impl

`trait Checked { fn checked_add( … ) }`, implemented by each struct, then
`impl< T : Checked > Add for T`. Rejected: refused by the compiler. `Add` is
foreign and `T` is any type, so the orphan rule (E0210) applies, however
local the trait bound is.

### Option 3: One `Amount` with one shared `impl` block

Write `checked_add`, `parse` and the rest once, for every `K`. Rejected: the
kinds' rules differ. `Qty` must refuse a negative result, and `Qty` and
`PriceAt` must not gain `checked_neg`, `MIN`, `from_int` or `whole`. Hiding
those per kind would take as many branches as it removes, and every method
body would change. Keeping one `impl` block per kind keeps every body as it
was.

### Option 4: An open `Kind` trait

Simpler: no private module. Rejected: any crate could declare a kind and
write its own `checked_add` for it, and the kind hooks would be a second
public way to add, beside `checked_add` and `+`.

### Option 5: Derived `Debug`

One line less. Rejected: the kind is printed only as part of a type name, and
the output changes for every `{:?}` in every test and log.

## Consequences

**Positive:**
- `+` and `-` are four plain generic impls, read without expanding a macro.
- One shape, declared once: a change to the representation is made in one
  struct, not three.
- No caller changes. Every other crate in the family compiles unchanged, and
  all existing tests pass unchanged.
- The `Qty` choke point and the per-kind API are as they were; the seal
  keeps the set of kinds closed.

**Negative:**
- A compiler message or `rustdoc` now names `Amount< QtyKind, 8 >` where it
  named `Qty< 8 >`.
- One more public trait (`Kind`), three markers and a private module to read.
- `{:?}` of a `Qty` or a `PriceAt` changes shape: it no longer nests a
  `Decimal`. No code in the family parses `Debug` text, but a test or a log
  that compared it would see the new form.
- `Amount` is also the name of `exchange_order::Amount`, an alias of `Money`
  in the exchange. The two never meet in one scope today.
- `Qty` and `PriceAt` read `self.value()` and `Self::from_value( … )` instead
  of the `value` field.
- ADR-001 says `Qty` is "a distinct struct wrapping one `Decimal< SCALE >`
  field", and ADR-003 says the same of `PriceAt`. Their text is not edited:
  the rule each records still holds, and this decision replaces only the
  shape.

**Neutral:**
- No new dependency and no new error variant. The size of every type is
  unchanged at 8 bytes, pinned by `tests/amount_test.rs`.

## Related

- [ADR-001](001_non_negativity_enforced_at_construction.md) — the non-negativity rule, kept; its struct shape replaced
- [ADR-003](003_price_generic_over_scale.md) — `PriceAt< SCALE >`, kept as an alias
- [ADR-005](005_checked_operators_return_a_result.md) — `+` and `-` return a `Result`; this decision is how they are written once
- [Amount](../item/struct/001_amount.md), [Kind](../item/trait/001_kind.md) — the new items
