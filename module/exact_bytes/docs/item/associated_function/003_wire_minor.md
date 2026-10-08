# 003: Wire::minor

## Representation

The minor-unit count carried.

**Never actually called — a verified discrepancy worth stating plainly.**
All three `*_from_wire` functions read `w.minor` as a direct private-field
access (`src/lib.rs:212,233,251`) rather than through this accessor, since
they live in the same module and have that privilege. No test calls it
either — the test suite only ever asserts on `.kind()` (see
[Wire::kind](005_wire_kind.md)) or on the round-tripped `Money`/`Quantity`/
`Price` value as a whole. Confirmed via an exhaustive grep for `.minor(` and
`Wire::minor` across `exact_bytes` and `exact_arith`: the only `.minor()`
calls found resolve to `exact_kind::Money::minor`/`Quantity::minor`/
`Price::minor` on a *different* receiver type, never on a `Wire`.

## Kind

Associated Function/Method (§ Item Kind Taxonomy : Associated Item Kinds #1)

## Definition

`module/exact_bytes/src/lib.rs:148-151`

```rust
pub const fn minor( self ) -> i64
{
  self.minor
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 148-151 | Declaration |

No file anywhere calls this method — an honest empty finding, and the
sharpest one on this type: a public accessor that exists, compiles, and is
exported through the facade, but has zero call sites of any kind, test or
production, anywhere in the workspace.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_bytes` | `(defining crate)` | Declared but unused even by this crate's own tests |

## Caller Tree

No caller anywhere, intra-crate or external, test or production — an honest
empty tree.

## Callee Tree

No callees — returns the private `minor` field directly.
