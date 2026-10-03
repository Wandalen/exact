# 001: Minor

## Representation

A count of minor units — the family's base subunit type. It wraps the backing
`i64` in a type of its own, so a count of minor units cannot be mixed up with
any other `i64`. The field is private: the only ways in and out are
[minor_from_i64](../function/008_minor_from_i64.md) and
[minor_to_i64](../function/009_minor_to_i64.md).

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
| `src/lib.rs` | throughout | Parameter and return type of every function in the crate |
| `tests/*.rs` | throughout | Every test builds its inputs with `minor_from_i64` |
| `exact_arith/src/lib.rs:65` | — | Facade re-export |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_minor` | `(defining crate)` | The type every function takes and returns |
| `exact_arith` | `src/lib.rs` | Re-export only |
