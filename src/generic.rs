pub fn concat<T: std::fmt::Display, U: std::fmt::Display>(left: T, right: U) -> String {
	left.to_string() + &right.to_string()
}

pub fn longest<'a>(left: &'a str, right: &'a str) -> &'a str {
	if left.len() > right.len() {
		return left
	} 
	return right
}