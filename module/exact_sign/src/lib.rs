//! Sign classification and the family's negative-value admission policy.
//!
//! Tier 1 of the family, depending on `exact_minor` alone for the backing
//! primitive a sign is classified over.
//!
//! Net-new: the family's prior shape encoded "can this go negative" as a
//! type-level choice (`Decimal` signed, `Qty` never) rather than a runtime
//! policy value, so there is no real-code sign classifier to port — every
//! item below is written fresh against the preferred design's own crate
//! specification.
//!
//! # Examples
//!
//! ```
//! use exact_sign::{ Sign, sign_of, sign_neg_allowed };
//!
//! assert_eq!( sign_of( -1 ), Sign::Neg );
//! assert!( !sign_neg_allowed( false, -1 ) );
//! ```

use exact_minor::Backing;

/// The sign of a backing value: negative, exactly zero, or positive.
#[ derive( Debug, Clone, Copy, PartialEq, Eq, Hash ) ]
pub enum Sign
{
  /// Strictly less than zero.
  Neg,

  /// Exactly zero — neither negative nor positive.
  Zero,

  /// Strictly greater than zero.
  Pos,
}

/// Classifies a backing value's sign.
#[ must_use ]
pub const fn sign_of( value : Backing ) -> Sign
{
  if value < 0
  {
    Sign::Neg
  }
  else if value == 0
  {
    Sign::Zero
  }
  else
  {
    Sign::Pos
  }
}

/// Whether a backing value is strictly negative.
#[ must_use ]
pub const fn is_negative( value : Backing ) -> bool
{
  matches!( sign_of( value ), Sign::Neg )
}

/// Whether a backing value is exactly zero.
#[ must_use ]
pub const fn is_zero( value : Backing ) -> bool
{
  matches!( sign_of( value ), Sign::Zero )
}

/// Whether `value` is admissible under a kind's own negative-value policy.
///
/// A policy function rather than a bare per-kind constant, so the decision
/// reads at the call site as a question about the *value* being checked,
/// not as a scattered `if Money { true } else { false }`. `exact_kind`
/// calls this once per kind, at construction, per the family's decision to
/// enforce non-negativity where a `Qty` is built rather than later at
/// arithmetic time.
#[ must_use ]
pub const fn sign_neg_allowed( neg_allowed : bool, value : Backing ) -> bool
{
  neg_allowed || !is_negative( value )
}
