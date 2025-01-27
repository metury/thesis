#!/bin/bash

set -ueo pipefail

rm -rf graphs programs programs/lp programs/ilp programs/sol-ilp programs/sol-lp images images/dot images/png images/svg apx.sol

mkdir -p graphs programs programs/lp programs/ilp programs/sol-ilp programs/sol-lp images images/dot images/png images/svg

graphs=$(cargo run -r -- --job gen)

for graph in $graphs; do

	input="graphs/$graph.in"
	ilp="programs/ilp/$graph.ilp"
	lp="programs/lp/$graph.lp"
	ilp_sol="programs/sol-ilp/$graph.sol"
	lp_sol="programs/sol-lp/$graph.sol"

	dot="images/dot/$graph.gv"
	dot_ilp_cut="images/dot/$graph-ilp-cut.gv"
	dot_lp_cut="images/dot/$graph-lp-cut.gv"
	dot_ilp_flow="images/dot/$graph-ilp-flow.gv"
	dot_lp_flow="images/dot/$graph-lp-flow.gv"
	dot_apx="images/dot/$graph-apx.gv"

	cargo run -r -- --job ilp -i "$input" -o "$ilp"
	cargo run -r -- --job lp -i "$input" -o "$lp"
	cargo run -r -- --job dot -i "$input" -o "$dot"

	dot -T png "$dot" -o "images/png/$graph.png"
	dot -T svg "$dot" -o "images/svg/$graph.svg"
	gurobi_cl ResultFile="$ilp_sol" "$ilp"
	gurobi_cl ResultFile="$lp_sol" "$lp"

	cargo run -r -- --job dot-cut -i "$input" -o "$dot_ilp_cut" -s "$ilp_sol"
	cargo run -r -- --job dot-flow -i "$input" -o "$dot_ilp_flow" -s "$ilp_sol"

	dot -T png "$dot_ilp_cut" -o "images/png/$graph-ilp-cut.png"
	dot -T svg "$dot_ilp_cut" -o "images/svg/$graph-ilp-cut.svg"
	dot -T png "$dot_ilp_flow" -o "images/png/$graph-ilp-flow.png"
	dot -T svg "$dot_ilp_flow" -o "images/svg/$graph-ilp-flow.svg"

	cargo run -r -- --job dot-cut -i "$input" -o "$dot_lp_cut" -s "$lp_sol"
	cargo run -r -- --job dot-flow -i "$input" -o "$dot_lp_flow" -s "$lp_sol"

	dot -T png "$dot_lp_cut" -o "images/png/$graph-lp-cut.png"
	dot -T svg "$dot_lp_cut" -o "images/svg/$graph-lp-cut.svg"
	dot -T png "$dot_lp_flow" -o "images/png/$graph-lp-flow.png"
	dot -T svg "$dot_lp_flow" -o "images/svg/$graph-lp-flow.svg"

	cargo run -r -- --job apx -i "$input" -o "$dot_apx" -s "$lp_sol" >> apx.sol

	dot -T png "$dot_apx" -o "images/png/$graph-apx.png"
	dot -T svg "$dot_apx" -o "images/svg/$graph-apx.svg"

done
