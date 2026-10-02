# 007: minor_saturating_sub

## Representation

Subtract two counts of minor units, clamping to the backing width's own
bounds rather than failing.

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_minor/src/lib.rs:125`

```rust
pub const fn minor_saturating_sub( a : Backing, b : Backing ) -> Backing
{
  a.saturating_sub( b )
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 125-128 | Declaration |
| `tests/saturating_arithmetic_test.rs` | throughout | In-range match with checked subtraction; clamping past `Backing::MIN` |
| `exact_arith/src/lib.rs:71` | — | Facade re-export only |

No file outside `exact_minor` calls `minor_saturating_sub` directly — the
same architectural reason as
[minor_saturating_add](006_minor_saturating_add.md): `exact_add` clamps to
the declared ceiling, not the raw backing width.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_minor` | `(defining crate)` | Exercised by its own clamping tests |
| `exact_arith` | `src/lib.rs` | Re-export only |

## Caller Tree

No caller anywhere, intra-crate or external — an honest empty tree.

## Callee Tree

- **External:** `i64::saturating_sub` — `a.saturating_sub( b )`
