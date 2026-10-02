//! Checked arithmetic over the raw backing width, with no ceiling in front of
//! it.
//!
//! `exact_kind`'s `Decimal`-equivalent never lets a value past its declared
//! ceiling reach these functions at all — its own tests assert the backing
//! edge is unreachable through its public API. Here, with no ceiling, the
//! edge is the first and only thing left to check: these are free functions
//! over raw `Backing`, and `Backing::MAX`/`Backing::MIN` are ordinary,
//! directly-reachable inputs.

use exact_minor::{ Backing, MinorError, minor_checked_add, minor_checked_neg, minor_checked_sub };

/// Ordinary in-range addition and subtraction are exact.
#[ test ]
fn in_range_addition_and_subtraction_are_exact()
{
  assert_eq!( minor_checked_add( 300_000, 200_000 ).unwrap(), 500_000 );
  assert_eq!( minor_checked_sub( 500_000, 200_000 ).unwrap(), 300_000 );
}

/// Addition past `Backing::MAX` is refused, not wrapped.
#[ test ]
fn addition_is_refused_past_the_backing_maximum()
{
  assert_eq!
  (
    minor_checked_add( Backing::MAX, 1 ),
    Err( MinorError::Overflow { operation : "add" } ),
  );
  assert!( minor_checked_add( Backing::MAX, 0 ).is_ok() );
}

/// Subtraction past `Backing::MIN` is refused, not wrapped.
#[ test ]
fn subtraction_is_refused_past_the_backing_minimum()
{
  assert_eq!
  (
    minor_checked_sub( Backing::MIN, 1 ),
    Err( MinorError::Overflow { operation : "sub" } ),
  );
  assert!( minor_checked_sub( Backing::MIN, 0 ).is_ok() );
}

/// Negation is total except at the one asymmetric edge, `Backing::MIN`.
#[ test ]
fn negation_is_total_except_at_the_backing_minimum()
{
  for value in [ 0, 1, -1, Backing::MAX ]
  {
    assert_eq!( minor_checked_neg( value ).unwrap(), -value );
  }
  assert_eq!
  (
    minor_checked_neg( Backing::MIN ),
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
  let error = minor_checked_neg( Backing::MIN ).unwrap_err();
  assert_eq!( error.to_string(), "neg left the representable range" );
  let _ : &dyn core::error::Error = &error;
}
