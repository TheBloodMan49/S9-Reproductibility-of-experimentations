#![warn(
     clippy::all,
     clippy::restriction,
     clippy::pedantic,
     clippy::nursery,
     clippy::cargo,
 )]

use core::f64::consts::E;
use std::fs;
use std::io::Write;

fn main() {
  fs::write("./banking_test.txt", "").expect("Unable to write file");
  let mut file = fs::OpenOptions::new().append(true).open("./banking_test.txt").expect("Unable to open file");


  let mut investment: f64 = E;

  for year in 1..=50 {
    investment = (investment-1.0)*f64::from(year);
    println!("After year {year}: {investment:.2}");
  }

  println!("With e value from core::f64::consts::E :");
  writeln!(file, "With e value from core::f64::consts::E :").expect("Unable to write file");
  println!("After 50 years you can retrieve: {:.2}", investment-1.0);
  writeln!(file, "After 50 years you can retrieve: {:.2}", investment-1.0).expect("Unable to write file");

  investment = 2.7183;

  for year in 1..=50 {
    investment = (investment-1.0)*f64::from(year);
  }

  println!("With e value approximated to 4 decimals :");
  writeln!(file, "With e value approximated to 4 decimals :").expect("Unable to write file");
  println!("After 50 years you can retrieve: {:.2}", investment-1.0);
  writeln!(file, "After 50 years you can retrieve: {:.2}", investment-1.0).expect("Unable to write file");

}