use lettre::smtp::authentication::Credentials;
use lettre::{SmtpClient, Transport};
use lettre_email::Email;
use std::sync::{Arc, Mutex};

pub struct Mailer {
    transport: Arc<Mutex<lettre::SmtpTransport>>,
    from: String,
}

impl Mailer {
    pub fn new(server: String, username: String, password: String) -> Self {
        let from = username.clone();
        let creds = Credentials::new(username, password);

        let transport = SmtpClient::new_simple(server.as_str())
            .unwrap()
            .credentials(creds)
            .transport();

        Self {
            transport: Arc::new(Mutex::new(transport)),
            from,
        }
    }

    pub fn send_email(&self, recipient: &str, subject: &str, text: &str) {
        log::info!("Send {:?} {:?} to {}", subject, text, recipient);

        let from = self.from.as_str();

        let email = Email::builder()
            .to(recipient)
            .from(from)
            .subject(subject)
            .text(text)
            .build()
            .unwrap();

        let result = self.transport.lock().unwrap().send(email.into());

        if result.is_ok() {
            log::debug!("Email sent");
        } else {
            log::error!("Could not send email: {:?}", result);
        }

        assert!(result.is_ok());
    }
}
