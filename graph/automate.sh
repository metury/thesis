#!/bin/bash

set -ueo pipefail

mkdir -p graphs programs programs/lp programs/ilp programs/sol-ilp programs/sol-lp images images/dot images/png images/svg

echo "s=0
k=6
[0;1][1;2][2;3][3;4][4;5][5;6][6;7][7;8][8;9] # The path.
[0;10][10;11][10;12][10;13][10;14][10;15][10;16][10;17][10;18][10;19][10;20]
" > graphs/test.in

for graph in "test$@"; do

	input="graphs/$graph.in"
	ilp="programs/ilp/$graph.ilp"
	lp="programs/lp/$graph.lp"
	ilp_sol="programs/sol-ilp/$graph.sol"
	lp_sol="programs/sol-lp/$graph.sol"

	dot="images/dot/$graph.gv"
	dot_ilp_cut="images/dot/ilp-cut-$graph.gv"
	dot_lp_cut="images/dot/lp-cut-$graph.gv"
	dot_ilp_flow="images/dot/ilp-flow-$graph.gv"
	dot_lp_flow="images/dot/lp-flow-$graph.gv"

	cargo run -r -- --job ilp -i "$input" -o "$ilp"
	cargo run -r -- --job lp -i "$input" -o "$lp"
	cargo run -r -- --job dot -i "$input" -o "$dot"

	dot -T png "$dot" -o "images/png/$graph.png"
	dot -T svg "$dot" -o "images/svg/$graph.svg"
	gurobi_cl ResultFile="$ilp_sol" "$ilp"
	gurobi_cl ResultFile="$lp_sol" "$lp"

	cargo run -r -- --job dot-cut -i "$input" -o "$dot_ilp_cut" -s "$ilp_sol"
	cargo run -r -- --job dot-flow -i "$input" -o "$dot_ilp_flow" -s "$ilp_sol"

	dot -T png "$dot_ilp_cut" -o "images/png/ilp-cut-$graph.png"
	dot -T svg "$dot_ilp_cut" -o "images/svg/ilp-cut-$graph.svg"
	dot -T png "$dot_ilp_flow" -o "images/png/ilp-flow-$graph.png"
	dot -T svg "$dot_ilp_flow" -o "images/svg/ilp-flow-$graph.svg"

	cargo run -r -- --job dot-cut -i "$input" -o "$dot_lp_cut" -s "$lp_sol"
	cargo run -r -- --job dot-flow -i "$input" -o "$dot_lp_flow" -s "$lp_sol"

	dot -T png "$dot_lp_cut" -o "images/png/lp-cut-$graph.png"
	dot -T svg "$dot_lp_cut" -o "images/svg/lp-cut-$graph.svg"
	dot -T png "$dot_lp_flow" -o "images/png/lp-flow-$graph.png"
	dot -T svg "$dot_lp_flow" -o "images/svg/lp-flow-$graph.svg"

done
