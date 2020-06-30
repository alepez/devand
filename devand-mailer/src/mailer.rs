use lettre::smtp::authentication::Credentials;
use lettre::{SmtpClient, Transport};
use lettre_email::{Email, Mailbox};
use std::sync::mpsc;

pub struct Mailer {
    from: Mailbox,
    tx: mpsc::Sender<Email>,
    #[allow(dead_code)]
    thread: std::thread::JoinHandle<()>,
}

impl Mailer {
    pub fn new(server: String, username: String, password: String, from_name: String) -> Self {
        let from = Mailbox::new_with_name(from_name, username.clone());

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
        let email = create_email(&self.from, recipient, subject, text);

        match self.tx.send(email) {
            Ok(_) => {
                log::debug!("Email sent");
            }
            Err(_) => {
                log::debug!("Error sending email");
            }
        }
    }
}

fn create_email(from: &Mailbox, recipient: &str, subject: &str, text: &str) -> Email {
    let html = comrak::markdown_to_html(text, &comrak::ComrakOptions::default());

    Email::builder()
        .to(recipient)
        .from(from.clone())
        .subject(subject)
        .text(text)
        .html(html)
        .message_type(lettre_email::MimeMultipartType::Alternative)
        .build()
        .unwrap()
}
