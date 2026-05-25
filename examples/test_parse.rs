use malachite_q::Rational;
use std::str::FromStr;

fn main() {
    println!("{:?}", Rational::from_str("0.5"));
    println!("{:?}", Rational::from_str("1/2"));
    println!("{:?}", Rational::from_str("123"));
}
