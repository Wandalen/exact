# 001: sign_of

## Representation

Classifies a backing value's sign.

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_sign/src/lib.rs:39`

```rust
pub const fn sign_of( value : Backing ) -> Sign
{
  if value < 0
  {
    Sign::Neg
  }
  else if value == 0
  {
    Sign::Zero
  }
  else
  {
    Sign::Pos
  }
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 39,59,66 | Declaration; `sign_is_negative`/`sign_is_zero`'s delegation |
| `tests/sign_classification_test.rs:7-12` | — | All 3 classifications |
| `exact_arith/src/lib.rs:86` | — | Facade re-export |
| `exact_arith/tests/facade_test.rs:57` | — | `sign_of( -5 )` via the re-exported path |

No production (non-test) file outside `exact_sign` calls `sign_of` directly
— `exact_add`, the crate's one production consumer, calls
[`sign_is_negative`](002_sign_is_negative.md) instead.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_sign` | `(defining crate)` | Backs `sign_is_negative`/`sign_is_zero`; exercised by its own tests |
| `exact_arith` | `src/lib.rs`, `tests/facade_test.rs` | Re-export; test-only direct call |

## Caller Tree

- [sign_is_negative](002_sign_is_negative.md) (`src/lib.rs:59`)
- [sign_is_zero](003_sign_is_zero.md) (`src/lib.rs:66`)
- **External:** `exact_arith`'s own test (`tests/facade_test.rs:57`) — test-context, not production

No production external caller.

## Callee Tree

- None — a pure three-way comparison, no function call.
