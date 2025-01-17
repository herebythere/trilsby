// add ownership

"CREATE TABLE IF NOT EXISTS bookmark_collections (
	id INTEGER PRIMARY KEY,
	title TEXT NOT NULL,
	belongs_to INTEGER NOT NULL,
	updated_at INTEGER,
	deleted_at INTEGER
)"

// CREATE
"
INSERT INTO bookmark_collections
(id, title, belongs_to)
VALUES
(?1, ?2, ?3);
"

// READ
"
SELECT *
FROM bookmark_collections
WHERE id LIKE ?1 AND belongs_to ?2;
"

// READ BY PEOPLE
"
SELECT *
FROM bookmark_collections
WHERE belongs_to LIKE ?1;
"

// UPDATE title
"
UPDATE bookmark_collections
SET
	title = ?1
	updated_at = ?2
WHERE id = ?3;
"

// UPDATE password hash
"
UPDATE bookmark_collections
SET
	password_hash_params
	updated_at = ?2
WHERE id = ?3;
"

// DELETE
"
UPDATE bookmark_collections
SET deleted_at = ?1
WHERE id = ?2 and belongs_to = ?3;
"
