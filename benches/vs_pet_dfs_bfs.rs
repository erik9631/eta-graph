use std::hint::black_box;
use criterion::{criterion_group, criterion_main, BenchmarkGroup, Criterion};
use criterion::measurement::WallTime;
use eta_graph::algorithms::general::ControlFlow::Resume;
use eta_graph::handles::{eh, eh_pack};
use eta_graph::handles::types::Ci;

pub fn bfs_bench_eta(data_size: usize, c: &mut BenchmarkGroup<WallTime>){
    use eta_graph::graph;
    use eta_graph::algorithms::general::bfs;

    let mut graph = graph::Graph::new();
    let root = graph.create(0, data_size as Ci);
    let mut number_of_nodes = 1;
    for i in 0..data_size {
        let child = graph.create_and_connect(root, i+1, data_size as Ci);
        number_of_nodes += 1;
        for j in 0..data_size {
            graph.create_and_connect_0(child, j*data_size);
            number_of_nodes += 1;
        }
    }
    let mut sum = 0;

    c.bench_function("bfs_eta", |b| b.iter(|| {
        bfs(&mut graph.edge_storage, eh_pack(root), number_of_nodes, |vertex, layer| {
            sum += 1;
            Resume
        });
        black_box(sum);
    }));
}

fn bfs_bench_pet(data_size: usize, c: &mut BenchmarkGroup<WallTime>) {
    use petgraph::Graph;
    use petgraph::prelude::Bfs;
    // Prepare data
    let mut graph = Graph::<i32, ()>::new();
    let root = graph.add_node(0);
    let mut number_of_nodes = 1;

    for i in 0..data_size {
        let child = graph.add_node((i + 1) as i32);
        graph.add_edge(root, child, ());
        number_of_nodes += 1;

        for j in 0..data_size {
            let grandchild = graph.add_node((j * data_size) as i32);
            graph.add_edge(child, grandchild, ());
            number_of_nodes += 1;
        }
    }

    c.bench_function("bfs_pet", |b| {
        let mut sum = 0;
        b.iter(|| {
            let mut bfs = Bfs::new(&graph, root);
            while let Some(nx) = bfs.next(&graph) {
                sum += 1;
            }
            black_box(sum);
        })
    });
}

fn vs_pet_bfs_bench(c: &mut Criterion) {
    // Prepare data
    let data_size = 4000;
    let mut group = c.benchmark_group("vs_pet_bfs");
    bfs_bench_eta(data_size, &mut group);
    bfs_bench_pet(data_size, &mut group);
}

criterion_group!(vs_pet_dfs, vs_pet_bfs_bench);
criterion_main!(vs_pet_dfs);