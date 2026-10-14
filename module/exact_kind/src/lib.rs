//! The conserved value type family: `Money`, `Qty`, and `Price`, each a
//! fixed-point decimal whose scale lives in the type.
//!
//! Tier 1 of the family, depending on `exact_minor` for the backing integer,
//! `exact_scale` for powers of ten and the ceiling, `exact_round` for `rescale`.
//! Consolidates what the real codebase built as two separate crates —
//! `exact_decimal`'s signed `Decimal< SCALE >` and `exact_qty`'s
//! non-negative `Qty< SCALE >` — into the one crate the preferred design
//! names `exact_kind`, carrying both representations forward unchanged in
//! behaviour.
//!
//! # Disclosed deviations from the preferred design's own type listing
//!
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
//! `exact_qty` without behavioural change. What they are built on now comes
//! from the two Tier-0 crates rather than being declared again here: the
//! stored count is an `exact_minor::Minor`, added, subtracted and negated by
//! `exact_minor`'s own checked functions, and the scale constants and powers
//! of ten come from `exact_scale`.

use core::fmt;
use core::marker::PhantomData;
use exact_minor::
{
  Backing,
  Minor,
  MinorError,
  minor_checked_add,
  minor_checked_neg,
  minor_checked_sub,
  minor_from_i64,
  minor_to_i64,
  minor_zero,
};
use exact_round::{ RoundError, Rounding, round_div };
use exact_scale::{ CEILING_MINOR_UNITS, MONEY_SCALE, pow10 };

/// A value at the standard money scale.
pub type Money = Decimal< MONEY_SCALE >;

/// A non-negative quantity at the standard money scale.
pub type Quantity = Qty< MONEY_SCALE >;

/// A price at the standard money scale.
pub type Price = PriceAt< MONEY_SCALE >;

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
  /// A value with more fractional digits than the target scale can hold:
  /// text given to `parse`, or a value `rescale` narrows under `Exact`.
  /// Rejected rather than repaired — truncating destroys the excess
  /// silently; rounding invents a value the caller did not ask for.
  ExcessPrecision
  {
    /// Fractional digits the input carried: as typed, or the value's own.
    digits : u32,
    /// Fractional digits the target type can hold.
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

mod sealed
{
  use super::{ Amount, KindError };

  /// Keeps the set of kinds closed — only this crate can add one — and carries
  /// each kind's own `checked_add` and `checked_sub` to the one generic `+` and
  /// `-`, out of reach outside this crate.
  pub trait Sealed : Sized
  {
    /// The name `Debug` prints for this kind — the alias a caller writes.
    const NAME : &'static str;
    /// This kind's own `checked_add`.
    fn checked_add< const SCALE : u32 >( a : Amount< Self, SCALE >, b : Amount< Self, SCALE > ) -> Result< Amount< Self, SCALE >, KindError >;
    /// This kind's own `checked_sub`.
    fn checked_sub< const SCALE : u32 >( a : Amount< Self, SCALE >, b : Amount< Self, SCALE > ) -> Result< Amount< Self, SCALE >, KindError >;
  }
}

/// One kind of amount — [`MoneyKind`], [`QtyKind`] or [`PriceKind`], the only
/// three, as `Kind` is sealed. Two kinds are two [`Amount`] types and never mix.
/// `Copy + Ord + Hash + Debug`, so generic code over `K : Kind` gets `Amount`'s.
///
/// No crate but this one can add a kind — a fourth one could bring its own
/// `checked_add`, past the ceiling and the non-negativity checks:
///
/// ```compile_fail
/// use exact_kind::Kind;
/// #[ derive( Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash ) ]
/// struct Loose;
/// impl Kind for Loose {}
/// ```
///
/// And the hooks that take each kind's own `checked_add` to `+` stay inside
/// it — they are not another public way to add:
///
/// ```compile_fail
/// use exact_kind::{ Money, MoneyKind };
/// let _ = MoneyKind::checked_add( Money::ZERO, Money::ZERO );
/// ```
pub trait Kind : sealed::Sealed + Copy + Ord + core::hash::Hash + fmt::Debug {}

/// The kind of [`Decimal`]: money, which may be negative.
#[ derive( Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash ) ]
pub struct MoneyKind;

/// The kind of [`Qty`]: a quantity, never below zero.
#[ derive( Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash ) ]
pub struct QtyKind;

/// The kind of [`PriceAt`]: a price, which may be negative — a discount.
#[ derive( Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash ) ]
pub struct PriceKind;

impl Kind for MoneyKind {}
impl Kind for QtyKind {}
impl Kind for PriceKind {}

/// A fixed-point amount of kind `K`, carrying its scale in its type.
///
/// One struct for every kind: [`Decimal`], [`Qty`] and [`PriceAt`] are its
/// aliases, one per kind. The kind costs nothing at run time — `PhantomData`
/// is zero-sized, so every amount is one `i64` — but two kinds are two types,
/// so they still never mix.
#[ derive( Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash ) ]
pub struct Amount< K, const SCALE : u32 >
{
  minor : Minor,
  kind : PhantomData< K >,
}

impl< K, const SCALE : u32 > Amount< K, SCALE >
{
  /// The same count of minor units as money — what `Qty` and `PriceAt` held
  /// as their `value` field when each was a struct of its own.
  const fn value( self ) -> Decimal< SCALE >
  {
    Amount { minor : self.minor, kind : PhantomData }
  }

  /// Money's count of minor units as this kind, unchecked: each kind's own
  /// constructor decides when it may be called — for `Qty`, only after
  /// `from_decimal` has refused a negative value.
  const fn from_value( value : Decimal< SCALE > ) -> Self
  {
    Amount { minor : value.minor, kind : PhantomData }
  }
}

/// `Debug` names the kind — `Qty { minor: Minor(50000000) }` — not a derived
/// `Amount { minor: …, kind: PhantomData<…> }` that hides it in a type name. As
/// structs, `Qty` and `PriceAt` printed `Qty { value: Decimal { minor: … } }`.
impl< K : Kind, const SCALE : u32 > fmt::Debug for Amount< K, SCALE >
{
  fn fmt( &self, f : &mut fmt::Formatter< '_ > ) -> fmt::Result
  {
    f.debug_struct( K::NAME ).field( "minor", &self.minor ).finish()
  }
}

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
///
/// Two scales never mix — a scale-6 value plus a scale-2 value does not compile:
///
/// ```compile_fail
/// use exact_kind::Decimal;
/// let six : Decimal< 6 > = Decimal::from_int( 1 ).unwrap();
/// let two : Decimal< 2 > = Decimal::from_int( 1 ).unwrap();
/// let _ = six.checked_add( two );
/// ```
pub type Decimal< const SCALE : u32 > = Amount< MoneyKind, SCALE >;

/// Report a failure of `exact_minor`'s arithmetic as this crate's own
/// overflow, keeping the name of the operation that failed.
const fn kind_overflow( e : MinorError ) -> KindError
{
  match e
  {
    MinorError::Overflow { operation } | MinorError::Underflow { operation } =>
      KindError::Overflow { operation },
  }
}

impl< const SCALE : u32 > Decimal< SCALE >
{
  /// One whole unit, in minor units.
  pub const ONE_MINOR : Backing = pow10( SCALE );

  /// Fails the build for a scale whose whole unit, `10^SCALE`, does not fit
  /// the backing integer — `pow10`'s own limit of 18 places. Every way to
  /// build a `Decimal` evaluates it, so a scale past 18 cannot be built.
  const REPRESENTABLE : () = assert!( Self::ONE_MINOR > 0 );

  /// Zero — the one infallible constructor, representable at every valid scale.
  pub const ZERO : Self = { let () = Self::REPRESENTABLE; Self { minor : minor_zero(), kind : PhantomData } };

  /// The smallest non-zero magnitude this type can express.
  pub const EPSILON : Self = { let () = Self::REPRESENTABLE; Self { minor : minor_from_i64( 1 ), kind : PhantomData } };

  /// The largest value this type can hold — exactly the declared ceiling.
  ///
  /// The clamp target for saturating arithmetic: a wider clamp (to the raw
  /// backing width rather than the declared ceiling) would produce a minor
  /// count this type's own `from_minor` would refuse to hold.
  pub const MAX : Self = { let () = Self::REPRESENTABLE; Self { minor : minor_from_i64( CEILING_MINOR_UNITS ), kind : PhantomData } };

  /// The smallest (most negative) value this type can hold.
  pub const MIN : Self = { let () = Self::REPRESENTABLE; Self { minor : minor_from_i64( -CEILING_MINOR_UNITS ), kind : PhantomData } };

  /// Build from a count of minor units.
  ///
  /// # Errors
  ///
  /// [`KindError::ExceedsCeiling`] when `|minor|` is past the ceiling.
  pub const fn from_minor( minor : Backing ) -> Result< Self, KindError >
  {
    let () = Self::REPRESENTABLE;
    if minor > CEILING_MINOR_UNITS || minor < -CEILING_MINOR_UNITS
    {
      return Err( KindError::ExceedsCeiling { minor } );
    }
    Ok( Self { minor : minor_from_i64( minor ), kind : PhantomData } )
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
    minor_to_i64( self.minor )
  }

  /// The same value at scale `TO`. Going finer multiplies by `10^(TO - SCALE)`
  /// and can only overflow; going coarser divides by `10^(SCALE - TO)` and
  /// rounds per `rounding` — [`Rounding::Exact`] refuses any digit that would
  /// be lost.
  ///
  /// `TO` is at most 18, like every scale: a coarser one does not compile.
  ///
  /// ```compile_fail
  /// use exact_kind::Decimal;
  /// use exact_round::Rounding;
  /// let one = Decimal::< 6 >::from_minor( 1 ).unwrap();
  /// let _ = one.rescale::< 20 >( Rounding::Exact ); // 10^20 does not fit the backing integer
  /// ```
  ///
  /// # Errors
  ///
  /// [`KindError::Overflow`] or [`KindError::ExceedsCeiling`] when the value
  /// does not fit at scale `TO`. [`KindError::ExcessPrecision`] when
  /// `rounding` is [`Rounding::Exact`] and going coarser would drop a digit.
  pub fn rescale< const TO : u32 >( self, rounding : Rounding ) -> Result< Decimal< TO >, KindError >
  {
    let minor = self.minor();
    let rescaled = if TO >= SCALE
    {
      minor.checked_mul( pow10( TO - SCALE ) ).ok_or( KindError::Overflow { operation : "rescale" } )?
    }
    else
    {
      round_div( minor, pow10( SCALE - TO ), rounding ).map_err( | e | match e
      {
        RoundError::Inexact => KindError::ExcessPrecision { digits : fraction_digits( minor, SCALE ), scale : TO },
        RoundError::DivZero | RoundError::Overflow => KindError::Overflow { operation : "rescale" },
      } )?
    };
    Decimal::< TO >::from_minor( rescaled )
  }


  /// The whole-unit part, truncated toward zero.
  #[ must_use ]
  pub const fn whole( self ) -> Backing
  {
    self.minor() / Self::ONE_MINOR
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
    match minor_checked_add( self.minor, rhs.minor )
    {
      Ok( sum ) => Self::from_minor( minor_to_i64( sum ) ),
      Err( e ) => Err( kind_overflow( e ) ),
    }
  }

  /// Subtract two values of the same scale.
  ///
  /// # Errors
  ///
  /// As [`checked_add`](Self::checked_add).
  pub const fn checked_sub( self, rhs : Self ) -> Result< Self, KindError >
  {
    match minor_checked_sub( self.minor, rhs.minor )
    {
      Ok( diff ) => Self::from_minor( minor_to_i64( diff ) ),
      Err( e ) => Err( kind_overflow( e ) ),
    }
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
    let Some( minor ) = self.minor().checked_mul( n )
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
    match minor_checked_neg( self.minor )
    {
      Ok( neg ) => Self::from_minor( minor_to_i64( neg ) ),
      Err( e ) => Err( kind_overflow( e ) ),
    }
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
  /// Render exactly, with trailing fractional zeros trimmed — or, when the
  /// caller asks for a precision (`{:.2}`), with exactly that many digits
  /// after the point. Width, fill, alignment, `+` and `0` are honoured the way
  /// they are for integers: `{:>10.2}` of `1.5` is `      1.50`. A precision
  /// past the scale pads zeros that `parse` at this scale refuses: `{:.4}` of a
  /// `Decimal< 2 >` `1.5` is `1.5000`, which `Decimal::< 2 >::parse` rejects as
  /// `ExcessPrecision`.
  fn fmt( &self, f : &mut fmt::Formatter< '_ > ) -> fmt::Result
  {
    render( self.minor(), SCALE, f )
  }
}

/// The one renderer behind `Display` for `Decimal`, `Qty` and `PriceAt`.
///
/// The digits after the point are the scale's own with trailing zeros trimmed
/// for `{}`, or exactly `places` for `{:.N}`: more places than the scale holds
/// are zeros, fewer round the dropped digits half to even — the family's
/// default, and the rule `{:.N}` follows for `f64`. Only the text is rounded,
/// never the value. The sign is taken after rounding, so `-0.004` at `{:.2}`
/// is `0.00`.
///
/// Then the text is padded as `Formatter::pad_integral` pads an integer: `+`
/// adds a sign to a non-negative value, `0` puts zeros between the sign and
/// the digits, and otherwise the fill goes where the alignment says, right by
/// default. Everything is written straight into the formatter — no buffer, no
/// allocation, and no limit on the precision.
fn render( minor : Backing, scale : u32, f : &mut fmt::Formatter< '_ > ) -> fmt::Result
{
  let places = f.precision();
  // Fewer places than the scale holds: round the dropped digits away first.
  let ( shown, shown_scale ) = match places.map( u32::try_from )
  {
    Some( Ok( p ) ) if p < scale =>
    (
      round_div( minor, pow10( scale - p ), Rounding::HalfEven ).map_err( | _ | fmt::Error )?,
      p,
    ),
    _ => ( minor, scale ),
  };
  let unit = pow10( shown_scale ).unsigned_abs();
  let magnitude = shown.unsigned_abs();
  let whole = magnitude / unit;
  let ( frac, frac_width, extra_zeros ) = match places
  {
    // `{:.N}`: every digit the shown scale holds, then zeros up to `N`.
    Some( places ) => ( magnitude % unit, shown_scale as usize, places.saturating_sub( shown_scale as usize ) ),
    None =>
    {
      // Fix(exact_kind_display_allocated_per_render): the fraction was padded
      // into a `String` with `format!` and then trimmed — one heap allocation
      // per render, against feature 016's non-allocating display. The trailing
      // zeros are now counted arithmetically and the digits written directly.
      //
      // Root cause: `format!` used as a scratch buffer inside `fmt`.
      // Pitfall: `write!` into the formatter does not allocate but `format!`
      //   does, and the rendered text is identical — output tests cannot tell.
      let frac = magnitude % unit;
      let trailing_zeros = if frac == 0
      {
        shown_scale as usize
      }
      else
      {
        ( 1..=shown_scale ).take_while( | &k | frac.is_multiple_of( pow10( k ).unsigned_abs() ) ).count()
      };
      ( frac / pow10( trailing_zeros as u32 ).unsigned_abs(), shown_scale as usize - trailing_zeros, 0 )
    }
  };
  let point = frac_width + extra_zeros > 0;
  let sign = if shown < 0 { Some( '-' ) } else if f.sign_plus() { Some( '+' ) } else { None };
  let len = usize::from( sign.is_some() )
  + whole.checked_ilog10().map_or( 1, | l | l as usize + 1 )
  + if point { 1 + frac_width + extra_zeros } else { 0 };
  let padding = f.width().map_or( 0, | w | w.saturating_sub( len ) );
  let ( before, after, fill ) = if f.sign_aware_zero_pad()
  {
    ( 0, 0, '0' )
  }
  else
  {
    match f.align()
    {
      Some( fmt::Alignment::Left ) => ( 0, padding, f.fill() ),
      Some( fmt::Alignment::Center ) => ( padding / 2, padding.div_ceil( 2 ), f.fill() ),
      Some( fmt::Alignment::Right ) | None => ( padding, 0, f.fill() ),
    }
  };
  for _ in 0..before
  {
    fmt::Write::write_char( f, fill )?;
  }
  if let Some( sign ) = sign
  {
    fmt::Write::write_char( f, sign )?;
  }
  // `0`: the zeros go between the sign and the digits.
  if f.sign_aware_zero_pad()
  {
    for _ in 0..padding
    {
      f.write_str( "0" )?;
    }
  }
  write!( f, "{whole}" )?;
  if point
  {
    f.write_str( "." )?;
    if frac_width > 0
    {
      write!( f, "{frac:0frac_width$}" )?;
    }
    // More places than the scale holds: the rest are zeros.
    for _ in 0..extra_zeros
    {
      f.write_str( "0" )?;
    }
  }
  for _ in 0..after
  {
    fmt::Write::write_char( f, fill )?;
  }
  Ok( () )
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
pub type Qty< const SCALE : u32 > = Amount< QtyKind, SCALE >;

impl< const SCALE : u32 > Qty< SCALE >
{
  /// Nothing held — the one infallible constructor.
  pub const ZERO : Self = Self::from_value( Decimal::ZERO );

  /// The smallest non-zero quantity this type can express.
  pub const EPSILON : Self = Self::from_value( Decimal::EPSILON );

  /// The largest quantity this type can hold — exactly the declared ceiling.
  pub const MAX : Self = Self::from_value( Decimal::MAX );

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
    Ok( Self::from_value( value ) )
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
    self.value()
  }

  /// The count of minor units held.
  #[ must_use ]
  pub const fn minor( self ) -> Backing
  {
    self.value().minor()
  }

  /// The same quantity at scale `TO`, as [`Decimal::rescale`] — still
  /// non-negative, since every mode rounds a non-negative value to one.
  ///
  /// # Errors
  ///
  /// As [`Decimal::rescale`].
  pub fn rescale< const TO : u32 >( self, rounding : Rounding ) -> Result< Qty< TO >, KindError >
  {
    Qty::from_decimal( self.value().rescale::< TO >( rounding )? )
  }


  /// The whole-unit part, truncated.
  #[ must_use ]
  pub const fn whole( self ) -> Backing
  {
    self.value().whole()
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
    match self.value().checked_add( rhs.value() )
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
    match self.value().checked_sub( rhs.value() )
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
    match self.value().checked_mul_int( n )
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

/// Renders a quantity exactly as its underlying decimal renders.
///
/// Arithmetic between a quantity and a money value does not compile: it is a
/// compile error, not a runtime one — the readme's
/// "non-interchangeable types" promise, which no runtime test can observe, so
/// the examples below pin it. (Money against [`Price`] is pinned on `Price`
/// itself.) Same-kind arithmetic compiles, which proves the failing examples
/// below fail only because they mix kinds:
///
/// ```
/// use exact_kind::{ Money, Quantity };
/// let cash = Money::from_int( 1 ).unwrap();
/// let shares = Quantity::from_int( 1 ).unwrap();
/// let _ = cash.checked_add( cash );
/// let _ = shares.checked_add( shares );
/// ```
///
/// Money plus a quantity does not compile:
///
/// ```compile_fail
/// use exact_kind::{ Money, Quantity };
/// let cash = Money::from_int( 1 ).unwrap();
/// let shares = Quantity::from_int( 1 ).unwrap();
/// let _ = cash.checked_add( shares );
/// ```
///
/// Nor a quantity plus money:
///
/// ```compile_fail
/// use exact_kind::{ Money, Quantity };
/// let cash = Money::from_int( 1 ).unwrap();
/// let shares = Quantity::from_int( 1 ).unwrap();
/// let _ = shares.checked_add( cash );
/// ```
///
/// Nor a quantity standing in for money:
///
/// ```compile_fail
/// use exact_kind::{ Money, Quantity };
/// let shares = Quantity::from_int( 1 ).unwrap();
/// let _ : Money = shares;
/// ```
impl< const SCALE : u32 > fmt::Display for Qty< SCALE >
{
  fn fmt( &self, f : &mut fmt::Formatter< '_ > ) -> fmt::Result
  {
    // Forward the formatter itself, not `"{}"`, so a precision reaches the decimal.
    fmt::Display::fmt( &self.value(), f )
  }
}

/// A price at scale `SCALE` — [`Price`] is the one at the standard money
/// scale.
///
/// A type of its own, so a price cannot stand in for money, or money for a
/// price. Like [`Money`] it may be negative — a discount. Same-kind
/// arithmetic compiles:
///
/// ```
/// use exact_kind::Price;
/// let price = Price::parse( "1.25" ).unwrap();
/// let _ = price.checked_add( price );
/// ```
///
/// Money plus a price does not compile:
///
/// ```compile_fail
/// use exact_kind::{ Money, Price };
/// let cash = Money::from_int( 1 ).unwrap();
/// let price = Price::parse( "1.25" ).unwrap();
/// let _ = cash.checked_add( price );
/// ```
///
/// Nor a price plus money:
///
/// ```compile_fail
/// use exact_kind::{ Money, Price };
/// let cash = Money::from_int( 1 ).unwrap();
/// let price = Price::parse( "1.25" ).unwrap();
/// let _ = price.checked_add( cash );
/// ```
///
/// Nor two prices at different scales:
///
/// ```compile_fail
/// use exact_kind::PriceAt;
/// let cents : PriceAt< 2 > = PriceAt::parse( "1.25" ).unwrap();
/// let fine : PriceAt< 6 > = PriceAt::parse( "1.25" ).unwrap();
/// let _ = cents.checked_add( fine );
/// ```
pub type PriceAt< const SCALE : u32 > = Amount< PriceKind, SCALE >;

impl< const SCALE : u32 > PriceAt< SCALE >
{
  /// No price at all.
  pub const ZERO : Self = Self::from_value( Decimal::< SCALE >::ZERO );

  /// The largest price this type can hold — exactly the declared ceiling.
  pub const MAX : Self = Self::from_value( Decimal::< SCALE >::MAX );

  /// Build from a count of minor units.
  ///
  /// # Errors
  ///
  /// As [`Decimal::from_minor`].
  pub const fn from_minor( minor : Backing ) -> Result< Self, KindError >
  {
    match Decimal::< SCALE >::from_minor( minor )
    {
      Ok( value ) => Ok( Self::from_value( value ) ),
      Err( e ) => Err( e ),
    }
  }

  /// The count of minor units this price holds.
  #[ must_use ]
  pub const fn minor( self ) -> Backing
  {
    self.value().minor()
  }

  /// Add two prices.
  ///
  /// # Errors
  ///
  /// As [`Decimal::checked_add`].
  pub const fn checked_add( self, rhs : Self ) -> Result< Self, KindError >
  {
    match self.value().checked_add( rhs.value() )
    {
      Ok( value ) => Ok( Self::from_value( value ) ),
      Err( e ) => Err( e ),
    }
  }

  /// Subtract two prices.
  ///
  /// # Errors
  ///
  /// As [`Decimal::checked_sub`].
  pub const fn checked_sub( self, rhs : Self ) -> Result< Self, KindError >
  {
    match self.value().checked_sub( rhs.value() )
    {
      Ok( value ) => Ok( Self::from_value( value ) ),
      Err( e ) => Err( e ),
    }
  }

  /// Parse a decimal string exactly, or say why it cannot be.
  ///
  /// # Errors
  ///
  /// As [`Decimal::parse`].
  pub fn parse( text : &str ) -> Result< Self, KindError >
  {
    Ok( Self::from_value( Decimal::< SCALE >::parse( text )? ) )
  }

  /// The same price at scale `TO`, as [`Decimal::rescale`].
  ///
  /// # Errors
  ///
  /// As [`Decimal::rescale`].
  pub fn rescale< const TO : u32 >( self, rounding : Rounding ) -> Result< PriceAt< TO >, KindError >
  {
    Ok( PriceAt::from_value( self.value().rescale::< TO >( rounding )? ) )
  }
}

/// Renders a price exactly as money of the same amount renders.
impl< const SCALE : u32 > fmt::Display for PriceAt< SCALE >
{
  fn fmt( &self, f : &mut fmt::Formatter< '_ > ) -> fmt::Result
  {
    // Forward the formatter itself, not `"{}"`, so a precision reaches the decimal.
    fmt::Display::fmt( &self.value(), f )
  }
}

// Each kind's own `checked_add` and `checked_sub`, handed to the one generic
// `+` and `-` below. The inherent methods stay where they are; these only
// point at them.
impl sealed::Sealed for MoneyKind
{
  const NAME : &'static str = "Decimal";
  fn checked_add< const SCALE : u32 >( a : Decimal< SCALE >, b : Decimal< SCALE > ) -> Result< Decimal< SCALE >, KindError > { a.checked_add( b ) }
  fn checked_sub< const SCALE : u32 >( a : Decimal< SCALE >, b : Decimal< SCALE > ) -> Result< Decimal< SCALE >, KindError > { a.checked_sub( b ) }
}

impl sealed::Sealed for QtyKind
{
  const NAME : &'static str = "Qty";
  fn checked_add< const SCALE : u32 >( a : Qty< SCALE >, b : Qty< SCALE > ) -> Result< Qty< SCALE >, KindError > { a.checked_add( b ) }
  fn checked_sub< const SCALE : u32 >( a : Qty< SCALE >, b : Qty< SCALE > ) -> Result< Qty< SCALE >, KindError > { a.checked_sub( b ) }
}

impl sealed::Sealed for PriceKind
{
  const NAME : &'static str = "PriceAt";
  fn checked_add< const SCALE : u32 >( a : PriceAt< SCALE >, b : PriceAt< SCALE > ) -> Result< PriceAt< SCALE >, KindError > { a.checked_add( b ) }
  fn checked_sub< const SCALE : u32 >( a : PriceAt< SCALE >, b : PriceAt< SCALE > ) -> Result< PriceAt< SCALE >, KindError > { a.checked_sub( b ) }
}

/// `+` and `-` for every kind at one scale — `Decimal`, `Qty` and `PriceAt` —
/// each that kind's own `checked_add`/`checked_sub` spelled as an operator. The
/// `Output` is a `Result`: nothing panics and nothing wraps, so a sum past the
/// ceiling, or a quantity below zero, is an error the caller must handle. The
/// impls with a `Result` on the left let a chain carry on: `( a + b + c )?`
/// takes one `?` and stops at the first error.
///
/// ```
/// use exact_kind::{ Decimal, KindError, Qty };
/// let a : Decimal< 2 > = Decimal::parse( "1.00" ).unwrap();
/// let b : Decimal< 2 > = Decimal::parse( "2.00" ).unwrap();
/// let c : Decimal< 2 > = Decimal::parse( "0.50" ).unwrap();
/// assert_eq!( ( a + b + c ).unwrap(), Decimal::parse( "3.50" ).unwrap() );
/// let one = Qty::< 8 >::parse( "1" ).unwrap();
/// let two = Qty::< 8 >::parse( "2" ).unwrap();
/// assert!( matches!( one - two, Err( KindError::Negative { .. } ) ) );
/// ```
///
/// Two scales, or two kinds, do not add:
///
/// ```compile_fail
/// use exact_kind::Decimal;
/// let cents : Decimal< 2 > = Decimal::parse( "1.50" ).unwrap();
/// let sats : Decimal< 8 > = Decimal::parse( "0.00000001" ).unwrap();
/// let _ = cents + sats;
/// ```
///
/// ```compile_fail
/// use exact_kind::{ Price, Quantity };
/// let _ = Price::parse( "65000" ).unwrap() + Quantity::parse( "0.5" ).unwrap();
/// ```
impl< K : Kind, const SCALE : u32 > core::ops::Add for Amount< K, SCALE >
{
  type Output = Result< Self, KindError >;

  fn add( self, rhs : Self ) -> Self::Output
  {
    K::checked_add( self, rhs )
  }
}

/// `( a + b ) + c`: an error on the left is passed on; otherwise `c` is added.
impl< K : Kind, const SCALE : u32 > core::ops::Add< Amount< K, SCALE > > for Result< Amount< K, SCALE >, KindError >
{
  type Output = Result< Amount< K, SCALE >, KindError >;

  fn add( self, rhs : Amount< K, SCALE > ) -> Self::Output
  {
    K::checked_add( self?, rhs )
  }
}

/// `a - b`: the checked subtraction, spelled as an operator.
impl< K : Kind, const SCALE : u32 > core::ops::Sub for Amount< K, SCALE >
{
  type Output = Result< Self, KindError >;

  fn sub( self, rhs : Self ) -> Self::Output
  {
    K::checked_sub( self, rhs )
  }
}

/// `( a + b ) - c`: an error on the left is passed on; otherwise `c` is subtracted.
impl< K : Kind, const SCALE : u32 > core::ops::Sub< Amount< K, SCALE > > for Result< Amount< K, SCALE >, KindError >
{
  type Output = Result< Amount< K, SCALE >, KindError >;

  fn sub( self, rhs : Amount< K, SCALE > ) -> Self::Output
  {
    K::checked_sub( self?, rhs )
  }
}

/// The fractional digits `minor` has at `scale`, trailing zeros not counted:
/// `1.005` held at scale 6 (`1_005_000`) has 3. What `rescale` reports when
/// `Exact` refuses a dropped digit.
//
// Fix(exact_kind_rescale_reported_the_scale_as_its_digits): `rescale` set
// `digits` to `SCALE`, so `1.005` held at scale 6 was refused as "6
// fractional digits" while `parse` refuses the same text as 3.
//
// Root cause: the scale a value is stored at taken for the digits it has.
// Pitfall: the two agree for a value with every place used, which is all the
//   first tests checked.
//
// Pure scale math, yet private here: `rescale` is its one caller. It moves to
// `exact_scale` when a second crate needs it (wanguardd, rounds 2-3 of #10).
fn fraction_digits( minor : Backing, scale : u32 ) -> u32
{
  let frac = minor.unsigned_abs() % pow10( scale ).unsigned_abs();
  let trailing_zeros = ( 1..=scale ).take_while( | &k | frac.is_multiple_of( pow10( k ).unsigned_abs() ) ).last().unwrap_or( 0 );
  scale - trailing_zeros
}
