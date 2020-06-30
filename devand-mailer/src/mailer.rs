use lettre::smtp::authentication::Credentials;
use lettre::{SmtpClient, Transport};
use lettre_email::Email;
use std::sync::mpsc;

pub struct Mailer {
    from: String,
    thread: std::thread::JoinHandle<()>,
    tx: mpsc::Sender<Email>,
}

impl Mailer {
    pub fn new(server: String, username: String, password: String) -> Self {
        let from = username.clone();
        let creds = Credentials::new(username, password);

        let mut transport = SmtpClient::new_simple(server.as_str())
            .unwrap()
            .credentials(creds)
            .transport();

        let (tx, rx): (mpsc::Sender<Email>, mpsc::Receiver<Email>) = mpsc::channel();

        let thread = std::thread::spawn(move || {
            for email in rx {
                log::debug!("Email: {:?}", email);

                let result = transport.send(email.into());

                if result.is_err() {
                    log::error!("Could not send email: {:?}", result);
                }
            }
        });

        Self { from, thread, tx }
    }

    pub fn send_email(&self, recipient: &str, subject: &str, text: &str) {
        let from = self.from.as_str();

        let email = Email::builder()
            .to(recipient)
            .from(from)
            .subject(subject)
            .text(text)
            .build()
            .unwrap();

        self.tx.send(email);
    }
}
