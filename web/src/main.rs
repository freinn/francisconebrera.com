#![feature(use_extern_macros)]

// rocket
#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate itertools;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
extern crate serde_json;
extern crate regex;
extern crate chrono;
extern crate file;
extern crate htmlescape;

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate lazy_static;

mod routes;
mod security;

mod tera_template_contexts;

fn main() {
    rocket::ignite().mount("/", routes![
        routes::secciones_simples::get_index,
        routes::secciones_simples::get_portfolio,
        routes::file_ops::files,
    ])
    .mount("/blog", routes![
        routes::blog::get_entradas_blog,
        routes::blog::vista_entrada_blog,
    ])
    .launch();
}
