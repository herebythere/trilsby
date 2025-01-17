"CREATE TABLE IF NOT EXISTS people (
	id INTEGER PRIMARY KEY,
	email TEXT NOT NULL UNIQUE,
	password_hash_params TEXT NOT NULL,
	updated_at INTEGER,
	deleted_at INTEGER
)"

// CRUD

// for security reasons
// you cannot set an email AND a password except on initial signup

// CREATE
"
INSERT INTO people
(id, email, password_hash_params)
VALUES
(?1, ?2, ?3);
"

// READ
"
SELECT *
FROM people
WHERE email LIKE ?1;
"

// UPDATE email
"
UPDATE people
SET
	email = ?1
	updated_at = ?2
WHERE id = ?3;
"

// UPDATE password hash
"
UPDATE people
SET
	password_hash_params
	updated_at = ?2
WHERE id = ?3;
"

// DELETE
"
UPDATE people
SET deleted_at = ?1
WHERE id = ?2;
"
