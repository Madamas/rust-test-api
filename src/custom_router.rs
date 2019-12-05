use rocket::{Route, Outcome, Request};
use rocket::http::{Method};

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

// https://github.com/SergioBenitez/Rocket/blob/master/examples/manual_routes/src/main.rs
// Need more time to understand manual routing

// pub fn parameter_route(name: String, age: Option<u8>) -> String {
// 	match age {
// 		Some(age) => format!("You are {} and your age is {} yrs", name, age),
// 		None => format!("You dingus")
// 	}
// }

// pub fn multiple_handler(req: &Request, _: Data) -> Outcome<'static> {
// 	Outcome::of(parameter_route(name: String, age: Option<u8>))
// }

// pub fn multiple_handlers() {
// 	let mut routes = vec![];

// 	for route in &["/user/<name>/<age>", "/user?<name>&<age>"] {
// 		routes.push(Route::new(Method::Get, *route, parameter_route))
// 	}
// }