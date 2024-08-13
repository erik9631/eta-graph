use crate::edge_storage::EdgeStorage;
use crate::traits::{EdgeStore, GraphOperate};

#[test]
fn edge_storage_iter_test(){
    let mut edge_storage = EdgeStorage::new();
    let a= edge_storage.create_edges_entry(3);
    let b= edge_storage.create_edges_entry(3);
    let c= edge_storage.create_edges_entry(3);
    edge_storage.add_edges(a, &[1,2,3]);
    edge_storage.add_edges(b, &[4,5,6]);
    edge_storage.add_edges(c, &[7,8,9]);

    for (index, edge) in edge_storage.iter().enumerate(){
        assert_eq!(*edge as usize, index+1);
    }
}
#[test]
fn edge_storage_iter_mut_test(){
    let mut edge_storage = EdgeStorage::new();
    let a= edge_storage.create_edges_entry(3);
    let b= edge_storage.create_edges_entry(3);
    let c= edge_storage.create_edges_entry(3);
    edge_storage.add_edges(a, &[1,2,3]);
    edge_storage.add_edges(b, &[4,5,6]);
    edge_storage.add_edges(c, &[7,8,9]);

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
    edge_storage.create_edges_entry(10);
    edge_storage.create_edges_entry(10);
    edge_storage.create_edges_entry(10);

    for edge in edge_storage.iter(){
        assert!(false);
    }
}