use crate::Mailer;
use crate::PgDevandConn;
use devand_core::{User, UserId};

pub(crate) fn notify_chat_members(mailer: &Mailer, conn: &PgDevandConn, from: &User, to: &[UserId]) {
    // TODO Base url from configuration
    // TODO Chat can have more than one user by design, but the url is for just two users
    let chat_url = format!("https://devand.dev/chat/{}", &from.username);

    let subject = format!("DevAndDev - {} sent you a new message", &from.visible_name);
    let text = format!(
        "You have a message from {}. View on DevAndDev: {}",
        &from.visible_name, chat_url
    );

    let recipients: Vec<_> = to
        .iter()
        .filter(|&&u| u != from.id)
        .filter_map(|&user_id| devand_db::load_user_by_id(user_id, &conn).map(|u| u.email))
        .collect();

    // TODO This call is blocking and takes too much time. Just send and forget
    mailer.send_email(recipients, subject.to_string(), text.to_string());
}
