DROP TABLE IF EXISTS chats;
DROP TABLE IF EXISTS messages;
DROP TABLE IF EXISTS unread_messages;
DROP INDEX IF EXISTS messages_chat_id_index;

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE chats (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  members INTEGER[] NOT NULL
);

CREATE TABLE messages (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  chat_id UUID NOT NULL,
  created_at TIMESTAMP NOT NULL,
  txt VARCHAR(512) NOT NULL,
  author INTEGER NOT NULL
);

CREATE TABLE unread_messages (
  message_id UUID NOT NULL,
  user_id INTEGER NOT NULL,
  PRIMARY KEY(message_id, user_id)
)
