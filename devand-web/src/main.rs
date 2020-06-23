#![feature(proc_macro_hygiene, decl_macro, never_type)]

#[macro_use]
extern crate rocket;

mod api;
mod auth;
mod pages;
mod state;

use rocket::fairing::AdHoc;
use rocket::Rocket;
use rocket_contrib::database;
use rocket_contrib::databases::diesel;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

#[database("pg_devand")]
struct PgDevandConn(diesel::PgConnection);

struct StaticDir(String);

fn static_files(rocket: Rocket) -> Result<Rocket, Rocket> {
    const DEFAULT_DIR: &'static str = concat!(env!("CARGO_MANIFEST_DIR"), "/static");

    let dir = rocket
        .config()
        .get_str("static_dir")
        .unwrap_or(DEFAULT_DIR)
        .to_string();

    let static_files = StaticFiles::from(dir.clone());

    let rocket = rocket.manage(StaticDir(dir)).mount("/static", static_files);

    Ok(rocket)
}

fn run_db_migrations(rocket: Rocket) -> Result<Rocket, Rocket> {
    let conn = PgDevandConn::get_one(&rocket).expect("database connection");
    match devand_db::run_migrations(&*conn) {
        Ok(()) => Ok(rocket),
        Err(_) => Err(rocket),
    }
}

fn init_wsmc(rocket: Rocket) -> Result<Rocket, Rocket> {
    let conn = PgDevandConn::get_one(&rocket).expect("database connection");
    let conn = conn.0;
    let wsmc = rocket.state::<WeekScheduleMatrix>();
    if let Some(wsmc) = wsmc {
        let mut wsmc = wsmc.0.write().unwrap();
        wsmc.init(&*conn);
    }
    Ok(rocket)
}

#[derive(Default)]
struct CodeNowUsers(pub std::sync::RwLock<state::CodeNowUsersMap>);

#[derive(Default)]
struct WeekScheduleMatrix(pub std::sync::RwLock<state::WeekScheduleMatrixCache>);

fn main() {
    env_logger::init();

    rocket::ignite()
        .manage(CodeNowUsers::default())
        .manage(WeekScheduleMatrix::default())
        .attach(Template::fairing())
        .attach(PgDevandConn::fairing())
        .attach(AdHoc::on_attach("Database Migrations", run_db_migrations))
        .attach(AdHoc::on_attach("Static files", static_files))
        .attach(AdHoc::on_attach("WeekScheduleMatrixCache", init_wsmc))
        .mount("/", pages::routes())
        .mount("/api", api::routes())
        .launch();
}
