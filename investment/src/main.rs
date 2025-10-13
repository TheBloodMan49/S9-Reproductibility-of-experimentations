#![warn(
     clippy::all,
     clippy::restriction,
     clippy::pedantic,
     clippy::nursery,
     clippy::cargo,
 )]

use core::f64::consts::E;

fn main() {
  let mut investment: f64 = E;

  for year in 1..=50 {
    investment = (investment-1.0)*f64::from(year);
    println!("After year {year}: {investment:.2}");
  }

  println!("With e value from core::f64::consts::E :");
  println!("After 50 years you can retrieve: {:.2}", investment-1.0);

  investment = 2.7183;

  for year in 1..=50 {
    investment = (investment-1.0)*f64::from(year);
  }

  println!("With e value approximated to 4 decimals :");
  println!("After 50 years you can retrieve: {:.2}", investment-1.0);

}