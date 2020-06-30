use lettre::smtp::authentication::Credentials;
use lettre::{SmtpClient, Transport};
use lettre_email::Mailbox;
use std::sync::mpsc;

#[derive(Debug)]
struct Email {
    recipient: String,
    subject: String,
    text: String,
    address_must_be_verified: bool,
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

        let conn = devand_db::establish_connection();

        let thread = std::thread::spawn(move || {
            rx.iter()
                .filter(|email| {
                    if !email.address_must_be_verified {
                        true
                    } else {
                        let verified = devand_db::is_verified_email(&email.recipient, &conn);
                        if !verified {
                            log::debug!(
                                "Not sending email to {} because it is not a verified address",
                                &email.recipient
                            );
                        }
                        verified
                    }
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

    // TODO [optimize] pass String
    pub fn send_email(&self, recipient: &str, subject: &str, text: &str) {
        let email = Email {
            recipient: recipient.to_string(),
            subject: subject.to_string(),
            text: text.to_string(),
            address_must_be_verified: true,
        };

        // Note: an error here can only be caused by problem on channel
        self.tx.send(email).expect("Sending email");
    }

    pub fn verify_address(&self, recipient: String) {
        // FIXME
        let subject = "FIXME Email verification";
        let text = "FIXME";

        let email = Email {
            recipient,
            subject: subject.to_string(),
            text: text.to_string(),
            address_must_be_verified: false,
        };

        // Note: an error here can only be caused by problem on channel
        self.tx.send(email).expect("Sending email");
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
