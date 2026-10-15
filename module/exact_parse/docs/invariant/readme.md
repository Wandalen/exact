# Invariant Doc Definition

### Scope

- **Purpose**: State the properties this crate holds regardless of caller behavior, so a consumer can rely on them without re-checking.
- **Responsibility**: The compile-time guard against `exact_kind`/`exact_scale` scale drift.
- **In Scope**: The one `const _ : ()` assertion this crate declares.
- **Out of Scope**: The parsing dispatch and grammar themselves (→ `../algorithm/`).

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [Cross-Crate Scale Consistency Is Compile-Time Enforced](001_scale_consistency_is_compile_time_enforced.md) | `exact_kind`'s money scale and `exact_scale::MONEY_SCALE` can never silently drift apart | 🔄 |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/module/exact_parse/docs/invariant
printf 'instances:              '; ls [0-9][0-9][0-9]_*.md | wc -l
printf 'rows in Overview Table: '; grep -E '^\| [0-9]{3} \|' readme.md | wc -l
# instances:              1
# rows in Overview Table: 1
```
