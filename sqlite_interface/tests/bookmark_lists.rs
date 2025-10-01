use rusqlite::{Connection, Result};
use sqlite_interface::bookmark_lists;
use type_flyweight::BookmarkList;

#[test]
fn crud_operations() -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = Connection::open_in_memory()?;

    if let Err(_e) = bookmark_lists::create_table(&mut conn) {
        assert!(false, "failed to create bookmark_lists table");
    }

    let incorrect_bookmark_list = BookmarkList {
        id: 0,
        people_id: 41,
        deleted_at: None,
    };

    // create
    let bookmark_list_entry = match bookmark_lists::create(&mut conn, 1, 3) {
        Ok(ck) => ck,
        Err(e) => {
            assert!(false, "failed to create bookmark_list entry");
            return Err(e.into());
        }
    };

    // read
    let mut bookmark_list_read_entry = match bookmark_lists::read(&mut conn, 1, 0) {
        Ok(mut ck) => ck.pop(),
        Err(e) => return Err(e.into()),
    };

    assert!(None != bookmark_list_entry);
    assert!(bookmark_list_entry == bookmark_list_read_entry);
    assert!(Some(incorrect_bookmark_list.clone()) != bookmark_list_read_entry);

    // read by id
    let mut bookmark_list_read_by_id_entry = match bookmark_lists::read_by_id(&mut conn, 1) {
        Ok(mut ck) => ck,
        Err(e) => return Err(e.into()),
    };

    assert!(None != bookmark_list_entry);
    assert!(bookmark_list_entry == bookmark_list_read_entry);
    assert!(Some(incorrect_bookmark_list.clone()) != bookmark_list_read_entry);

    // read by people id
    let mut bookmark_list_read_by_id_entry =
        match bookmark_lists::read_by_people_id(&mut conn, 3, 1, 0) {
            Ok(mut ck) => ck.pop(),
            Err(e) => return Err(e.into()),
        };

    assert!(None != bookmark_list_entry);
    assert!(bookmark_list_entry == bookmark_list_read_entry);
    assert!(Some(incorrect_bookmark_list.clone()) != bookmark_list_read_entry);

    Ok(())
}
