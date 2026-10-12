# docs

Design documentation for `exact_kind`, as typed doc definitions.

| Directory | Responsibility |
|------|-----------------|
| `type/` | `Decimal`/`Qty`/`PriceAt` and the `Money`/`Quantity`/`Price` aliases built from them |
| `invariant/` | No float anywhere, every checked operation total, and every scale at most 18, at the decimal/kind level |
| `decisions/` | Why non-negativity is a separate wrapper type, why crossing scales is an explicit `rescale`, why a price carries its own scale, and how a display precision rounds |
| `algorithm/` | `Qty::checked_sub`'s exact below-zero procedure |
| `definition/` | Module Index — every definition in this crate, in one place |
| `workaround/` | External constraints this crate absorbs — none |
| `item/` | One page per declaration, with every file and crate that uses it |

This is Tier 1 of the family, depending on `exact_minor` for the backing
integer and `exact_scale` for the power-of-ten table and the declared
ceiling. It consolidates what the real codebase built as two separate
crates — `exact_decimal`'s signed `Decimal<SCALE>` and `exact_qty`'s
non-negative `Qty<SCALE>` — into the one crate the family's preferred design
names `exact_kind`, carrying both representations forward unchanged in
behaviour, plus `PriceAt< SCALE >` as its own type wrapping a `Decimal`, with `Price` its alias at the standard scale; see each
collection's own instances for the two disclosed deviations from that
preferred design (no `Scaled` trait, `KindError` with no `ScaleMismatch`).

`Decimal::parse`'s grammar and `Display`'s rendering are documented as part
of [Conserved Value Type Family](type/001_conserved_value_type_family.md)
rather than as their own `algorithm/` instance — the parsing logic is real
and tested, but not complex enough on its own to warrant a dedicated
procedure doc the way `Qty::checked_sub`'s branch structure does; a crate
carrying fewer collections than an imagined "complete set" is expected, not a
gap.
