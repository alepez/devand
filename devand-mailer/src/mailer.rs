use devand_crypto::{EmailVerification, Signable};
use devand_text::Text;
use lettre::smtp::authentication::Credentials;
use lettre::smtp::ConnectionReuseParameters;
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
    encoder: devand_crypto::Encoder,
}

impl Mailer {
    pub fn new(
        server: String,
        username: String,
        password: String,
        from_name: String,
        encoder: devand_crypto::Encoder,
    ) -> Self {
        let from = Mailbox::new_with_name(from_name, username.clone());

        let creds = Credentials::new(username, password);

        // Reuse connection to avoid server rate limit when sending bulk email
        let mut transport = SmtpClient::new_simple(server.as_str())
            .unwrap()
            .credentials(creds)
            .connection_reuse(ConnectionReuseParameters::ReuseUnlimited)
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
                .filter_map(|email| {
                    create_email(from.clone(), email.recipient, email.subject, email.text)
                })
                .map(|email| transport.send(email.into()))
                .for_each(|result| {
                    if result.is_err() {
                        log::error!("Could not send email: {:?}", result);
                    }
                })
        });

        Self {
            thread,
            tx,
            encoder,
        }
    }

    pub fn send_email(&self, recipient: String, subject: String, text: String) {
        let email = Email {
            recipient,
            subject,
            text,
            address_must_be_verified: true,
        };

        // Note: an error here can only be caused by problem on channel
        self.tx.send(email).expect("Sending email");
    }

    pub fn verify_address(&self, recipient: String) {
        let data = EmailVerification {
            address: recipient.clone(),
        };

        let token = data.sign(&self.encoder);

        // FIXME Base url
        let url = format!("https://devand.dev/verify_email/{}", token);

        let subject = Text::EmailVerifySubject.to_string();
        let text = Text::EmailVerifyBodyMarkdown(&url).to_string();

        let email = Email {
            recipient,
            subject,
            text,
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
) -> Option<lettre_email::Email> {
    let html = comrak::markdown_to_html(&text, &comrak::ComrakOptions::default());

    let result = lettre_email::Email::builder()
        .to(recipient)
        .from(from)
        .subject(subject)
        .text(text)
        .html(html)
        .message_type(lettre_email::MimeMultipartType::Alternative)
        .build();

    match result {
        Err(err) => {
            log::error!("Error while creating email: {:?}", err);
            None
        }
        Ok(email) => Some(email),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mail_body() {
        let url = "https://devand.dev/verify_email/foobar";
        let body = Text::EmailVerifyBodyMarkdown(url).to_string();
        // We check lenght for regressions, especially to be sure that unwanted
        // blank spaces are added.
        assert_eq!(body.len(), 202);
    }
}
