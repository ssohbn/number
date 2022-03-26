mod tests;

/// All types of values for a digit
/// # Examples
/// ```
/// use numba::numba::{Numba, ValuePlaceholder, DigitsPlaceholder};
/// let nine = ValuePlaceholder::Nine;
/// let hundreds = DigitsPlaceholder::Hundreds;
/// let numba = Numba::new(nine, hundreds);
/// ```
#[derive(Debug)]
pub enum ValuePlaceholder {
  Zero,
  One,
  Two,
  Three,
  Four,
  Five,
  Six,
  Seven,
  Eight,
  Nine,
}

impl ValuePlaceholder {
	fn get_value(&self) -> i32 {
		let value = match self {
			ValuePlaceholder::Zero => 0,
			ValuePlaceholder::One => 1,
			ValuePlaceholder::Two => 2,
			ValuePlaceholder::Three => 3,
			ValuePlaceholder::Four => 4,
			ValuePlaceholder::Five => 5,
			ValuePlaceholder::Six => 6,
			ValuePlaceholder::Seven => 7,
			ValuePlaceholder::Eight => 8,
			ValuePlaceholder::Nine => 9,
		};
		value
	}

	pub fn text(&self) -> String {
		let value = match self {
			ValuePlaceholder::Zero => "zero",
			ValuePlaceholder::One => "one",
			ValuePlaceholder::Two => "two",
			ValuePlaceholder::Three => "three",
			ValuePlaceholder::Four => "four",
			ValuePlaceholder::Five => "five",
			ValuePlaceholder::Six => "six",
			ValuePlaceholder::Seven => "seven",
			ValuePlaceholder::Eight => "eight",
			ValuePlaceholder::Nine => "nine",
		};
		value.to_string()
	}
}

/// All types of places for each digit
/// # Examples
/// ```
/// use numba::numba::{Numba, ValuePlaceholder, DigitsPlaceholder};
/// let nine = ValuePlaceholder::Nine;
/// let hundreds = DigitsPlaceholder::Hundreds;
/// let numba = Numba::new(nine, hundreds);
/// ```
#[derive(Debug)]
pub enum DigitsPlaceholder {
  Ones,
  Tens,
  Hundreds,
  Thousands,
  TenThousands,
  HundredThousands,
  Millions,
}

impl DigitsPlaceholder {
	pub fn get_place(&self) -> i32 {
		let place = match self {
			DigitsPlaceholder::Ones => 1,
			DigitsPlaceholder::Tens => 2,
			DigitsPlaceholder::Hundreds => 3,
			DigitsPlaceholder::Thousands => 4,
			DigitsPlaceholder::TenThousands => 5,
			DigitsPlaceholder::HundredThousands => 6,
			DigitsPlaceholder::Millions => 7,
		};	
		place
	}
	pub fn text(&self) -> String {
		match self {
			DigitsPlaceholder::Ones => "ones".to_string(),
			DigitsPlaceholder::Tens => "tens".to_string(),
			DigitsPlaceholder::Hundreds => "hundreds".to_string(),
			DigitsPlaceholder::Thousands => "thousands".to_string(),
			DigitsPlaceholder::TenThousands => "ten thousands".to_string(),
			DigitsPlaceholder::HundredThousands => "hundred thousands".to_string(),
			DigitsPlaceholder::Millions => "millions".to_string(),
		}
	}
}

/// Contains the value and place of a number
/// 
/// # Examples
/// ```
/// use numba::numba::Numba;
/// use numba::numba::ValuePlaceholder;
/// use numba::numba::DigitsPlaceholder;
/// 
/// let numba = Numba::new(ValuePlaceholder::One, DigitsPlaceholder::Ones);
/// assert_eq!(numba.get_value(), 1);
/// assert_eq!(numba.get_place(), 1);
/// ```
#[derive(Debug)]
pub struct Numba {
  value: ValuePlaceholder,
  place: DigitsPlaceholder,
}

impl Numba {
	pub fn new(value: ValuePlaceholder, place: DigitsPlaceholder) -> Self {
		Numba {
			value,
			place,
		}
	}

	pub fn get_value(&self) -> i32 {
		self.value.get_value()
	}
	pub fn get_place(&self) -> i32 {
		self.place.get_place()
	}
	
	// i hate this stupid spaghetti 
	// im gonna have to update this every 
	// single time i want to update the 
	// amount of numbers arent i :(
	pub fn get_as_i32(&self) -> i32 {
		let value = self.value.get_value();

		let mut place = 1;
		for _ in 1..self.place.get_place() {
			place *= 10;
		}

		value * place
	}

	pub fn place(&self) -> &DigitsPlaceholder {
		&self.place
	}

	pub fn text(&self) -> String {
		format!("{}-{}", self.value.text(), self.place.text())
	}

}

