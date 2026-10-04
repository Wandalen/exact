# Feature: Exact Arith v0.1

### Scope

- **Purpose**: Fix what this facade's re-export surface covers — the full value substrate's 14 leaf crates — and record, against the family's original scope commitments, which now actually hold.
- **Responsibility**: State the facade's own version scope, deliberate exclusions, and exit criteria, with pointers to the leaf crate that actually realises each commitment.
- **In Scope**: The full 14-crate value substrate as re-exported here — the conserved value type family, the checked-operation contract (including the rounding/overflow policy, now decided), conservation-exact splitting, the audit deliverable, and wire serialization.
- **Out of Scope**: Any arithmetic semantics this facade does not itself implement — owned by whichever leaf crate declares it (→ [`docs/definition/readme.md`](../definition/readme.md)); game-side pricing, currency design, and market content — a downstream consumer's domain.

### Design

This facade's own contract in one line: **re-export the full value substrate
as a single dependency; declare nothing new.** The family behind it keeps the
contract this doc originally stated: fixed-point/decimal types with exact
checked operations, audited by a conservation auditor that consumes a plain
transaction log. The corpus bans floats for balances outright; this family is
where that ban becomes types the compiler enforces — now assembled from 14
single-concern crates instead of the original 3
(`exact_decimal`/`exact_qty`/`exact_audit`).

**Commitments:**

- **Exact types for everything conserved.** Fixed-point decimal types for
  money and commodity quantities, with no rounding drift in their
  representation — met. `exact_kind::Decimal<SCALE>` (aliased `Money`, and
  wrapped by `Price`) and `exact_kind::Qty<SCALE>` (aliased `Quantity`) store
  one `exact_minor::Minor` (an `i64`) and never pass through a float, including in
  their parser's grammar, which explicitly refuses `"1e6"`/`"NaN"`/`"inf"`
  rather than rounding them
  (→ [`exact_minor`: No Float In Representation](../../../exact_minor/docs/invariant/001_no_float_in_representation.md),
  [`exact_kind`: No Float In The Public Constructor Surface](../../../exact_kind/docs/invariant/001_no_float_in_the_public_constructor_surface.md),
  realised by [`exact_kind`: Conserved Value Type Family](../../../exact_kind/docs/type/001_conserved_value_type_family.md)).
  Energy, named in the original scope, has no crate in the current 14 —
  dropped rather than deferred; nothing in this family's source or tests
  names it.
- **Checked operations, totally.** Every arithmetic operation either returns
  the mathematically exact result or an explicit error — met at both the raw
  `Backing` level and the typed level
  (→ [`exact_minor`: Checked Operations Total](../../../exact_minor/docs/invariant/002_checked_operations_total.md),
  [`exact_kind`: Checked Operations Total](../../../exact_kind/docs/invariant/002_checked_operations_total.md)).
  The family also ships explicitly-named `saturating_*` siblings
  (`minor_saturating_add`, `money_saturating_add`, `qty_saturating_add`, …) —
  this does not weaken the commitment, which bans *silent* wrap/saturate/round
  on the checked path, not the existence of a separately-named function a
  caller must opt into by name.
- **Conservation-exact splitting.** Dividing a quantity produces parts that
  sum back to the whole with zero remainder slack, whatever rounding the
  division itself required — met, and proved rather than merely tested: see
  Exit Criterion 2 below.
- **The conservation auditor.** A library procedure consuming a plain
  transaction log and proving the books balance — met for balance detection,
  narrower than originally specified for fault localisation: see Exit
  Criterion 3 below. Carried forward from the retired `exact_audit` as
  `exact_conserve`, which adds a typed per-kind convenience layer
  (`money_conserve_into`, `money_sum_assert_zero`, …) on top of the original
  plain-log `Entry`/`Report`/`verify` triple, unchanged in behaviour.
- **Serialization stable across platforms.** The byte form of every exact
  type is identical on every target — met: see Exit Criterion 1 below
  (→ [`exact_bytes`: Wire Record Encoding](../../../exact_bytes/docs/format/001_wire_record_encoding.md)).

**Now decided** (named but deliberately left open in the original v0.1
scope): *which* rounding and overflow policy each type declares, and the
backing width and decimal scale, are no longer TBD.
[`exact_round::Rounding`](../../../exact_round/src/lib.rs) (`Down | Up |
HalfEven`, with its own `rounding_default()`) is the one rounding-mode type
the whole family shares — `exact_ratio`, `exact_dust`, and `exact_snap` each
drive `exact_round::round_div` through it rather than rolling their own. The
backing width is `i64` (`exact_minor::Backing`); the decimal scale is `6`
digits (`exact_scale::MONEY_SCALE`), with a `9_000_000_000`-whole-unit
ceiling and a `1000`×`HEADROOM_FACTOR` of headroom kept clear of `i64`'s own
limit
(→ [`exact_scale`: Representable Range and Headroom](../../../exact_scale/docs/non_functional_requirement/001_representable_range_and_headroom.md)
for the budget arithmetic these constants had to close, and still do).

**Deliberate exclusions:**

- Float interop for conserved values — still true for this facade and all 14
  leaves it re-exports: no lossy constructor, no `as f64` escape hatch, and
  `Display` is the one sanctioned float-free rendering path. `f64` appears
  exactly once in the whole family, deliberately, as the required-to-disagree
  control arm in a different crate
  (→ [`smoke_exact_market_split`](../../../smoke_exact_market_split/readme.md)'s
  own `f64 appears in this crate and in no other crate of the family`).
- General-purpose bignum or computer-algebra ambitions — still true; no such
  code exists anywhere in the 14 leaves or this facade.
- Game-side pricing, currency design, and market content — still true, a
  downstream consumer's domain.

**Exit criteria** — the original v0.1 scope named three. Checked against the
real, current 14-crate source rather than carried forward unexamined:

1. ~~The type family round-trips serialization byte-identically across the
   workspace's targets, with every operation checked per the invariant.~~
   **Met.** `exact_bytes::Wire` fixes every multi-byte field little-endian
   (`to_le_bytes`/`from_le_bytes`), which has no native-endian path to diverge
   across targets — byte-identity holds by construction, not merely by test.
   `wire_roundtrip_test.rs` confirms per-kind (`Money`/`Quantity`/`Price`)
   fidelity including negative values, through both the `Wire` struct and raw
   bytes, and every documented failure mode (`BadKind`, `BadScale`,
   `Truncated`, `Overflow`, `Negative`) is refused rather than silently
   accepted. Every `*_from_wire` function returns `Result<_, WireError>`; only
   `Wire::new`/`to_bytes` are infallible, and neither carries a per-kind
   invariant left unchecked.
2. ~~Splitting property-tested: for arbitrary quantities and split counts, the
   parts sum to the whole exactly.~~ **Met, by proof rather than generated-input
   property testing.** No `proptest`/`quickcheck` dependency exists anywhere
   in this family (checked by grep across every `Cargo.toml` and source
   file). Instead, `exact_dust` derives `share * parts + leftover ==
   total_minor` as an identity — `leftover` is *defined* as that difference,
   never accumulated from independent per-share rounding error — so the total
   is unconditionally accounted for under every `Rounding` mode and every
   `DustTo` destination
   (→ [`exact_dust`: Equal-Parts Dust Split](../../../exact_dust/docs/algorithm/001_equal_parts_dust_split.md)'s
   own "Property" section). Representative-case tests cover all three
   `Rounding` modes and all three `DustTo` destinations, plus the
   `Quantity`-only negative-slot refusal under `Up` rounding; this facade's
   own `facade_test.rs` and `smoke_exact_market_split`'s `lane_test.rs` each
   independently re-confirm recombination end-to-end through the facade
   alone.
3. ~~The auditor proves balance on a known-good transaction log and pinpoints
   the offending transaction on a known-corrupted one.~~ **Partially met —
   narrower than originally specified, by a disclosed decision rather than an
   oversight.** `exact_conserve::verify` proves balance exactly
   (`Report::is_balanced`, zero-tolerance equality) and names a discrepancy's
   signed magnitude (`discrepancy_minor`) — `conservation_test.rs`'s
   million-unit-turnover case keeps one leaked minor unit visible. It does
   **not** pinpoint which entry caused a discrepancy: `Report` carries only an
   aggregate `net_minor` over the whole log. The predecessor `exact_audit`'s
   own position-tracking `AccumulatorOverflow { at_entry }` field was dropped
   rather than carried forward — a disclosed deviation in `exact_conserve`'s
   own module doc, which states plainly that a caller needing to bisect a
   failing log must do so externally. The "proves balance" half is met; the
   "pinpoints" half was deliberately descoped during the migration.

### Invariants

| File | Relationship |
|------|--------------|
| [Facade Re-Exports Only](../invariant/001_facade_re_exports_only.md) | This crate's own restraint — the mechanism that keeps the facade from growing a 15th implementation |
| [`exact_minor`: No Float In Representation](../../../exact_minor/docs/invariant/001_no_float_in_representation.md) | The float ban's foundation — the backing integer itself |
| [`exact_kind`: No Float In The Public Constructor Surface](../../../exact_kind/docs/invariant/001_no_float_in_the_public_constructor_surface.md) | The float ban at the surface a caller actually touches |
| [`exact_minor`: Checked Operations Total](../../../exact_minor/docs/invariant/002_checked_operations_total.md) | The checked-operation contract's foundation |
| [`exact_kind`: Checked Operations Total](../../../exact_kind/docs/invariant/002_checked_operations_total.md) | The checked-operation contract at the typed level |

### Algorithms

| File | Relationship |
|------|--------------|
| [`exact_dust`: Equal-Parts Dust Split](../../../exact_dust/docs/algorithm/001_equal_parts_dust_split.md) | Conservation-exact splitting — Exit Criterion 2 |

### Formats

| File | Relationship |
|------|--------------|
| [`exact_bytes`: Wire Record Encoding](../../../exact_bytes/docs/format/001_wire_record_encoding.md) | The cross-platform serialization commitment — Exit Criterion 1 |

### Non-Functional Requirements

| File | Relationship |
|------|--------------|
| [`exact_scale`: Representable Range and Headroom](../../../exact_scale/docs/non_functional_requirement/001_representable_range_and_headroom.md) | The budget arithmetic the backing width and scale had to close, and now do |

### Types

| File | Relationship |
|------|--------------|
| [`exact_kind`: Conserved Value Type Family](../../../exact_kind/docs/type/001_conserved_value_type_family.md) | The family realising commitment one |

### Sources

| File | Relationship |
|------|--------------|
| `../../src/lib.rs` | Crate root; implemented — the `pub use` facade over all 14 leaves (→ [`../readme.md`](../readme.md)'s family table, and [`../definition/readme.md`](../definition/readme.md) for every individual item) |
| `../../../exact_conserve/src/lib.rs` | The conservation auditor this facade re-exports — no `docs/algorithm/` instance exists yet for that crate, so cited directly rather than linked |

### Tests

| File | Relationship |
|------|--------------|
| `../../tests/facade_test.rs` | Implemented — an end-to-end settlement through the facade alone, a representative re-exported name from each tier resolving to its own tier crate's item, the facade-purity check, and the single-declaration check for `Backing` |
| `../../../exact_conserve/tests/conservation_test.rs` | Exit Criterion 3's "proves balance" half, including the million-unit-turnover case; no test exercises "pinpoints the offending transaction" because the auditor does not implement it |
