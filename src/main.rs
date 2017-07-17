#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;

use rocket_contrib::Json;

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

//use serde_json::Error;

mod message;
use message::Message;

#[post("/", data = "<message>")]
fn hello(message: Json<Message>) {
    println!("{:?}", message);
}

fn main() {
    rocket::ignite().mount("/hello", routes![hello]).launch();
}