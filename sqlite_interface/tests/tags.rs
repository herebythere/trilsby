use rusqlite::{Connection, Result};
use sqlite_interface::tags;
use type_flyweight::tags::Tag;

#[test]
fn crud_operations() -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = Connection::open_in_memory()?;

    if let Err(_e) = tags::create_table(&mut conn) {
        assert!(false, "failed to create tags table");
    }

    // create
    let tag_entry = match tags::create(&mut conn, 10, 3, "public") {
        Ok(ck) => ck,
        Err(e) => {
            assert!(false, "failed to create tag entry");
            return Err(e.into());
        }
    };
    assert!(None != tag_entry);

    // read
    let tag_read_entry = match tags::read(&mut conn, 1, 0, "DESC") {
        Ok(mut ck) => ck.pop(),
        Err(e) => return Err(e.into()),
    };

    assert!(tag_entry == tag_read_entry);

    // read by tag kind id
    let tag_read_by_kind_id_entry = match tags::read_by_id(&mut conn, 10) {
        Ok(ck) => ck,
        Err(e) => return Err(e.into()),
    };

    assert!(tag_entry == tag_read_by_kind_id_entry);

    // read by people id
    let tag_read_by_people_id_entry = match tags::read_by_people_id(&mut conn, 3, 1, 0) {
        Ok(mut ck) => ck.pop(),
        Err(e) => return Err(e.into()),
    };

    assert!(tag_entry == tag_read_by_people_id_entry);

    // read by title
    let tag_read_by_title_entry = match tags::read_by_title(&mut conn, "public") {
        Ok(mut ck) => ck,
        Err(e) => return Err(e.into()),
    };

    assert!(tag_entry == tag_read_by_title_entry);

    // soft delete by id
    let tag_soft_delete_entry = match tags::delete(&mut conn, 10, 42) {
        Ok(mut ck) => ck,
        Err(e) => return Err(e.into()),
    };

    assert!(None != tag_soft_delete_entry);

    match (tag_entry.clone(), tag_soft_delete_entry.clone()) {
        (Some(entry), Some(soft_entry)) => {
            assert!(entry.id == soft_entry.id);
            assert!(None != soft_entry.deleted_at);
        }
        _ => assert!(false, "entries to delete do non exist"),
    }

    // dangerously delete
    let tag_dangerous_delete_entry =
        match tags::dangerously_delete_stale_entries(&mut conn, 52, 10, 10) {
            Ok(ck) => ck,
            Err(e) => return Err(e.into()),
        };

    assert!(0 != tag_dangerous_delete_entry.len());

    // read, or fail to read, deleted
    let mut tag_re_read_entry = match tags::read(&mut conn, 1, 0, "DESC") {
        Ok(mut ck) => ck.pop(),
        Err(e) => return Err(e.into()),
    };

    assert!(None == tag_re_read_entry);

    Ok(())
}
