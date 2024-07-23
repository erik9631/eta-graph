use crate::handles::{create_handle, set_vert_id, set_weight, vert_id, weight};
use crate::handles::types::{VertId, Weight};

#[test]
pub fn handle_test_vert_id(){
    let handle = create_handle(1, 2);
    assert_eq!(vert_id(handle), 1);
    assert_eq!(weight(handle), 2);

    let handle = create_handle(7, 2);
    assert_eq!(vert_id(handle), 7);
    assert_eq!(weight(handle), 2);

    let handle = create_handle(VertId::MAX, 2);
    assert_eq!(vert_id(handle), VertId::MAX);
    assert_eq!(weight(handle), 2);
}
#[test]
pub fn handle_test_weight(){
    let handle = create_handle(1, 7);
    assert_eq!(vert_id(handle), 1);
    assert_eq!(weight(handle), 7);

    let handle = create_handle(7, Weight::MAX);
    assert_eq!(vert_id(handle), 7);
    assert_eq!(weight(handle), Weight::MAX);
}

#[test]
pub fn handle_test_weight_negative(){
    let handle = create_handle(1, -7);
    assert_eq!(vert_id(handle), 1);
    assert_eq!(weight(handle), -7);

    let handle = create_handle(7, -Weight::MAX);
    assert_eq!(vert_id(handle), 7);
    assert_eq!(weight(handle), -Weight::MAX);
}

#[test]
pub fn handle_test_weight_combined(){
    let handle = create_handle(1, 7);
    assert_eq!(vert_id(handle), 1);
    assert_eq!(weight(handle), 7);

    let handle = create_handle(VertId::MAX, Weight::MAX);
    assert_eq!(vert_id(handle), VertId::MAX);
    assert_eq!(weight(handle), Weight::MAX);
}
#[test]
pub fn set_vert_id_test(){
    let mut handle = create_handle(1, 7);
    handle = set_vert_id(handle, 8);
    assert_eq!(vert_id(handle), 8);
    assert_eq!(weight(handle), 7);

    handle = set_vert_id(handle, VertId::MAX);
    assert_eq!(vert_id(handle), VertId::MAX);
    assert_eq!(weight(handle), 7);

    handle = set_vert_id(handle, 8);
    assert_eq!(vert_id(handle), 8);
    assert_eq!(weight(handle), 7);

    handle = set_vert_id(handle,15);
    assert_eq!(vert_id(handle), 15);
    assert_eq!(weight(handle), 7);

    handle = set_vert_id(handle, 127);
    assert_eq!(vert_id(handle), 127);
    assert_eq!(weight(handle), 7);

    handle = set_vert_id(handle, 1);
    assert_eq!(vert_id(handle), 1);
    assert_eq!(weight(handle), 7);
}

#[test]
pub fn set_weight_test(){
    let mut handle = create_handle(1, 7);
    handle = set_weight(handle, 8);
    assert_eq!(vert_id(handle), 1);
    assert_eq!(weight(handle), 8);

    handle = set_weight(handle, Weight::MAX);
    assert_eq!(vert_id(handle), 1);
    assert_eq!(weight(handle), Weight::MAX);

    handle = set_weight(handle, 8);
    assert_eq!(vert_id(handle), 1);
    assert_eq!(weight(handle), 8);

    handle = set_weight(handle,-15);
    assert_eq!(vert_id(handle), 1);
    assert_eq!(weight(handle), -15);

    handle = set_weight(handle, -Weight::MAX);
    assert_eq!(vert_id(handle), 1);
    assert_eq!(weight(handle), -Weight::MAX);

    handle = set_weight(handle, 1);
    assert_eq!(vert_id(handle), 1);
    assert_eq!(weight(handle), 1);
}

#[test]
pub fn combined_set_test(){
    let mut handle = create_handle(1, 1);
    handle = set_vert_id(handle, 8);
    handle = set_weight(handle, 8);

    assert_eq!(vert_id(handle), 8);
    assert_eq!(weight(handle), 8);

    handle = set_vert_id(handle, VertId::MAX);
    handle = set_weight(handle, Weight::MAX);

    assert_eq!(vert_id(handle), VertId::MAX);
    assert_eq!(weight(handle), Weight::MAX);

    handle = set_vert_id(handle, 1231);
    handle = set_weight(handle, 8997);

    assert_eq!(vert_id(handle), 1231);
    assert_eq!(weight(handle), 8997);

    handle = set_vert_id(handle, VertId::MAX);
    handle = set_weight(handle, -Weight::MAX);

    assert_eq!(vert_id(handle), VertId::MAX);
    assert_eq!(weight(handle), -Weight::MAX);
}