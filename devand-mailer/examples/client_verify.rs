#[cfg(feature = "client")]
fn main() {
    use devand_mailer::{Client, ClientConf};
    dotenv::dotenv().ok();

    let conf = ClientConf {
        url: std::env::var("DEVAND_MAILER_SERVER_URL").unwrap(),
    };

    let client = Client::new(conf);

    client.verify_address("admin@devand.dev".to_string());
}

#[cfg(not(feature = "client"))]
fn main() {}
