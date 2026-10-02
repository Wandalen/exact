# workaround

External constraints `exact_cmp` absorbs.

### Scope

- **Purpose**: Record every external constraint this crate compensates for, so each one carries a cost and a deletion condition.
- **Responsibility**: Document this crate's workarounds, or record explicitly that it has none.
- **In Scope**: Constraints originating outside this repository — the language, the toolchain, the targets, and any published (non-workspace) crate.
- **Out of Scope**: This crate's own design decisions, which are not workarounds however unusual they look; constraints compensated in shared tooling outside this crate; constraints a future dependency absorbs on its own behalf, to be documented in that crate's own `docs/workaround/`, not duplicated here.

### Overview

**None.**

`exact_cmp` depends on exactly one crate, `exact_kind`, and zero published
crates. Verify with:

```bash
# from this crate's root ( module/exact_cmp/ )
cargo tree --depth 1
```

**Expected:** `exact_kind` alone, resolving to a `(path = ...)` source.

### Sources

| File | Relationship |
|------|--------------|
| `Cargo.toml` | The dependency surface examined for this finding — `exact_kind` — the sole dependency, an in-workspace path dependency |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/module/exact_cmp
printf 'workaround instances:   '; ls docs/workaround/[0-9][0-9][0-9]_*.md 2>/dev/null | wc -l
printf 'declared dependencies:  '; cargo tree --depth 1 --edges normal 2>/dev/null | tail -n +2 | wc -l
# workaround instances:   0
# declared dependencies:  1
```
