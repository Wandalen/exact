//! Smoke lane — this family's slice, run end to end.
//!
//! Everything this binary drives lives in the crate's library, so the lane is
//! measured like the rest of the family rather than dropped from the
//! coverage denominator the way a `src/main.rs` is. See
//! [`smoke_exact_market_split`] for the six steps and why so little is left
//! here.

fn main()
{
  smoke_exact_market_split::run();
}
