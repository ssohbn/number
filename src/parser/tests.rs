#[cfg(test)]
use super::*;

#[test]
fn test_zeros_amount() {
	assert_eq!(0, zeros_amount(3));
	assert_eq!(0, zeros_amount(4));
	assert_eq!(1, zeros_amount(20));
	assert_eq!(1, zeros_amount(13));
	assert_eq!(0, zeros_amount(-3));
	assert_eq!(0, zeros_amount(-4));
	assert_eq!(1, zeros_amount(-20));
	assert_eq!(1, zeros_amount(-13));
}

#[test]
fn test_number_to_placeholder() {
	assert!(matches!(number_to_placeholder(3), DigitsPlaceholder::Thousands));
	assert!(matches!(number_to_placeholder(4), DigitsPlaceholder::TenThousands));
	assert!(matches!(number_to_placeholder(1), DigitsPlaceholder::Tens));
	assert!(matches!(number_to_placeholder(00_001), DigitsPlaceholder::Tens));
}

#[test]
fn test_value() {
	assert!(matches!(value(9), ValuePlaceholder::Nine));
	assert!(matches!(value(8), ValuePlaceholder::Eight));
	assert!(matches!(value(3), ValuePlaceholder::Three));
}
#[test]
#[should_panic]
fn test_value_panic() {
	value(10);
}

#[test]
#[should_panic]
fn test_number_to_placeholder_panic() {
	number_to_placeholder(7);
}

#[test]
fn test_parse_to_i32() {
	assert_eq!(Numba::new(ValuePlaceholder::One, DigitsPlaceholder::Ones).get_as_i32(), 1);
	assert_eq!(Numba::new(ValuePlaceholder::Two, DigitsPlaceholder::Ones).get_as_i32(), 2);
	assert_eq!(Numba::new(ValuePlaceholder::Three, DigitsPlaceholder::Tens).get_as_i32(), 30);
}

#[test]
fn test_verify_numbas() {
	verify_numbas(&vec![
		Numba::new(ValuePlaceholder::One, DigitsPlaceholder::Ones),
		Numba::new(ValuePlaceholder::One, DigitsPlaceholder::Tens),
		Numba::new(ValuePlaceholder::One, DigitsPlaceholder::Hundreds),
	]);
	panic!("panic panic panic panic panic");
}