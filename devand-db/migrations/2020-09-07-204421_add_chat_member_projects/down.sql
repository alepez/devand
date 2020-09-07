DROP VIEW chat_members;

CREATE VIEW chat_members AS
SELECT chats.id as chat_id,
       chats.members as members,
       users.id as user_id,
       users.username as username,
       users.visible_name as visible_name,
       users.bio as bio,
       users.settings->'languages' as languages,
       users.settings->'spoken_languages' as spoken_languages
FROM chats INNER JOIN users ON members @> ARRAY[users.id];
