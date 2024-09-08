-- Add last login timestamp to user table

ALTER TABLE users ADD COLUMN last_login;
UPDATE users SET last_login = CURRENT_TIMESTAMP;