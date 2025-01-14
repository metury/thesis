use petgraph::dot::{Dot, Config};
use std::io::Write;
use clap::Parser;
use std::fs;

mod parser;
mod lp;
mod aprox;

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

/// Main function of the program.
fn main() {
    let args = Args::parse();
    // Create an instance.
    let inst = parser::read_file(&args.inputfile);

    // Start the job.
    if args.job == "ilp" {
        let _ = lp::create_lp(true, &inst, &args.outputfile);
    } else if args.job == "lp" {
        let _ = lp::create_lp(false, &inst, &args.outputfile);
    } else if args.job == "apx" {

    } else if args.job == "dot" {
        let file = fs::File::create(args.outputfile);
        let _ = writeln!(file.unwrap(), "{:?}", Dot::with_config(inst.graph(), &[Config::EdgeNoLabel, Config::NodeNoLabel]));
    } else if args.job == "dot-cut" {
        let (cut_graph, _) = parser::parse_solution(&args.solutionfile, inst.graph());
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
        let (_, flow_graph) = parser::parse_solution(&args.solutionfile, inst.graph());
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
