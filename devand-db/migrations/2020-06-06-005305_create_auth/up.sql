CREATE TABLE auth (
  user_id SERIAL PRIMARY KEY,
  enc_password VARCHAR(1024) NOT NULL
);
