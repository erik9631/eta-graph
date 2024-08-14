use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use eta_graph::handles::types::Weight;

fn tree_graph_benchmark(c: &mut Criterion){
    use eta_graph::traits::EdgeStore;
    use eta_graph::weighted_graph::WeightedGraph;
    use eta_algorithms::data_structs::queue::Queue;
    use eta_graph::handles::types::VHandle;
    use eta_graph::algorithms::path_finding::dijkstra;
    let children_count = 10;
    let elements_to_generate = 1000000;


    let mut graph = WeightedGraph::new();

    let root = graph.graph.create((), children_count);
    let mut to_expand = Queue::<VHandle>::new_pow2_sized(elements_to_generate as usize);
    let mut generated_elements = 1;
    let mut last_element = 0;

    to_expand.push(root);
    while generated_elements < elements_to_generate {
        let current = to_expand.dequeue().unwrap();
        for i in 0 .. children_count {
            let new_vertex = graph.create_and_connect_weighted(current, (), i as Weight, children_count);
            generated_elements += 1;
            to_expand.push(new_vertex);
            last_element = new_vertex;
        }

    }
    c.bench_function("tree_graph_benchmark", |b| {
        b.iter(|| {
            black_box(dijkstra(&mut graph.graph.edge_storage, root, last_element, generated_elements as usize))
        })
    });

}

criterion_group!(dijkstra, tree_graph_benchmark);
criterion_main!(dijkstra);