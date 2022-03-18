mod tests;

use crate::numba::{self, ValuePlaceholder, DigitsPlaceholder, Numba};
use std::env;

/// read the single argument given by user
pub fn parse_arg() -> i32 {
  let args = &env::args().collect::<Vec<String>>();
  let numba_as_str = args.get(1).expect("not enough arg");
  let numba: i32 = numba_as_str.parse().expect("arg was not a number");
  numba
}

///creates string of Numba struct from a number
pub fn assign_placeholders(input_number: i32) -> Vec<Numba> {
  let mut numbas: Vec<Numba> = vec![];
  let zeroes = 0;
  while input_number != 0 {
    numbas.push(
      Numba::new(   value(input_number%10), number_to_placeholder(zeroes) ),
    );
  }
  numbas
}

fn zeros_amount(val: i32) -> i32 {
	let mut val = val.abs();
  let mut zero_count: i32 = 0;
  while val > 10 {
    val /= 10;
    zero_count+=1;
  }
  zero_count
}

/// finds a nice placeholder for a given amount of zeroes
///#eg
///1000000 -> 6 zeros -> DigitsPlaceholder::Millions
pub fn number_to_placeholder(zeroes: i32) -> DigitsPlaceholder {
  match zeroes {
    0 => DigitsPlaceholder::Ones,
    1 => DigitsPlaceholder::Tens,
    2 => DigitsPlaceholder::Hundreds,
    3 => DigitsPlaceholder::Thousands,
    4 => DigitsPlaceholder::TenThousands,
    5 => DigitsPlaceholder::HundredThousands,
    6 => DigitsPlaceholder::Millions,
    _ => panic!("unsupported amount of zeros"),
  }
}

pub fn value(digit: i32)-> ValuePlaceholder {
  match digit { // take remainder
    9 => ValuePlaceholder::Nine,
    8 => ValuePlaceholder::Eight,
    7 => ValuePlaceholder::Seven,
    6 => ValuePlaceholder::Six,
    5 => ValuePlaceholder::Five,
    4 => ValuePlaceholder::Four,
    3 => ValuePlaceholder::Three,
    2 => ValuePlaceholder::Two,
    1 => ValuePlaceholder::One,
    0 => ValuePlaceholder::Zero,
    _ => panic!("Program cant handle this number"),     
  }
}

fn verify_numbas(numbas: &Vec<Numba>) {
  for (index, numba) in numbas.iter().enumerate() {
    println!("index: {}\nvalue:{}", index, numba.get_as_i32());
  }
}


fn parse_to_i32( numbas: &Vec<Numba> ) -> i32 {
  let mut value = 0;
  for numba in numbas {
    value += numba.get_as_i32();
  }

  value
}