mod tests;

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
}


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
}


/// Numba struct
/// Contains the number of digits and the value of the number
/// as well as the place of the number in the number of digits
/// (ones, tens, hundreds, thousands, ten thousands, hundred thousands, millions)
/// and the value of the number in that place
/// (0, 1, 2, 3, 4, 5, 6, 7, 8, 9)
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

}

