use clap::Parser;
use petgraph::dot::{Config, Dot};
use std::fs;
use std::io::Write;

mod apx;
mod generator;
mod lp;
mod parser;

/// These arguments are available. You must select a job and than provide an input file to the graph.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// What job to do: ilp - integer linear program, lp - linear program, apx - approximate the result, dot - create dot file for graph.
    #[arg(short, long)]
    job: String,
    /// Graph in the input file.
    #[arg(short,long, default_value_t = String::new())]
    inputfile: String,
    /// Where to output the result.
    #[arg(short,long, default_value_t = String::new())]
    outputfile: String,
    /// Where is the file with the solution.
    #[arg(short,long, default_value_t = String::new())]
    solutionfile: String,
}

/// Main function of the program.
fn main() {
    let args = Args::parse();
    if args.job == "gen" {
        generator::generate();
        std::process::exit(0);
    }
    let instance = parser::read_file(&args.inputfile);
    // Start the job.
    if args.job == "ilp" {
        let _ = lp::create_lp(true, &instance, &args.outputfile);
    } else if args.job == "lp" {
        let _ = lp::create_lp(false, &instance, &args.outputfile);
    } else if args.job == "apx" {
        let (_, mut flow_graph) = parser::parse_solution(&args.solutionfile, instance.graph());
        let cut = apx::approximate(&instance, &flow_graph);
        for v in cut {
            if let Some(weight) = flow_graph.node_weight_mut(v.into()) {
                *weight = -1f64;
            }
        }
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
                if weight == &-1f64 {
                    format!("color=orange")
                } else {
                    format!("color=black")
                }
            },
        );
        let _ = writeln!(file.unwrap(), "{:?}", dot);
    } else if args.job == "dot" {
        let file = fs::File::create(args.outputfile);
        let _ = writeln!(
            file.unwrap(),
            "{:?}",
            Dot::with_config(
                instance.graph(),
                &[Config::EdgeNoLabel, Config::NodeNoLabel]
            )
        );
    } else if args.job == "dot-cut" {
        let (cut_graph, _) = parser::parse_solution(&args.solutionfile, instance.graph());
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
            &|_, _| "".to_string(),
        );
        let _ = writeln!(file.unwrap(), "{:?}", dot);
    } else if args.job == "dot-flow" {
        let (_, flow_graph) = parser::parse_solution(&args.solutionfile, instance.graph());
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
