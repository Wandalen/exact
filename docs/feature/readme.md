# Feature Doc Definition

### Scope

- **Purpose**: Document the capabilities the preferred/target 15-crate decomposition specifies for workstream 006, so a future implementer knows what surface to build toward.
- **Responsibility**: The 22 features enumerated in `codename_vm_ecs_export_1.md`'s Prompt-1 answer for the target `exact_*` crate family.
- **In Scope**: Capability-level features — parsing, arithmetic, rounding, snapping, serialization, comparison, display.
- **Out of Scope**: Which crate owns each feature (→ `../crate/`); the per-crate struct/function surface (→ `../type/`); the hard problems these features solve (→ `../hard_problem/`).

**Design status**: realized across the real 15-crate family (`exact_minor`, `exact_scale`, `exact_kind`, `exact_sign`, `exact_round`, `exact_add`, `exact_ratio`, `exact_dust`, `exact_parse`, `exact_fmt`, `exact_bytes`, `exact_snap`, `exact_conserve`, `exact_cmp`, `exact_arith`) plus the `smoke_exact_market_split` demo lane, not the old 5-crate implementation (`exact_decimal`/`exact_qty`/`exact_audit`/`exact_arithmetic`/`smoke_exact_arithmetic`) this corpus originally named — see each instance's own Design status line for which real crate(s) realize it and whether it matches the proposal or deviates. Not every feature maps onto a built capability: [Feature 019](019_scale_convert_explicit.md) (explicit scale conversion) was superseded by a compile-time guarantee instead, and [Feature 022](022_bench_note_vs_f64.md) (a documented `f64` benchmark) was never built.

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Minor As I64 With I128 Feature](001_minor_as_i64_with_i128_feature.md) | Raw subunit type, `i128` gated behind a feature flag | 🔄 |
| 002 | [Scale Digits After Point](002_scale_digits_after_point.md) | `u8` count of fractional digits carried as data | 🔄 |
| 003 | [Newtypes Money Qty Price](003_newtypes_money_qty_price.md) | Three distinct kinds instead of one bare integer | 🔄 |
| 004 | [From Minor To Minor Conversion](004_from_minor_to_minor_conversion.md) | Explicit conversion in and out of the raw subunit | 🔄 |
| 005 | [Exact From Str Parsing](005_exact_from_str_parsing.md) | Decimal-string parsing that rejects extra digits | 🔄 |
| 006 | [Checked Add And Sub](006_checked_add_and_sub.md) | Addition/subtraction that fails instead of wrapping | 🔄 |
| 007 | [Mul Ratio Checked](007_mul_ratio_checked.md) | Checked multiply-by-ratio for fee/split math | 🔄 |
| 008 | [Rounding Mode Enum](008_rounding_mode_enum.md) | Rounding mode as data: down, up, half-even | 🔄 |
| 009 | [Div Round With Mode](009_div_round_with_mode.md) | Division that takes an explicit rounding mode | 🔄 |
| 010 | [Remainder Assign Dust Destination](010_remainder_assign_dust_destination.md) | A named destination for the division remainder | 🔄 |
| 011 | [Exact Eq And Ord No Epsilon](011_exact_eq_and_ord_no_epsilon.md) | Bit-exact equality and ordering, no tolerance window | 🔄 |
| 012 | [Checked Saturating Panicking Variants](012_checked_saturating_panicking_variants.md) | Three named overflow policies per operation | 🔄 |
| 013 | [Sum Assert Zero](013_sum_assert_zero.md) | Fold a slice of legs and assert conservation | 🔄 |
| 014 | [Conservation Error](014_conservation_error.md) | A typed error for a non-zero conservation fold | 🔄 |
| 015 | [Bytes Minor Plus Scale](015_bytes_minor_plus_scale.md) | Wire encoding as integer plus scale, never float | 🔄 |
| 016 | [Display With Scale](016_display_with_scale.md) | Human-readable printing driven by the type's scale | 🔄 |
| 017 | [Snap Tick Snap Lot](017_snap_tick_snap_lot.md) | Snap a price/quantity onto an instrument grid | 🔄 |
| 018 | [Zero Is Zero Is Negative Sign](018_zero_is_zero_is_negative_sign.md) | Sign and zero queries as first-class operations | 🔄 |
| 019 | [Scale Convert Explicit](019_scale_convert_explicit.md) | Cross-scale conversion only through an explicit call | 🔄 |
| 020 | [Reject Non Finite Extra Digits](020_reject_non_finite_extra_digits.md) | Parse-time rejection of malformed numeric strings | 🔄 |
| 021 | [Neg Only Where Kind Allows Debt](021_neg_only_where_kind_allows_debt.md) | Negation gated per-kind, not universally available | 🔄 |
| 022 | [Bench Note Vs F64](022_bench_note_vs_f64.md) | A documented performance comparison against `f64` | 🔄 |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/docs/feature
printf 'instances:              '; ls [0-9][0-9][0-9]_*.md | wc -l
printf 'rows in Overview Table: '; grep -E '^\| [0-9]{3} \|' readme.md | wc -l
# instances:              22
# rows in Overview Table: 22
```
