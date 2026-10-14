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
| `src/lib.rs` | 73,123,129-134,138,304,309,318,321,326,357,362,368,369,385,387,390,395,399,402,404,409,413,417,420,425,430,432,434,438,443,445,453,455,456,458,468,475,479,483,489,494,504,511,513,668,670,675,678,683,685,686,691,697,699,700,706,711,712,715,733,749,751,753,758,760,764,766,767,772,774,778,780,782,787,789,793,795,796,799,966,1014,1021,1043,1043,1043,1043,1043,1043 | Return type / constructed variant across every fallible constructor and operation on both `Decimal` and `Qty`, and the `Display`/`Error` impls it carries; `rescale` on `Decimal`/`Qty`/`PriceAt` — `Overflow` past the backing width, and `ExcessPrecision` when `Exact` refuses a dropped digit (246-258, 536, 772); the error in the `Output` of the `+` and `-` impls `checked_operators!` writes, and in the `Result` the chaining impls take on the left |
| `tests/*.rs` (4 files) | throughout | Matched against specific variants |
| `exact_parse/src/lib.rs:39,53-75` | — | `use` import; return type of all 3 `*_from_str` functions |
| `exact_bytes/src/lib.rs:39,98-100` | — | `use` import; matched variant by variant in `kind_error_to_wire_error` — `Negative` to `WireError::Negative`, the other four to `WireError::Overflow` |
| `exact_conserve/src/lib.rs:83` | — | `use` import |
| `exact_dust/src/lib.rs:62,164,182` | — | `use` import; the error type of the `make` parameter of the private `split_with`/`split_into_with`, folded into `DustError::Overflow` |
| `exact_add/src/lib.rs:46` | — | `use` import; return type of every `money_*`/`qty_*`/`price_*` checked function |
| `exact_ratio/src/lib.rs:47,88-90` | — | `use` import; matched variant by variant in `kind_error_to_ratio_error` — `Negative` to `RatioError::Negative`, the other four to `RatioError::Overflow` |
| `exact_arith/src/lib.rs:88` | — | Facade re-export |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | Every fallible operation's error type on both `Decimal` and `Qty` |
| `exact_parse`, `exact_add` | `src/lib.rs` | Direct return type — these crates' own errors are `KindError`, not a wrapping type |
| `exact_bytes`, `exact_ratio` | `src/lib.rs` | Mapped into a crate-local error type (`WireError`, `RatioError`) via an explicit match, not a `From` impl |
| `exact_conserve` | `src/lib.rs` | Imported; used in error-path construction |
| `exact_dust` | `src/lib.rs` | Imported; names the `make` constructor's error type, every value folded into `DustError::Overflow` |
| `exact_arith` | `src/lib.rs` | Re-export only |
