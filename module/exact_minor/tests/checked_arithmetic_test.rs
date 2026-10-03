//! Checked arithmetic over the raw backing width, with no ceiling in front of
//! it.
//!
//! `exact_kind`'s `Decimal`-equivalent never lets a value past its declared
//! ceiling reach these functions at all — its own tests assert the backing
//! edge is unreachable through its public API. Here, with no ceiling, the
//! edge is the first and only thing left to check: these are free functions
//! over `Minor`, and `Backing::MAX`/`Backing::MIN` are ordinary,
//! directly-reachable inputs.

use exact_minor::{ Backing, MinorError, minor_checked_add, minor_checked_neg, minor_checked_sub, minor_from_i64 };

/// Ordinary in-range addition and subtraction are exact.
#[ test ]
fn in_range_addition_and_subtraction_are_exact()
{
  assert_eq!( minor_checked_add( minor_from_i64( 300_000 ), minor_from_i64( 200_000 ) ), Ok( minor_from_i64( 500_000 ) ) );
  assert_eq!( minor_checked_sub( minor_from_i64( 500_000 ), minor_from_i64( 200_000 ) ), Ok( minor_from_i64( 300_000 ) ) );
}

/// Addition past `Backing::MAX` is refused, not wrapped.
#[ test ]
fn addition_is_refused_past_the_backing_maximum()
{
  assert_eq!
  (
    minor_checked_add( minor_from_i64( Backing::MAX ), minor_from_i64( 1 ) ),
    Err( MinorError::Overflow { operation : "add" } ),
  );
  assert!( minor_checked_add( minor_from_i64( Backing::MAX ), minor_from_i64( 0 ) ).is_ok() );
}

/// Subtraction past `Backing::MIN` is refused, not wrapped.
#[ test ]
fn subtraction_is_refused_past_the_backing_minimum()
{
  assert_eq!
  (
    minor_checked_sub( minor_from_i64( Backing::MIN ), minor_from_i64( 1 ) ),
    Err( MinorError::Underflow { operation : "sub" } ),
  );
  assert!( minor_checked_sub( minor_from_i64( Backing::MIN ), minor_from_i64( 0 ) ).is_ok() );
}

/// Negation is total except at the one asymmetric edge, `Backing::MIN`.
#[ test ]
fn negation_is_total_except_at_the_backing_minimum()
{
  for value in [ 0, 1, -1, Backing::MAX ]
  {
    assert_eq!( minor_checked_neg( minor_from_i64( value ) ), Ok( minor_from_i64( -value ) ) );
  }
  assert_eq!
  (
    minor_checked_neg( minor_from_i64( Backing::MIN ) ),
    Err( MinorError::Overflow { operation : "neg" } ),
  );
}

/// The error names the operation that failed, and is a standard error type.
///
/// The `Display` text is what a log or an investigation reads, so its wording
/// is checked exactly; the `&dyn Error` binding fails to compile if the
/// `core::error::Error` impl is ever removed.
#[ test ]
fn overflow_error_names_the_failed_operation()
{
  let error = minor_checked_neg( minor_from_i64( Backing::MIN ) ).unwrap_err();
  assert_eq!( error.to_string(), "neg rose above the representable range" );
  assert_eq!( minor_checked_sub( minor_from_i64( Backing::MIN ), minor_from_i64( 1 ) ).unwrap_err().to_string(), "sub fell below the representable range" );
  let _ : &dyn core::error::Error = &error;
}

/// Each operation is refused in its other direction too: a negative operand
/// pushes addition past `MIN`, and subtraction past `MAX`.
#[ test ]
fn overflow_is_refused_in_the_other_direction()
{
  assert_eq!( minor_checked_add( minor_from_i64( Backing::MIN ), minor_from_i64( -1 ) ), Err( MinorError::Underflow { operation : "add" } ) );
  assert_eq!( minor_checked_sub( minor_from_i64( Backing::MAX ), minor_from_i64( -1 ) ), Err( MinorError::Overflow { operation : "sub" } ) );
}
