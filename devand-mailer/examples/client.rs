use devand_mailer::{CcnEmail, Client, ClientConf};

fn main() {
    dotenv::dotenv().ok();

    let conf = ClientConf {
        url: std::env::var("DEVAND_MAILER_SERVER_URL").unwrap(),
    };

    let client = Client::new(conf);

    let email = CcnEmail {
        recipients: vec!["admin@devand.dev".to_string()],
        subject: "Hei".to_string(),
        text: "Hello!\n\nThis is a **markdown** message.\n\n## This is a title".to_string(),
    };

    client.send_email(email).unwrap();
}
