# Wall Smoke Doc Definition

### Scope

- **Purpose**: Record the proposed single "wall smoke" demo for workstream 006 — the Prompt-4 answer in full (name, preconditions, coverage claim, golden print, pass criteria) — so the target demo's full shape traces back to its source conversation, not only its step list.
- **Responsibility**: The one proposed demo's identity, preconditions, "what it forces" / "not in this smoke" coverage claims, golden print, and pass criteria.
- **In Scope**: Everything Prompt 4's answer states about `smoke_exact_market_split` except the 10 individual scene steps.
- **Out of Scope**: The 10 individual scene steps themselves (→ `../scene/`); the 15-crate surface the demo exercises (→ `../crate/`, `../type/`).

**Design status**: implemented — the real lane, [`smoke_exact_market_split`](../../module/smoke_exact_market_split/readme.md), keeps the proposed name and shape, and its step 6 prints this proposal's golden print line for line with every pass criterion asserted, after five steps of its own (a parse/render round trip, an exact-vs-`f64` summation control arm, a `Quantity` below-zero refusal, a ledger conservation audit, and a 3-way market split). Two golden lines print a scale-6 number rather than the proposed scale-2 one, because `exact_dust` and `exact_bytes` take `Money`, fixed at scale 6. See the one instance below for the full account.

### Overview Table

| ID | Name | Purpose | Status |
|----|------|---------|--------|
| 001 | [smoke_exact_market_split](001_smoke_exact_market_split.md) | The workstream's one proposed wall-smoke demo | 🔄 |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/docs/wall_smoke
printf 'instances:              '; ls [0-9][0-9][0-9]_*.md | wc -l
printf 'rows in Overview Table: '; grep -E '^\| [0-9]{3} \|' readme.md | wc -l
# instances:              1
# rows in Overview Table: 1
```
