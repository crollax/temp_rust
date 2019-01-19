#![feature(proc_macro_hygiene, decl_macro, plugin)]
// #![plugin(rocket_codegen)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
// extern crate handlebars;

use std::collections::HashMap;
use rocket_contrib::templates::Template;
use rocket::response::Redirect;

#[get("/")]
fn root() -> Redirect {
    Redirect::to("/home")
}

#[get("/home")]
fn header() -> Template {
    let header = HashMap::<String, String>::new();
    Template::render("header", header)
}

#[get("/home", rank = 2)]
fn home() -> Template {
    let context = HashMap::<String, String>::new();
    Template::render("index", context)
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![root, header, home])
        .attach(Template::fairing())
}

fn main() {
    rocket().launch();
}