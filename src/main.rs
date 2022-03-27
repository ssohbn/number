mod numba;
mod parser;

fn main() {
  let input_number: i32 = parser::parse_arg();
  let numbas = parser::assign_placeholders(input_number);
  println!("{}", parser::text(&numbas));
  
}