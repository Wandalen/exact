# Invariant: Ordering Is Total, With No Incomparable Case

### Scope

- **Purpose**: State that every pair of same-kind values has a definite order — never an incomparable or inconsistent result — so a caller can sort, min/max, or binary-search over this family's values with the same confidence a plain integer comparison offers, which a float comparison does not.
- **Responsibility**: `money_cmp`, `qty_cmp`, `price_cmp`, and the `Ord`/`PartialEq` they dispatch to.
- **In Scope**: Totality, reflexivity, antisymmetry, and transitivity of the comparison these functions provide.
- **Out of Scope**: Why no `CmpError`/conditional `Ord` exists at all — the compile-time scale argument (→ `../decisions/001_no_cmp_error_unconditional_ord.md`, which this invariant's totality claim depends on but does not restate).

### Statement

For any two values of the same kind, `money_cmp`/`qty_cmp`/`price_cmp` return
exactly one of `Less`, `Equal`, `Greater` — never a case the caller must treat
as undefined or incomparable. This holds because each dispatches directly to
`exact_kind::Decimal`/`Qty`'s derived `Ord`, itself built over a plain `i64`
minor-unit count with no not-a-number state: every `i64` compares
consistently with every other `i64`. The ordering is therefore a true total
order — reflexive (`cmp(a, a) == Equal`), antisymmetric, and transitive — with
no pair of values for which the comparison is inconsistent across repeated
calls or disagrees with `money_eq`'s own equality check.

### Rationale

A float comparison's one real hazard is `NaN`, which compares unequal to
everything including itself and breaks exactly the transitivity and
reflexivity a sort, a `BTreeMap` key, or a binary search silently assumes —
the kind of bug that surfaces only on the specific input that happens to
reach the broken case. This family's representation has no analogous
not-a-number state to smuggle in: a `Decimal`/`Qty` is constructed through a
checked path that already refused anything that could not become a definite
integer, so by the time a value reaches `money_cmp`, totality is not a
property this crate has to additionally verify — it inherits it unconditionally
from the representation underneath. Stating it here is what lets a caller
building a sorted structure or a binary search over these kinds cite this
invariant instead of re-deriving, from `exact_kind`'s own representation,
that no exceptional value could have reached them.

### Sources

| File | Relationship |
|------|--------------|
| `src/lib.rs:34-53` | `money_cmp`, `qty_cmp`, `price_cmp`, `money_eq` — direct dispatch to the derived `Ord`/`PartialEq` this invariant is a property of |
| `../../../exact_kind/src/lib.rs` | `Decimal`/`Qty`'s own `#[derive(..., PartialOrd, Ord, ...)]` over a plain `i64` field |

### Tests

| File | Relationship |
|------|--------------|
| `tests/cmp_test.rs` | Comparison and equality coverage per kind |
