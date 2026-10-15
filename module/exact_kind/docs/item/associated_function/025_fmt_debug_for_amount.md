# 025: Debug::fmt for Amount

## Representation

Writes `Name { minor: … }`, where `Name` is the kind's `Sealed::NAME` —
`Decimal`, `Qty` or `PriceAt` — and `minor` is the stored `Minor`, printed by
its own `Debug`. The zero-sized `kind` field is not printed.

## Kind

Associated Function/Method (§ Item Kind Taxonomy : Associated Item Kinds #1)

## Definition

`module/exact_kind/src/lib.rs:233`

```rust
fn fmt( &self, f : &mut fmt::Formatter< '_ > ) -> fmt::Result
{
  f.debug_struct( K::NAME ).field( "minor", &self.minor ).finish()
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 233-236 | Declaration |
| `tests/amount_test.rs` | 37-39 | `format!( "{:?}", … )` on all three kinds |
| `tests/operator_test.rs` | 31 | `{past:?}` |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | Exercised by `tests/amount_test.rs` |

## Caller Tree

- **Tests:** `debug_names_the_kind` (`tests/amount_test.rs:37-39`); any failing `assert_eq!` on an amount

No intra-crate caller.

## Callee Tree

- The private `Sealed::NAME` of `K` (`src/lib.rs:987`, `994` or `1001`)
- **External:** `fmt::Formatter::debug_struct`, `DebugStruct::field`, `DebugStruct::finish`; `Debug` for `exact_minor::Minor`
