# 001: sign_of

## Representation

Classifies a backing value's sign.

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_sign/src/lib.rs:30`

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
| `src/lib.rs` | 30,50,57 | Declaration; `is_negative`/`is_zero`'s delegation |
| `tests/sign_classification_test.rs:7-12` | — | All 3 classifications |
| `exact_arith/src/lib.rs:79` | — | Facade re-export |
| `exact_arith/tests/facade_test.rs:57` | — | `sign_of( -5 )` via the re-exported path |

No production (non-test) file outside `exact_sign` calls `sign_of` directly
— `exact_add`, the crate's one production consumer, calls
[`is_negative`](002_is_negative.md) instead.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_sign` | `(defining crate)` | Backs `is_negative`/`is_zero`; exercised by its own tests |
| `exact_arith` | `src/lib.rs`, `tests/facade_test.rs` | Re-export; test-only direct call |

## Caller Tree

- [is_negative](002_is_negative.md) (`src/lib.rs:50`)
- [is_zero](003_is_zero.md) (`src/lib.rs:57`)
- **External:** `exact_arith`'s own test (`tests/facade_test.rs:57`) — test-context, not production

No production external caller.

## Callee Tree

- None — a pure three-way comparison, no function call.
