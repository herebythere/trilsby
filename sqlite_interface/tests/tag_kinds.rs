use rusqlite::{Connection, Result};
use sqlite_interface::tag_kinds;
use type_flyweight::tags::TagKind;

#[test]
fn crud_operations() -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = Connection::open_in_memory()?;

    if let Err(_e) = tag_kinds::create_table(&mut conn) {
        assert!(false, "failed to create tag_kinds table");
    }

    let incorrect_tag_kind = TagKind {
        id: 0,
        kind: "the hubris of our science".to_string(),
        people_id: 2,
        deleted_at: None,
    };

    // create
    let tag_kind = match tag_kinds::create(&mut conn, 1, "bright and loud", 3) {
        Ok(ck) => ck,
        Err(e) => return Err(e.into()),
    };

    // read
    let tag_kind_read_by_id = match tag_kinds::read(&mut conn, 1, 0) {
        Ok(mut ck) => ck.pop(),
        Err(e) => return Err(e.into()),
    };

    assert!(None != tag_kind);
    assert!(tag_kind == tag_kind_read_by_id);
    assert!(Some(incorrect_tag_kind.clone()) != tag_kind_read_by_id);
    if let Some(ref entry) = tag_kind {
        assert!(entry.id == 1);
        assert!(entry.people_id == 3);
    }

    // read by kind
    let tag_kind_read_by_kind = match tag_kinds::read_by_kind(&mut conn, "bright and loud") {
        Ok(ck) => ck,
        Err(e) => return Err(e.into()),
    };

    assert!(None != tag_kind);
    assert!(tag_kind == tag_kind_read_by_kind);
    assert!(Some(incorrect_tag_kind) != tag_kind_read_by_kind);

    Ok(())
}
