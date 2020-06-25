use devand_mailer::{Client, ClientConf};

fn main() {
    dotenv::dotenv().ok();

    let conf = ClientConf {
        url: std::env::var("DEVAND_MAILER_SERVER_URL").unwrap(),
    };

    let client = Client::new(conf);

    client.send_email(
        vec!["admin@devand.dev".to_string()],
        "Hei".to_string(),
        "Ciao!".to_string(),
    );
}
