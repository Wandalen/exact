//! Formatting per kind, and a buffer-writing primitive that renders without
//! allocating.
//!
//! Tier 2, depending on `exact_kind` for the `Display` implementation every
//! function here renders through.
//!
//! # Disclosed deviation: `Display` cannot move here
//!
//! The preferred design states `Display` is implemented "only as a wrapper
//! over `fmt_into` — no independent formatting logic", which would mean this
//! crate owns the trait impl and `exact_kind`'s types have none of their
//! own. That is impossible under Rust's orphan rules in the dependency
//! direction this migration already chose: `exact_fmt` depends on
//! `exact_kind`, so neither the trait (`core::fmt::Display`, foreign) nor
//! the type (`exact_kind::Decimal`, foreign to this crate) is local here,
//! and Rust refuses the impl outright. Reversing the dependency so
//! `exact_kind` depended on `exact_fmt` instead would contradict the
//! family's own topological tier order for no behavioural gain.
//!
//! `Display` therefore stays on `exact_kind::Decimal`/`Qty` (and `Price`, which delegates),
//! ported from the real codebase, then reworked not to allocate. This crate provides the preferred
//! design's per-kind names and the buffer-writing primitive as a layer over
//! that existing impl, which is the closest satisfiable reading of "no
//! independent formatting logic" — every function here renders through the
//! one real `Display` impl rather than reimplementing rendering.
//!
//! # Examples
//!
//! ```
//! use exact_fmt::{ fmt_into, money_fmt };
//! use exact_kind::Money;
//!
//! let v = Money::parse( "1.5" ).unwrap();
//! assert_eq!( money_fmt( v ), "1.5" );
//!
//! let mut buf = [ 0_u8; 16 ];
//! let written = fmt_into( v, &mut buf ).unwrap();
//! assert_eq!( &buf[ .. written ], b"1.5" );
//! ```

use exact_kind::{ Money, Price, Quantity };

/// Why a buffer-writing render could not complete.
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
pub enum FmtError
{
  /// The buffer was too small to hold the rendered text.
  BufFull,
}

impl core::fmt::Display for FmtError
{
  fn fmt( &self, f : &mut core::fmt::Formatter< '_ > ) -> core::fmt::Result
  {
    match self
    {
      Self::BufFull => write!( f, "the buffer was too small to hold the rendered text" ),
    }
  }
}

impl core::error::Error for FmtError {}

struct ByteBufWriter< 'a >
{
  buf : &'a mut [ u8 ],
  len : usize,
}

impl core::fmt::Write for ByteBufWriter< '_ >
{
  fn write_str( &mut self, s : &str ) -> core::fmt::Result
  {
    let bytes = s.as_bytes();
    if self.len + bytes.len() > self.buf.len()
    {
      return Err( core::fmt::Error );
    }
    self.buf[ self.len .. self.len + bytes.len() ].copy_from_slice( bytes );
    self.len += bytes.len();
    Ok( () )
  }
}

/// Render any of this family's kinds into a caller-provided byte buffer,
/// with no allocation.
///
/// Returns the number of bytes written.
///
/// # Errors
///
/// [`FmtError::BufFull`] when `buf` is too small to hold the rendered text —
/// pieces written before the one that did not fit stay in `buf`, so its
/// contents are unspecified on error.
pub fn fmt_into( value : impl core::fmt::Display, buf : &mut [ u8 ] ) -> Result< usize, FmtError >
{
  use core::fmt::Write;
  let mut writer = ByteBufWriter { buf, len : 0 };
  write!( writer, "{value}" ).map_err( | _ | FmtError::BufFull )?;
  Ok( writer.len )
}

/// Render a money value.
#[ must_use ]
pub fn money_fmt( v : Money ) -> String
{
  v.to_string()
}

/// Render a quantity.
#[ must_use ]
pub fn qty_fmt( v : Quantity ) -> String
{
  v.to_string()
}

/// Render a price.
#[ must_use ]
pub fn price_fmt( v : Price ) -> String
{
  v.to_string()
}
