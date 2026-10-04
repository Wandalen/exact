# docs

Design documentation for `exact_kind`, as typed doc definitions.

| Directory | Responsibility |
|------|-----------------|
| `type/` | `Decimal`/`Qty`/`Price` and the `Money`/`Quantity` aliases built from them |
| `invariant/` | No float anywhere, and every checked operation total, at the decimal/kind level |
| `decisions/` | Why non-negativity is a separate wrapper type, enforced at construction |
| `algorithm/` | `Qty::checked_sub`'s exact below-zero procedure |
| `definition/` | Module Index — every definition in this crate, in one place |
| `workaround/` | External constraints this crate absorbs — none |

This is Tier 1 of the family, depending on `exact_minor` for the backing
integer and `exact_scale` for the power-of-ten table and the declared
ceiling. It consolidates what the real codebase built as two separate
crates — `exact_decimal`'s signed `Decimal<SCALE>` and `exact_qty`'s
non-negative `Qty<SCALE>` — into the one crate the family's preferred design
names `exact_kind`, carrying both representations forward unchanged in
behaviour; see each collection's own instances for the three disclosed
deviations from that preferred design (`Price == Money`, no `Scaled` trait,
`KindError` with no `ScaleMismatch`).

`Decimal::parse`'s grammar and `Display`'s rendering are documented as part
of [Conserved Value Type Family](type/001_conserved_value_type_family.md)
rather than as their own `algorithm/` instance — the parsing logic is real
and tested, but not complex enough on its own to warrant a dedicated
procedure doc the way `Qty::checked_sub`'s branch structure does; a crate
carrying fewer collections than an imagined "complete set" is expected, not a
gap.
