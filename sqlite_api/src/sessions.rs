// only 
// CREATE READ DELETE

"CREATE TABLE IF NOT EXISTS sessions (
	id INTEGER PRIMARY KEY,
	session INTEGER NOT NUll,
	role INTEGER NOT NULL,
	session_length_ms INTEGER NOT NULL,
	belongs_to INTEGER NOT NULL,
	deleted_at INTEGER
)"

// CREATE
"
INSERT INTO sessions
(id, session, role, session_length_ms, belongs_to)
VALUES
(?1, ?2, ?3);
"

// READ
"
SELECT *
FROM sessions
WHERE id LIKE ?1 and session LIKE ?2;
"

// READ BY PERSON
"
SELECT *
FROM sessions
WHERE belongs_to LIKE ?1;
"

// DELETE
"
UPDATE sessions
SET deleted_at = ?1
WHERE id = ?2;
"
