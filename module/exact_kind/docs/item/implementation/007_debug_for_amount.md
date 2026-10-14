# 007: impl Debug for Amount

## Representation

Prints an amount under its kind's own name — `Decimal { minor: Minor(12340000) }`,
`Qty { minor: Minor(50000000) }`, `PriceAt { minor: Minor(-125) }`. For
`Decimal` that is its output as a struct of its own, unchanged; `Qty` and
`PriceAt`, which were wrappers, printed `Qty { value: Decimal { minor: … } }`
and now print one level, without the nested `Decimal`. Written by hand rather than
derived: a derived `Debug` would print `Amount { minor: …, kind: PhantomData<…> }`,
hiding the kind in a type name. The name is the kind's private `Sealed::NAME`,
which is why the bound is `K : Kind`
(→ [ADR-006](../../decisions/006_one_amount_struct_generic_over_kind.md)).

## Kind

Implementation (§ Item Kind Taxonomy : Stable Item Kinds #12)

## Definition

`module/exact_kind/src/lib.rs:231`

```rust
impl< K : Kind, const SCALE : u32 > fmt::Debug for Amount< K, SCALE >
{
  fn fmt( &self, f : &mut fmt::Formatter< '_ > ) -> fmt::Result
  {
    f.debug_struct( K::NAME ).field( "minor", &self.minor ).finish()
  }
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 228-237 | Declaration |
| `tests/amount_test.rs` | 37-39 | The exact text for `Money`, `Qty< 8 >` and `PriceAt< 2 >` |
| `tests/operator_test.rs` | 31 | `{past:?}` in an assertion message |
| `tests/*.rs` | throughout | Every `assert_eq!` on an amount needs `Debug`, to print both sides when it fails |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | Exercised by `tests/amount_test.rs`; required by every `assert_eq!` on an amount |

Every crate that compares amounts with `assert_eq!` in its tests needs this
`Debug` too, as it needed each struct's derived one before; none depends on
the printed text.
