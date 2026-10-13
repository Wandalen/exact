//! Comparison, equality, and min/max per kind, under the preferred design's
//! free-function names.
//!
//! Tier 2, depending on `exact_kind` alone for the `Ord`/`PartialEq`
//! `exact_kind::Decimal`/`Qty` already derive.
//!
//! # Disclosed deviation: no `CmpError`, no conditional `Ord`
//!
//! The preferred design names `CmpError { ScaleMismatch }` and warns against
//! implementing `Ord` unconditionally where scales may differ "without a
//! documented same-scale invariant." Both exist for a representation where
//! scale is a runtime value carried alongside the integer, so two values
//! could reach a comparison already holding different scales. Under this
//! family's const-generic representation, `Decimal< 6 >` and `Decimal< 9 >`
//! are different Rust types — a scale mismatch is a compile error at the
//! call site, never a value these functions could receive, so there is
//! nothing for `CmpError` to report and no scale check for `Ord` to be
//! conditioned on. Every function here is therefore infallible.
//!
//! # Examples
//!
//! ```
//! use exact_cmp::{ money_cmp, price_min };
//! use exact_kind::{ Money, Price };
//!
//! let a = Money::parse( "1" ).unwrap();
//! let b = Money::parse( "2" ).unwrap();
//! assert_eq!( money_cmp( a, b ), core::cmp::Ordering::Less );
//! assert_eq!( price_min( Price::parse( "1" ).unwrap(), Price::parse( "2" ).unwrap() ).to_string(), "1" );
//! ```

use exact_kind::{ Decimal, PriceAt, Qty };

/// Compare two money values.
#[ must_use ]
pub fn money_cmp< const S : u32 >( a : Decimal< S >, b : Decimal< S > ) -> core::cmp::Ordering
{
  a.cmp( &b )
}

/// Compare two quantities.
#[ must_use ]
pub fn qty_cmp< const S : u32 >( a : Qty< S >, b : Qty< S > ) -> core::cmp::Ordering
{
  a.cmp( &b )
}

/// Compare two prices.
#[ must_use ]
pub fn price_cmp< const S : u32 >( a : PriceAt< S >, b : PriceAt< S > ) -> core::cmp::Ordering
{
  a.cmp( &b )
}

/// Whether two money values are equal.
#[ must_use ]
pub fn money_eq< const S : u32 >( a : Decimal< S >, b : Decimal< S > ) -> bool
{
  a == b
}

/// The lesser of two prices.
#[ must_use ]
pub fn price_min< const S : u32 >( a : PriceAt< S >, b : PriceAt< S > ) -> PriceAt< S >
{
  a.min( b )
}

/// The greater of two prices.
#[ must_use ]
pub fn price_max< const S : u32 >( a : PriceAt< S >, b : PriceAt< S > ) -> PriceAt< S >
{
  a.max( b )
}
