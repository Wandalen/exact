//! `Wire`, a fixed-width byte encoding for a conserved value, and its
//! to/from conversions per kind.
//!
//! Tier 2, depending on `exact_kind` for the conserved value types and
//! `exact_scale` for the scale this family's wire encoding carries.
//!
//! Net-new: no real precedent exists in the 5 real crates — the closest is
//! the real codebase's own `format/001_transaction_log_encoding.md`, which
//! was itself an unimplemented spec for an 8-byte `amount` field. This crate
//! widens that to a self-describing 10-byte record (minor, scale, kind)
//! rather than a bare 8-byte amount, because a bare amount cannot be
//! decoded back into a specific kind without an external convention
//! recording which kind and scale it was written at.
//!
//! # Disclosed deviation: two error variants beyond the doc's three
//!
//! The preferred design lists `WireError { BadKind, BadScale, Truncated }`.
//! None of the three covers a `minor` value that decodes to a valid `i64`
//! but still breaches the declared ceiling, or (for `Qty`) a negative one —
//! both genuinely reachable from a decoded `Wire`, since `minor` is a bare
//! `i64` with no range check of its own until a kind's own constructor sees
//! it. `WireError::Overflow` and `WireError::Negative` cover these, the same
//! two additions `exact_ratio` already needed for the same underlying
//! reason.
//!
//! # Examples
//!
//! ```
//! use exact_bytes::{ money_to_wire, money_from_wire };
//! use exact_kind::Money;
//!
//! let v = Money::parse( "1.5" ).unwrap();
//! let wire = money_to_wire( v );
//! let bytes = wire.to_bytes();
//! let decoded = exact_bytes::Wire::from_bytes( &bytes ).unwrap();
//! assert_eq!( money_from_wire( decoded ).unwrap(), v );
//! ```

use exact_kind::{ KindError, Money, Price, Quantity };

/// The wire discriminator for [`Money`].
pub const KIND_MONEY : u8 = 0;
/// The wire discriminator for [`Quantity`].
pub const KIND_QTY : u8 = 1;
/// The wire discriminator for [`Price`].
pub const KIND_PRICE : u8 = 2;

const _ : () = assert!( exact_scale::MONEY_SCALE <= u8::MAX as u32 );

/// The scale every kind is written at, as the one byte a wire record stores —
/// the assertion above proves the cast keeps it whole.
const SCALE_BYTE : u8 = exact_scale::MONEY_SCALE as u8;

/// Why a `Wire` could not be decoded, or a decoded value could not be
/// turned into a specific kind.
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
pub enum WireError
{
  /// The `kind` byte did not match the kind being decoded into.
  BadKind,
  /// The `scale` byte did not match the scale the kind expects.
  BadScale,
  /// The byte slice was shorter than [`Wire::ENCODED_LEN`].
  Truncated,
  /// The decoded `minor` value left the representable or declared range.
  Overflow,
  /// The decoded `minor` value was below zero, for a kind that refuses it.
  Negative
  {
    /// The offending count of minor units.
    minor : i64,
  },
}

impl core::fmt::Display for WireError
{
  fn fmt( &self, f : &mut core::fmt::Formatter< '_ > ) -> core::fmt::Result
  {
    match self
    {
      Self::BadKind => write!( f, "the wire's kind byte did not match the kind being decoded into" ),
      Self::BadScale => write!( f, "the wire's scale byte did not match the kind's expected scale" ),
      Self::Truncated => write!( f, "the byte slice was shorter than the wire encoding's fixed length" ),
      Self::Overflow => write!( f, "the decoded value left the representable or declared range" ),
      Self::Negative { minor } => write!( f, "{minor} minor units is below zero, which this kind cannot hold" ),
    }
  }
}

impl core::error::Error for WireError {}

fn kind_error_to_wire_error( e : KindError ) -> WireError
{
  match e
  {
    KindError::Negative { minor } => WireError::Negative { minor },
    KindError::Overflow { .. } | KindError::ExceedsCeiling { .. }
    | KindError::ExcessPrecision { .. } | KindError::Malformed { .. } => WireError::Overflow,
  }
}

/// A record's kind and scale, checked before its `minor` is trusted — the
/// kind first, so a record of another kind is refused as `BadKind` whatever
/// its scale.
fn check_header( w : Wire, kind : u8 ) -> Result< (), WireError >
{
  if w.kind != kind
  {
    return Err( WireError::BadKind );
  }
  if w.scale != SCALE_BYTE
  {
    return Err( WireError::BadScale );
  }
  Ok( () )
}

/// A fixed-width wire encoding for one conserved value: its minor-unit
/// count, the scale it was written at, and which kind it is.
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
pub struct Wire
{
  minor : i64,
  scale : u8,
  kind : u8,
}

impl Wire
{
  /// The encoded length in bytes: an 8-byte little-endian `minor`, one
  /// `scale` byte, and one `kind` byte.
  pub const ENCODED_LEN : usize = 10;

  /// Build a wire record directly from its three fields.
  ///
  /// Infallible: `Wire` carries no invariant of its own to check — a
  /// mismatched kind or scale, or a `minor` outside a kind's range, is
  /// detected by the `*_from_wire` functions that interpret the record, not
  /// by this constructor.
  #[ must_use ]
  pub const fn new( minor : i64, scale : u8, kind : u8 ) -> Self
  {
    Self { minor, scale, kind }
  }

  /// The minor-unit count carried.
  #[ must_use ]
  pub const fn minor( self ) -> i64
  {
    self.minor
  }

  /// The scale this value was written at.
  #[ must_use ]
  pub const fn scale( self ) -> u8
  {
    self.scale
  }

  /// The kind discriminator — one of [`KIND_MONEY`], [`KIND_QTY`], [`KIND_PRICE`].
  #[ must_use ]
  pub const fn kind( self ) -> u8
  {
    self.kind
  }

  /// Encode to a fixed-size byte array.
  #[ must_use ]
  pub fn to_bytes( self ) -> [ u8; Self::ENCODED_LEN ]
  {
    let mut out = [ 0_u8; Self::ENCODED_LEN ];
    out[ 0 .. 8 ].copy_from_slice( &self.minor.to_le_bytes() );
    out[ 8 ] = self.scale;
    out[ 9 ] = self.kind;
    out
  }

  /// Decode from a byte slice.
  ///
  /// # Errors
  ///
  /// [`WireError::Truncated`] when `bytes` is shorter than [`Self::ENCODED_LEN`].
  pub fn from_bytes( bytes : &[ u8 ] ) -> Result< Self, WireError >
  {
    let Some( encoded ) = bytes.get( 0 .. Self::ENCODED_LEN )
    else
    {
      return Err( WireError::Truncated );
    };
    let minor = i64::from_le_bytes( encoded[ 0 .. 8 ].try_into().expect( "exactly 8 bytes" ) );
    Ok( Self { minor, scale : encoded[ 8 ], kind : encoded[ 9 ] } )
  }
}

/// Encode a money value to its wire form.
#[ must_use ]
pub fn money_to_wire( v : Money ) -> Wire
{
  Wire { minor : v.minor(), scale : SCALE_BYTE, kind : KIND_MONEY }
}

/// Decode a money value from its wire form.
///
/// # Errors
///
/// [`WireError::BadKind`] when `w` was not encoded as [`KIND_MONEY`].
/// [`WireError::BadScale`] when `w`'s scale does not match
/// [`exact_scale::MONEY_SCALE`]. [`WireError::Overflow`] on ceiling breach.
pub fn money_from_wire( w : Wire ) -> Result< Money, WireError >
{
  check_header( w, KIND_MONEY )?;
  Money::from_minor( w.minor ).map_err( kind_error_to_wire_error )
}

/// Encode a quantity to its wire form.
#[ must_use ]
pub fn qty_to_wire( v : Quantity ) -> Wire
{
  Wire { minor : v.minor(), scale : SCALE_BYTE, kind : KIND_QTY }
}

/// Decode a quantity from its wire form.
///
/// # Errors
///
/// [`WireError::BadKind`] when `w` was not encoded as [`KIND_QTY`].
/// [`WireError::BadScale`] on a scale mismatch. [`WireError::Negative`] when
/// the decoded value is below zero. [`WireError::Overflow`] on ceiling
/// breach.
pub fn qty_from_wire( w : Wire ) -> Result< Quantity, WireError >
{
  check_header( w, KIND_QTY )?;
  Quantity::from_minor( w.minor ).map_err( kind_error_to_wire_error )
}

/// Encode a price to its wire form.
#[ must_use ]
pub fn price_to_wire( v : Price ) -> Wire
{
  Wire { minor : v.minor(), scale : SCALE_BYTE, kind : KIND_PRICE }
}

/// Decode a price from its wire form.
///
/// # Errors
///
/// As [`money_from_wire`], checked against [`KIND_PRICE`] instead.
pub fn price_from_wire( w : Wire ) -> Result< Price, WireError >
{
  check_header( w, KIND_PRICE )?;
  Price::from_minor( w.minor ).map_err( kind_error_to_wire_error )
}
