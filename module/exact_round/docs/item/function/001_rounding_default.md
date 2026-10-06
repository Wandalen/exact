# 001: rounding_default

## Representation

The family's default rounding policy where a call site states none.
`HalfEven` — the only one of the three with no directional bias over a long
run, the property a conserved-value family needs most.

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_round/src/lib.rs:64`

```rust
pub const fn rounding_default() -> Rounding
{
  Rounding::HalfEven
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 64 | Declaration |
| `tests/rounding_mode_test.rs:9` | — | Confirms the default is `HalfEven` |
| `exact_arith/src/lib.rs:84` | — | Facade re-export |

No file anywhere — production or test, in `exact_round` or in any of its 4
downstream consumers — calls `rounding_default()` to actually obtain a
default; every call site that needs `HalfEven` writes the variant literally.
An honest empty finding: the function exists and is re-exported, but nothing
yet exercises it as a "give me the default" call, only as a direct-equality
check in its own test.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_round` | `(defining crate)` | Exercised by its own test as a direct-value check |
| `exact_arith` | `src/lib.rs` | Re-export only |

## Caller Tree

No caller anywhere, intra-crate or external — an honest empty tree, not an
omission (confirmed via grep across all of `module/`).

## Callee Tree

- None — a single enum-variant literal, no function call.
