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
		let value = match self.value {
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

		let place = match self.place {
			DigitsPlaceholder::Ones => 1,
			DigitsPlaceholder::Tens => 10,
			DigitsPlaceholder::Hundreds => 100,
			DigitsPlaceholder::Thousands => 1_000,
			DigitsPlaceholder::TenThousands => 10_000,
			DigitsPlaceholder::HundredThousands => 100_000,
			DigitsPlaceholder::Millions => 1_000_000,
		};

		value * place
	}
	
}

