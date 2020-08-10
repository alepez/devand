use env_logger::Env;

#[cfg(feature = "client")]
fn main() {
    use devand_mailer::{Client, ClientConf};
    dotenv::dotenv().ok();

    env_logger::from_env(Env::default().default_filter_or("devand_verify_address_reminder=info"))
        .init();

    let conf = ClientConf {
        url: std::env::var("DEVAND_MAILER_SERVER_URL")
            .unwrap_or_else(|_| "http://127.0.0.1:80".to_string()),
    };

    let conn = devand_db::establish_connection();

    let client = Client::new(conf);

    log::info!("Listing unverified emails...");
    let unverified_emails = devand_db::list_unverified_emails(&conn).unwrap();

    if unverified_emails.is_empty() {
        log::info!("All address are verified");
    } else {
        log::info!("{} addresses must be verified", unverified_emails.len());
    }

    unverified_emails.into_iter().for_each(|address| {
        log::info!("Sending verification email to {}", &address);

        if let Err(err) = client.verify_address(address.clone()) {
            log::error!("Cannot verify {}: {:?}", address, err);
        }
    });
}

#[cfg(not(feature = "client"))]
fn main() {}
