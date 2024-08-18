use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use eta_graph::graph;
use eta_graph::traits::EdgeConnect;

fn graph_disconnect_bench(c: &mut criterion::Criterion) {
    c.bench_function("graph_disconnect", |b| b.iter_batched(|| {
        // prepare data
        let data_size = 64000;
        let mut graph = graph::Graph::new();
        let root = graph.create(0, data_size);
        let mut handles = Vec::with_capacity(data_size as usize);
        for i in 0..data_size {
            handles.push(graph.create_and_connect_0(root, i + 1));
        }
        (graph, handles, root)
    }, |handles| {
        let (mut graph, mut handles, root) = handles;
        while let Some(handle) = handles.pop() {
            
            graph.edge_storage.disconnect(root, handle);
        }
    }, BatchSize::LargeInput));
}

criterion_group!{
    name = general;
    config = Criterion::default().sample_size(50);
    targets = graph_disconnect_bench
}

criterion_main!(general);