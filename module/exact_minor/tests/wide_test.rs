//! `MinorWide` — only built with `--features i128` (see `required-features` in `Cargo.toml`).

use exact_minor::
{
  Backing,
  Minor,
  MinorError,
  MinorWide,
  minor_from_i64,
  minor_wide_checked_add,
  minor_wide_checked_neg,
  minor_wide_checked_sub,
  minor_wide_is_zero,
  minor_wide_saturating_add,
  minor_wide_saturating_sub,
  minor_wide_zero,
};

/// Widening a `Minor` keeps its value exactly, both extremes included.
#[ test ]
fn widening_a_minor_keeps_its_value()
{
  for value in [ 0, 1, -1, Backing::MAX, Backing::MIN ]
  {
    assert_eq!( MinorWide::from( minor_from_i64( value ) ), MinorWide( i128::from( value ) ) );
  }
}

/// `MinorWide` holds a magnitude no `Minor` can — the reason it exists.
#[ test ]
fn minor_wide_holds_values_past_i64()
{
  let past_max = MinorWide( i128::from( Backing::MAX ) + 1 );
  assert!( past_max > MinorWide::from( minor_from_i64( Backing::MAX ) ) );
}

/// Narrowing back to `Minor` succeeds exactly when the value fits `i64`,
/// and names the bound it crossed when it does not.
#[ test ]
fn narrowing_back_to_minor_refuses_what_does_not_fit()
{
  assert_eq!
  (
    Minor::try_from( MinorWide( i128::from( Backing::MAX ) ) ),
    Ok( minor_from_i64( Backing::MAX ) )
  );
  assert_eq!
  (
    Minor::try_from( MinorWide( i128::from( Backing::MIN ) ) ),
    Ok( minor_from_i64( Backing::MIN ) )
  );
  assert_eq!
  (
    Minor::try_from( MinorWide( i128::from( Backing::MAX ) + 1 ) ),
    Err( MinorError::Overflow { operation : "narrow" } )
  );
  assert_eq!
  (
    Minor::try_from( MinorWide( i128::from( Backing::MIN ) - 1 ) ),
    Err( MinorError::Underflow { operation : "narrow" } )
  );
}

/// Zero is zero, and nothing else is — at both `i128` extremes too.
#[ test ]
fn only_wide_zero_is_zero()
{
  assert!( minor_wide_is_zero( minor_wide_zero() ) );
  for value in [ 1, -1, i128::MAX, i128::MIN ]
  {
    assert!( !minor_wide_is_zero( MinorWide( value ) ), "{value} was reported as zero" );
  }
}

/// Checked arithmetic is exact in range, and names the direction it left `i128` in.
#[ test ]
fn wide_checked_arithmetic_refuses_both_directions()
{
  // a sum no `i64` could hold, computed exactly
  let big = MinorWide( i128::from( Backing::MAX ) );
  assert_eq!( minor_wide_checked_add( big, big ), Ok( MinorWide( i128::from( Backing::MAX ) * 2 ) ) );

  assert_eq!
  (
    minor_wide_checked_add( MinorWide( i128::MAX ), MinorWide( 1 ) ),
    Err( MinorError::Overflow { operation : "add" } )
  );
  assert_eq!
  (
    minor_wide_checked_add( MinorWide( i128::MIN ), MinorWide( -1 ) ),
    Err( MinorError::Underflow { operation : "add" } )
  );
  assert_eq!
  (
    minor_wide_checked_sub( MinorWide( i128::MIN ), MinorWide( 1 ) ),
    Err( MinorError::Underflow { operation : "sub" } )
  );
  assert_eq!
  (
    minor_wide_checked_sub( MinorWide( i128::MAX ), MinorWide( -1 ) ),
    Err( MinorError::Overflow { operation : "sub" } )
  );
  assert_eq!( minor_wide_checked_neg( MinorWide( 5 ) ), Ok( MinorWide( -5 ) ) );
  assert_eq!
  (
    minor_wide_checked_neg( MinorWide( i128::MIN ) ),
    Err( MinorError::Overflow { operation : "neg" } )
  );
}

/// Saturating arithmetic clamps to the `i128` bound the result crossed.
#[ test ]
fn wide_saturating_arithmetic_clamps_to_the_crossed_bound()
{
  assert_eq!( minor_wide_saturating_add( MinorWide( i128::MAX ), MinorWide( 1 ) ), MinorWide( i128::MAX ) );
  assert_eq!( minor_wide_saturating_add( MinorWide( i128::MIN ), MinorWide( -1 ) ), MinorWide( i128::MIN ) );
  assert_eq!( minor_wide_saturating_sub( MinorWide( i128::MIN ), MinorWide( 1 ) ), MinorWide( i128::MIN ) );
  assert_eq!( minor_wide_saturating_sub( MinorWide( i128::MAX ), MinorWide( -1 ) ), MinorWide( i128::MAX ) );
}

/// Inside the range, wide checked subtraction is exact and saturating
/// arithmetic leaves the result unclamped.
#[ test ]
fn wide_arithmetic_in_range_is_exact_and_unclamped()
{
  let past_i64 = MinorWide( i128::from( i64::MAX ) + 10 );
  let max_i64 = MinorWide( i128::from( i64::MAX ) );
  assert_eq!( minor_wide_checked_sub( past_i64, MinorWide( 10 ) ), Ok( max_i64 ) );
  assert_eq!( minor_wide_saturating_add( MinorWide( -5 ), MinorWide( 3 ) ), MinorWide( -2 ) );
  assert_eq!( minor_wide_saturating_sub( MinorWide( -5 ), MinorWide( 3 ) ), MinorWide( -8 ) );
}
