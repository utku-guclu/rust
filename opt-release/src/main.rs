use std::num::ParseIntError;  //

// fn main() {
//   let number = "23";  // a string of type &str
//   let parsed_number = number.parse::<usize>().unwrap();  // attempting to parse number with unwrap
//   panic!();  // end program prematurely with panic macro
//   println!("{}", parsed_number);  // code unreachable because of panic
// }

// fn main() -> Result<(), ParseIntError> {
// 	let number = "23"; // a string of type &str
// 	let parsed_number = number.parse::<usize>();
// 	match parsed_number {
//     		Ok(val) => println!("{}", val),
//     		Err(err) => return Err(err),
// 	}
// 	Ok(())
// }

fn main() -> Result<(), ParseIntError> {
	let number = "23x"; // a string of type &str
	let parsed_number = number.parse::<usize>()?;  // Replaced with the question mark instead of a match
	println!("{}", parsed_number);
	Ok(())
}
