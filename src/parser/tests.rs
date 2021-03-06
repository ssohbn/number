#[cfg(test)]
use super::*;

#[test]
fn test_places_amount() {
	assert_eq!(1, places_amount(1));
	assert_eq!(2, places_amount(10));
}

#[test]
fn test_number_to_placeholder() {
	assert!(matches!(number_to_placeholder(3), DigitsPlaceholder::Hundreds));
	assert!(matches!(number_to_placeholder(4), DigitsPlaceholder::Thousands));
	assert!(matches!(number_to_placeholder(1), DigitsPlaceholder::Ones));
	assert!(matches!(number_to_placeholder(00_001), DigitsPlaceholder::Ones));
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
	number_to_placeholder(8);
}

#[test]
fn test_parse_to_i32() {
	assert_eq!(Numba::new(ValuePlaceholder::One, DigitsPlaceholder::Ones).get_as_i32(), 1);
	assert_eq!(Numba::new(ValuePlaceholder::Two, DigitsPlaceholder::Ones).get_as_i32(), 2);
	assert_eq!(Numba::new(ValuePlaceholder::Three, DigitsPlaceholder::Tens).get_as_i32(), 30);
}

#[test]
fn test_verify_numbas_order() {
	verify_numbas_order(&vec![
		Numba::new(ValuePlaceholder::One, DigitsPlaceholder::Ones),
		Numba::new(ValuePlaceholder::One, DigitsPlaceholder::Tens),
		Numba::new(ValuePlaceholder::One, DigitsPlaceholder::Hundreds),
	]);
	// ordered properly
}

#[test]
#[should_panic]
fn test_verify_numbas_order_panic() {
	verify_numbas_order(
		&vec![
			Numba::new(ValuePlaceholder::One, DigitsPlaceholder::Tens)
		]
	);
}

#[test]
#[should_panic]
fn test_verify_numbas_order_panic_2() {
	verify_numbas_order(
		&vec![
			Numba::new(ValuePlaceholder::One, DigitsPlaceholder::Hundreds)
		]
	);
}