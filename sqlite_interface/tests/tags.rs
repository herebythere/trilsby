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
    assert!(Some(incorrect_tag.clone()) != tag_read_entry);

    // read by tag kind id
    let mut tag_read_by_kind_id_entry = match tags::read_by_tag_kind_id(&mut conn, 2, 1, 0) {
        Ok(mut ck) => ck.pop(),
        Err(e) => return Err(e.into()),
    };

    assert!(None != tag_entry);
    assert!(tag_entry == tag_read_by_kind_id_entry);
    assert!(Some(incorrect_tag.clone()) != tag_read_by_kind_id_entry);

    // read by bookmark id
    let mut tag_read_by_bookmark_id_entry = match tags::read_by_bookmark_id(&mut conn, 3, 1, 0) {
        Ok(mut ck) => ck.pop(),
        Err(e) => return Err(e.into()),
    };

    assert!(None != tag_entry);
    assert!(tag_entry == tag_read_by_bookmark_id_entry);
    assert!(Some(incorrect_tag.clone()) != tag_read_by_bookmark_id_entry);

    // read by people id
    let mut tag_read_by_people_id_entry = match tags::read_by_people_id(&mut conn, 4, 1, 0) {
        Ok(mut ck) => ck.pop(),
        Err(e) => return Err(e.into()),
    };

    assert!(None != tag_entry);
    assert!(tag_entry == tag_read_by_people_id_entry);
    assert!(Some(incorrect_tag.clone()) != tag_read_by_people_id_entry);

    Ok(())
}
