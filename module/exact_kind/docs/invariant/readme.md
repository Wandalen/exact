# Invariant Doc Definition

### Scope

- **Purpose**: State the properties `Decimal`/`Qty` hold regardless of caller behavior, so a consumer can rely on them without re-checking.
- **Responsibility**: The float ban and the totality of every checked operation, at the decimal/kind level.
- **In Scope**: Representation-level and operation-level guarantees of `Decimal<SCALE>` and `Qty<SCALE>` themselves.
- **Out of Scope**: The same two properties one tier down, over the raw backing width (→ `exact_minor`'s own `invariant/`); non-negativity itself, which is a decision rather than a representation/operation-totality property (→ `decisions/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [No Float In The Public Constructor Surface](001_no_float_in_the_public_constructor_surface.md) | No `f32`/`f64` anywhere in `Decimal`/`Qty`'s construction, inspection, or rendering | 🔄 |
| 002 | [Checked Operations Total](002_checked_operations_total.md) | Every arithmetic method and constructor returns `Result<Self, KindError>` and never panics | 🔄 |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/module/exact_kind/docs/invariant
printf 'instances:              '; ls [0-9][0-9][0-9]_*.md | wc -l
printf 'rows in Overview Table: '; grep -E '^\| [0-9]{3} \|' readme.md | wc -l
# instances:              2
# rows in Overview Table: 2
```
