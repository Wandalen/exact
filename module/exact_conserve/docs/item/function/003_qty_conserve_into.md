# 003: qty_conserve_into

## Representation

Fold one more quantity leg into a running total via `exact_add`'s own
checked arithmetic. Named and shaped for `Iterator::try_fold`.

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_conserve/src/lib.rs:234-237`

```rust
pub fn qty_conserve_into( acc : Quantity, leg : Quantity ) -> Result< Quantity, ConservationError >
{
  exact_add::qty_add( acc, leg ).map_err( kind_error_to_conservation_error )
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 234-237 | Declaration |
| `tests/conservation_test.rs:137-143` | — | `try_fold` usage |
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

- **External:** `exact_add::qty_add` (`exact_add/src/lib.rs:55`)
- `kind_error_to_conservation_error` (`src/lib.rs:134`, private — no Item Instance of its own)
