# 002: rounding_name

## Representation

A stable, human-readable name for a rounding mode, for diagnostics and logs
only — never parsed back into a `Rounding`, which is why there is no
corresponding `rounding_from_name`.

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_round/src/lib.rs:72`

```rust
pub const fn rounding_name( rounding : Rounding ) -> &'static str
{
  match rounding
  {
    Rounding::Down => "down",
    Rounding::Up => "up",
    Rounding::HalfEven => "half_even",
  }
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 72 | Declaration |
| `tests/rounding_mode_test.rs:14-17` | — | All 3 names checked |
| `exact_arith/src/lib.rs:80` | — | Facade re-export |

No file anywhere — production or test, in `exact_round` or in any downstream
consumer — calls `rounding_name` outside its own direct-mapping test. An
honest empty finding: no diagnostic or logging call site exists yet anywhere
in the family that would need this name.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_round` | `(defining crate)` | Exercised by its own name-mapping test |
| `exact_arith` | `src/lib.rs` | Re-export only |

## Caller Tree

No caller anywhere, intra-crate or external — an honest empty tree.

## Callee Tree

- None — a pure match returning a `&'static str` literal per arm.
