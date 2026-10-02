# 008: money_saturating_add

## Representation

Add two money values, clamping to the declared ceiling instead of refusing.
Clamps to `Money::MAX`/`Money::MIN` — the declared ceiling, not the raw
backing width — because a `Money` value is only ever constructible inside
that ceiling in the first place. The clamp direction is `b`'s sign: when
`checked_add` fails, `a` and `b` necessarily share a sign (operands of
opposite sign can never overflow a sum), so `b`'s sign is also the true
mathematical sum's sign — this is why the function consults
[`exact_sign::is_negative`](../../../../exact_sign/docs/item/function/002_is_negative.md)
rather than re-deriving the sign some other way.

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_add/src/lib.rs:110`

```rust
pub const fn money_saturating_add( a : Money, b : Money ) -> Money
{
  match a.checked_add( b )
  {
    Ok( sum ) => sum,
    Err( _ ) => if exact_sign::is_negative( b.minor() ) { Money::MIN } else { Money::MAX },
  }
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 110,115 | Declaration; its own clamp-direction decision |
| `tests/checked_and_saturating_add_test.rs:60-61,70` | — | Clamps at both signs; matches checked addition in range |
| `exact_arith/src/lib.rs:87` | — | Facade re-export |

No production (non-test) file outside `exact_add` calls
`money_saturating_add` — an honest empty finding, and notably the facade's
own "runs end-to-end through the facade alone" test (`exact_arith/tests/facade_test.rs`)
does not reach for it either, despite re-exporting it (see [readme](../readme.md)
Notable Findings).

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_add` | `(defining crate)` | Exercised by its own clamp-boundary and in-range-parity tests |
| `exact_arith` | `src/lib.rs` | Re-export only |

## Caller Tree

No caller anywhere, intra-crate or external, outside its own tests — an
honest empty tree.

## Callee Tree

- **External:** `exact_kind::Decimal::checked_add` (via `a.checked_add( b )`)
- **External:** `exact_kind::Decimal::minor` (via `b.minor()`, to obtain the raw backing value `is_negative` takes)
- **External:** `exact_sign::is_negative` (via `exact_sign::is_negative( b.minor() )`, the crate's one production-reachable call into `exact_sign` — see [`exact_sign`'s own Caller Tree for `is_negative`](../../../../exact_sign/docs/item/function/002_is_negative.md))
