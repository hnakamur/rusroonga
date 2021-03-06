extern crate rusroonga;
extern crate tempdir;

use rusroonga as grn;
use std::{env, fs};
use std::rc::Rc;

#[test]
fn test_open_non_existent_database() {
    grn::Groonga::new().unwrap();

    let work_dir = tempdir::TempDir::new(env::temp_dir().to_str().unwrap()).unwrap();
    let mut buf = work_dir.into_path();
    buf.push("test.db");
    let path = buf.to_str().unwrap();

    let ctx = Rc::new(grn::Context::new().unwrap());
    match grn::Database::open(ctx.clone(), path) {
        Err(e) => assert_eq!(rusroonga::NO_SUCH_FILE_OR_DIRECTORY, e.code),
        Ok(_) => assert!(false, "should fail to open database"),
    }
}

#[test]
fn test_create_database() {
    grn::Groonga::new().unwrap();

    let work_dir = tempdir::TempDir::new(env::temp_dir().to_str().unwrap()).unwrap();
    let mut buf = work_dir.into_path();
    buf.push("test.db");
    let path = buf.to_str().unwrap();

    let ctx = Rc::new(grn::Context::new().unwrap());
    let db = grn::Database::create(ctx.clone(), path).unwrap();
    assert_eq!(path, db.path().unwrap());
    assert!(fs::metadata(path).unwrap().is_file());
}

#[test]
fn test_open_or_create_database() {
    grn::Groonga::new().unwrap();

    let work_dir = tempdir::TempDir::new(env::temp_dir().to_str().unwrap()).unwrap();
    let mut buf = work_dir.into_path();
    buf.push("test.db");
    let path = buf.to_str().unwrap();

    let ctx = Rc::new(grn::Context::new().unwrap());
    let db = grn::Database::open_or_create(ctx.clone(), path).unwrap();
    assert_eq!(path, db.path().unwrap());
    assert!(fs::metadata(path).unwrap().is_file());
}

#[test]
fn test_open_database() {
    grn::Groonga::new().unwrap();

    let work_dir = tempdir::TempDir::new(env::temp_dir().to_str().unwrap()).unwrap();
    let mut buf = work_dir.into_path();
    buf.push("test.db");
    let path = buf.to_str().unwrap();

    let ctx = Rc::new(grn::Context::new().unwrap());
    let mut db = grn::Database::create(ctx.clone(), path).unwrap();
    db.close();

    db = grn::Database::open(ctx.clone(), path).unwrap();
    assert_eq!(path, db.path().unwrap());
    assert!(fs::metadata(path).unwrap().is_file());
}

#[test]
fn test_remove_database() {
    grn::Groonga::new().unwrap();

    let work_dir = tempdir::TempDir::new(env::temp_dir().to_str().unwrap()).unwrap();
    let mut buf = work_dir.into_path();
    buf.push("test.db");
    let path = buf.to_str().unwrap();

    let ctx = Rc::new(grn::Context::new().unwrap());
    let mut db = grn::Database::create(ctx.clone(), path).unwrap();
    let rv = db.remove();
    assert!(rv.is_ok(), "failed to remove database");
    assert!(fs::metadata(path).is_err(), "db file should not exist")
}
