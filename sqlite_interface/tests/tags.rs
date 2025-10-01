use rusqlite::{Connection, Result};
use sqlite_interface::tags;
use type_flyweight::Tag;

#[test]
fn crud_operations() -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = Connection::open_in_memory()?;

    if let Err(_e) = tags::create_table(&mut conn) {
        assert!(false, "failed to create tags table");
    }

    let incorrect_tag = Tag {
        id: 0,
        tag_kind_id: 7,
        bookmark_id: 13,
        people_id: 41,
        deleted_at: None,
    };

    // create
    let tag_entry = match tags::create(&mut conn, 1, 2, 3, 4) {
        Ok(ck) => ck,
        Err(e) => {
            assert!(false, "failed to create tag entry");
            return Err(e.into());
        }
    };

    // read
    let mut tag_read_entry = match tags::read(&mut conn, 1, 0) {
        Ok(mut ck) => ck.pop(),
        Err(e) => return Err(e.into()),
    };

    assert!(None != tag_entry);
    assert!(tag_entry == tag_read_entry);
    assert!(Some(incorrect_tag) != tag_read_entry);

    Ok(())
}
