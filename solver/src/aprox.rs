use petgraph::graph::DiGraph;
use petgraph::visit::EdgeRef;
use std::collections::{HashMap, HashSet};

pub fn _approximate(inst: &crate::lp::Instance, graph: &DiGraph<f64, f64>) -> HashSet<u32> {
    let mut cut_vertices: HashSet<u32> = Default::default();
    let current = inst.source();
    let mut neighbours: HashMap<u32, f64> = Default::default();

  while cut_vertices.len() < inst.capacity() as usize {
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
