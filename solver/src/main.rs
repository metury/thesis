use petgraph::graph::{UnGraph, DiGraph};
use petgraph::visit::EdgeRef;
use petgraph::dot::{Dot, Config};
use clap::Parser;
use regex::Regex;
use std::fs;
use std::io::{self, Write};
use std::collections::{HashSet, HashMap};
use petgraph::algo::dijkstra;

/// Shorter type for graph.
type Graph = UnGraph<(),()>;

/// Strucutre for minimum k connected cut.
struct Instance {
    g: Graph,
    k: u32,
    s: u32,
}

/// These arguments are available. You must select a job and than either provide an input file to the graph or create some graph.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// What job to do: ilp, lp, apx.
    #[arg(short, long)]
    job: String,

    /// Input file.
    #[arg(short,long)]
    inputfile: String,

     /// Output file.
    #[arg(short,long)]
    outputfile: String,

    /// Solution file.
    #[arg(short,long, default_value_t = String::new())]
    solutionfile: String,
}

/// Read graph from file which is provided via a filpeath.
/// Considered format of a graph is by giving out edges in a from [idfrom;idto].
/// Also the file itself has source vertex and the number k.
fn read_file(filepath: &str) -> Instance {
    let contents = fs::read_to_string(filepath).unwrap();

    let edge_regex = Regex::new(r"\[\s*([0-9]*)\s*;\s*([0-9]*)\s*\]").unwrap();
    let s_rgx = Regex::new(r"s\s*=\s*([0-9]+)");
    let k_rgx = Regex::new(r"k\s*=\s*([0-9]+)");

    let mut raw_edges: Vec<(u32,u32)> = vec![];

    for (_, [id_from, id_to]) in edge_regex.captures_iter(&contents).map(|c| c.extract()) {
        raw_edges.push((id_from.parse().unwrap(), id_to.parse().unwrap()));
    }

    let g = UnGraph::<(), ()>::from_edges(&raw_edges);
    let s: u32 = (&s_rgx.expect("REASON").captures(&contents).unwrap()[1]).parse().unwrap();
    let k: u32 = (&k_rgx.expect("REASON").captures(&contents).unwrap()[1]).parse().unwrap();

    Instance {g, k, s}
}

/// Parse the solution to initialize new graph.
fn parse_solution(filepath: &str, g: &Graph) -> (DiGraph<f64, f64>, DiGraph<f64, f64>) {
    let mut cut_graph: DiGraph<f64, f64> = Default::default();
    let mut flow_graph: DiGraph<f64, f64> = Default::default();
    for _ in g.node_indices() {
        cut_graph.add_node(0f64);
        flow_graph.add_node(0f64);
    }

    for e in g.edge_indices() {
        if let Some((from, to)) = g.edge_endpoints(e) {
            cut_graph.add_edge(from, to, 0f64);
            flow_graph.add_edge(from, to, 0f64);
            cut_graph.add_edge(to, from, 0f64);
            flow_graph.add_edge(to, from, 0f64);
        }
    }
    let contents = fs::read_to_string(filepath).unwrap();

    let cut_regex = Regex::new(r"x_([0-9]+)_([0-9]+)\s([0-9\.e\-\+]+)").unwrap();

    for (_, [id_from, id_to, value]) in cut_regex.captures_iter(&contents).map(|c| c.extract()) {
        let from = id_from.parse::<u32>().unwrap();
        let to = id_to.parse::<u32>().unwrap();
        let val = value.parse::<f64>().unwrap();
        if let Some(edge) = cut_graph.find_edge(from.into(), to.into()) {
            cut_graph[edge] = val;
        }
    }

    let flow_edge_regex = Regex::new(r"f_([0-9]+)_([0-9]+)\s([0-9\.e\-\+]+)").unwrap();
    let flow_node_regex = Regex::new(r"f_([0-9]+)\s([0-9\.e\-\+]+)").unwrap();

    for (_, [id_from, id_to, value]) in flow_edge_regex.captures_iter(&contents).map(|c| c.extract()) {
        let from = id_from.parse::<u32>().unwrap();
        let to = id_to.parse::<u32>().unwrap();
        let val = value.parse::<f64>().unwrap();
        if let Some(edge) = flow_graph.find_edge(from.into(), to.into()) {
            flow_graph[edge] = val;
        }
    }

    for (_, [id, value]) in flow_node_regex.captures_iter(&contents).map(|c| c.extract()) {
        let v = id.parse::<u32>().unwrap();
        let val = value.parse::<f64>().unwrap();
        if let Some(weight) = flow_graph.node_weight_mut(v.into()) {
            *weight = val;
        }
    }

    (cut_graph, flow_graph)
}

/// Create linear program.
/// ilp is for integer linear program.
fn create_lp(ilp: bool, inst: &Instance, ofile:& String) -> io::Result<()> {
    let g = &inst.g;
    let mut first = true;

    let dist = dijkstra(g, inst.s.into(), None, |_| 1);

    let mut file = fs::File::create(ofile)?;

    writeln!(file, "Minimize")?;
    for e in g.edge_indices() {
        if !first {
            write!(file, " + ")?;
        }
        first = false;
        if let Some((from, to)) = g.edge_endpoints(e) {
            write!(file, "x_{0}_{1} + x_{1}_{0}", from.index(), to.index())?;
        }
    }
    writeln!(file, "")?;
    writeln!(file, "Subject to")?;

    // Defining the cut x.
    for e in g.edge_indices() {
        if let Some((from, to)) = g.edge_endpoints(e) {
            writeln!(file, "x_{0}_{1} - f_{0} + f_{1} >= 0", from.index(), to.index())?;
            writeln!(file, "x_{0}_{1} - f_{1} + f_{0} >= 0", from.index(), to.index())?;
            writeln!(file, "x_{1}_{0} - f_{0} + f_{1} >= 0", from.index(), to.index())?;
            writeln!(file, "x_{1}_{0} - f_{1} + f_{0} >= 0", from.index(), to.index())?;
        }
    }

    // The sum of the outgoing flow from s is k-1.
    first = true;
    for e in g.edges(inst.s.into()) {
        let from = e.source().index();
        let to = e.target().index();
        if !first {
            write!(file, " + ")?;
        }
        first = false;
        if from == inst.s as usize {
            write!(file, "f_{}_{}", inst.s, to)?;
        } else {
            write!(file, "f_{}_{}", inst.s, from)?;
        }
    }
    writeln!(file, " = {}", inst.k - 1)?;

    // f_s = 1
    writeln!(file, "f_{} = 1", inst.s)?;

    // The flow is correct.
    for v in g.node_indices() {
        first = true;
        if v != inst.s.into() {
            for e in g.edges(v) {
                if !first {
                    write!(file, " + ")?;
                }
                first = false;
                let (from, to) = (e.source(), e.target());
                if from == v {
                    write!(file, "f_{0}_{1} - f_{1}_{0}", to.index(), v.index())?;
                } else {
                    write!(file, "f_{0}_{1} - f_{1}_{0}", from.index(), v.index())?;
                }
            }
            writeln!(file, " - f_{} = 0", v.index())?;
        }
    }

    // Choose k vertices.
    first = true;
    for v in g.node_indices() {
        if !first {
            write!(file, " + ")?;
        }
        first = false;
        write!(file, "f_{}", v.index())?;
    }
    writeln!(file, " = {}", inst.k)?;

    // Force the absorption.
    for v in g.node_indices() {
        if v != inst.s.into() {
            write!(file, "{} f_{}", (inst.k - dist[&v]), v.index())?;
            for e in g.edges(v) {
                let (from, to) = (e.source(), e.target());
                if from == v {
                    write!(file, " - f_{}_{}", to.index(), v.index())?;
                } else {
                    write!(file, " - f_{}_{}", from.index(), v.index())?;
                }
            }
            writeln!(file, " >= 0")?;
        }
    }

    writeln!(file, "Bounds")?;

    for v in g.node_indices() {
        writeln!(file, "0 <= f_{} <= 1", v.index())?;
    }

    for e in g.edge_indices() {
        if let Some((from, to)) = g.edge_endpoints(e) {
            writeln!(file, "f_{}_{} >= 0", from.index(), to.index())?;
            writeln!(file, "f_{}_{} >= 0", to.index(), from.index())?;
            writeln!(file, "0 <= x_{}_{} <= 1", from.index(), to.index())?;
            writeln!(file, "0 <= x_{}_{} <= 1", to.index(), from.index())?;
        }
    }

    if ilp {
        writeln!(file, "Generals")?;
        for v in g.node_indices() {
            write!(file,"f_{} ", v.index())?;
        }
        for e in g.edge_indices() {
            if let Some((from, to)) = g.edge_endpoints(e) {
                write!(file,"x_{0}_{1} x_{1}_{0} ", from.index(), to.index())?;
            }
        }
        writeln!(file, "")?;
    }

    writeln!(file, "End")?;
    Ok(())
}

fn approximate(inst: &Instance, graph: &DiGraph<f64, f64>) -> HashSet<u32> {
    let mut cut_vertices: HashSet<u32> = Default::default();
    let mut current = inst.s;
    let mut neighbours: HashMap<u32, f64> = Default::default();

    while cut_vertices.len() < inst.k as usize {
        cut_vertices.insert(current);
        for e in graph.edges(current.into()) {
            let (from, to) = (e.source(), e.target());
            if from.index() as u32 == current {
                if let Some(val) = neighbours.get(&(to.index() as u32)) {
                    neighbours.insert(to.index() as u32, val + *e.weight());
                } else {
                    neighbours.insert(to.index() as u32, *e.weight());
                }

            } else {
                if let Some(val) = neighbours.get(&(from.index() as u32)) {
                    neighbours.insert(from.index() as u32, val + *e.weight());
                } else {
                    neighbours.insert(from.index() as u32, *e.weight());
                }
            }
            // Choose neighbour based on probability.
        }
    }
    cut_vertices
}

/// Main function of the program.
fn main() {
    let args = Args::parse();
    // Create an instance.
    let inst = read_file(&args.inputfile);

    // Start the job.
    if args.job == "ilp" {
        let _ = create_lp(true, &inst, &args.outputfile);
    } else if args.job == "lp" {
        let _ = create_lp(false, &inst, &args.outputfile);
    } else if args.job == "apx" {

    } else if args.job == "dot" {
        let file = fs::File::create(args.outputfile);
        let _ = writeln!(file.unwrap(), "{:?}", Dot::with_config(&inst.g, &[Config::EdgeNoLabel, Config::NodeNoLabel]));
    } else if args.job == "dot-cut" {
        let (cut_graph, _) = parse_solution(&args.solutionfile, &inst.g);
        let file = fs::File::create(args.outputfile);
        let dot = Dot::with_attr_getters(
            &cut_graph,
            &[Config::NodeNoLabel],
            &|_, edge| {
                if *edge.weight() > 0f64 {
                    format!("color=red")
                } else {
                    format!("color=black")
                }
            },
            &|_, _| "".to_string()
        );
        let _ = writeln!(file.unwrap(), "{:?}", dot);
    } else if args.job == "dot-flow" {
        let (_, flow_graph) = parse_solution(&args.solutionfile, &inst.g);
        let file = fs::File::create(args.outputfile);
        let dot = Dot::with_attr_getters(
            &flow_graph,
            &[],
            &|_, edge| {
                if *edge.weight() > 0f64 {
                    format!("color=blue")
                } else {
                    format!("color=black")
                }
            },
            &|_, (_, weight)| {
                if *weight > 0f64 {
                    format!("color=blue")
                } else {
                    format!("color=black")
                }
            },
        );
        let _ = writeln!(file.unwrap(), "{:?}", dot);
    }
}
