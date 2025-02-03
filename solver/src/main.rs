use clap::Parser;
use petgraph::dot::{Config, Dot};
use std::fs;
use std::io::Write;
use std::collections::HashSet;

mod apx;
mod generator;
mod lp;
mod parser;

/// These arguments are available. You must select a job and than provide an input file to the graph.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// What job to do: ilp - integer linear program, lp - linear program, apx - approximate the result, dot - create dot file for graph, dot-flow, dot-cut.
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
        let _ = lp::create_lp(true, &instance, &args.outputfile, &Default::default());
    } else if args.job == "lp" {
        let _ = lp::create_lp(false, &instance, &args.outputfile, &Default::default());
    } else if args.job == "apx" {
        let (_, mut flow_graph) = parser::parse_solution(&args.solutionfile, instance.graph());
        let cut = apx::approximate(&instance, &flow_graph);
        apx::update_graph(&mut flow_graph, &cut);
        let file = fs::File::create(args.outputfile);
        let dot = Dot::with_attr_getters(
            &flow_graph,
            &[Config::EdgeNoLabel, Config::NodeNoLabel],
            &|_, edge| {
                if *edge.weight() < 0f64 {
                    format!("color=teal")
                } else {
                    format!("color=grey")
                }
            },
            &|_, (_, weight)| {
                if weight < &0f64 {
                    format!("color=orange")
                } else {
                    format!("color=grey")
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
    } else if args.job == "enh" {
        let mut edges: HashSet<(usize, usize)> = Default::default();
        let (cut_graph, flow_graph) = parser::parse_solution(&args.solutionfile, instance.graph());
        for e in flow_graph.edge_indices() {
            if let Some((from, to)) = flow_graph.edge_endpoints(e) {
                if let Some(f) = cut_graph.find_edge(from, to) {
                    if flow_graph.edge_weight(e).unwrap_or(&0f64) > &1f64 && cut_graph.edge_weight(f).unwrap_or(&0f64) > &0f64 {
                        edges.insert((from.index(), to.index()));
                        edges.insert((to.index(), from.index()));
                    }
                }
            }
        }
        let _ = lp::create_lp(false, &instance, &args.outputfile, &edges);
        if edges.len() > 0 {
            println!("Enhanced!");
        }
    } else {
        println!("The job type {} is not known.", args.job);
    }
}
