use petgraph::graph::UnGraph;
use petgraph::visit::EdgeRef;
use petgraph::dot::{Dot, Config};
use clap::Parser;
use regex::Regex;
use std::fs;
use std::io::{self, Write};

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
    #[arg(short,long, default_value_t = String::new())]
    inputfile: String,

     /// Output file.
    #[arg(short,long, default_value_t = String::new())]
    outputfile: String,

    /// Source vertex.
    #[arg(short, long, default_value_t = 0)]
    source: u32,

    /// What graph to create: complete, path.
    #[arg(short, long, default_value_t = String::new())]
    graph: String,

    /// How big the graph should be.
    #[arg(short, default_value_t = 0)]
    n: u32,

    /// How big the connected cut should be.
    #[arg(short, default_value_t = 0)]
    k: u32,
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

/// Create linear program.
/// ilp is for integer linear program.
fn create_lp(ilp: bool, inst: &Instance, ofile:& String) -> io::Result<()> {
    let g = &inst.g;
    let mut first = true;

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
            write!(file, "{} f_{}", inst.k - 1, v.index())?;
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

/// Create a complete graph with n vertices.
fn complete_graph(n: u32) -> Graph {
    let mut raw_edges: Vec<(u32, u32)> = vec![];
    for i in 0..n {
        for j in (i+1)..n {
            raw_edges.push((i,j));
        }
    }
    UnGraph::<(), ()>::from_edges(&raw_edges)
}

/// Main function of the program.
fn main() {
    let args = Args::parse();
    // Create an instance.
    let mut inst = Instance {g: complete_graph(1), s:0, k:0};
    if !args.inputfile.is_empty() {
        inst = read_file(&args.inputfile);
    } else if args.graph == "complete" {
        inst = Instance{g: complete_graph(args.n), k: args.k, s: args.source};
    }

    // Start the job.
    if args.job == "ilp" {
        let _ = create_lp(true, &inst, &args.outputfile);
    } else if args.job == "lp" {
        let _ = create_lp(false, &inst, &args.outputfile);
    } else if args.job == "apx" {

    } else if args.job == "dot" {
        let file = fs::File::create(args.outputfile);
        let _ = writeln!(file.unwrap(), "{:?}", Dot::with_config(&inst.g, &[Config::EdgeNoLabel, Config::NodeNoLabel]));
    }
}
