use std::collections::VecDeque;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkGroup, Criterion};
use criterion::measurement::WallTime;
use eta_graph::handles::types::{Ci, Weight};

fn tree_graph_eta_benchmark(children_count: usize, elements_to_generate: usize, c: &mut BenchmarkGroup<WallTime>){
    use eta_graph::traits::EdgeStore;
    use eta_graph::weighted_graph::WeightedGraph;
    use eta_algorithms::data_structs::queue::Queue;
    use eta_graph::handles::types::VHandle;
    use eta_graph::algorithms::path_finding::dijkstra;


    let mut graph = WeightedGraph::new();

    let root = graph.graph.create((), children_count as Ci);
    let mut to_expand = Queue::<VHandle>::new_pow2_sized(elements_to_generate as usize);
    let mut generated_elements = 1;
    let mut last_element = 0;

    to_expand.push(root);
    while generated_elements < elements_to_generate {
        let current = to_expand.dequeue().unwrap();
        for i in 0 .. children_count {
            let new_vertex = graph.create_and_connect_weighted(current, (), i as Weight, children_count as Ci);
            generated_elements += 1;
            to_expand.push(new_vertex);
            last_element = new_vertex;
        }

    }
    c.bench_function("tree_graph_eta_benchmark", |b| {
        b.iter(|| {
            black_box(dijkstra(&mut graph.graph.edge_storage, root, last_element, generated_elements as usize))
        })
    });

}

fn tree_graph_petgraph_benchmark(children_count: usize, elements_to_generate: usize, c: &mut BenchmarkGroup<WallTime>){
    use petgraph::Graph;
    use petgraph::algo::dijkstra;

    let mut graph = Graph::<(), i32>::new();
    let root = graph.add_node(());
    let mut to_expand = VecDeque::with_capacity(elements_to_generate);
    let mut generated_elements = 1;
    let mut last_element = root;

    to_expand.push_back(root);
    while generated_elements < elements_to_generate {
        let current = to_expand.pop_front().unwrap();
        for i in 0..children_count {
            let new_vertex = graph.add_node(());
            graph.add_edge(current, new_vertex, i as i32);
            generated_elements += 1;
            to_expand.push_back(new_vertex);
            last_element = new_vertex;
        }
    }

    c.bench_function("tree_graph_petgraph_benchmark", |b| {
        b.iter(|| {
            let _result = dijkstra(&graph, root, Some(last_element), |e| *e.weight());

        })
    });
}

fn petgraph_vs_eta_graph(c: &mut Criterion) {
    let children_count = 10;
    let elements_to_generate = 1000000;
    let mut group = c.benchmark_group("vs_pet_dijkstra");
    tree_graph_eta_benchmark(children_count, elements_to_generate, &mut group);
    tree_graph_petgraph_benchmark(children_count, elements_to_generate, &mut group);
}

criterion_group!(vs_pet_dijkstra, petgraph_vs_eta_graph);
criterion_main!(vs_pet_dijkstra);