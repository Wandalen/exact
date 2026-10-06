# 001: Minor

## Representation

A count of minor units — the family's base subunit type. It wraps the backing
`i64` in a type of its own, so a count of minor units cannot be mixed up with
any other `i64`. The field is private: the ways in and out are
[minor_from_i64](../function/008_minor_from_i64.md) and
[minor_to_i64](../function/009_minor_to_i64.md), and, with `--features i128`,
`From`/`TryFrom` with [MinorWide](002_minor_wide.md) (`src/lib.rs:44-45`).

## Kind

Struct (§ Item Kind Taxonomy : Stable Item Kinds #6)

## Definition

`module/exact_minor/src/lib.rs:51`

```rust
pub struct Minor( Backing );
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | throughout | Parameter and return type of every `minor_*` function; `minor_from_i64` takes an `i64` and `minor_to_i64` returns one, and the `minor_wide_*` functions take `MinorWide` |
| `tests/*.rs` | throughout | Every test builds its inputs with `minor_from_i64` |
| `exact_kind/src/lib.rs:159` | — | **Production** — the type of `Decimal`'s stored count |
| `exact_arith/src/lib.rs:69` | — | Facade re-export |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_minor` | `(defining crate)` | The type every non-wide arithmetic function takes and returns |
| `exact_kind` | `src/lib.rs` | **Production** — every `Money`, `Price` and `Quantity` stores one |
| `exact_arith` | `src/lib.rs` | Re-export only |
