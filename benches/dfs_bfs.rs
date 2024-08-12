use std::time::Instant;
use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use eta_graph::algorithms::general::ControlFlow::Resume;
use eta_graph::algorithms::general::{bfs, dfs};
use eta_graph::graph;
use eta_graph::handles::{vh, vh_pack};
use eta_graph::traits::GraphOperate;
fn dfs_bench(c: &mut criterion::Criterion) {
    // prepare data
    let data_size = 2000;
    let mut graph = graph::Graph::new();
    let root = graph.create(0, data_size);
    let mut number_of_nodes = 1;

    for i in 0..data_size {
        let child = graph.create_and_connect(root, i+1, data_size);
        number_of_nodes += 1;
        for j in 0..data_size {
            graph.create_and_connect_0(child, j*data_size);
            number_of_nodes += 1;
        }
    }

    c.bench_function("dfs", |b| b.iter(|| {
        dfs(&mut graph.edge_storage, vh_pack(root), number_of_nodes, |vertex| {
            graph.vertices[vh(*vertex)] = 0;
            Resume
        }, |vertex| {});
    }));
}

pub fn bfs_bench(c: &mut criterion::Criterion){
    // prepare data
    let data_size = 2000;
    let mut graph = graph::Graph::new();
    let root = graph.create(0, data_size);
    let mut number_of_nodes = 1;
    for i in 0..data_size {
        let child = graph.create_and_connect(root, i+1, data_size);
        number_of_nodes += 1;
        for j in 0..data_size {
            graph.create_and_connect_0(child, j*data_size);
            number_of_nodes += 1;
        }
    }

    c.bench_function("bfs", |b| b.iter(|| {
        bfs(&mut graph.edge_storage, vh_pack(root), number_of_nodes, |vertex, layer| {
            graph.vertices[vh(*vertex)] = 0;
            Resume
        });
    }));
}

criterion_group!{name = general;
    config = Criterion::default().sample_size(100);
    targets = dfs_bench, bfs_bench
}
criterion_main!(general);