use crate::fixture_path;
use git_odb::compound::Db;

fn db() -> Db {
    Db::at(fixture_path("objects")).expect("valid object path")
}

#[test]
fn size_of_compound_object() {
    assert_eq!(std::mem::size_of::<git_odb::compound::Object>(), 856);
}

mod init {
    use crate::compound::db;

    #[test]
    fn has_packs() {
        assert_eq!(db().packs.len(), 3)
    }
}

mod locate {
    use crate::{compound::db, hex_to_id};
    use git_odb::compound::Db;

    fn can_locate(db: &Db, hex_id: &str) {
        let mut buf = vec![];
        assert!(db.locate(hex_to_id(hex_id), &mut buf).expect("no read error").is_some());
    }

    #[test]
    fn loose_object() {
        can_locate(&db(), "37d4e6c5c48ba0d245164c4e10d5f41140cab980");
    }

    #[test]
    fn pack_object() {
        can_locate(&db(), "501b297447a8255d3533c6858bb692575cdefaa0"); // pack 11fd
        can_locate(&db(), "4dac9989f96bc5b5b1263b582c08f0c5f0b58542"); // pack a2bf
        can_locate(&db(), "dd25c539efbb0ab018caa4cda2d133285634e9b5"); // pack c043
    }
}
