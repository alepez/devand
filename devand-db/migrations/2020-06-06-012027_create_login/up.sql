CREATE VIEW login AS
SELECT users.id   as user_id,
       users.username as username,
       auth.enc_password as enc_password
FROM users INNER JOIN auth ON users.id = auth.user_id
;

