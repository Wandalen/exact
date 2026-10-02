//! The conserved value type family: `Money`, `Qty`, and `Price`, each a
//! fixed-point decimal whose scale lives in the type.
//!
//! Tier 1 of the family, depending on `exact_minor` for the backing integer
//! and `exact_scale` for the power-of-ten table and the declared ceiling.
//! Consolidates what the real codebase built as two separate crates —
//! `exact_decimal`'s signed `Decimal< SCALE >` and `exact_qty`'s
//! non-negative `Qty< SCALE >` — into the one crate the preferred design
//! names `exact_kind`, carrying both representations forward unchanged in
//! behaviour.
//!
//! # Disclosed deviations from the preferred design's own type listing
//!
//! - **`Money` and `Price` are the same type today.** The preferred design
//!   lists `Money`, `Qty` and `Price` as three independent structs. `Qty`
//!   earns that independence: its non-negativity refusal is real,
//!   behaviourally-distinguishing logic worth a separate type. `Price` does
//!   not yet — it has no real consumer anywhere in this codebase and no
//!   behaviour that differs from `Money`, so hand-duplicating `Decimal`'s
//!   entire method surface onto a second wrapper purely to make them
//!   nominally distinct would be speculative work against the Approach
//!   Gate's YAGNI check, done before any concrete need demonstrates what
//!   that distinction should even enforce. `Price` is therefore a plain
//!   alias of `Decimal< MONEY_SCALE >`, exactly like `Money`, until a real
//!   consumer's requirement gives the distinction content.
//! - **No `Scaled` trait.** The preferred design's trait returns a runtime
//!   `Scale` from a value — a shape built for the runtime `Scale(u8)`
//!   representation this family's migration plan explicitly rejected in
//!   favour of keeping scale a compile-time const generic (every call site
//!   already knows `SCALE` statically, which is most of why the trait would
//!   add little). Deferred rather than built against a representation this
//!   crate does not use.
//! - **`KindError` carries no `ScaleMismatch`.** Two `Decimal< SCALE >`
//!   values of different `SCALE` are different Rust types and cannot reach
//!   a shared function to be compared or combined at all — the compiler
//!   refuses it before any runtime check could run. An error variant with
//!   no reachable construction site is dead code wearing a doc comment; the
//!   same reasoning already applies to `exact_cmp`'s unconditional derived
//!   `Ord`. In its place, `KindError::Negative` carries `exact_qty`'s one
//!   real refusal forward, per this migration's decision to enforce
//!   non-negativity at construction rather than later at arithmetic time.
//!
//! # What stays unchanged
//!
//! Every checked operation, the parser, the renderer, and the declared
//! ceiling's headroom relation are ported from `exact_decimal` and
//! `exact_qty` without behavioural change — only the backing alias and the
//! scale constants now come from `exact_minor` and `exact_scale` rather
//! than being declared again here.

use core::fmt;
use exact_minor::Backing;
use exact_scale::{ CEILING_MINOR_UNITS, MONEY_SCALE, pow10 };

/// A value at the standard money scale.
pub type Money = Decimal< MONEY_SCALE >;

/// A price at the standard money scale.
///
/// Identical to [`Money`] today — see the module-level disclosed deviation
/// on why this is a plain alias rather than a hand-duplicated wrapper.
pub type Price = Decimal< MONEY_SCALE >;

/// A non-negative quantity at the standard money scale.
pub type Quantity = Qty< MONEY_SCALE >;

/// Why a value could not be constructed, or an operation could not be completed.
///
/// Every variant names the specific condition rather than a generic failure:
/// a conserved value that goes wrong is going to be investigated, and an
/// error that says only "invalid" moves the investigation to the debugger.
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
pub enum KindError
{
  /// An operation left the representable range of the backing width.
  Overflow
  {
    /// Which operation — `add`, `sub`, `mul_int`, `neg`, `parse`, `from_int`.
    operation : &'static str,
  },
  /// A value inside the backing width but outside the declared ceiling.
  ///
  /// Distinct from [`Overflow`](KindError::Overflow) on purpose: the
  /// ceiling is a deployment's own declared limit, and a value that
  /// breaches it is a different report from one the arithmetic could not
  /// represent.
  ExceedsCeiling
  {
    /// The offending count of minor units.
    minor : Backing,
  },
  /// A decimal string with more fractional digits than the type's scale.
  ///
  /// Rejected rather than repaired. Truncating destroys the excess
  /// silently; rounding invents a value the caller did not write.
  ExcessPrecision
  {
    /// Fractional digits the text carried.
    digits : u32,
    /// Fractional digits the type can hold.
    scale : u32,
  },
  /// Text the grammar does not accept.
  Malformed
  {
    /// What specifically was wrong.
    reason : &'static str,
  },
  /// The result would have been below zero, for a kind that refuses it.
  ///
  /// Distinct from [`Overflow`](KindError::Overflow): the arithmetic
  /// succeeded and produced a perfectly representable value, which a
  /// non-negative kind refuses to hold. Reporting it as an overflow would
  /// send an investigation to the range budget instead of to the
  /// withdrawal that asked for more than was there.
  Negative
  {
    /// The count of minor units the operation would have produced.
    minor : Backing,
  },
}

impl fmt::Display for KindError
{
  fn fmt( &self, f : &mut fmt::Formatter< '_ > ) -> fmt::Result
  {
    match self
    {
      Self::Overflow { operation } => write!( f, "{operation} left the representable range" ),
      Self::ExceedsCeiling { minor } => write!( f, "{minor} minor units exceeds the declared ceiling {CEILING_MINOR_UNITS}" ),
      Self::ExcessPrecision { digits, scale } => write!( f, "{digits} fractional digits into a type of scale {scale}" ),
      Self::Malformed { reason } => write!( f, "malformed decimal: {reason}" ),
      Self::Negative { minor } => write!( f, "{minor} minor units is below zero, which this kind cannot hold" ),
    }
  }
}

impl core::error::Error for KindError {}

/// A fixed-point decimal carrying its scale in its type.
///
/// `SCALE` is the number of decimal places. The stored integer counts minor
/// units, so the denoted value is `minor × 10⁻ˢᶜᵃˡᵉ`.
///
/// ```
/// use exact_kind::Decimal;
///
/// let a : Decimal< 6 > = Decimal::parse( "0.1" ).unwrap();
/// let b : Decimal< 6 > = Decimal::parse( "0.2" ).unwrap();
/// assert_eq!( a.checked_add( b ).unwrap(), Decimal::parse( "0.3" ).unwrap() );
/// ```
#[ derive( Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash ) ]
pub struct Decimal< const SCALE : u32 >
{
  minor : Backing,
}

impl< const SCALE : u32 > Decimal< SCALE >
{
  /// One whole unit, in minor units.
  pub const ONE_MINOR : Backing = pow10( SCALE );

  /// Zero — the one infallible constructor, representable at every scale.
  pub const ZERO : Self = Self { minor : 0 };

  /// The smallest non-zero magnitude this type can express.
  pub const EPSILON : Self = Self { minor : 1 };

  /// The largest value this type can hold — exactly the declared ceiling.
  ///
  /// The clamp target for saturating arithmetic: a wider clamp (to the raw
  /// backing width rather than the declared ceiling) would produce a minor
  /// count this type's own `from_minor` would refuse to hold.
  pub const MAX : Self = Self { minor : CEILING_MINOR_UNITS };

  /// The smallest (most negative) value this type can hold.
  pub const MIN : Self = Self { minor : -CEILING_MINOR_UNITS };

  /// Build from a count of minor units.
  ///
  /// # Errors
  ///
  /// [`KindError::ExceedsCeiling`] when `|minor|` is past the ceiling.
  pub const fn from_minor( minor : Backing ) -> Result< Self, KindError >
  {
    if minor > CEILING_MINOR_UNITS || minor < -CEILING_MINOR_UNITS
    {
      return Err( KindError::ExceedsCeiling { minor } );
    }
    Ok( Self { minor } )
  }

  /// Build from a whole number of units.
  ///
  /// # Errors
  ///
  /// [`KindError::Overflow`] when scaling the integer leaves the backing
  /// width, and [`KindError::ExceedsCeiling`] when the result is past the
  /// declared ceiling.
  pub const fn from_int( whole : Backing ) -> Result< Self, KindError >
  {
    let Some( minor ) = whole.checked_mul( Self::ONE_MINOR )
    else
    {
      return Err( KindError::Overflow { operation : "from_int" } );
    };
    Self::from_minor( minor )
  }

  /// The count of minor units this value holds.
  #[ must_use ]
  pub const fn minor( self ) -> Backing
  {
    self.minor
  }

  /// The whole-unit part, truncated toward zero.
  #[ must_use ]
  pub const fn whole( self ) -> Backing
  {
    self.minor / Self::ONE_MINOR
  }

  /// Add two values of the same scale.
  ///
  /// # Errors
  ///
  /// [`KindError::ExceedsCeiling`] on breaching the declared ceiling —
  /// the error this operation actually returns for any operand pair
  /// obtainable through this type's public API.
  pub const fn checked_add( self, rhs : Self ) -> Result< Self, KindError >
  {
    let Some( minor ) = self.minor.checked_add( rhs.minor )
    else
    {
      return Err( KindError::Overflow { operation : "add" } );
    };
    Self::from_minor( minor )
  }

  /// Subtract two values of the same scale.
  ///
  /// # Errors
  ///
  /// As [`checked_add`](Self::checked_add).
  pub const fn checked_sub( self, rhs : Self ) -> Result< Self, KindError >
  {
    let Some( minor ) = self.minor.checked_sub( rhs.minor )
    else
    {
      return Err( KindError::Overflow { operation : "sub" } );
    };
    Self::from_minor( minor )
  }

  /// Multiply by a dimensionless integer, holding the scale.
  ///
  /// # Errors
  ///
  /// [`KindError::Overflow`] when `self.minor() * n` leaves the backing
  /// width. [`KindError::ExceedsCeiling`] when an in-width product still
  /// breaches the declared ceiling.
  pub const fn checked_mul_int( self, n : Backing ) -> Result< Self, KindError >
  {
    let Some( minor ) = self.minor.checked_mul( n )
    else
    {
      return Err( KindError::Overflow { operation : "mul_int" } );
    };
    Self::from_minor( minor )
  }

  /// Negate.
  ///
  /// # Errors
  ///
  /// [`KindError::Overflow`] is checked for defensively but is currently
  /// unreachable through this type: the only backing value that fails to
  /// negate is `Backing::MIN`, and the declared ceiling keeps every
  /// constructible value's magnitude far below that edge.
  pub const fn checked_neg( self ) -> Result< Self, KindError >
  {
    let Some( minor ) = self.minor.checked_neg()
    else
    {
      return Err( KindError::Overflow { operation : "neg" } );
    };
    Self::from_minor( minor )
  }

  /// Parse a decimal string exactly, or say why it cannot be.
  ///
  /// The grammar is deliberately narrow and total: an optional sign, at
  /// least one integer digit, and an optional fractional part of at most
  /// `SCALE` digits.
  ///
  /// # Errors
  ///
  /// [`KindError::Malformed`] for text outside the grammar,
  /// [`KindError::ExcessPrecision`] for more fractional digits than the
  /// type holds, and the range errors of [`from_minor`](Self::from_minor).
  pub fn parse( text : &str ) -> Result< Self, KindError >
  {
    let ( negative, digits ) = match text.strip_prefix( '-' )
    {
      Some( rest ) => ( true, rest ),
      None => ( false, text.strip_prefix( '+' ).unwrap_or( text ) ),
    };

    let ( int_part, frac_part ) = match digits.split_once( '.' )
    {
      Some( ( _, "" ) ) => return Err( KindError::Malformed { reason : "no digit after the decimal point" } ),
      Some( ( i, f ) ) => ( i, f ),
      None => ( digits, "" ),
    };

    if int_part.is_empty()
    {
      return Err( KindError::Malformed { reason : "no integer digit" } );
    }
    if !int_part.bytes().all( | b | b.is_ascii_digit() )
    {
      return Err( KindError::Malformed { reason : "non-digit in integer part" } );
    }
    if !frac_part.bytes().all( | b | b.is_ascii_digit() )
    {
      return Err( KindError::Malformed { reason : "non-digit in fractional part" } );
    }

    let supplied = u32::try_from( frac_part.len() ).unwrap_or( u32::MAX );
    if supplied > SCALE
    {
      return Err( KindError::ExcessPrecision { digits : supplied, scale : SCALE } );
    }

    let whole : Backing = int_part
    .parse()
    .map_err( | _ | KindError::Overflow { operation : "parse" } )?;

    let frac : Backing = if frac_part.is_empty()
    {
      0
    }
    else
    {
      frac_part
      .parse::< Backing >()
      .map_err( | _ | KindError::Overflow { operation : "parse" } )?
      * pow10( SCALE - supplied )
    };

    let magnitude = whole
    .checked_mul( Self::ONE_MINOR )
    .and_then( | m | m.checked_add( frac ) )
    .ok_or( KindError::Overflow { operation : "parse" } )?;

    Self::from_minor( if negative { -magnitude } else { magnitude } )
  }
}

impl< const SCALE : u32 > fmt::Display for Decimal< SCALE >
{
  /// Render exactly, with trailing fractional zeros trimmed.
  fn fmt( &self, f : &mut fmt::Formatter< '_ > ) -> fmt::Result
  {
    let unit = Self::ONE_MINOR;
    let magnitude = self.minor.unsigned_abs();
    let unit_u = unit.unsigned_abs();
    let whole = magnitude / unit_u;
    let frac = magnitude % unit_u;

    if self.minor < 0
    {
      write!( f, "-" )?;
    }
    write!( f, "{whole}" )?;

    if SCALE == 0 || frac == 0
    {
      return Ok( () );
    }
    let trailing_zeros = ( 1..=SCALE ).take_while( | &k | frac.is_multiple_of( 10_u64.pow( k ) ) ).count();
    write!( f, ".{:0width$}", frac / 10_u64.pow( trailing_zeros as u32 ), width = SCALE as usize - trailing_zeros )
  }
}

/// A non-negative quantity at a type-level scale.
///
/// ```
/// use exact_kind::Quantity;
///
/// let held = Quantity::from_int( 3 ).unwrap();
/// let taken = Quantity::from_int( 5 ).unwrap();
/// assert!( held.checked_sub( taken ).is_err() );
/// ```
#[ derive( Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash ) ]
pub struct Qty< const SCALE : u32 >
{
  value : Decimal< SCALE >,
}

impl< const SCALE : u32 > Qty< SCALE >
{
  /// Nothing held — the one infallible constructor.
  pub const ZERO : Self = Self { value : Decimal::ZERO };

  /// The smallest non-zero quantity this type can express.
  pub const EPSILON : Self = Self { value : Decimal::EPSILON };

  /// The largest quantity this type can hold — exactly the declared ceiling.
  pub const MAX : Self = Self { value : Decimal::MAX };

  /// Wrap a decimal, refusing a negative one.
  ///
  /// # Errors
  ///
  /// [`KindError::Negative`] when the decimal is below zero.
  pub const fn from_decimal( value : Decimal< SCALE > ) -> Result< Self, KindError >
  {
    if value.minor() < 0
    {
      return Err( KindError::Negative { minor : value.minor() } );
    }
    Ok( Self { value } )
  }

  /// Build from a count of minor units.
  ///
  /// # Errors
  ///
  /// [`KindError::Negative`] below zero, or the range errors of
  /// [`Decimal::from_minor`].
  pub const fn from_minor( minor : Backing ) -> Result< Self, KindError >
  {
    match Decimal::from_minor( minor )
    {
      Ok( value ) => Self::from_decimal( value ),
      Err( e ) => Err( e ),
    }
  }

  /// Build from a whole number of units.
  ///
  /// # Errors
  ///
  /// As [`from_minor`](Self::from_minor), plus overflow while scaling.
  pub const fn from_int( whole : Backing ) -> Result< Self, KindError >
  {
    match Decimal::from_int( whole )
    {
      Ok( value ) => Self::from_decimal( value ),
      Err( e ) => Err( e ),
    }
  }

  /// The decimal beneath, for arithmetic that legitimately leaves the type.
  ///
  /// Not a hole in the invariant: what comes back is a signed decimal, and
  /// getting a quantity out of it again means passing back through
  /// [`from_decimal`](Self::from_decimal), which is where the refusal lives.
  #[ must_use ]
  pub const fn as_decimal( self ) -> Decimal< SCALE >
  {
    self.value
  }

  /// The count of minor units held.
  #[ must_use ]
  pub const fn minor( self ) -> Backing
  {
    self.value.minor()
  }

  /// The whole-unit part, truncated.
  #[ must_use ]
  pub const fn whole( self ) -> Backing
  {
    self.value.whole()
  }

  /// Add two quantities.
  ///
  /// # Errors
  ///
  /// Range errors on overflow or ceiling breach. Cannot produce
  /// [`KindError::Negative`] — two non-negative values do not sum below
  /// zero.
  pub const fn checked_add( self, rhs : Self ) -> Result< Self, KindError >
  {
    match self.value.checked_add( rhs.value )
    {
      Ok( value ) => Self::from_decimal( value ),
      Err( e ) => Err( e ),
    }
  }

  /// Subtract, refusing to go below zero.
  ///
  /// # Errors
  ///
  /// [`KindError::Negative`] when `rhs` exceeds `self`.
  pub const fn checked_sub( self, rhs : Self ) -> Result< Self, KindError >
  {
    match self.value.checked_sub( rhs.value )
    {
      Ok( value ) => Self::from_decimal( value ),
      Err( e ) => Err( e ),
    }
  }

  /// Multiply by a dimensionless non-negative integer.
  ///
  /// # Errors
  ///
  /// [`KindError::Negative`] when the product would be negative. Range
  /// errors on overflow or ceiling breach.
  pub const fn checked_mul_int( self, n : Backing ) -> Result< Self, KindError >
  {
    match self.value.checked_mul_int( n )
    {
      Ok( value ) => Self::from_decimal( value ),
      Err( e ) => Err( e ),
    }
  }

  /// Parse a decimal string, refusing a negative one.
  ///
  /// # Errors
  ///
  /// As [`Decimal::parse`], plus [`KindError::Negative`].
  pub fn parse( text : &str ) -> Result< Self, KindError >
  {
    Self::from_decimal( Decimal::parse( text )? )
  }
}

impl< const SCALE : u32 > fmt::Display for Qty< SCALE >
{
  fn fmt( &self, f : &mut fmt::Formatter< '_ > ) -> fmt::Result
  {
    write!( f, "{}", self.value )
  }
}
