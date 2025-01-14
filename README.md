# sqlite api

people (1 per user) < bookmark collection < bookmarks

CRUD operations for 

// people
id
email
password_hash_params

// bookmark collections / playlist
id
person_id int64
title TEXT

bookmarks
id int64
people_id int64
bookmark_collection_id int64
url TEXT (limit by 2048? No Query params?)



// main point is "get random"

tags

eventually need a:
- descriptions, comments
- search by tags
- search by "does not require login"
- get random only from "does not require logic" or "confidently flagged as AI"
- subscribe / follow / alert by tags
- flag link as against guidelines
	- AI
	- Walled Gardens (ie: Reddit, only fans, requires login)
- admin site to edit and remove content