use rusqlite::{Connection, Result};
use sqlite_interface::bookmarks;
use type_flyweight::bookmarks::Bookmark;

#[test]
fn crud_operations() -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = Connection::open_in_memory()?;

    if let Err(_e) = bookmarks::create_table(&mut conn) {
        assert!(false, "failed to create bookmarks table");
    }

    let incorrect_bookmark = Bookmark {
        id: 0,
        url: "https://pup-pup.com".to_string(),
        people_id: 41,
        deleted_at: None,
    };

    // create
    let bookmark_entry = match bookmarks::create(&mut conn, 1, "https://w-lfpup.com", 3) {
        Ok(ck) => ck,
        Err(e) => {
            assert!(false, "failed to create tag entry");
            return Err(e.into());
        }
    };

    // read
    let mut bookmark_read_entry = match bookmarks::read(&mut conn, 1, 0) {
        Ok(mut ck) => ck.pop(),
        Err(e) => return Err(e.into()),
    };

    assert!(None != bookmark_entry);
    assert!(bookmark_entry == bookmark_read_entry);
    assert!(Some(incorrect_bookmark) != bookmark_read_entry);

    Ok(())
}
