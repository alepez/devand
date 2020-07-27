use crate::auth::{self, AuthData, ExpectedCaptcha};
use crate::StaticDir;
use crate::{Mailer, PgDevandConn};
use devand_crypto::{EmailVerification, PasswordReset, Signable, SignedToken};
use rocket::http::{ContentType, Cookies};
use rocket::request::{FlashMessage, Form};
use rocket::response::{Content, Flash, NamedFile, Redirect};
use rocket::{Route, State};
use rocket_contrib::templates::Template;
use serde::Serialize;

const BASE_URL: Option<&'static str> = option_env!("DEVAND_BASE_URL");
const DEFAULT_BASE_URL: &'static str = "http://localhost:8000";

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

#[get("/login")]
fn login_page(
    auth_data: Option<auth::AuthData>,
    flash: Option<FlashMessage>,
) -> Result<Template, Redirect> {
    if auth_data.is_some() {
        // When user is authenticated, /login just redirect to index
        Err(Redirect::to(uri!(index)))
    } else {
        // When user is not authenticated, /login displays a form
        #[derive(Serialize)]
        struct Context {
            title: &'static str,
            flash_msg: Option<String>,
            flash_name: Option<String>,
            authenticated: bool,
        }

        let context = Context {
            title: "Sign in to DevAndDev",
            flash_msg: flash.as_ref().map(|x| x.msg().to_string()),
            flash_name: flash.as_ref().map(|x| x.name().to_string()),
            authenticated: false,
        };

        Ok(Template::render("login", &context))
    }
}

// TODO https://github.com/alepez/devand/issues/69 This should be POST and frontend should use a form
/// /logout just remove the cookie
#[get("/logout")]
fn logout(mut cookies: Cookies) -> Flash<Redirect> {
    auth::logout(&mut cookies);
    Flash::success(Redirect::to(uri!(login_page)), "Successfully logged out.")
}

#[get("/password_reset")]
fn password_reset_page(flash: Option<FlashMessage>) -> Template {
    #[derive(Serialize)]
    struct Context {
        title: &'static str,
        flash_msg: Option<String>,
        flash_name: Option<String>,
        authenticated: bool,
    }

    let context = Context {
        title: "Reset your password",
        flash_msg: flash.as_ref().map(|x| x.msg().to_string()),
        flash_name: flash.as_ref().map(|x| x.name().to_string()),
        authenticated: false,
    };

    Template::render("password_reset", &context)
}

#[derive(FromForm)]
pub struct PasswordReset1 {
    email: String,
}

#[post("/password_reset", data = "<password_reset>")]
fn password_reset(
    password_reset: Form<PasswordReset1>,
    real_ip: auth::RealIp,
    mailer: State<Mailer>,
    crypto_encoder: State<devand_crypto::Encoder>,
    conn: PgDevandConn,
) -> Result<Redirect, Flash<Redirect>> {
    let err_msg = "That address is not associated with a personal user account.";
    let PasswordReset1 { email } = password_reset.0;

    if let Some(user) = devand_db::load_user_by_email(email.as_str(), &conn) {
        let user_id = user.id;
        let data = PasswordReset { user_id: user_id.0 };

        let token = data.sign(&crypto_encoder);

        crate::notifications::password_reset(
            BASE_URL.unwrap_or(DEFAULT_BASE_URL),
            &mailer,
            user.email,
            token,
        );

        Ok(Redirect::to(uri!(password_reset_wait_page)))
    } else {
        log_fail(real_ip.0);
        let redirect = Redirect::to(uri!(password_reset_page));
        Err(Flash::error(redirect, err_msg))
    }
}

#[get("/password_reset/<token>")]
fn password_reset_token_page(
    token: String,
    flash: Option<FlashMessage>,
    crypto_decoder: State<devand_crypto::Decoder>,
) -> Template {
    #[derive(Serialize)]
    struct Context {
        title: &'static str,
        token: String,
        flash_msg: Option<String>,
        flash_name: Option<String>,
        authenticated: bool,
        valid_token: bool,
    }

    // Here we decode the token just to give an immediate feedback to user about its validity
    // It is checked again on form submission
    let valid_token =
        PasswordReset::try_from_token(&token.clone().into(), &crypto_decoder).is_some();

    let context = Context {
        title: "Reset your password",
        token,
        flash_msg: flash.as_ref().map(|x| x.msg().to_string()),
        flash_name: flash.as_ref().map(|x| x.name().to_string()),
        authenticated: false,
        valid_token,
    };

    Template::render("password_reset_new", &context)
}

#[get("/password_reset/wait")]
fn password_reset_wait_page() -> Template {
    #[derive(Serialize)]
    struct Context {
        title: &'static str,
        authenticated: bool,
    }

    let context = Context {
        title: "Check your email",
        authenticated: false,
    };

    Template::render("password_reset_wait", &context)
}

#[derive(FromForm)]
pub struct PasswordReset2 {
    password: String,
}

#[post("/password_reset/<token>", data = "<password_reset>")]
fn password_reset_token(
    real_ip: auth::RealIp,
    token: String,
    password_reset: Form<PasswordReset2>,
    crypto_decoder: State<devand_crypto::Decoder>,
    conn: PgDevandConn,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let ok_msg = "Your password has been changed! You can now sign in with your new password.";
    let redirect_ok = Redirect::to(uri!(login_page));
    let redirect_err = Redirect::to(uri!(password_reset_token_page: token.clone()));

    let PasswordReset2 { password } = password_reset.0;

    if !devand_core::auth::is_valid_password(&password) {
        let err_msg = "Invalid password";
        return Err(Flash::error(redirect_err, err_msg));
    }

    let token = SignedToken::from(token);
    let signed_data = PasswordReset::try_from_token(&token, &crypto_decoder);

    match signed_data {
        Some(PasswordReset { user_id }) => {
            let user_id = devand_core::UserId(user_id);
            devand_db::auth::set_password(user_id, &password, &conn)
                .ok()
                .expect("Password to be updated on database");
            Ok(Flash::success(redirect_ok, ok_msg))
        }
        None => {
            log_fail(real_ip.0);
            let err_msg = "Invalid token";
            Err(Flash::error(redirect_err, err_msg))
        }
    }
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
    mailer: State<Mailer>,
    conn: PgDevandConn,
) -> Result<Redirect, Flash<Redirect>> {
    let email_address = join_data.email.clone();
    auth::join(&mut cookies, join_data.0, expected_captcha, &conn)
        .map(|_| mailer.verify_address(email_address))
        .map(|_| Redirect::to(uri!(dashboard_index)))
        .map_err(|err| {
            log_fail(real_ip.0);
            Flash::error(Redirect::to(uri!(join_page)), err.to_string())
        })
}

#[get("/join", rank = 2)]
fn join_page(
    flash: Option<FlashMessage>,
    join_data: Option<auth::JoinData>,
    auth_data: Option<AuthData>,
) -> Result<Template, Redirect> {
    if auth_data.is_some() {
        // When user is authenticated, /join just redirect to index
        Err(Redirect::to(uri!(index)))
    } else {
        // When user is not authenticated, /join displays a form
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

        Ok(Template::render("join", &context))
    }
}

/// Generate a captcha png
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

#[get("/chat/<_username>")]
fn dashboard_chat(auth_data: AuthData, _username: String) -> Template {
    dashboard(auth_data)
}

#[get("/schedule")]
fn dashboard_schedule(auth_data: AuthData) -> Template {
    dashboard(auth_data)
}

#[get("/settings/password")]
fn dashboard_settings_password(auth_data: AuthData) -> Template {
    dashboard(auth_data)
}

#[get("/u/<_username>")]
fn dashboard_user_profile(auth_data: AuthData, _username: String) -> Template {
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
        title: "Find your pair-programming pal",
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

#[get("/help")]
fn help(auth_data: Option<AuthData>) -> Template {
    #[derive(Serialize)]
    struct Context {
        title: &'static str,
        authenticated: bool,
    }

    let context = Context {
        title: "Need help?",
        authenticated: auth_data.is_some(),
    };

    Template::render("help", &context)
}

#[get("/favicon.ico")]
fn favicon(static_dir: State<StaticDir>) -> Option<NamedFile> {
    NamedFile::open(std::path::Path::new(&static_dir.0).join("favicon.ico")).ok()
}

// TODO https://github.com/alepez/devand/issues/69 This should render a form
#[get("/verify_email/<token>")]
fn verify_email_token(
    token: String,
    auth_data: Option<AuthData>,
    crypto_decoder: State<devand_crypto::Decoder>,
    conn: PgDevandConn,
) -> Template {
    let token = SignedToken::from(token);
    let verified_data = EmailVerification::try_from_token(&token, &crypto_decoder);

    let verified = if let Some(EmailVerification { address }) = verified_data {
        devand_db::set_verified_email(&address, &conn)
            .ok()
            .expect("Email can be verified");
        true
    } else {
        false
    };

    #[derive(Serialize)]
    struct Context {
        title: &'static str,
        authenticated: bool,
        verified: bool,
    }

    let context = Context {
        title: "Email address verification",
        authenticated: auth_data.is_some(),
        verified,
    };

    Template::render("email_verification", &context)
}

pub fn routes() -> Vec<Route> {
    routes![
        index,
        join,
        join_page,
        join_captcha,
        login,
        login_page,
        password_reset,
        password_reset_page,
        password_reset_token,
        password_reset_token_page,
        password_reset_wait_page,
        logout,
        dashboard_index,
        dashboard_affinities,
        dashboard_code_now,
        dashboard_schedule,
        dashboard_chat,
        dashboard_settings_password,
        dashboard_user_profile,
        privacy,
        code_of_conduct,
        help,
        favicon,
        verify_email_token,
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
