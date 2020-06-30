use lettre::smtp::authentication::Credentials;
use lettre::{SmtpClient, Transport};
use lettre_email::Mailbox;
use std::sync::mpsc;

#[derive(Debug)]
struct Email {
    recipient: String,
    subject: String,
    text: String,
}

pub struct Mailer {
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
            rx.iter()
                .filter(|email| {
                    // TODO Filter verified emails
                    true
                })
                .map(|email| create_email(from.clone(), email.recipient, email.subject, email.text))
                .map(|email| transport.send(email.into()))
                .for_each(|result| {
                    if result.is_err() {
                        log::error!("Could not send email: {:?}", result);
                    }
                })
        });

        Self { thread, tx }
    }

    pub fn send_email(&self, recipient: &str, subject: &str, text: &str) {
        let email = Email {
            recipient: recipient.to_string(),
            subject: subject.to_string(),
            text: text.to_string(),
        };

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

fn create_email(
    from: Mailbox,
    recipient: String,
    subject: String,
    text: String,
) -> lettre_email::Email {
    let html = comrak::markdown_to_html(&text, &comrak::ComrakOptions::default());

    lettre_email::Email::builder()
        .to(recipient)
        .from(from)
        .subject(subject)
        .text(text)
        .html(html)
        .message_type(lettre_email::MimeMultipartType::Alternative)
        .build()
        .unwrap()
}
