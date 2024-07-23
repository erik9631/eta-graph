use crate::handles::{pack, set_vid, set_wgt, vid, wgt};
use crate::handles::types::{Vid, Weight};

#[test]
pub fn handle_test_vert_id(){
    let handle = pack(1, 2);
    assert_eq!(vid(handle), 1);
    assert_eq!(wgt(handle), 2);

    let handle = pack(7, 2);
    assert_eq!(vid(handle), 7);
    assert_eq!(wgt(handle), 2);

    let handle = pack(Vid::MAX, 2);
    assert_eq!(vid(handle), Vid::MAX);
    assert_eq!(wgt(handle), 2);
}
#[test]
pub fn handle_test_weight(){
    let handle = pack(1, 7);
    assert_eq!(vid(handle), 1);
    assert_eq!(wgt(handle), 7);

    let handle = pack(7, Weight::MAX);
    assert_eq!(vid(handle), 7);
    assert_eq!(wgt(handle), Weight::MAX);
}

#[test]
pub fn handle_test_weight_negative(){
    let handle = pack(1, -7);
    assert_eq!(vid(handle), 1);
    assert_eq!(wgt(handle), -7);

    let handle = pack(7, -Weight::MAX);
    assert_eq!(vid(handle), 7);
    assert_eq!(wgt(handle), -Weight::MAX);
}

#[test]
pub fn handle_test_weight_combined(){
    let handle = pack(1, 7);
    assert_eq!(vid(handle), 1);
    assert_eq!(wgt(handle), 7);

    let handle = pack(Vid::MAX, Weight::MAX);
    assert_eq!(vid(handle), Vid::MAX);
    assert_eq!(wgt(handle), Weight::MAX);
}
#[test]
pub fn set_vert_id_test(){
    let mut handle = pack(1, 7);
    handle = set_vid(handle, 8);
    assert_eq!(vid(handle), 8);
    assert_eq!(wgt(handle), 7);

    handle = set_vid(handle, Vid::MAX);
    assert_eq!(vid(handle), Vid::MAX);
    assert_eq!(wgt(handle), 7);

    handle = set_vid(handle, 8);
    assert_eq!(vid(handle), 8);
    assert_eq!(wgt(handle), 7);

    handle = set_vid(handle, 15);
    assert_eq!(vid(handle), 15);
    assert_eq!(wgt(handle), 7);

    handle = set_vid(handle, 127);
    assert_eq!(vid(handle), 127);
    assert_eq!(wgt(handle), 7);

    handle = set_vid(handle, 1);
    assert_eq!(vid(handle), 1);
    assert_eq!(wgt(handle), 7);
}

#[test]
pub fn set_weight_test(){
    let mut handle = pack(1, 7);
    handle = set_wgt(handle, 8);
    assert_eq!(vid(handle), 1);
    assert_eq!(wgt(handle), 8);

    handle = set_wgt(handle, Weight::MAX);
    assert_eq!(vid(handle), 1);
    assert_eq!(wgt(handle), Weight::MAX);

    handle = set_wgt(handle, 8);
    assert_eq!(vid(handle), 1);
    assert_eq!(wgt(handle), 8);

    handle = set_wgt(handle, -15);
    assert_eq!(vid(handle), 1);
    assert_eq!(wgt(handle), -15);

    handle = set_wgt(handle, -Weight::MAX);
    assert_eq!(vid(handle), 1);
    assert_eq!(wgt(handle), -Weight::MAX);

    handle = set_wgt(handle, 1);
    assert_eq!(vid(handle), 1);
    assert_eq!(wgt(handle), 1);
}

#[test]
pub fn combined_set_test(){
    let mut handle = pack(1, 1);
    handle = set_vid(handle, 8);
    handle = set_wgt(handle, 8);

    assert_eq!(vid(handle), 8);
    assert_eq!(wgt(handle), 8);

    handle = set_vid(handle, Vid::MAX);
    handle = set_wgt(handle, Weight::MAX);

    assert_eq!(vid(handle), Vid::MAX);
    assert_eq!(wgt(handle), Weight::MAX);

    handle = set_vid(handle, 1231);
    handle = set_wgt(handle, 8997);

    assert_eq!(vid(handle), 1231);
    assert_eq!(wgt(handle), 8997);

    handle = set_vid(handle, Vid::MAX);
    handle = set_wgt(handle, -Weight::MAX);

    assert_eq!(vid(handle), Vid::MAX);
    assert_eq!(wgt(handle), -Weight::MAX);
}