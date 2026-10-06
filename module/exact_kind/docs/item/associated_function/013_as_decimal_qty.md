# 013: Qty::as_decimal

## Representation

The decimal beneath, for arithmetic that legitimately leaves the type. Not a
hole in the invariant: what comes back is a signed decimal, and turning it
back into a `Qty` means passing through `from_decimal` again, which is where
the refusal lives.

## Kind

Associated Function/Method (§ Item Kind Taxonomy : Associated Item Kinds #1)

## Definition

`module/exact_kind/src/lib.rs:485`

```rust
pub const fn as_decimal( self ) -> Decimal< SCALE >
{
  self.value
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 485 | Declaration |
| `tests/non_negative_test.rs:149` | — | Extracts the inner decimal to negate it (testing that negating a non-negative-derived `Qty`'s decimal and re-wrapping correctly fails) |

No file outside `exact_kind` calls `Qty::as_decimal` — an honest empty
finding; no downstream crate currently needs to escape `Quantity` back to a
signed `Decimal`.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | Exercised once, by its own invariant-boundary test |

## Caller Tree

No caller anywhere, intra-crate or external — an honest empty tree, not an
omission.

## Callee Tree

- None — a pure field access.
