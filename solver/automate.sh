#!/bin/bash

set -ueo pipefail

solutions="results.md"
report="results.pdf"
report_page="results.html"
directories="graphs programs programs/lp programs/ilp programs/sol-ilp programs/sol-lp images images/dot images/pdf programs/enh programs/sol-enh"

rm -rf $directories "$solutions" "$report" "$report_page"
mkdir -p $directories

graphs=$(cargo run -r -- --job gen)


echo "# Report of running the connected cut algorithm" > "$solutions"
echo "" >> "$solutions"
echo "This is an automatically generated report which runs the algorithm on given graphs. There is a graph, its integer linear program solution, linear program solution, enhancement of the linear program and approximation result." >> "$solutions"
echo "" >> "$solutions"
echo "| Graph | ILP | LP | Enhancement | Aproximation | Enh Apx |" >> "$solutions"
echo "|-------|-----|----|-------------|--------------|---------|" >> "$solutions"

for graph in $graphs; do
	input="graphs/$graph.in"
	ilp="programs/ilp/$graph.ilp"
	lp="programs/lp/$graph.lp"
	enh="programs/enh/$graph.lp"
	ilp_sol="programs/sol-ilp/$graph.sol"
	lp_sol="programs/sol-lp/$graph.sol"
	enh_sol="programs/sol-enh/$graph.sol"

	dot="images/dot/$graph.gv"
	dot_ilp_cut="images/dot/$graph-ilp-cut.gv"
	dot_lp_cut="images/dot/$graph-lp-cut.gv"
	dot_ilp_flow="images/dot/$graph-ilp-flow.gv"
	dot_lp_flow="images/dot/$graph-lp-flow.gv"
	dot_enh_flow="images/dot/$graph-enh-flow.gv"
	dot_enh_cut="images/dot/$graph-enh-cut.gv"
	dot_apx="images/dot/$graph-apx.gv"
	dot_apx_enh="images/dot/$graph-enh-apx.gv"

	image="images/pdf/$graph.pdf"
	pdf_ilp_cut="images/pdf/$graph-ilp-cut.pdf"
	pdf_lp_cut="images/pdf/$graph-lp-cut.pdf"
	pdf_ilp_flow="images/pdf/$graph-ilp-flow.pdf"
	pdf_lp_flow="images/pdf/$graph-lp-flow.pdf"
	pdf_enh_flow="images/pdf/$graph-enh-flow.pdf"
	pdf_enh_cut="images/pdf/$graph-enh-cut.pdf"
	pdf_apx="images/pdf/$graph-apx.pdf"
	pdf_apx_enh="images/pdf/$graph-enh-apx.pdf"

	# +---------------------------------------------------+ #
	# | Create ILP, LP and picture of the original graph. | #
	# +---------------------------------------------------+ #
	cargo run -r -- --job ilp -i "$input" -o "$ilp"
	cargo run -r -- --job lp -i "$input" -o "$lp"
	cargo run -r -- --job dot -i "$input" -o "$dot"

	# +-------------------------------------------+ #
	# | Run the dot program and gurobi optimiser. | #
	# +-------------------------------------------+ #
	dot -T pdf "$dot" -o "$image"
	gurobi_cl ResultFile="$ilp_sol" "$ilp"
	gurobi_cl ResultFile="$lp_sol" "$lp"

	# +-----------------------------+ #
	# | Enhance the linear program. | #
	# +-----------------------------+ #
	cp "$lp_sol" "$enh_sol"
	while [ $(cargo run -r -- -j enh -i "$input" -s "$enh_sol" -o "$enh") ]; do
		gurobi_cl ResultFile="$enh_sol" "$enh"
	done

	# +---------------------+ #
	# | Report the results. | #
	# +---------------------+ #
	printf "| [$graph](#$graph) |" >> "$solutions"
	printf " $(head -n 1 "$ilp_sol" | cut -d " " -f 5) |" >> "$solutions"
	printf " $(head -n 1 "$lp_sol" | cut -d " " -f 5) |" >> "$solutions"
	printf " $(head -n 1 "$enh_sol" | cut -d " " -f 5) |" >> "$solutions"

	# +------------------------+ #
	# | Generate ILP pictures. | #
	# +------------------------+ #
	cargo run -r -- --job dot-cut -i "$input" -o "$dot_ilp_cut" -s "$ilp_sol"
	cargo run -r -- --job dot-flow -i "$input" -o "$dot_ilp_flow" -s "$ilp_sol"
	dot -T pdf "$dot_ilp_cut" -o "$pdf_ilp_cut"
	dot -T pdf "$dot_ilp_flow" -o "$pdf_ilp_flow"

	# +-----------------------------+ #
	# | Generate enhanced pictures. | #
	# +-----------------------------+ #
	cargo run -r -- --job dot-cut -i "$input" -o "$dot_enh_cut" -s "$enh_sol"
	cargo run -r -- --job dot-flow -i "$input" -o "$dot_enh_flow" -s "$enh_sol"
	dot -T pdf "$dot_enh_cut" -o "$pdf_enh_cut"
	dot -T pdf "$dot_enh_flow" -o "$pdf_enh_flow"

	# +-----------------------+ #
	# | Generate LP pictures. | #
	# +-----------------------+ #
	cargo run -r -- --job dot-cut -i "$input" -o "$dot_lp_cut" -s "$lp_sol"
	cargo run -r -- --job dot-flow -i "$input" -o "$dot_lp_flow" -s "$lp_sol"
	dot -T pdf "$dot_lp_cut" -o "$pdf_lp_cut"
	dot -T pdf "$dot_lp_flow" -o "$pdf_lp_flow"

	# +------------------------+ #
	# | Run the approximation. | #
	# +------------------------+ #
	printf " $(cargo run -r -- --job apx -i "$input" -o "$dot_apx" -s "$lp_sol" | cut -d " " -f 5) |" >> "$solutions"
	echo " $(cargo run -r -- --job apx -i "$input" -o "$dot_apx_enh" -s "$enh_sol" | cut -d " " -f 5) |" >> "$solutions"

	# +---------------------------------+ #
	# | Generate approximated pictures. | #
	# +---------------------------------+ #
	dot -T pdf "$dot_apx" -o "$pdf_apx"
	dot -T pdf "$dot_apx_enh" -o "$pdf_apx_enh"
done

# +------------------------------------------------------------+ #
# | Finally summarize the results into one file with pictures. | #
# +------------------------------------------------------------+ #
for graph in $graphs; do
	input="graphs/$graph.in"
	image="images/pdf/$graph.pdf"
	pdf_ilp_cut="images/pdf/$graph-ilp-cut.pdf"
	pdf_lp_cut="images/pdf/$graph-lp-cut.pdf"
	pdf_ilp_flow="images/pdf/$graph-ilp-flow.pdf"
	pdf_lp_flow="images/pdf/$graph-lp-flow.pdf"
	pdf_enh_flow="images/pdf/$graph-enh-flow.pdf"
	pdf_enh_cut="images/pdf/$graph-enh-cut.pdf"
	pdf_apx="images/pdf/$graph-apx.pdf"
	pdf_apx_enh="images/pdf/$graph-enh-apx.pdf"
	echo "# $graph" >> "$solutions"
	echo "" >> "$solutions"
	echo "The source vertex is \$$(grep -Eoh "s=([0-9]+)" "$input")\$ and capacity is \$$(grep -Eoh "k=([0-9]+)" "$input")\$." >> "$solutions"
	echo "" >> "$solutions"
	echo "![]("$image")" >> "$solutions"
	echo "" >> "$solutions"
	echo "## Integer linear program" >> "$solutions"
	echo "" >> "$solutions"
	echo "### Flow" >> "$solutions"
	echo "" >> "$solutions"
	echo "![]("$pdf_ilp_flow")" >> "$solutions"
	echo "" >> "$solutions"
	echo "### Cut" >> "$solutions"
	echo "" >> "$solutions"
	echo "![]("$pdf_ilp_cut")" >> "$solutions"
	echo "" >> "$solutions"
	echo "## Linear program" >> "$solutions"
	echo "" >> "$solutions"
	echo "### Flow" >> "$solutions"
	echo "" >> "$solutions"
	echo "![]("$pdf_lp_flow")" >> "$solutions"
	echo "" >> "$solutions"
	echo "### Cut" >> "$solutions"
	echo "" >> "$solutions"
	echo "![]("$pdf_lp_cut")" >> "$solutions"
	echo "" >> "$solutions"
	echo "## Enhancement" >> "$solutions"
	echo "" >> "$solutions"
	echo "### Flow" >> "$solutions"
	echo "" >> "$solutions"
	echo "![]("$pdf_enh_flow")" >> "$solutions"
	echo "" >> "$solutions"
	echo "### Cut" >> "$solutions"
	echo "" >> "$solutions"
	echo "![]("$pdf_enh_cut")" >> "$solutions"
	echo "" >> "$solutions"
	echo "## Aproximation" >> "$solutions"
	echo "" >> "$solutions"
	echo "![]("$pdf_apx")" >> "$solutions"
	echo "" >> "$solutions"
	echo "## Aproximation - enhanced" >> "$solutions"
	echo "" >> "$solutions"
	echo "![]("$pdf_apx_enh")" >> "$solutions"
	echo "" >> "$solutions"
done

echo -e "\033[34;1mGenerating $report and $report_page.\033[0"
pandoc "$solutions" -o "$report"
pandoc "$solutions" -o "$report_page"
