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

    // read
    let mut tag_read_entry = match tags::read(&mut conn, 1, 0, "DESC") {
        Ok(mut ck) => ck.pop(),
        Err(e) => return Err(e.into()),
    };

    assert!(None != tag_entry);
    assert!(tag_entry == tag_read_entry);

    // read by tag kind id
    let mut tag_read_by_kind_id_entry = match tags::read_by_id(&mut conn, 10) {
        Ok(ck) => ck,
        Err(e) => return Err(e.into()),
    };

    assert!(None != tag_entry);
    assert!(tag_entry == tag_read_by_kind_id_entry);

    // read by people id
    let mut tag_read_by_people_id_entry = match tags::read_by_people_id(&mut conn, 3, 1, 0) {
        Ok(mut ck) => ck.pop(),
        Err(e) => return Err(e.into()),
    };

    assert!(None != tag_entry);
    assert!(tag_entry == tag_read_by_people_id_entry);

    // read by people id
    let mut tag_read_by_title_entry = match tags::read_by_title(&mut conn, "public") {
        Ok(mut ck) => ck,
        Err(e) => return Err(e.into()),
    };

    assert!(None != tag_entry);
    assert!(tag_entry == tag_read_by_title_entry);

    Ok(())
}
