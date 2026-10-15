//! Scale-factor math and the declared ceiling: powers of ten and the headroom
//! and ceiling constants derived from them.
//!
//! Tier 0 of this family's fifteen crates, alongside `exact_minor` and
//! `exact_round`. It is a sibling root, not a dependent of `exact_minor` —
//! Tier 0 crates have no edges to one another by design, so this crate names
//! the backing primitive (`i64`) directly rather than importing an alias
//! across an edge the dependency graph does not have. `exact_kind`, which
//! depends on both, is where the two converge under one shared name again.
//!
//! Scale here stays a compile-time fact carried in a type's own const
//! generic parameter, not a runtime value — so this crate stays the
//! constants and the power-of-ten table behind that mechanism, not a
//! runtime `Scale` type.

/// The headroom the range budget requires between the ceiling and the width.
///
/// A factor of 1000 is about ten bits, and it is a magnitude allowance for
/// intermediates rather than a safety blanket: it makes accumulation of a
/// thousand ceiling-sized amounts safe, which is the shape of the audit's
/// inner fold, and it does not make multiply-before-divide safe.
pub const HEADROOM_FACTOR : i64 = 1000;

/// The declared maximum holdings of one asset kind, in whole units.
///
/// A deployment input this crate cannot know. What it can do — and what the
/// range budget actually requires — is make the ceiling a declared constant
/// with a checked relationship to the width, which the assertion below is.
pub const CEILING_WHOLE_UNITS : i64 = 9_000_000_000;

/// The scale the family's money-like values are expressed at.
///
/// Six places is the usual settlement precision for a currency whose smallest
/// physical unit is two places: it leaves four places of sub-cent room for
/// per-unit prices, which is where precision is actually lost when a price is
/// quoted per thousand and a fill is for seven.
pub const MONEY_SCALE : u32 = 6;

/// The declared ceiling on the stored integer, at every scale.
///
/// Derived from [`CEILING_WHOLE_UNITS`] at [`MONEY_SCALE`], and then applied to
/// the *stored count of minor units* whatever the scale is, because that is the
/// quantity the headroom argument is actually about — the integer is what
/// overflows, not the value it denotes.
pub const CEILING_MINOR_UNITS : i64 = CEILING_WHOLE_UNITS * pow10( MONEY_SCALE );

// The range budget's item 1, checked where the constant is declared rather
// than only in a test that would have to sample its way to the top.
const _ : () = assert!( CEILING_MINOR_UNITS <= i64::MAX / HEADROOM_FACTOR );

/// `10ⁿ` as a backing value, for scales the backing width can hold.
///
/// ```
/// assert_eq!( exact_scale::pow10( 0 ), 1 );
/// assert_eq!( exact_scale::pow10( 6 ), 1_000_000 );
/// ```
///
/// # Panics
///
/// Panics if `n` exceeds 18, the largest power of ten an `i64` holds. In a
/// `const` position the panic becomes a compile error; a runtime call with
/// `n > 18` panics at runtime, as this crate's own test demonstrates.
#[ must_use ]
pub const fn pow10( n : u32 ) -> i64
{
  assert!( n <= 18, "10^n exceeds the backing width for n > 18" );
  let mut out : i64 = 1;
  let mut i = 0;
  while i < n
  {
    out *= 10;
    i += 1;
  }
  out
}
