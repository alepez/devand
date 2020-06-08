#![feature(proc_macro_hygiene, decl_macro, never_type)]

#[macro_use]
extern crate rocket;

mod api;
mod auth;
mod pages;

use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use rocket_contrib::databases::diesel;
use rocket_contrib::database;

#[database("pg_devand")]
struct PgDevandConn(diesel::PgConnection);

const STATIC_PATH: &'static str = concat!(env!("CARGO_MANIFEST_DIR"), "/static");

fn main() {
    let static_files = StaticFiles::from(STATIC_PATH);

    rocket::ignite()
        .attach(Template::fairing())
        .attach(PgDevandConn::fairing())
        .mount("/", pages::routes())
        .mount("/api", api::routes())
        .mount("/public", static_files)
        .launch();
}
