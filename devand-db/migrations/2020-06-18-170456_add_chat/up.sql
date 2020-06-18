CREATE TABLE chats (
  id SERIAL PRIMARY KEY,
  members JSONB NOT NULL
);

CREATE TABLE messages (
  id SERIAL PRIMARY KEY,
  chat_id INTEGER NOT NULL,
  created_at TIMESTAMP NOT NULL,
  txt VARCHAR(512) NOT NULL,
  author INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS messages_chat_id_index ON messages (chat_id);
