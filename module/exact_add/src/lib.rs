//! Checked and saturating addition and subtraction, dispatched per kind
//! under the preferred design's free-function names.
//!
//! Tier 2, depending on `exact_kind` for the conserved value types and
//! `exact_sign` for classifying which direction a saturating operation
//! clamps toward.
//!
//! # Disclosed deviations from the preferred design's own type listing
//!
//! - **No `AddError`.** The preferred design lists `AddError { Overflow,
//!   ScaleMismatch, NegNotAllowed }`. `Overflow` already exists as
//!   [`exact_kind::KindError::Overflow`]; `ScaleMismatch` is unreachable for
//!   the same reason `exact_kind` and `exact_cmp` already drop it — two
//!   different `SCALE` values are two different Rust types under this
//!   family's const-generic representation, caught at compile time, never
//!   at runtime; `NegNotAllowed` has no reachable call site, since the only
//!   negation this crate exposes is [`money_checked_neg`], over a kind that
//!   is already signed. Rather than wrap `KindError` in a same-shaped enum
//!   with no new reachable variant, every fallible function here returns
//!   [`exact_kind::KindError`] directly.
//! - **No panicking variant.** The preferred design's crate entry mentions
//!   "checked/saturating/panicking variants, named" among its owned
//!   features. No real consumer anywhere in this migration calls for a
//!   panicking arithmetic entry point — every one of the 5 real crates this
//!   family is drawn from is checked-only — so building one now would be
//!   speculative. Deferred until a concrete call site asks for it.
//! - **Saturating coverage is add-only, money/qty-only.** [`money_saturating_add`]
//!   and [`qty_saturating_add`] exist; `money_saturating_sub`, `qty_saturating_sub`,
//!   and any `price_saturating_*` do not. Same reasoning as the panicking
//!   variant above: no concrete call site has asked for a clamping subtract or
//!   a clamping price operation yet, so the remaining combinations are
//!   deferred rather than built out speculatively for symmetry's own sake.
//!
//! # Examples
//!
//! ```
//! use exact_add::{ money_add, money_saturating_add };
//! use exact_kind::Money;
//!
//! let a = Money::parse( "0.1" ).unwrap();
//! let b = Money::parse( "0.2" ).unwrap();
//! assert_eq!( money_add( a, b ).unwrap(), Money::parse( "0.3" ).unwrap() );
//! assert_eq!( money_saturating_add( Money::MAX, Money::EPSILON ), Money::MAX );
//! ```

use exact_kind::{ Decimal, KindError, PriceAt, Qty };

/// Add two money values.
///
/// # Errors
///
/// See [`exact_kind::Decimal::checked_add`].
pub const fn money_add< const S : u32 >( a : Decimal< S >, b : Decimal< S > ) -> Result< Decimal< S >, KindError >
{
  a.checked_add( b )
}

/// Subtract two money values.
///
/// # Errors
///
/// See [`exact_kind::Decimal::checked_sub`].
pub const fn money_sub< const S : u32 >( a : Decimal< S >, b : Decimal< S > ) -> Result< Decimal< S >, KindError >
{
  a.checked_sub( b )
}

/// Add two quantities.
///
/// # Errors
///
/// See [`exact_kind::Qty::checked_add`].
pub const fn qty_add< const S : u32 >( a : Qty< S >, b : Qty< S > ) -> Result< Qty< S >, KindError >
{
  a.checked_add( b )
}

/// Subtract two quantities, refusing to go below zero.
///
/// # Errors
///
/// See [`exact_kind::Qty::checked_sub`].
pub const fn qty_sub< const S : u32 >( a : Qty< S >, b : Qty< S > ) -> Result< Qty< S >, KindError >
{
  a.checked_sub( b )
}

/// Add two prices.
///
/// # Errors
///
/// See [`exact_kind::PriceAt::checked_add`].
pub const fn price_add< const S : u32 >( a : PriceAt< S >, b : PriceAt< S > ) -> Result< PriceAt< S >, KindError >
{
  a.checked_add( b )
}

/// Subtract two prices.
///
/// # Errors
///
/// See [`exact_kind::PriceAt::checked_sub`].
pub const fn price_sub< const S : u32 >( a : PriceAt< S >, b : PriceAt< S > ) -> Result< PriceAt< S >, KindError >
{
  a.checked_sub( b )
}

/// Negate a money value. Money is signed, so negation is always allowed.
///
/// # Errors
///
/// See [`exact_kind::Decimal::checked_neg`].
pub const fn money_checked_neg< const S : u32 >( a : Decimal< S > ) -> Result< Decimal< S >, KindError >
{
  a.checked_neg()
}

/// Add two money values, clamping to the declared ceiling instead of refusing.
///
/// Clamps to [`Decimal::MAX`]/[`Decimal::MIN`] — the declared ceiling, not the
/// raw backing width — because a `Decimal` value is only ever constructible
/// inside that ceiling in the first place; a wider clamp would produce a
/// minor count the type's own constructor would refuse to hold. The clamp
/// direction is `b`'s sign: when `checked_add` fails, `a` and `b` necessarily
/// share a sign (operands of opposite sign can never overflow a sum), so
/// `b`'s sign is also the true mathematical sum's sign.
#[ must_use ]
pub const fn money_saturating_add< const S : u32 >( a : Decimal< S >, b : Decimal< S > ) -> Decimal< S >
{
  match a.checked_add( b )
  {
    Ok( sum ) => sum,
    Err( _ ) => if exact_sign::sign_is_negative( b.minor() ) { Decimal::< S >::MIN } else { Decimal::< S >::MAX },
  }
}

/// Add two quantities, clamping to the declared ceiling instead of refusing.
///
/// Only the upper bound can ever clamp — two non-negative quantities never
/// sum below zero.
#[ must_use ]
pub const fn qty_saturating_add< const S : u32 >( a : Qty< S >, b : Qty< S > ) -> Qty< S >
{
  match a.checked_add( b )
  {
    Ok( sum ) => sum,
    Err( _ ) => Qty::< S >::MAX,
  }
}
