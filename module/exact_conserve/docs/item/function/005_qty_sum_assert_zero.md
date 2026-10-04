# 005: qty_sum_assert_zero

## Representation

Assert a slice of quantity legs sums to exactly zero. Narrower in practice
than `money_sum_assert_zero`: every `Quantity` is individually non-negative,
so the sum can only be zero when every leg already is
`exact_kind::Qty::ZERO` (doc comment, `src/lib.rs:264-267`) — a real but
narrow check (e.g. "nothing left unaccounted after a full reconciliation"),
not a general credit/debit conservation check.

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_conserve/src/lib.rs:273-288`

```rust
pub fn qty_sum_assert_zero( legs : &[ Quantity ] ) -> Result< (), ConservationError >
{
  let mut net : i128 = 0;
  for leg in legs
  {
    net = net.checked_add( i128::from( leg.minor() ) ).ok_or( ConservationError::Overflow )?;
  }
  if net == 0
  {
    Ok( () )
  }
  else
  {
    Err( ConservationError::NotZero { got : net } )
  }
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 273-288 | Declaration |
| `tests/conservation_test.rs:166-173` | — | All-zero slice passes; a nonzero holding is refused |
| `exact_arith/src/lib.rs:129` | — | Facade re-export |

Confirmed via a full-workspace grep: no call site anywhere outside this
crate's own tests.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_conserve` | `(defining crate)` | Exercised by its own test |
| `exact_arith` | `src/lib.rs` | Re-export only — not called by the facade's own test suite |

## Caller Tree

No caller anywhere, intra-crate or external — an honest empty tree.

## Callee Tree

No callee of its own — folds via `i128::from`/`i128::checked_add` directly
and constructs [ConservationError](../enum/001_conservation_error.md) on
failure; no further hop into another Item Instance. Body is byte-for-byte
identical to [money_sum_assert_zero](004_money_sum_assert_zero.md)'s except
for the parameter type — the two do not share an implementation.
