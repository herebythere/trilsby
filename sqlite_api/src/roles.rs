// only 
// CREATE READ DELETE

"CREATE TABLE IF NOT EXISTS roles (
	id INTEGER PRIMARY KEY,
	role TEXT NOT NULL,
	belongs_to INTEGER NOT NULL,
	deleted_at INTEGER
)"

// CREATE
"
INSERT INTO roles
(id, belongs_to, password_hash_params)
VALUES
(?1, ?2, ?3);
"

// READ
"
SELECT *
FROM roles
WHERE belongs_to LIKE ?1 and role LIKE ?2;
"

// READ ALL
"
SELECT *
FROM roles
WHERE belongs_to LIKE = ?1;
"

// DELETE
"
UPDATE roles
SET deleted_at = ?1
WHERE id = ?2 AND belongs_to = ?3;
"
