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