//! Text parsing per kind, under the preferred design's free-function names.
//!
//! Tier 2, depending on `exact_kind` for the parser each function dispatches
//! to and `exact_scale` for the cross-crate consistency guard below.
//!
//! # Disclosed deviations from the preferred design's own type listing
//!
//! - **No `ParseError`.** The preferred design lists `ParseError { Empty,
//!   BadChar, ExtraDigits, ScaleTooLarge, Overflow, Sign }`. `Empty`,
//!   `BadChar` and `Sign` are all grammar failures `exact_kind`'s parser
//!   already reports as one `KindError::Malformed { reason }`, naming the
//!   specific problem in the reason string rather than a separate variant
//!   per grammar rule. `ExtraDigits` is `KindError::ExcessPrecision`, and
//!   `Overflow` already exists under that name. `ScaleTooLarge` is
//!   unreachable: `SCALE` is a compile-time const generic, so an
//!   unrepresentable scale is already a compile error (`Decimal`'s own
//!   `ONE_MINOR` associated const panics at const-eval time), never a value
//!   a running parse call could receive. Every function here therefore
//!   returns [`exact_kind::KindError`] directly, matching `exact_add`'s
//!   identical reasoning for dropping its own doc-specified error enum.
//! - **No standalone `parse_reject_extra_digits`.** The guard already lives
//!   inside `exact_kind`'s parser, where the digit count is already in
//!   scope. Extracting it as a free function here with no call site besides
//!   that same parser would either duplicate the check or require
//!   `exact_kind` to depend on this crate — the wrong direction for the
//!   family's own dependency graph. Deferred until a consumer needs to run
//!   the guard independently of a full parse.
//!
//! # Examples
//!
//! ```
//! use exact_parse::money_from_str;
//!
//! let v = money_from_str( "1.23" ).unwrap();
//! assert_eq!( v.minor(), 1_230_000 );
//! ```

use exact_kind::{ KindError, Money, Price, Quantity };

// A real cross-crate consistency guard: `exact_kind`'s `Money`/`Price` scale
// and `exact_scale`'s own `MONEY_SCALE` constant are declared in two
// different crates now, connected only by a shared numeric literal at each
// definition site. This fails the build the moment they drift apart, rather
// than waiting for a parse to silently use the wrong scale.
const _ : () = assert!( Money::ONE_MINOR == exact_scale::pow10( exact_scale::MONEY_SCALE ) );

/// Parse a money value from text.
///
/// # Errors
///
/// See [`exact_kind::Decimal::parse`].
pub fn money_from_str( text : &str ) -> Result< Money, KindError >
{
  Money::parse( text )
}

/// Parse a quantity from text, refusing a negative one.
///
/// # Errors
///
/// See [`exact_kind::Qty::parse`].
pub fn qty_from_str( text : &str ) -> Result< Quantity, KindError >
{
  Quantity::parse( text )
}

/// Parse a price from text.
///
/// # Errors
///
/// As [`money_from_str`] — `Price` is `Money` under today's disclosed
/// deviation in `exact_kind`.
pub fn price_from_str( text : &str ) -> Result< Price, KindError >
{
  Price::parse( text )
}
