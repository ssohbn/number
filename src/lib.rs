pub mod parser;
pub mod numba;

#[cfg(test)]
mod tests {
    use super::parser;
    use super::numba::{Numba, ValuePlaceholder, DigitsPlaceholder};

    #[test]
    fn it_works() {
        assert_eq!(2+2, 4);
    }

	
    // #[test]
    // fn test_assign_placeholders() {
    //     assert_eq!(parser::assign_placeholders(5), vec![Numba::new(ValuePlaceholder::Five, DigitsPlaceholder::Ones)])
    // }
}