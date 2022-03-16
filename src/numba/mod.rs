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
}

