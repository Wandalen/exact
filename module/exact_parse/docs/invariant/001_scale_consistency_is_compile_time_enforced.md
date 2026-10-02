# Invariant: Cross-Crate Scale Consistency Is Compile-Time Enforced

### Scope

- **Purpose**: State that `exact_kind`'s money scale and `exact_scale`'s own `MONEY_SCALE` constant can never silently drift apart, so a parse here can be trusted to use the same scale every other crate in the family assumes.
- **Responsibility**: The compile-time assertion linking `Money::ONE_MINOR` to `exact_scale::pow10(exact_scale::MONEY_SCALE)`.
- **In Scope**: The one `const _ : ()` assertion this crate declares.
- **Out of Scope**: The parsing dispatch itself, and the grammar it inherits from `exact_kind` (→ `../algorithm/`); `exact_scale`'s own range/headroom guarantee, which this assertion depends on but does not restate (→ [`exact_scale`'s own `invariant/`](../../../exact_scale/docs/invariant/readme.md)).

### Statement

`src/lib.rs:36` declares `const _ : () = assert!( Money::ONE_MINOR ==
exact_scale::pow10( exact_scale::MONEY_SCALE ) );`. `Money`'s scale (fixed by
its own const-generic parameter in `exact_kind`) and `exact_scale::MONEY_SCALE`
are declared in two different crates, connected only by each site
independently writing the literal `6`. This assertion is checked at compile
time, not merely asserted in prose — a build of this crate fails outright the
moment the two literals diverge.

### Rationale

Two constants declared in separate crates, agreeing only because each author
independently wrote the same literal, is exactly the kind of coincidental
agreement that erodes silently: a future edit to either `exact_kind`'s own
`MONEY_SCALE`-equivalent or `exact_scale::MONEY_SCALE` has no mechanism
forcing the other to notice, short of this check. Without it, a drift would
surface only as a confusing, far-downstream symptom — values parsing to a
plausible-looking but wrong magnitude — with no error message pointing at
the actual cause. Placing the check here, at the one real call site where
both constants are already in scope together, catches the drift at the
earliest possible point: a build failure in this crate, naming exactly the
two values that disagree.

### Sources

| File | Relationship |
|------|--------------|
| `src/lib.rs:31-36` | The comment explaining the drift risk, and the compile-time assertion itself |

### Tests

| File | Relationship |
|------|--------------|
| `tests/from_str_test.rs` | Parsing coverage that exercises the scale this assertion protects, indirectly — the assertion's own enforcement is compile-time and has no corresponding runtime test |
