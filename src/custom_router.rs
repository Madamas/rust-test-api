use rocket::http::uri::{Segments};
use rocket::request::FromSegments;
use std::fmt;
use std::error::Error;


#[derive(Debug)]
pub enum UserError{
	NoName,
	NoAge,
}

impl fmt::Display for UserError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			UserError::NoName => f.write_str("No name was provided"),
			UserError::NoAge => f.write_str("No age was provided"),
		}
	}
}

impl Error for UserError {
	fn description(&self) -> &str {
		match *self {
			UserError::NoName => "No age was provided",
			UserError::NoAge => "No name was provided",
		}
	}
}

#[derive(Debug)]
pub struct User {
	name: String,
	age: Option<u8>
}

impl fmt::Display for User {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self.age {
			Some(age) => f.write_str(&format!("{}{}", self.name ,age.to_string())),
			None => f.write_str(&self.name),
		}
	}
}

impl<'a> FromSegments<'a> for User {
	type Error = UserError;

	fn from_segments(segments: Segments<'a>) -> Result<Self, Self::Error> {
		println!("{:?}", segments);

		Ok(User{
			name: String::from("good"),
			age: None
		})
	}
}




#[get("/world")]
pub fn someroute() -> &'static str {
	"Hello work"
}

pub mod another {
	#[get("/another")]
	pub fn anroute() -> &'static str {
		"WOT IS DIS"
	}
}

#[get("/user/<user..>")]
pub fn parameter_route(user: User) -> String {
	println!("pathbuf {:?}", user);
	format!("hello")
	// match age {
	// 	Some(age) => format!("You are {} and your age is {} yrs", name, age),
	// 	None => format!("Good lord, what are you?")
	// }
}


// https://github.com/SergioBenitez/Rocket/blob/master/examples/manual_routes/src/main.rs
// Need more time to understand manual routing

// pub fn multiple_handler(req: &Request, _: Data) -> Outcome<'static> {
// 	Outcome::of(parameter_route(name: String, age: Option<u8>))
// }

// pub fn multiple_handlers() {
// 	let mut routes = vec![];

// 	for route in &["/user/<name>/<age>", "/user?<name>&<age>"] {
// 		routes.push(Route::new(Method::Get, *route, parameter_route))
// 	}
// }