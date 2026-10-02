# 001: KindError

## Representation

Why a value could not be constructed, or an operation could not be
completed. Five variants, each naming the specific condition: `Overflow`
(left the backing width), `ExceedsCeiling` (in-width but past the deployment
ceiling), `ExcessPrecision` (more fractional digits than the scale),
`Malformed` (text outside the parse grammar), `Negative` (arithmetic
succeeded but the result is below zero, for a kind that refuses it). The
union of `exact_decimal`'s `DecimalError` and `exact_qty`'s `QtyError`, minus
the nested `QtyError::Decimal(DecimalError)` wrapping (flattened — one flat
enum now, not two types one wrapping the other) and minus a `ScaleMismatch`
variant the preferred design's own spec lists but which has no reachable
construction site: two `Decimal< SCALE >` values of different `SCALE` are
different Rust types and cannot reach a shared function to compare, so the
compiler refuses the mismatch before any runtime check could run (module doc
comment, `src/lib.rs:33-41`).

## Kind

Enum (§ Item Kind Taxonomy : Stable Item Kinds #7)

## Definition

`module/exact_kind/src/lib.rs:73`

```rust
pub enum KindError
{
  Overflow { operation : &'static str },
  ExceedsCeiling { minor : Backing },
  ExcessPrecision { digits : u32, scale : u32 },
  Malformed { reason : &'static str },
  Negative { minor : Backing },
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 73,123,129-134,138,184,188,197,200,205,226,228,231,236,241,244,246,251,256,260,263,268,273,275,277,281,286,289,297,299,300,302,312,319,323,327,333,338,348,355,357,415,417,422,425,430,432,433,438,444,446,447,453,458,459,462,484,486,488,493,495,499,501,502,507,509,513,515,517,522,524,528,530,531,534 | Return type / constructed variant across every fallible constructor and operation on both `Decimal` and `Qty`, and the `Display`/`Error` impls it carries |
| `tests/*.rs` (3 files) | throughout | Matched against specific variants |
| `exact_parse/src/lib.rs:29,43-66` | — | `use` import; return type of all 3 `*_from_str` functions |
| `exact_bytes/src/lib.rs:26,79` | — | `use` import; mapped to `WireError::Negative` in `kind_error_to_wire_error` |
| `exact_conserve/src/lib.rs:77` | — | `use` import |
| `exact_add/src/lib.rs:28` | — | `use` import; return type of every `money_*`/`qty_*`/`price_*` checked function |
| `exact_ratio/src/lib.rs:33,71` | — | `use` import; mapped to `RatioError::Negative` in `kind_error_to_ratio_error` |
| `exact_arith/src/lib.rs:81` | — | Facade re-export |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | Every fallible operation's error type on both `Decimal` and `Qty` |
| `exact_parse`, `exact_add` | `src/lib.rs` | Direct return type — these crates' own errors are `KindError`, not a wrapping type |
| `exact_bytes`, `exact_ratio` | `src/lib.rs` | Mapped into a crate-local error type (`WireError`, `RatioError`) via an explicit match, not a `From` impl |
| `exact_conserve` | `src/lib.rs` | Imported; used in error-path construction |
| `exact_arith` | `src/lib.rs` | Re-export only |
