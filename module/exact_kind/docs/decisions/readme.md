# Decisions Doc Definition

### Scope

- **Purpose**: Record genuine judgment calls behind this crate's design, so a later reader finds the rejected alternatives instead of re-litigating them.
- **Responsibility**: ADR-style records — the question, the options considered, the choice, and why.
- **In Scope**: Decisions about this crate's own public surface.
- **Out of Scope**: Decisions closed in another crate (→ that crate's own `decisions/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Non-Negativity As A Separate Type, Enforced At Construction](001_non_negativity_enforced_at_construction.md) | Why `Qty` is a distinct wrapper rather than a flag on `Decimal`, and how `KindError::Negative` enforces it today | 🔄 |
| 002 | [Crossing Scales Is An Explicit, Rounded `rescale`](002_explicit_rescale_between_scales.md) | Why scale conversion is a method taking a `Rounding`, and why `exact_kind` now depends on `exact_round` | 🔄 |
| 003 | [A Price Carries Its Own Scale, As `PriceAt< SCALE >`](003_price_generic_over_scale.md) | Why `Price` became an alias of a generic struct rather than a struct with a default scale | 🔄 |
| 004 | [A Requested Display Precision Rounds Half To Even](004_display_precision_rounds_half_to_even.md) | Why `{:.N}` pads with zeros or rounds half to even, and how `Qty`/`PriceAt` pass it through | 🔄 |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/module/exact_kind/docs/decisions
printf 'instances:              '; ls [0-9][0-9][0-9]_*.md | wc -l
printf 'rows in Overview Table: '; grep -E '^\| [0-9]{3} \|' readme.md | wc -l
# instances:              4
# rows in Overview Table: 4
```
