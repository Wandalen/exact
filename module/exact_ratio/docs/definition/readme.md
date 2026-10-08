# Definition Doc Definition

### Scope

- **Purpose**: Index every public definition in this crate in one place, so a reader can find where something is declared without grepping.
- **Responsibility**: A flat module index — one row per public item, not a duplicate explanation of each (the typed doc-definitions above already carry the explanation).
- **In Scope**: Every `pub` item in `src/lib.rs`.
- **Out of Scope**: Rationale and invariants for any one item — link to the owning doc-definition instead of restating it here.

### Module Index

| Item | Kind | Declared | Documented in |
|------|------|----------|----------------|
| `RatioError` | enum | `src/lib.rs:51` | [Ratio Error Without Scale Mismatch Or Bad Rounding](../decisions/001_ratio_error_without_scale_mismatch_or_bad_rounding.md) |
| `RatioError`'s `Display` impl | trait impl | `src/lib.rs:65` | [Ratio Error Without Scale Mismatch Or Bad Rounding](../decisions/001_ratio_error_without_scale_mismatch_or_bad_rounding.md) |
| `Ratio` | struct | `src/lib.rs:92` | [Rational Multiplier](../type/001_rational_multiplier.md) |
| `Ratio::n` | fn | `src/lib.rs:102` | [Rational Multiplier](../type/001_rational_multiplier.md) |
| `Ratio::d` | fn | `src/lib.rs:109` | [Rational Multiplier](../type/001_rational_multiplier.md) |
| `ratio_new` | fn | `src/lib.rs:126` | [Rational Multiplier](../type/001_rational_multiplier.md) |
| `money_mul_ratio` | fn | `src/lib.rs:162` | [Widened Multiply Before Narrow](../algorithm/001_widened_multiply_before_narrow.md) |
| `qty_mul_ratio` | fn | `src/lib.rs:175` | [Widened Multiply Before Narrow](../algorithm/001_widened_multiply_before_narrow.md) |
| `price_mul_ratio` | fn | `src/lib.rs:186` | [Widened Multiply Before Narrow](../algorithm/001_widened_multiply_before_narrow.md) |
| `money_div_round` | fn | `src/lib.rs:207` | — |
| `qty_div_round` | fn | `src/lib.rs:220` | — |
| `price_mul_qty` | fn | `src/lib.rs:236` | [Widened Multiply Before Narrow](../algorithm/001_widened_multiply_before_narrow.md) |

No numbered instance file in this directory — this index is the whole of
`definition/` for this crate.
