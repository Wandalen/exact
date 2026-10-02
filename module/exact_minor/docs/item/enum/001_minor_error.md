# 001: MinorError

## Representation

Why a checked operation could not be completed. One variant, not a separate
overflow/underflow split: `Backing`'s own `checked_add`/`checked_sub`/
`checked_neg` already report both directions of range failure the same way,
and a sign-based distinction those primitives do not make would be a check
with no observable behaviour behind it.

## Kind

Enum (§ Item Kind Taxonomy : Stable Item Kinds #7)

## Definition

`module/exact_minor/src/lib.rs:30`

```rust
pub enum MinorError
{
  Overflow { operation : &'static str },
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 30,40,44-47,51,72-78,86-92,101-107 | Return type / constructed variant of all 3 checked functions, and both trait impls it carries |
| `tests/checked_arithmetic_test.rs` | throughout | Matched by equality against the exact variant and `operation` string |
| `exact_arith/src/lib.rs:65` | — | Facade re-export only |

No file outside `exact_minor` and the facade re-export constructs, matches,
or renders a `MinorError` — an honest gap: neither `exact_sign` nor
`exact_kind` (the crate's only two real dependents) ever receives one, since
neither calls any of the 3 checked functions this error type is returned
from (see each function's own Caller Tree).

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_minor` | `(defining crate)` | Every checked operation's error type; exercised by its own tests |
| `exact_arith` | `src/lib.rs` | Re-export only |
