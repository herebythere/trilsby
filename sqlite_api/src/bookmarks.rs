"CREATE TABLE IF NOT EXISTS bookmarks (
	id INTEGER PRIMARY KEY,
	url TEXT NOT NULL,
	bookmark_collection_id INTEGER NOT NULL,
	belongs_to INTEGER NOT NULL,
	updated_at INTEGER,
	deleted_at INTEGER
)"

// CREATE
"
INSERT INTO bookmarks
(id, url, bookmark_collection_id, belongs_to)
VALUES
(?1, ?2, ?3 ?4);
"

// READ
"
SELECT *
FROM bookmarks
WHERE id LIKE ?1;
"

// READ BY PEOPLE
"
SELECT *
FROM bookmarks
WHERE belongs_to LIKE ?1;
"

// READ BY COLLECTION
"
SELECT *
FROM bookmarks
WHERE bookmark_collection_id = ?1 AND belongs_to LIKE ?2;
"

// UPDATE title
"
UPDATE bookmarks
SET
	url = ?1
	updated_at = ?2
WHERE id = ?3;
"

// UPDATE bookmark_collection_id
"
UPDATE bookmarks
SET
	bookmark_collection_id = ?1
	updated_at = ?2
WHERE id = ?3;
"

// DELETE
"
UPDATE bookmarks
SET deleted_at = ?1
WHERE id = ?2 and belongs_to = ?3;
"
