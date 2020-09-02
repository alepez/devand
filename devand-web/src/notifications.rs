use crate::Mailer;
use crate::PgDevandConn;
use devand_core::{User, UserId};
use devand_crypto::SignedToken;
use devand_db::load_user_by_id;
use devand_mailer::CcnEmail;

// TODO Subject/Text from text template
pub(crate) fn notify_chat_members(
    base_url: &str,
    mailer: &Mailer,
    conn: &PgDevandConn,
    from: &User,
    to: &[UserId],
) {
    // TODO Chat can have more than one user by design, but the url is for just two users
    let chat_url = format!("{}/chat/{}", base_url, &from.username);

    let subject = format!("DevAndDev - {} sent you a new message", &from.visible_name);

    let text = format!(
        "You have a message from {}. View on DevAndDev: {}",
        &from.visible_name, chat_url
    );

    let email_address_from_id = |&user_id| load_user_by_id(user_id, &conn).map(|u| u.email);

    let recipients: Vec<_> = to
        .iter()
        .filter(|&&u| u != from.id)
        .filter_map(email_address_from_id)
        .collect();

    let email = CcnEmail {
        recipients,
        subject,
        text,
    };

    if mailer.send_email(email).is_err() {
        log::error!("Cannot send email");
    }
}

// TODO Subject/Text from text template
pub(crate) fn password_reset(
    base_url: &str,
    mailer: &Mailer,
    recipient: String,
    token: SignedToken,
) {
    let token_url = format!("{}/password_reset/{}", base_url, token);
    let retry_url = format!("{}/password_reset", base_url);
    let subject = "DevAndDev - Please reset your password";
    let text = format!(
"We heard that you lost your DevAndDev password. Sorry about that!\n
\n
But don’t worry! You can use the following link to reset your password:\n
\n
{}\n
\n
If you don’t use this link within 3 hours, it will expire. To get a new password reset link, visit {}\n
\n
Thanks,\n
The DevAndDev team\n", token_url, retry_url);

    let email = CcnEmail {
        recipients: vec![recipient],
        subject: subject.into(),
        text,
    };

    mailer.send_email(email).unwrap()
}
