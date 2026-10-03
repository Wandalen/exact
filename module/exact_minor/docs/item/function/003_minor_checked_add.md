# 003: minor_checked_add

## Representation

Add two counts of minor units, refusing a sum that leaves the backing width.

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_minor/src/lib.rs:238`

```rust
pub const fn minor_checked_add( a : Minor, b : Minor ) -> Result< Minor, MinorError >
{
  match a.0.checked_add( b.0 )
  {
    Some( sum ) => Ok( Minor( sum ) ),
    None if b.0 > 0 => Err( MinorError::Overflow { operation : "add" } ),
    None => Err( MinorError::Underflow { operation : "add" } ),
  }
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 238-246 | Declaration |
| `tests/checked_arithmetic_test.rs` | throughout | Ordinary sum and `Backing::MAX`-boundary refusal |
| `exact_arith/src/lib.rs:67` | — | Facade re-export only |

No file outside `exact_minor` calls `minor_checked_add` directly — an honest
gap. `exact_kind`'s `Decimal::checked_add` performs the equivalent check by
calling `i64::checked_add` directly on its own stored field rather than
delegating here; the two implementations are independent, not layered.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_minor` | `(defining crate)` | Exercised by its own boundary tests |
| `exact_arith` | `src/lib.rs` | Re-export only |

## Caller Tree

No caller anywhere, intra-crate or external — an honest empty tree. Not an
omission: `exact_kind`, the family's one real arithmetic consumer, reimplements
this check inline on `i64` rather than depending on it (see
[Backing](../type_alias/001_backing.md)'s Representation).

## Callee Tree

- **External:** `i64::checked_add` — `a.checked_add( b )`
