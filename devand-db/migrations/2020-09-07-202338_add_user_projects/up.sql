ALTER TABLE users
ADD COLUMN projects text[] DEFAULT '{}' NOT NULL;
