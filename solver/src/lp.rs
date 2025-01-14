use petgraph::graph::UnGraph;
use std::io::{self, Write};
use petgraph::algo::dijkstra;
use petgraph::visit::EdgeRef;
use std::fs;

/// Shorter type for graph.
pub type Graph = UnGraph<(),()>;

/// Structure for minimum c-connected cut.
pub struct Instance {
	g: Graph,
	c: u32,
	s: u32,
}

impl Instance {
	pub fn new(g: Graph, k: u32, s: u32) -> Self {
		Instance{g,c: k,s}
	}
	pub fn capacity(&self) -> u32 {
		self.c
	}
	pub fn source(&self) -> u32 {
		self.s
	}
	pub fn graph(&self) -> &Graph {
		&self.g
	}
}

/// Create linear program and its integer version.
pub fn create_lp(ilp: bool, inst: &Instance, ofile:& String) -> io::Result<()> {
	let graph = &inst.g;
	let mut first = true;
	let distances = dijkstra(graph, inst.source().into(), None, |_| 1);
	let mut file = fs::File::create(ofile)?;
	// Minimize the sum of all cut values x.
	writeln!(file, "Minimize")?;
	for e in graph.edge_indices() {
		if !first {
			write!(file, " + ")?;
		}
		first = false;
		if let Some((from, to)) = graph.edge_endpoints(e) {
			write!(file, "x_{0}_{1} + x_{1}_{0}", from.index(), to.index())?;
		}
	}
	writeln!(file, "")?;
	writeln!(file, "Subject to")?;
	// Defining the cut x.
	for e in graph.edge_indices() {
		if let Some((from, to)) = graph.edge_endpoints(e) {
			writeln!(file, "x_{0}_{1} - f_{0} + f_{1} >= 0", from.index(), to.index())?;
			writeln!(file, "x_{0}_{1} - f_{1} + f_{0} >= 0", from.index(), to.index())?;
			writeln!(file, "x_{1}_{0} - f_{0} + f_{1} >= 0", from.index(), to.index())?;
			writeln!(file, "x_{1}_{0} - f_{1} + f_{0} >= 0", from.index(), to.index())?;
		}
	}
	// The sum of the outgoing flow from s is k-1.
	first = true;
	for e in graph.edges(inst.source().into()) {
		let from = e.source().index();
		let to = e.target().index();
		if !first {
			write!(file, " + ")?;
		}
		first = false;
		if from == inst.source() as usize {
			write!(file, "f_{}_{}", inst.source(), to)?;
		} else {
			write!(file, "f_{}_{}", inst.source(), from)?;
		}
	}
	writeln!(file, " = {}", inst.c - 1)?;
	// Flow in the source vertex is 1.
	writeln!(file, "f_{} = 1", inst.source())?;
	// Ensure that the flow-sum of outgoing edges and the vertex itself is same as ingoing edges.
	for v in graph.node_indices() {
		first = true;
		if v != inst.source().into() {
			for e in graph.edges(v) {
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
	// The vertex flow-sum is supposed to be c.
	first = true;
	for v in graph.node_indices() {
		if !first {
			write!(file, " + ")?;
		}
		first = false;
		write!(file, "f_{}", v.index())?;
	}
	writeln!(file, " = {}", inst.capacity())?;
	// Force the absorption.
	for v in graph.node_indices() {
		if v != inst.source().into() {
			write!(file, "{} f_{}", (inst.c - distances[&v]), v.index())?;
			for e in graph.edges(v) {
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
	// Define the boundaries of all variables.
	writeln!(file, "Bounds")?;
	for v in graph.node_indices() {
		writeln!(file, "0 <= f_{} <= 1", v.index())?;
	}
	for e in graph.edge_indices() {
		if let Some((from, to)) = graph.edge_endpoints(e) {
			writeln!(file, "f_{}_{} >= 0", from.index(), to.index())?;
			writeln!(file, "f_{}_{} >= 0", to.index(), from.index())?;
			writeln!(file, "0 <= x_{}_{} <= 1", from.index(), to.index())?;
			writeln!(file, "0 <= x_{}_{} <= 1", to.index(), from.index())?;
		}
	}
	// Ensure the integer values if we want them.
	if ilp {
		writeln!(file, "Generals")?;
		for v in graph.node_indices() {
			write!(file,"f_{} ", v.index())?;
		}
		for e in graph.edge_indices() {
			if let Some((from, to)) = graph.edge_endpoints(e) {
			write!(file,"x_{0}_{1} x_{1}_{0} ", from.index(), to.index())?;
			}
		}
		writeln!(file, "")?;
	}
	writeln!(file, "End")?;
	Ok(())
}
