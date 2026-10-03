# Invariant: Saturating Clamp Direction Is Always Correct

### Scope

- **Purpose**: State that a saturating add never clamps to the wrong boundary, so a caller that has already decided a clamped answer is acceptable can trust the clamp's direction without re-deriving the sign argument themselves.
- **Responsibility**: `money_saturating_add`, `qty_saturating_add`.
- **In Scope**: Which boundary (`MAX` or `MIN`) a failed checked add clamps to.
- **Out of Scope**: Totality of the checked arithmetic this crate dispatches to (→ [`exact_kind`'s own `invariant/`](../../../exact_kind/docs/invariant/readme.md), which owns that guarantee at the type level); the saturating-coverage asymmetry — add-only, no subtract or price variants (→ `src/lib.rs`'s own "Disclosed deviations" module doc).

### Statement

`money_saturating_add`/`qty_saturating_add` clamp to [`Money::MAX`]/[`Money::MIN`]
(or [`Quantity::MAX`] — only the upper bound applies, since a quantity cannot
go negative) when the underlying `checked_add` fails, and the boundary chosen
is always the one the true, unbounded sum would actually have approached —
never the opposite one. The clamp never produces a result on the wrong side
of the operands' true sum.

### Rationale

A `checked_add` can only fail when `a` and `b` share a sign: two operands of
opposite sign move the sum toward zero, which can never leave a range the
operands themselves already fit inside. So whenever the checked path fails,
`b`'s sign alone already tells the saturating path which direction the true
sum overflowed in — `sign_is_negative(b.minor())` selects `Money::MIN`, otherwise
`Money::MAX`. There is no case where this reasoning could pick the wrong
boundary, because the premise (same-sign operands) is not a heuristic — it is
the only way `checked_add` fails in the first place. A clamp implementation
that instead inspected `a`'s sign, or the raw (potentially-overflowed) sum,
would risk reading a value that already wrapped; reading `b`'s sign — an
operand that never overflowed on its own — sidesteps that risk entirely.

### Sources

| File | Relationship |
|------|--------------|
| `src/lib.rs:106-123` | `money_saturating_add` — the clamp and its doc comment's full correctness argument |
| `src/lib.rs:125-137` | `qty_saturating_add` — the same shape, upper-bound-only |

### Tests

| File | Relationship |
|------|--------------|
| `tests/` | Saturating-arithmetic coverage at both backing extremes, both kinds |
