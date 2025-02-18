use petgraph::graph::DiGraph;
use petgraph::visit::EdgeRef;
use std::collections::{HashMap, HashSet};
use rand::prelude::*;

/// Randomly choose from neighbours based on their values.
fn choose(neighbours: &HashMap<u32, f64>) -> u32 {
	let total_weight: f64 = neighbours.iter().map(|(_, weight)| weight).sum();
	let normalized_elements: Vec<_> = neighbours
	.iter()
	.map(|(vertex, weight)| (vertex, weight / total_weight))
	.collect();
	let random_number: f64 = thread_rng().gen();
	let mut cumulative_weight = 0.0;
	let chosen_element = normalized_elements.iter().find(|&(_, weight)| {
		cumulative_weight += weight;
		cumulative_weight >= random_number
	});
	*chosen_element.unwrap().0
}

/// Compute the size of the cut.
fn size_of_cut(cut: &HashSet<u32>, graph: &DiGraph<f64, f64>) -> usize {
	let mut size = 0;
	for e in graph.edge_references() {
		let (from, to) = (e.source(), e.target());
		if cut.contains(&(from.index() as u32)) && !cut.contains(&(to.index() as u32)) {
			size += 1;
		} else if !cut.contains(&(from.index() as u32)) && cut.contains(&(to.index() as u32)) {
			size += 1;
		}
	}
	size
}

/// Set weights of edges to -1 if they are in the cut and similarly for vertices.
pub fn update_graph(graph: &mut DiGraph<f64, f64>, cut: &HashSet<u32>) {
	for v in cut {
		if let Some(weight) = graph.node_weight_mut((*v).into()) {
			*weight = -1f64;
		}
	}
	for e in graph.edge_indices() {
		match graph.edge_endpoints(e) {
			Some((from, to)) => {
				if cut.contains(&(from.index() as u32)) && !cut.contains(&(to.index() as u32)) {
					graph[e] = -1f64;
				} else if !cut.contains(&(from.index() as u32)) && cut.contains(&(to.index() as u32)) {
					graph[e] = -1f64;
				}
			}
			None => continue,
		}

	}
}

/// Run the approximation capacity-times on the given graph and solution of the LP.
pub fn approximate(inst: &crate::lp::Instance, graph: &DiGraph<f64, f64>) -> HashSet<u32> {
	let mut cut: HashSet<u32> = Default::default();
	let mut best = usize::MAX;

	for _ in 0..inst.capacity() {
		let mut cut_vertices: HashSet<u32> = Default::default();
		let mut current = inst.source();
		let mut neighbours: HashMap<u32, f64> = Default::default();

		while cut_vertices.len() < inst.capacity() as usize {
			cut_vertices.insert(current);
			for e in graph.edges(current.into()) {
				let (from, to) = (e.source().index() as u32, e.target().index() as u32);
				if from == current && !cut_vertices.contains(&to) && graph.node_weight(e.target()).unwrap() > &0f64 {
					neighbours.insert(to, *graph.node_weight(e.target()).unwrap());
				} else if to == current && !cut_vertices.contains(&from) && graph.node_weight(e.source()).unwrap() > &0f64{
					neighbours.insert(from, *graph.node_weight(e.source()).unwrap());
				}
			}
			let next = choose(&neighbours);
			current = next.into();
		}
		let size = size_of_cut(&cut_vertices, graph);
		if size < best {
			cut = cut_vertices;
			best = size;
		}
	}
	println!("The approximated size is {}", best);
	cut
}
