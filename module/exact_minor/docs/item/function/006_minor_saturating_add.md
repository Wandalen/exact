# 006: minor_saturating_add

## Representation

Add two counts of minor units, clamping to the backing width's own bounds
rather than failing. New in this crate — the family's prior shape only ever
offered checked operations.

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_minor/src/lib.rs:138`

```rust
pub const fn minor_saturating_add( a : Backing, b : Backing ) -> Backing
{
  a.saturating_add( b )
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 138-141 | Declaration |
| `tests/saturating_arithmetic_test.rs` | throughout | In-range match with checked addition; clamping past `Backing::MAX` |
| `exact_arith/src/lib.rs:70` | — | Facade re-export only |

No file outside `exact_minor` calls `minor_saturating_add` directly. This is
not the same gap as the checked functions above: `exact_add`'s own
`money_saturating_add` clamps to the *declared ceiling* (`Money::MAX`/`MIN`)
on a failed checked add, a semantically different clamp target than this
function's raw `Backing::MAX`/`MIN` — so `exact_add` correctly does not call
this function rather than merely duplicating it.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_minor` | `(defining crate)` | Exercised by its own clamping tests |
| `exact_arith` | `src/lib.rs` | Re-export only |

## Caller Tree

No caller anywhere, intra-crate or external — an honest empty tree, with a
genuine architectural reason (see File Usage) rather than an oversight.

## Callee Tree

- **External:** `i64::saturating_add` — `a.saturating_add( b )`
