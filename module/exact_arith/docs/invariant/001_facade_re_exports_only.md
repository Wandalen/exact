# Invariant: Facade Re-Exports Only

### Scope

- **Purpose**: State that this crate declares no type, function, or constant of its own, so a consumer depending on the facade is guaranteed to get exactly the 14 leaf crates' own surface and nothing in addition that only this crate's own tests would catch a regression in.
- **Responsibility**: Every non-comment, non-blank line of `src/lib.rs` outside its module doc comment.
- **In Scope**: `fn`, `struct`, `enum`, `trait`, `impl`, `const`, `static`, `type`, and `macro_rules` items at this crate's own top level.
- **Out of Scope**: Whether any one re-exported item is itself well-designed — that is the owning leaf crate's own concern (→ [`docs/definition/readme.md`](../definition/readme.md) for which leaf owns which item).

### Statement

`exact_arith` contains no `fn`, `struct`, `enum`, `trait`, `impl`, `const`,
`static`, `type`, or `macro_rules` keyword outside a comment, anywhere in
`src/lib.rs`. Every non-comment, non-blank line is either a `pub use`
re-export or part of a `pub use`'s own brace-delimited item list. Every name a
consumer sees through this crate is the exact item one of the 14 leaf crates
declared — never a facade-local wrapper around it.

### Rationale

The local rulebook's facade exception
(`../../../../../rulebook.md § Architecture : Single Concern Per Crate`)
reads: *"a facade that starts implementing something has stopped being one."*
A single helper function added here — even a one-line wrapper like the
`exact_zero_money`/`exact_zero_qty`/`exact_zero_price` functions the family's
own preferred design once named as this crate's only self-owned code — would
be arithmetic living outside the tier crate whose own tests actually grade
it, and the first such helper is how a facade turns into an undocumented,
untested fifteenth implementation. `src/lib.rs`'s own module doc discloses
exactly this near-miss and why it was declined: each of those three functions
would have duplicated a constant this facade already re-exports (`Money::ZERO`,
`Quantity::ZERO`, `Price::ZERO`), so the restraint held and a caller reaches
the same constant through the re-exported type instead.

This also makes the facade safely re-splittable: because it holds no logic,
the 14-crate family behind it can be re-cut without any consumer of
`exact_arith` noticing — the same property this crate's own `readme.md`
states as the reason the facade exists at all.

### Sources

| File | Relationship |
|------|--------------|
| `src/lib.rs:1-64` | The module doc comment: states the no-logic contract and discloses the one near-miss (the three declined zero-constructor wrappers) |
| `src/lib.rs:66-141` | Every `pub use` block — the entire non-comment body of the crate |

### Tests

| File | Relationship |
|------|--------------|
| `tests/facade_test.rs` | `the_facade_source_is_re_exports_and_documentation_only` — reads this crate's own compiled source via `include_str!` and asserts none of `fn `, `struct `, `enum `, `trait `, `impl `, `const `, `static `, `type `, `macro_rules` appears outside a comment |
| `tests/facade_test.rs` | `the_backing_width_is_declared_in_exactly_one_place` — a narrower, related mechanical check: greps `exact_minor`, `exact_kind`, and this crate's own source for `pub type Backing`, asserting only `exact_minor` declares it — the same single-declaration discipline applied to one specific type rather than every keyword |
