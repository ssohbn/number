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
	pub fn get_places(&self) -> i32 {
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
	
	// i hate this stupid spaghetti 
	// im gonna have to update this every 
	// single time i want to update the 
	// amount of numbers arent i :(
	pub fn get_as_i32(&self) -> i32 {
		let value = self.value.get_value();

		let mut place = 1;
		for _ in 1..self.place.get_places() {
			place *= 10;
		}

		value * place
	}

	pub fn place(&self) -> &DigitsPlaceholder {
		&self.place
	}
}

