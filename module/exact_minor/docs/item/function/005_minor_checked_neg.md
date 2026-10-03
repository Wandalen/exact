# 005: minor_checked_neg

## Representation

Negate a count of minor units, refusing the one backing value that cannot
negate, `Backing::MIN`.

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_minor/src/lib.rs:270`

```rust
pub const fn minor_checked_neg( a : Minor ) -> Result< Minor, MinorError >
{
  match a.0.checked_neg()
  {
    Some( neg ) => Ok( Minor( neg ) ),
    None => Err( MinorError::Overflow { operation : "neg" } ),
  }
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 270-277 | Declaration |
| `tests/checked_arithmetic_test.rs` | throughout | Round-trip negation over `[0, 1, -1, Backing::MAX]`, plus the `Backing::MIN` refusal |
| `exact_arith/src/lib.rs:68` | — | Facade re-export only |

No file outside `exact_minor` calls `minor_checked_neg` directly —
`exact_kind`'s `Decimal::checked_neg` calls `i64::checked_neg` directly on
its own field instead.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_minor` | `(defining crate)` | Exercised by its own round-trip and boundary tests |
| `exact_arith` | `src/lib.rs` | Re-export only |

## Caller Tree

No caller anywhere, intra-crate or external — an honest empty tree.

## Callee Tree

- **External:** `i64::checked_neg` — `a.checked_neg()`
