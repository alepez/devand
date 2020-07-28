CREATE TABLE unread_messages (
  message_id INTEGER NOT NULL,
  user_id INTEGER NOT NULL,
  PRIMARY KEY(message_id, user_id)
)
