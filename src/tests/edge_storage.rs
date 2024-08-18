use std::collections::HashMap;
use crate::edge_storage::EdgeStorage;
use crate::handles::{vh, pack};
use crate::handles::types::{VHandle};
use crate::traits::{EdgeStore, EdgeConnect, WeightedEdgeConnect};

#[test]
fn edge_storage_create_test(){
    let mut edge_storage = EdgeStorage::new();
    let a= edge_storage.create_vertex_entry(3);
    assert_eq!(edge_storage.edges_len(a), 0);
    assert_eq!(edge_storage.edges_capacity(a), 3);
}

#[test]
fn edge_storage_add_edge_test() {
    let mut edge_storage = EdgeStorage::new();
    let a = edge_storage.create_vertex_entry(3);
    let b = edge_storage.create_vertex_entry(3);
    let c = edge_storage.create_vertex_entry(3);
    assert_eq!(edge_storage.edges.capacity(), 9);
    edge_storage.connect_edges(a, &[1, 2, 3]);
    edge_storage.connect_edges(b, &[3, 2, 1]);
    edge_storage.connect_edges(c, &[2, 3, 1]);
    assert_eq!(edge_storage.edges_len(a), 3);
    assert_eq!(edge_storage.edges_len(b), 3);
    assert_eq!(edge_storage.edges_len(c), 3);

    let snap: Vec<VHandle> = vec![1, 2, 3];
    let index = edge_storage.edges_index(a);
    for i in 0..3 {
        assert_eq!(vh(edge_storage[index + i]), snap[i]);
    }

    let snap:Vec<VHandle>  = vec![3, 2, 1];
    let index = edge_storage.edges_index(b);
    for i in 0..3 {
        assert_eq!(vh(edge_storage[index + i]), snap[i]);
    }

    let snap:Vec<VHandle>  = vec![2, 3, 1];
    let index = edge_storage.edges_index(c);
    for i in 0..3 {
        assert_eq!(vh(edge_storage[index + i]), snap[i]);
    }
}

#[test]
fn edge_storage_connect_test() {
    let mut edge_storage = EdgeStorage::new();
    let root = edge_storage.create_vertex_entry(3);
    let b = edge_storage.create_vertex_entry(3);
    let c = edge_storage.create_vertex_entry(3);
    let d = edge_storage.create_vertex_entry(3);

    edge_storage.connect(root, b);
    edge_storage.connect(root, c);
    edge_storage.connect(root, d);

    assert_eq!(edge_storage.edges_len(root), 3);
    assert_eq!(edge_storage.edges_len(b), 0);
    assert_eq!(edge_storage.edges_len(c), 0);
    assert_eq!(edge_storage.edges_len(d), 0);

    edge_storage.connect(b, c);
    edge_storage.connect(c, d);

    assert_eq!(edge_storage.edges_len(root), 3);
    assert_eq!(edge_storage.edges_len(b), 1);
    assert_eq!(edge_storage.edges_len(c), 1);
    assert_eq!(edge_storage.edges_len(d), 0);

    let index = edge_storage.edges_index(root);
    assert_eq!(vh(edge_storage[index]), b);
    assert_eq!(vh(edge_storage[index + 1]), c);
    assert_eq!(vh(edge_storage[index + 2]), d);

    let index = edge_storage.edges_index(b);
    assert_eq!(vh(edge_storage[index]), c);

    let index = edge_storage.edges_index(c);
    assert_eq!(vh(edge_storage[index]), d);
}

#[test]
fn edge_storage_connect_test_weighted() {
    let mut edge_storage = EdgeStorage::new();
    let root = edge_storage.create_vertex_entry(3);
    let b = edge_storage.create_vertex_entry(3);
    let c = edge_storage.create_vertex_entry(3);
    let d = edge_storage.create_vertex_entry(3);

    edge_storage.connect_weighted(root, b, 1);
    edge_storage.connect_weighted(root, c, 2);
    edge_storage.connect_weighted(root, d, 3);

    assert_eq!(edge_storage.edges_len(root), 3);
    assert_eq!(edge_storage.edges_len(b), 0);
    assert_eq!(edge_storage.edges_len(c), 0);
    assert_eq!(edge_storage.edges_len(d), 0);

    edge_storage.connect_weighted(b, c, 5);
    edge_storage.connect_weighted(c, d, 6);

    assert_eq!(edge_storage.edges_len(root), 3);
    assert_eq!(edge_storage.edges_len(b), 1);
    assert_eq!(edge_storage.edges_len(c), 1);

    let index = edge_storage.edges_index(root);
    assert_eq!(edge_storage[index], pack(b, 1));
    assert_eq!(edge_storage[index + 1], pack(c, 2));
    assert_eq!(edge_storage[index + 2], pack(d, 3));

    let index = edge_storage.edges_index(b);
    assert_eq!(edge_storage[index], pack(c, 5));

    let index = edge_storage.edges_index(c);
    assert_eq!(edge_storage[index], pack(d, 6));
}

#[test]
fn edge_storage_disconnect_test() {
    let mut edge_storage = EdgeStorage::new();
    let a = edge_storage.create_vertex_entry(3);
    let b = edge_storage.create_vertex_entry(3);
    let c = edge_storage.create_vertex_entry(3);
    assert_eq!(edge_storage.edges.capacity(), 9);
    edge_storage.connect_edges(a, &[1, 2, 3]);
    edge_storage.connect_edges(b, &[3, 2, 1]);
    edge_storage.connect_edges(c, &[2, 3, 1]);
    assert_eq!(edge_storage.edges_len(a), 3);
    assert_eq!(edge_storage.edges_len(b), 3);
    assert_eq!(edge_storage.edges_len(c), 3);

    edge_storage.disconnect(a, 1);
    edge_storage.disconnect(b, 2);
    edge_storage.disconnect(c, 3);

    assert_eq!(edge_storage.edges_len(a), 2);
    assert_eq!(edge_storage.edges_len(b), 2);
    assert_eq!(edge_storage.edges_len(c), 2);

    let mut snap: HashMap::<VHandle, ()> = HashMap::from([(3, ()), (2, ())]);
    let index = edge_storage.edges_index(a);
    for i in 0..2 {
        let val = vh(edge_storage[index + i]);
        assert!(snap.remove(&val).is_some());
    }
    assert!(snap.is_empty());


    let mut snap2: HashMap::<VHandle, ()> = HashMap::from([(1, ()), (3, ())]);
    let index2 = edge_storage.edges_index(b);
    for i in 0..2 {
        let val = vh(edge_storage[index2 + i]);
        assert!(snap2.remove(&val).is_some());
    }
    assert!(snap2.is_empty());

    let mut snap3: HashMap::<VHandle, ()> = HashMap::from([(1, ()), (2, ())]);
    let index3 = edge_storage.edges_index(c);
    for i in 0..2 {
        let val = vh(edge_storage[index3 + i]);
        assert!(snap3.remove(&val).is_some());
    }
    assert!(snap3.is_empty());
}
#[test]
fn entry_as_slice_test(){
    let mut edge_storage = EdgeStorage::new();
    let a= edge_storage.create_vertex_entry(3);
    edge_storage.create_vertex_entry(0);
    edge_storage.create_vertex_entry(0);
    edge_storage.connect_edges(a, &[1,2,3]);

    for (idx, edge) in edge_storage.edges_as_slice(a).iter().enumerate(){
        assert_eq!(*edge as usize, idx + 1);
    }
}

#[test]
fn entry_as_slice_test_empty(){
    let mut edge_storage = EdgeStorage::new();
    let a= edge_storage.create_vertex_entry(0);

    for _ in edge_storage.edges_as_slice(a).iter(){
        assert!(false);
    }
}

#[test]
fn entry_as_slice_mut_test() {
    let mut edge_storage = EdgeStorage::new();
    let a = edge_storage.create_vertex_entry(3);
    let b = edge_storage.create_vertex_entry(3);
    let c = edge_storage.create_vertex_entry(3);
    edge_storage.connect_edges(a, &[1, 2, 3]);
    edge_storage.connect_edges(b, &[4, 5, 6]);
    edge_storage.connect_edges(c, &[7, 8, 9]);

    for edge in edge_storage.edges_as_mut_slice(a).iter_mut() {
        *edge = 0;
    }

    for edge in edge_storage.edges_as_slice(a).iter(){
        assert_eq!(*edge as usize, 0);
    }
}



#[test]
fn edge_storage_iter_test(){
    let mut edge_storage = EdgeStorage::new();
    let a= edge_storage.create_vertex_entry(3);
    let b= edge_storage.create_vertex_entry(3);
    let c= edge_storage.create_vertex_entry(3);
    edge_storage.connect_edges(a, &[1,2,3]);
    edge_storage.connect_edges(b, &[4,5,6]);
    edge_storage.connect_edges(c, &[7,8,9]);

    for (index, edge) in edge_storage.iter().enumerate(){
        assert_eq!(*edge as usize, index+1);
    }
}
#[test]
fn edge_storage_iter_mut_test(){
    let mut edge_storage = EdgeStorage::new();
    let a= edge_storage.create_vertex_entry(3);
    let b= edge_storage.create_vertex_entry(3);
    let c= edge_storage.create_vertex_entry(3);
    edge_storage.connect_edges(a, &[1,2,3]);
    edge_storage.connect_edges(b, &[4,5,6]);
    edge_storage.connect_edges(c, &[7,8,9]);

    for edge in edge_storage.iter_mut(){
        *edge = 100;
    }

    for edge in edge_storage.iter(){
        assert_eq!(*edge as usize, 100);
    }
}

#[test]
fn edge_storage_iter_test_empty(){
    let mut edge_storage = EdgeStorage::new();
    edge_storage.create_vertex_entry(10);
    edge_storage.create_vertex_entry(10);
    edge_storage.create_vertex_entry(10);

    for _ in edge_storage.iter(){
        assert!(false);
    }
}

#[test]
#[should_panic]
fn invalid_handle_test(){
    let mut edge_storage = EdgeStorage::new();
    edge_storage.create_vertex_entry(3);
    edge_storage.edges_index(5);
}

#[test]
#[should_panic]
fn invalid_handle_test_iter(){
    let mut edge_storage = EdgeStorage::new();
    edge_storage.create_vertex_entry(3);
    let _ = edge_storage.edges_iter(5);
}

#[test]
#[should_panic]
fn invalid_handle_test_iter_mut(){
    let mut edge_storage = EdgeStorage::new();
    edge_storage.create_vertex_entry(3);
    let _ = edge_storage.edges_iter_mut(5);
}

#[test]
#[should_panic]
fn invalid_handle_test_vertex_as_slice(){
    let mut edge_storage = EdgeStorage::new();
    edge_storage.create_vertex_entry(3);
    let _ = edge_storage.edges_as_slice(5);
}

#[test]
#[should_panic]
fn invalid_handle_test_vertex_as_slice_mut(){
    let mut edge_storage = EdgeStorage::new();
    edge_storage.create_vertex_entry(3);
    let _ = edge_storage.edges_as_mut_slice(5);
}

#[test]
#[should_panic]
fn invalid_handle_test_vertex_as_ptr(){
    let mut edge_storage = EdgeStorage::new();
    edge_storage.create_vertex_entry(3);
    let _ = edge_storage.edges_as_ptr(5);
}