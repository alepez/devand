#![feature(proc_macro_hygiene, decl_macro, never_type)]

#[macro_use]
extern crate rocket;

mod api;
mod auth;
mod pages;

use rocket::fairing::AdHoc;
use rocket_contrib::database;
use rocket_contrib::databases::diesel;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

#[database("pg_devand")]
struct PgDevandConn(diesel::PgConnection);

struct StaticDir(String);

fn static_files(rocket: rocket::Rocket) -> Result<rocket::Rocket, rocket::Rocket> {
    const DEFAULT_DIR: &'static str = concat!(env!("CARGO_MANIFEST_DIR"), "/static");

    let dir = rocket
        .config()
        .get_str("static_dir")
        .unwrap_or(DEFAULT_DIR)
        .to_string();

    let static_files = StaticFiles::from(dir.clone());

    let rocket = rocket.manage(StaticDir(dir)).mount("/public", static_files);

    Ok(rocket)
}

fn main() {
    rocket::ignite()
        .attach(Template::fairing())
        .attach(PgDevandConn::fairing())
        .mount("/", pages::routes())
        .mount("/api", api::routes())
        .attach(AdHoc::on_attach("static_files", static_files))
        .launch();
}
