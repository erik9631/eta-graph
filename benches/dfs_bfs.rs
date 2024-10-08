use criterion::{Criterion, criterion_group, criterion_main, black_box};
use eta_graph::algorithms::dfs_bfs::ControlFlow::Resume;
use eta_graph::algorithms::dfs_bfs::{bfs, dfs};
use eta_graph::graph;
use eta_graph::handles::{vh_pack};
fn dfs_bench(c: &mut Criterion) {
    // prepare data
    let data_size = 4000;
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
        let mut sum = 0;
        dfs(&mut graph.edge_storage, vh_pack(root), number_of_nodes, |vertex| {
            sum += 1;
            Resume
        }, |vertex| {});
        black_box(sum);
    }));
}

pub fn bfs_bench(c: &mut Criterion){
    // prepare data
    let data_size = 4000;
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
        let mut sum = 0;
        bfs(&mut graph.edge_storage, vh_pack(root), number_of_nodes, |vertex, layer| {
            sum += 1;
            Resume
        });
        black_box(sum);
    }));
}

criterion_group!{name = general;
    config = Criterion::default().sample_size(100);
    targets = dfs_bench, bfs_bench
}
criterion_main!(general);