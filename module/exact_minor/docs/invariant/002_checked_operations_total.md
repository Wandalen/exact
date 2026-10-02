# Invariant: Checked Operations Total

### Scope

- **Purpose**: Guarantee that the family's one declared backing width never silently wraps, so every crate layered on top can treat a `Backing` overflow as a reported error rather than a corrupted value.
- **Responsibility**: `minor_checked_add`, `minor_checked_sub`, `minor_checked_neg` — total over every pair/value of `Backing`, never panicking, never wrapping.
- **In Scope**: The three checked functions and `MinorError`, the one error type they return.
- **Out of Scope**: `minor_saturating_add`/`minor_saturating_sub`, which deliberately clamp to `Backing::MIN`/`Backing::MAX` instead of erroring — a different, equally total contract for call sites that have already decided a clamped answer is acceptable (both are exercised below as the crate's complete arithmetic surface, but only the checked half makes this invariant's "or an explicit error" claim); the declared ceiling, which does not exist at this tier (→ [`exact_kind`'s own instance](../../../exact_kind/docs/invariant/002_checked_operations_total.md), which layers `ExceedsCeiling` on top of this crate's `Overflow`).

### Statement

Every checked function in this crate returns the mathematically exact result
or `Err(MinorError::Overflow { operation })` — never a silently wrapped value,
and never a panic for any input `Backing` can represent. `minor_checked_add`
and `minor_checked_sub` delegate to `Backing::checked_add`/`checked_sub`,
which already report both directions of range failure identically, so
`MinorError` carries one variant rather than inventing a sign-based split with
no observable behaviour behind it. `minor_checked_neg` is total except at the
one backing value whose negation cannot be represented, `Backing::MIN` —
reported the same way, not as a special case.

The saturating pair, `minor_saturating_add`/`minor_saturating_sub`, is the
crate's other total contract: every input still produces a `Backing` with no
panic, but by clamping to the width's own bound instead of returning an
error. Both are new in this crate — the family's prior shape offered only
checked arithmetic — for call sites that have already decided a clamped
answer to range failure is acceptable.

### Rationale

A silently wrapped `Backing` at this tier would be invisible to every
arithmetic layered above it: `exact_kind`'s own `checked_add`/`checked_sub`
call `Backing`'s own `checked_add`/`checked_sub` directly rather than this
crate's wrapper functions, so this crate's totality is not inherited through
a call chain — it has to hold on its own, as the floor every sibling crate's
own checked arithmetic is built against. The checked/saturating split exists
so a caller's choice between "tell me when this can't be exact" and "give me
the closest representable answer" is a type-level decision at the call site,
never a default either way.

### Sources

| File | Relationship |
|------|--------------|
| `src/lib.rs:50-59` | `MinorError` — one `Overflow { operation }` variant, naming which of `add`/`sub`/`neg` failed |
| `src/lib.rs:93-129` | `minor_checked_add`, `minor_checked_sub`, `minor_checked_neg` |
| `src/lib.rs:138-149` | `minor_saturating_add`, `minor_saturating_sub` — the clamping counterpart |

### Tests

| File | Relationship |
|------|--------------|
| `tests/checked_arithmetic_test.rs` | Refusal past `Backing::MAX` and `Backing::MIN` in both directions (`MAX + 1`, `MIN + -1`, `MIN - 1`, `MAX - -1`), totality of negation except at `Backing::MIN`, and the error's rendered text |
| `tests/saturating_arithmetic_test.rs` | Clamping at the same boundaries in both directions, always to the bound the result crossed; matches checked arithmetic in range |
