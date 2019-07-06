#[macro_use] extern crate nickel;

use nickel::{Nickel, HttpRouter};

mod controllers;

fn main() {
    let mut server = Nickel::new();
    server.get("**", middleware!(controllers::main::hello_world()));
    server.listen("127.0.0.1:6767");
}