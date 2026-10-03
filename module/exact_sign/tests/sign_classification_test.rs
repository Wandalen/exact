//! Sign classification and the negative-value admission policy.

use exact_sign::{ Sign, sign_is_negative, sign_is_zero, sign_neg_allowed, sign_of };

/// Negative, zero, and positive values each classify to their own variant.
#[ test ]
fn sign_of_classifies_negative_zero_and_positive()
{
  assert_eq!( sign_of( -1 ), Sign::Neg );
  assert_eq!( sign_of( 0 ), Sign::Zero );
  assert_eq!( sign_of( 1 ), Sign::Pos );
}

/// The smallest and largest backing values classify too, so no value is left out.
#[ test ]
fn sign_of_classifies_both_ends_of_the_backing_range()
{
  assert_eq!( sign_of( i64::MIN ), Sign::Neg );
  assert_eq!( sign_of( i64::MAX ), Sign::Pos );
}

/// `sign_is_negative` and `sign_is_zero` agree with `sign_of` at the boundary.
#[ test ]
fn sign_is_negative_and_sign_is_zero_agree_with_sign_of_at_the_boundary()
{
  assert!( sign_is_negative( -1 ) );
  assert!( !sign_is_negative( 0 ) );
  assert!( sign_is_zero( 0 ) );
  assert!( !sign_is_zero( 1 ) );
  assert!( !sign_is_zero( -1 ) );
}

/// A policy that disallows negatives rejects a negative value and admits
/// zero and positive ones.
#[ test ]
fn neg_disallowed_policy_rejects_only_negative_values()
{
  assert!( !sign_neg_allowed( false, -1 ) );
  assert!( sign_neg_allowed( false, 0 ) );
  assert!( sign_neg_allowed( false, 1 ) );
}

/// A policy that allows negatives admits every value.
#[ test ]
fn neg_allowed_policy_admits_every_value()
{
  assert!( sign_neg_allowed( true, -1 ) );
  assert!( sign_neg_allowed( true, 0 ) );
  assert!( sign_neg_allowed( true, 1 ) );
}
