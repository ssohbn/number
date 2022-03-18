#[cfg(test)]
use super::*;
#[test]
fn test_get_as_i32() {
	
	assert_eq!(1, Numba::new(ValuePlaceholder::One, DigitsPlaceholder::Ones).get_as_i32());
	assert_eq!(10, Numba::new(ValuePlaceholder::One, DigitsPlaceholder::Tens).get_as_i32());
}