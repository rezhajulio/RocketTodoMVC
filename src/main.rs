#![feature(plugin)]
#![feature(custom_derive)]
#![plugin(rocket_codegen)]
extern crate dotenv;
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;

use rocket_contrib::Template;

mod controller;
mod database;
mod form;
mod model;
mod schema;

fn main() {
    rocket::ignite()
        .mount("/", routes![controller::todo_controller::index])
        .mount("/todos", routes![controller::todo_controller::create])
        .mount("/todos", routes![controller::todo_controller::index])
        .mount("/todos", routes![controller::todo_controller::show])
        .mount("/todos", routes![controller::todo_controller::new])
        .mount("/todos", routes![controller::todo_controller::edit])
        .mount("/todos", routes![controller::todo_controller::update])
        .mount("/todos", routes![controller::todo_controller::delete])
        .attach(Template::fairing())
        .launch();
}
