extern crate chrono;
extern crate musicbrainz_rs;

use musicbrainz_rs::entity::work::*;
use musicbrainz_rs::prelude::*;
use std::{thread, time};

#[test]
fn should_browse_work_by_artist() {
    let work_by_svinkels = Work::browse()
        .by_artist("770d490e-c89b-4775-8508-aca7c75142cd")
        .execute();

    assert!(work_by_svinkels.is_ok());

    let work_by_svinkels = work_by_svinkels.unwrap();

    assert!(work_by_svinkels.count > 1);
    assert_eq!(work_by_svinkels.offset, 0);
    assert!(!work_by_svinkels.entities.is_empty());

    thread::sleep(time::Duration::from_secs(1));
}

// TODO: find non empty result
#[test]
fn should_browse_work_by_collection() {
    let work_in_collection = Work::browse()
        .by_collection("91565a03-bce8-47e9-ab70-e4d4e1684d7f")
        .execute();

    assert!(work_in_collection.is_ok());
    thread::sleep(time::Duration::from_secs(1));
}
