#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
mod custom_router;
use custom_router::another;


#[get("/")]
fn index() -> &'static str {
    "Hello world"
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, custom_router::parameter_route])
        .mount("/custom", routes![custom_router::someroute, another::anroute])
        .launch();
}
