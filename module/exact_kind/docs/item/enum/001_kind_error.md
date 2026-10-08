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
comment, `src/lib.rs:21-29`).

## Kind

Enum (§ Item Kind Taxonomy : Stable Item Kinds #7)

## Definition

`module/exact_kind/src/lib.rs:68`

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
| `src/lib.rs` | 68,118,124-129,133,199,203,212,215,220,241,243,246,251,255,258,260,265,269,273,276,281,286,288,290,294,299,301,309,311,312,314,324,331,335,339,345,350,360,367,369,438,440,445,448,453,455,456,461,467,469,470,476,481,482,485,507,509,511,516,518,522,524,525,530,532,536,538,540,545,547,551,553,554,557 | Return type / constructed variant across every fallible constructor and operation on both `Decimal` and `Qty`, and the `Display`/`Error` impls it carries |
| `tests/*.rs` (4 files) | throughout | Matched against specific variants |
| `exact_parse/src/lib.rs:38,52-74` | — | `use` import; return type of all 3 `*_from_str` functions |
| `exact_bytes/src/lib.rs:39,96-98` | — | `use` import; matched variant by variant in `kind_error_to_wire_error` — `Negative` to `WireError::Negative`, the other four to `WireError::Overflow` |
| `exact_conserve/src/lib.rs:77` | — | `use` import |
| `exact_add/src/lib.rs:46` | — | `use` import; return type of every `money_*`/`qty_*`/`price_*` checked function |
| `exact_ratio/src/lib.rs:46,84` | — | `use` import; mapped to `RatioError::Negative` in `kind_error_to_ratio_error` |
| `exact_arith/src/lib.rs:88` | — | Facade re-export |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | Every fallible operation's error type on both `Decimal` and `Qty` |
| `exact_parse`, `exact_add` | `src/lib.rs` | Direct return type — these crates' own errors are `KindError`, not a wrapping type |
| `exact_bytes`, `exact_ratio` | `src/lib.rs` | Mapped into a crate-local error type (`WireError`, `RatioError`) via an explicit match, not a `From` impl |
| `exact_conserve` | `src/lib.rs` | Imported; used in error-path construction |
| `exact_arith` | `src/lib.rs` | Re-export only |
