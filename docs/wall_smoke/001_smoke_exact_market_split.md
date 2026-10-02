# Wall Smoke: smoke_exact_market_split

### Scope

- **Purpose**: State the one proposed demo's full contract — name, preconditions, coverage claim, golden output, and pass criteria — as a single traceable unit, since Prompt 4's answer describes it as one thing even though its 10 steps are catalogued separately.
- **Responsibility**: Everything about the proposed demo except its individual steps.
- **In Scope**: Preconditions, what it forces, what it excludes, the golden print, and the pass criteria, exactly as proposed.
- **Out of Scope**: The 10 individual scene steps (→ `../scene/`); the per-crate surface it calls (→ `../type/`).

**Design status**: implemented, but substantially diverged. The real lane, [`smoke_exact_market_split`](../../module/smoke_exact_market_split/readme.md), keeps the proposed name and the "headless, one file, no seed" shape, but:

- runs 5 steps, not 10 — only 2 of the proposed Scene steps have any real counterpart at all (see `../scene/readme.md`)
- its central claim is a control arm that runs the exact path and `f64` side by side and **asserts the two disagree** — a different mechanism than this proposal's checked-add-overflow / wire-round-trip / checksum steps
- its market split folds dust to the first share (`DustTo::First`), not to a sink as proposed here
- its golden print (real text quoted in `../../module/smoke_exact_market_split/readme.md`) bears no resemblance to the one proposed below — different steps, different literals, a different shape entirely

### Preconditions

Headless. One file. No book, no wallets. Seed is irrelevant — all inputs are constants.

### What It Forces

Float-free storage, scale, kinds, checked add, ratio split, dust, snap, parse reject, overflow, bytes, exact equality.

### Not In This Smoke

i128, half-even display padding, 002's book.

### Golden Print (as proposed)

```text
sum=0
parts=3.33,3.33,3.33 dust=0.01
tick=1.25 lot=9
extra=1 overflow=1
wire=1000
a=0x… b=0x…
ok
```

### Pass Criteria (as proposed)

`sum=0`, dust line matches the fixture, `tick=1.25`, `lot=9`, `extra=1`, `overflow=1`, `wire=1000` (scale-2 minor of 10.00), `a==b`.

### Sources

| File | Relationship |
|------|--------------|
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:523` | Prompt 4's own framing: "One final demo..." |
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:525-527` | Name (`smoke_exact_market_split`) and preconditions |
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:540-544` | "What it forces" and "Not in this smoke" |
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:546-556` | Golden print block |
| `../../../../codename_space_sandbox/docs/codename_vm_ecs_export_1.md:558` | Pass criteria |
