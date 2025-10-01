use rusqlite::{Connection, Result};
use sqlite_interface::bookmark_list_to_bookmarks;
use type_flyweight::BookmarkListToBookmark;

#[test]
fn crud_operations() -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = Connection::open_in_memory()?;

    if let Err(_e) = bookmark_list_to_bookmarks::create_table(&mut conn) {
        assert!(false, "failed to create bookmark_list_to_bookmarks table");
    }

    let incorrect_list_to_bookmark = BookmarkListToBookmark {
        id: 0,
        bookmark_list_id: 5,
        bookmark_id: 11,
        order_weight: 17,
        people_id: 41,
        deleted_at: None,
    };

    // create
    let list_to_bookmark_entry = match bookmark_list_to_bookmarks::create(&mut conn, 1, 2, 3, 4, 5)
    {
        Ok(ck) => ck,
        Err(e) => {
            assert!(false, "failed to create bookmark_list_to_bookmarks entry");
            return Err(e.into());
        }
    };

    // read
    let mut list_bookmark_read_entry = match bookmark_list_to_bookmarks::read(&mut conn, 1, 0) {
        Ok(mut ck) => ck.pop(),
        Err(e) => return Err(e.into()),
    };

    assert!(None != list_to_bookmark_entry);
    assert!(list_to_bookmark_entry == list_bookmark_read_entry);
    assert!(Some(incorrect_list_to_bookmark.clone()) != list_bookmark_read_entry);

    // read by people id
    let mut list_to_bookmark_read_by_people_id_entry =
        match bookmark_list_to_bookmarks::read_by_people_id(&mut conn, 5, 1, 0) {
            Ok(mut ck) => ck.pop(),
            Err(e) => return Err(e.into()),
        };

    assert!(None != list_to_bookmark_entry);
    assert!(list_to_bookmark_entry == list_to_bookmark_read_by_people_id_entry);
    assert!(Some(incorrect_list_to_bookmark.clone()) != list_to_bookmark_read_by_people_id_entry);

    // read by bookmark list id
    let mut list_to_bookmark_read_by_bookmark_list_id_entry =
        match bookmark_list_to_bookmarks::read_by_bookmark_list_id(&mut conn, 2, 1, 0) {
            Ok(mut ck) => ck.pop(),
            Err(e) => return Err(e.into()),
        };

    assert!(None != list_to_bookmark_entry);
    assert!(list_to_bookmark_entry == list_to_bookmark_read_by_bookmark_list_id_entry);
    assert!(
        Some(incorrect_list_to_bookmark.clone()) != list_to_bookmark_read_by_bookmark_list_id_entry
    );

    Ok(())
}
