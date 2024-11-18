use petgraph::graph::{UnGraph};
use regex::Regex;
use std::fs;

/// Shorter type for graph.
type Graph = UnGraph<(),()>;

/// Strucutre for minimum k connected cut.
struct Instance {
    g: Graph,
    k: u32,
    s: u32,
}

/// Read graph from file which is provided via a filpeath.
/// Considered format of a graph is by giving out edges in a from [idfrom;idto].
fn read_file(filepath: &str) -> Graph {
    let contents = fs::read_to_string(filepath).unwrap();

    let edge_regex = Regex::new(r"\[\s*([0-9]*)\s*;\s*([0-9]*)\s*\]").unwrap();
    let mut raw_edges: Vec<(u32,u32)> = vec![];

    for (_, [id_from, id_to]) in edge_regex.captures_iter(&contents).map(|c| c.extract()) {
        raw_edges.push((id_from.parse().unwrap(), id_to.parse().unwrap()));
    }

    UnGraph::<(), ()>::from_edges(&raw_edges)
}

/// Create linear program.
fn create_lp(ilp: bool, inst: &Instance) {
    let g = &inst.g;
    for e in g.edge_indices() {
        if let Some((from, to)) = g.edge_endpoints(e) {
            println!("From {:?} to {:?}", from, to);
        }
    }
    for v in g.node_indices() {
        println!("Adjecent edges to vertex {:?}:", v);
        for e in g.edges(v){
            println!("\t{:?}", e);
        }
    }
}

/// Main function of the program.
fn main() {
    println!("Hello, world!");
    let g = read_file("test");
    let inst = Instance {g: g, k: 4, s: 0};
    create_lp(false, &inst);
}
