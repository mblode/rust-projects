#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
extern crate serde_json;

#[get("/")]
fn index() -> &'static str {
    "Hello test!"
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
