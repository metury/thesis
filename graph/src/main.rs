use petgraph::graph::UnGraph;
use petgraph::visit::EdgeRef;
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
    println!("Minimize");
    for e in g.edge_indices() {
        if let Some((from, to)) = g.edge_endpoints(e) {
            print!("x_{0}_{1} + x_{1}_{0} + ", from.index(), to.index());
        }
    }
    println!("0");
    println!("Subject to");

    // Defining the cut x.
    for e in g.edge_indices() {
        if let Some((from, to)) = g.edge_endpoints(e) {
            println!("x_{0}_{1} - f_{0} + f_{1} >= 0", from.index(), to.index());
            println!("x_{0}_{1} - f_{1} + f_{0} >= 0", from.index(), to.index());
            println!("x_{1}_{0} - f_{0} + f_{1} >= 0", from.index(), to.index());
            println!("x_{1}_{0} - f_{1} + f_{0} >= 0", from.index(), to.index());
        }
    }

    // The sum of the outgoing flow from s is k-1.
    for e in g.edges(inst.s.into()) {
        let from = e.source().index();
        let to = e.target().index();
        if from == inst.s as usize {
            print!("f_{}_{} + ", inst.s, to);
        } else {
            print!("f_{}_{} + ", inst.s, from);
        }
    }
    println!("1 - {} = 0", inst.k);

    // f_s = 1
    println!("f_{} = 1", inst.s);

    // The flow is correct.
    for v in g.node_indices() {
        if v != inst.s.into() {
            for e in g.edges(v) {
                let (from, to) = (e.source(), e.target());
                if from == v {
                    print!("f_{0}_{1} - f_{1}_{0} + ", to.index(), v.index());
                } else {
                    print!("f_{0}_{1} - f_{1}_{0} + ", from.index(), v.index());
                }
            }
        }
        println!("f_{} = 0", v.index());
    }

    // Choose k vertices.
    for v in g.node_indices() {
        print!("f_{} + ", v.index());
    }
    println!("0 - {}", inst.k);

    // Force the absorption.
    for v in g.node_indices() {
        if v != inst.s.into() {
            print!("0 ");
            for e in g.edges(v) {
                let (from, to) = (e.source(), e.target());
                if from == v {
                    print!("- 1/{} f_{}_{}", inst.k, to.index(), v.index())
                } else {
                    print!("- 1/{} f_{}_{}", inst.k, from.index(), v.index())
                }
            }
            println!("+ f_{} >= 0", v.index());
        }
    }

    println!("Bounds");

    for v in g.node_indices() {
        println!("0 <= f_{} <= 1", v.index());
    }

    for e in g.edge_indices() {
        if let Some((from, to)) = g.edge_endpoints(e) {
            println!("0 <= f_{}_{}", from.index(), to.index());
            println!("0 <= f_{}_{}", to.index(), from.index());
            println!("0 <= x_{}_{} <= 1", from.index(), to.index());
            println!("0 <= x_{}_{} <= 1", to.index(), from.index());
        }
    }

    if ilp {
        println!("Generals");
        for v in g.node_indices() {
            print!("f_{} ", v.index());
        }
        for e in g.edge_indices() {
            if let Some((from, to)) = g.edge_endpoints(e) {
                print!("x_{0}_{1} x_{1}_{0} ", from.index(), to.index());
            }
        }
        println!("");
    }

    println!("End");
}

/// Main function of the program.
fn main() {
    let g = read_file("test");
    let inst = Instance {g: g, k: 4, s: 1};
    create_lp(false, &inst);
}
