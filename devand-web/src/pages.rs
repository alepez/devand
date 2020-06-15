use crate::auth::{self, AuthData, ExpectedCaptcha};
use crate::PgDevandConn;
use rocket::http::{ContentType, Cookies};
use rocket::request::{FlashMessage, Form};
use rocket::response::{Content, Flash, Redirect};
use rocket::Route;
use rocket_contrib::templates::Template;
use serde::Serialize;

// Handle authentication request
#[post("/login", data = "<credentials>")]
fn login(
    mut cookies: Cookies,
    credentials: Form<auth::Credentials>,
    real_ip: auth::RealIp,
    conn: PgDevandConn,
) -> Result<Redirect, Flash<Redirect>> {
    auth::login(&mut cookies, credentials.0, &conn)
        .map(|_| Redirect::to(uri!(dashboard_index)))
        .map_err(|_| {
            log_fail(real_ip.0);
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
    #[derive(Serialize)]
    struct Context {
        title: &'static str,
        flash: Option<String>,
        authenticated: bool,
    }

    let context = Context {
        title: "Sign in to DevAndDev",
        flash: flash.map(|x| x.msg().to_string()),
        authenticated: false,
    };

    Template::render("login", &context)
}

// /logout just remove the cookie
#[get("/logout")]
fn logout(mut cookies: Cookies) -> Flash<Redirect> {
    auth::logout(&mut cookies);
    Flash::success(Redirect::to(uri!(login_page)), "Successfully logged out.")
}

// Handle join request
// Note: cookies must be after expected_captcha, due to One-At-A-Time cookies
// restriction
#[post("/join", data = "<join_data>")]
fn join(
    join_data: Form<auth::JoinData>,
    expected_captcha: ExpectedCaptcha,
    mut cookies: Cookies,
    real_ip: auth::RealIp,
    conn: PgDevandConn,
) -> Result<Redirect, Flash<Redirect>> {
    auth::join(&mut cookies, join_data.0, expected_captcha, &conn)
        .map(|_| Redirect::to(uri!(dashboard_index)))
        .map_err(|err| {
            log_fail(real_ip.0);
            Flash::error(Redirect::to(uri!(join_page)), err.to_string())
        })
}

// When user is authenticated, /join just redirect to index
#[get("/join")]
fn join_authenticated(_auth_data: AuthData) -> Redirect {
    Redirect::to(uri!(index))
}

// When user is not authenticated, /join displays a form
#[get("/join", rank = 2)]
fn join_page(flash: Option<FlashMessage>, join_data: Option<auth::JoinData>) -> Template {
    #[derive(Serialize)]
    struct Context {
        title: &'static str,
        flash: Option<String>,
        username: Option<String>,
        email: Option<String>,
        password: Option<String>,
        authenticated: bool,
    }

    let context = Context {
        title: "Create your DevAndDev account",
        flash: flash.map(|x| x.msg().to_string()),
        username: join_data.as_ref().map(|x| x.username.to_string()),
        email: join_data.as_ref().map(|x| x.email.to_string()),
        password: join_data.as_ref().map(|x| x.password.to_string()),
        authenticated: false,
    };

    Template::render("join", &context)
}

// When user is not authenticated, /join displays a form
#[get("/join/captcha.png")]
fn join_captcha(mut cookies: Cookies) -> Option<Content<Vec<u8>>> {
    let captcha = auth::captcha(&mut cookies).unwrap();
    let data = captcha.into_data();
    Some(Content(ContentType::PNG, data))
}

// Some URLs respond with dashboard
#[get("/dashboard")]
fn dashboard_index(auth_data: AuthData) -> Template {
    dashboard(auth_data)
}

#[get("/affinities")]
fn dashboard_affinities(auth_data: AuthData) -> Template {
    dashboard(auth_data)
}

#[get("/code-now")]
fn dashboard_code_now(auth_data: AuthData) -> Template {
    dashboard(auth_data)
}

fn dashboard(_auth_data: AuthData) -> Template {
    #[derive(Serialize)]
    struct Context {
        title: &'static str,
        authenticated: bool,
    }

    let context = Context {
        title: "Your dashboard",
        authenticated: true,
    };

    Template::render("dashboard", &context)
}

#[get("/", rank = 2)]
fn index(auth_data: Option<AuthData>) -> Template {
    #[derive(Serialize)]
    struct Context {
        title: &'static str,
        authenticated: bool,
    }

    let context = Context {
        title: "DevAndDev",
        authenticated: auth_data.is_some(),
    };
    Template::render("index", &context)
}

#[get("/privacy")]
fn privacy(auth_data: Option<AuthData>) -> Template {
    #[derive(Serialize)]
    struct Context {
        title: &'static str,
        authenticated: bool,
    }

    let context = Context {
        title: "Privacy Policy",
        authenticated: auth_data.is_some(),
    };
    Template::render("privacy", &context)
}

#[get("/code-of-conduct")]
fn code_of_conduct(auth_data: Option<AuthData>) -> Template {
    #[derive(Serialize)]
    struct Context {
        title: &'static str,
        authenticated: bool,
    }

    let context = Context {
        title: "DevAndDev Code of Conduct",
        authenticated: auth_data.is_some(),
    };
    Template::render("code-of-conduct", &context)
}

pub fn routes() -> Vec<Route> {
    routes![
        index,
        join,
        join_authenticated,
        join_page,
        join_captcha,
        login,
        login_authenticated,
        login_page,
        logout,
        dashboard_index,
        dashboard_affinities,
        dashboard_code_now,
        privacy,
        code_of_conduct,
    ]
}

fn format_fail(ip_addr: std::net::IpAddr) -> String {
    format!("Fail: IpAddr({})", ip_addr)
}

// Used by fail2ban
fn log_fail(ip_addr: std::net::IpAddr) {
    println!("{}", format_fail(ip_addr));
}

#[cfg(test)]
mod test {
    use super::*;
    use std::net::IpAddr;
    use std::net::Ipv4Addr;

    #[test]
    fn log_fail_has_expected_format() {
        assert!(format_fail(IpAddr::V4(Ipv4Addr::new(1, 2, 3, 4))) == "Fail: IpAddr(1.2.3.4)");
    }
}
