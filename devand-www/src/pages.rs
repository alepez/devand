use super::STATIC_PATH;
use crate::auth::{self, AuthData};
use crate::PgDevandConn;
use rocket::http::Cookies;
use rocket::request::{FlashMessage, Form};
use rocket::response::{Flash, NamedFile, Redirect};
use rocket::Route;
use rocket_contrib::templates::Template;
use std::collections::HashMap;

// Handle authentication request
#[post("/login", data = "<credentials>")]
fn login(
    mut cookies: Cookies,
    credentials: Form<auth::Credentials>,
    conn: PgDevandConn,
) -> Result<Redirect, Flash<Redirect>> {
    auth::login(&mut cookies, credentials.0, &conn)
        .map(|_| Redirect::to(uri!(index)))
        .map_err(|_| {
            Flash::error(
                Redirect::to(uri!(login_page)),
                "Incorrect username or password",
            )
        })
}

// When user is authenticated, /login just redirect to index
#[get("/login")]
fn login_authenticated(_auth_data: auth::AuthData) -> Redirect {
    Redirect::to(uri!(index))
}

// When user is not authenticated, /login displays a form
#[get("/login", rank = 2)]
fn login_page(flash: Option<FlashMessage>) -> Template {
    let mut context = HashMap::new();
    if let Some(ref msg) = flash {
        context.insert("flash", msg.msg());
    }

    Template::render("login", &context)
}

// /logout just remove the cookie
#[get("/logout")]
fn logout(mut cookies: Cookies) -> Flash<Redirect> {
    auth::logout(&mut cookies);
    Flash::success(Redirect::to(uri!(login_page)), "Successfully logged out.")
}

// Handle join request
#[post("/join", data = "<join_data>")]
fn join(
    mut cookies: Cookies,
    join_data: Form<auth::JoinData>,
    conn: PgDevandConn,
) -> Result<Redirect, Flash<Redirect>> {
    auth::join(&mut cookies, join_data.0, &conn)
        .map(|_| Redirect::to(uri!(index)))
        .map_err(|err| Flash::error(Redirect::to(uri!(join_page)), err.to_string()))
}

// When user is authenticated, /join just redirect to index
#[get("/join")]
fn join_authenticated(_auth_data: AuthData) -> Redirect {
    Redirect::to(uri!(index))
}

// When user is not authenticated, /join displays a form
#[get("/join", rank = 2)]
fn join_page(flash: Option<FlashMessage>, join_data: Option<auth::JoinData>) -> Template {
    let mut context = HashMap::new();

    if let Some(ref msg) = flash {
        context.insert("flash", msg.msg());
    }

    if let Some(ref join_data) = join_data {
        context.insert("username", &join_data.username);
        context.insert("email", &join_data.email);
        context.insert("password", &join_data.password);
    }

    Template::render("join", &context)
}

// When user is authenticated, home page shows user's settings
#[get("/")]
fn settings(auth_data: AuthData) -> Template {
    Template::render("settings", &auth_data)
}

// When user is not authenticated, home page just serve a static file
#[get("/", rank = 2)]
fn index() -> NamedFile {
    let path = std::path::Path::new(STATIC_PATH).join("index.html");
    NamedFile::open(&path).unwrap()
}

pub fn routes() -> Vec<Route> {
    routes![
        index,
        join,
        join_authenticated,
        join_page,
        login,
        login_authenticated,
        login_page,
        logout,
        settings,
    ]
}
