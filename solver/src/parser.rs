use petgraph::graph::{DiGraph, UnGraph};
use regex::Regex;
use std::fs;
use std::collections::HashSet;

/// Read graph from file which is provided via a file path.
/// Considered format of a graph is by giving out edges in a from [idfrom;idto].
/// Also the file itself has source vertex and the capacity.
pub fn read_file(filepath: &str) -> crate::lp::Instance {
    let contents = fs::read_to_string(filepath).unwrap();
    let edge_regex = Regex::new(r"\[\s*([0-9]*)\s*;\s*([0-9]*)\s*\]").unwrap();
    let source_regex = Regex::new(r"s\s*=\s*([0-9]+)");
    let capacity_regex = Regex::new(r"k\s*=\s*([0-9]+)");
    let mut raw_edges: Vec<(u32, u32)> = vec![];
    for (_, [id_from, id_to]) in edge_regex.captures_iter(&contents).map(|c| c.extract()) {
        raw_edges.push((id_from.parse().unwrap(), id_to.parse().unwrap()));
    }
    let graph = UnGraph::<(), ()>::from_edges(&raw_edges);
    let source: u32 = (&source_regex.expect("REASON").captures(&contents).unwrap()[1])
        .parse()
        .unwrap();
    let capacity: u32 = (&capacity_regex.expect("REASON").captures(&contents).unwrap()[1])
        .parse()
        .unwrap();
    crate::lp::Instance::new(graph, capacity, source)
}

/// Parse the solution and create two graphs. One with cut values on the edges and the second one with flow values.
pub fn parse_solution(
    filepath: &str,
    g: &crate::lp::Graph,
) -> (DiGraph<f64, f64>, DiGraph<f64, f64>) {
    let mut cut_graph: DiGraph<f64, f64> = Default::default();
    let mut flow_graph: DiGraph<f64, f64> = Default::default();
    // Add all nodes.
    for _ in g.node_indices() {
        cut_graph.add_node(0f64);
        flow_graph.add_node(0f64);
    }
    // Add all edges with zero value.
    for e in g.edge_indices() {
        if let Some((from, to)) = g.edge_endpoints(e) {
            cut_graph.add_edge(from, to, 0f64);
            flow_graph.add_edge(from, to, 0f64);
            cut_graph.add_edge(to, from, 0f64);
            flow_graph.add_edge(to, from, 0f64);
        }
    }
    let contents = fs::read_to_string(filepath).unwrap();
    // Update cut values.
    let cut_regex = Regex::new(r"x_([0-9]+)_([0-9]+)\s([0-9\.e\-\+]+)").unwrap();
    for (_, [id_from, id_to, value]) in cut_regex.captures_iter(&contents).map(|c| c.extract()) {
        let from = id_from.parse::<u32>().unwrap();
        let to = id_to.parse::<u32>().unwrap();
        let val = value.parse::<f64>().unwrap();
        if let Some(edge) = cut_graph.find_edge(from.into(), to.into()) {
            cut_graph[edge] = val;
        }
    }
    // Update flow values.
    let flow_edge_regex = Regex::new(r"f_([0-9]+)_([0-9]+)\s([0-9\.e\-\+]+)").unwrap();
    let flow_node_regex = Regex::new(r"f_([0-9]+)\s([0-9\.e\-\+]+)").unwrap();
    for (_, [id_from, id_to, value]) in flow_edge_regex
        .captures_iter(&contents)
        .map(|c| c.extract())
    {
        let from = id_from.parse::<u32>().unwrap();
        let to = id_to.parse::<u32>().unwrap();
        let val = value.parse::<f64>().unwrap();
        if let Some(edge) = flow_graph.find_edge(from.into(), to.into()) {
            flow_graph[edge] = val;
        }
    }
    for (_, [id, value]) in flow_node_regex
        .captures_iter(&contents)
        .map(|c| c.extract())
    {
        let v = id.parse::<u32>().unwrap();
        let val = value.parse::<f64>().unwrap();
        if let Some(weight) = flow_graph.node_weight_mut(v.into()) {
            *weight = val;
        }
    }
    (cut_graph, flow_graph)
}

pub fn update_edges(filepath: &str, edges: & mut HashSet<(usize, usize)>) {
    if let Ok(contents) = fs::read_to_string(filepath) {
        let regex = Regex::new(r"x_([0-9]+)_([0-9]+) = 0").unwrap();
        for (_, [from_str, to_str]) in regex
            .captures_iter(&contents)
            .map(|c| c.extract())
            {
                let from = from_str.parse::<usize>().unwrap();
                let to = to_str.parse::<usize>().unwrap();
                edges.insert((from, to));
                edges.insert((to, from));
            }
    }
}
