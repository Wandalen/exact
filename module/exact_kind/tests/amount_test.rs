//! `Decimal`, `Qty` and `PriceAt` are one struct, `Amount< K, SCALE >`, with
//! the kind as a type parameter: each is still one `i64`, still its own type,
//! still prints its own name, and a quantity still passes through
//! `from_decimal` to exist.

use exact_kind::{ Amount, Decimal, Kind, KindError, Money, MoneyKind, Price, PriceAt, PriceKind, Qty, QtyKind, Quantity };

/// The kind is zero-sized: every amount, at every scale, is one `i64`.
#[ test ]
fn every_kind_is_one_i64()
{
  assert_eq!( core::mem::size_of::< Money >(), 8 );
  assert_eq!( core::mem::size_of::< Quantity >(), 8 );
  assert_eq!( core::mem::size_of::< Price >(), 8 );
  assert_eq!( core::mem::size_of::< Decimal< 2 > >(), 8 );
  assert_eq!( core::mem::size_of::< Qty< 8 > >(), 8 );
  assert_eq!( core::mem::size_of::< PriceAt< 18 > >(), 8 );
}

/// Each alias is `Amount` of its own kind — the names callers write are
/// unchanged, and spelled out they are the one struct.
#[ test ]
fn each_alias_is_amount_of_its_kind()
{
  let money : Amount< MoneyKind, 6 > = Money::from_int( 1 ).unwrap();
  let quantity : Amount< QtyKind, 6 > = Quantity::from_int( 1 ).unwrap();
  let price : Amount< PriceKind, 6 > = Price::parse( "1" ).unwrap();
  assert_eq!( money.minor(), quantity.minor() );
  assert_eq!( quantity.minor(), price.minor() );
}

/// `Debug` names the kind and prints its one count; the zero-sized label is not
/// printed. (As structs, `Qty` and `PriceAt` printed a nested `Decimal`.)
#[ test ]
fn debug_names_the_kind()
{
  assert_eq!( format!( "{:?}", Money::parse( "12.34" ).unwrap() ), "Decimal { minor: Minor(12340000) }" );
  assert_eq!( format!( "{:?}", Qty::< 8 >::parse( "0.5" ).unwrap() ), "Qty { minor: Minor(50000000) }" );
  assert_eq!( format!( "{:?}", PriceAt::< 2 >::parse( "-1.25" ).unwrap() ), "PriceAt { minor: Minor(-125) }" );
}

/// A quantity leaves for a signed decimal and comes back only through
/// `from_decimal`, the one place a negative value is refused.
#[ test ]
fn a_quantity_comes_back_only_through_from_decimal()
{
  let held = Quantity::from_int( 3 ).unwrap();
  let signed = held.as_decimal();
  assert_eq!( signed.minor(), held.minor() );
  assert_eq!( Quantity::from_decimal( signed ), Ok( held ) );
  let short = signed.checked_sub( Money::from_int( 5 ).unwrap() ).unwrap();
  assert_eq!( Quantity::from_decimal( short ), Err( KindError::Negative { minor : -2_000_000 } ) );
}

/// Amounts of one kind still order by value, as each struct's derived `Ord` did.
#[ test ]
fn one_kind_orders_by_value()
{
  let low = Price::parse( "0.90" ).unwrap();
  let high = Price::parse( "1.10" ).unwrap();
  assert!( low < high );
  assert_eq!( [ high, low ].iter().min(), Some( &low ) );
}

/// Code generic over the kind gets every derive `Amount` has: `Kind` implies
/// `Copy`, `Ord`, `Hash` and `Debug`, so `K : Kind` is the only bound needed.
#[ test ]
fn generic_code_over_any_kind_gets_the_derives()
{
  fn largest< K : Kind, const S : u32 >( items : &[ Amount< K, S > ] ) -> Option< Amount< K, S > >
  {
    items.iter().copied().max()
  }
  fn distinct< K : Kind, const S : u32 >( items : &[ Amount< K, S > ] ) -> usize
  {
    items.iter().collect::< std::collections::HashSet< _ > >().len()
  }
  let low = Quantity::from_int( 1 ).unwrap();
  let high = Quantity::from_int( 2 ).unwrap();
  assert_eq!( largest( &[ low, high, low ] ), Some( high ) );
  assert_eq!( distinct( &[ low, high, low ] ), 2 );
  let price = Price::parse( "1.25" ).unwrap();
  assert_eq!( largest( &[ price ] ), Some( price ) );
  assert_eq!( format!( "{:?}", largest::< MoneyKind, 6 >( &[] ) ), "None" );
}
