#[macro_use] extern crate nickel;

use nickel::{Nickel, HttpRouter};

mod controllers;

fn main() {
    let mut server = Nickel::new();
    server.get("**", middleware!(controllers::notes::hello_world()));
    server.listen("0.0.0.0:6767");
}