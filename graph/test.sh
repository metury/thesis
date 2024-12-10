#!/bin/bash

set -ueo pipefail

mkdir -p graphs programs programs/lp programs/ilp programs/sol-ilp programs/sol-lp dot

echo "s=0
k=6
[0;1][1;2][2;3][3;4][4;5][5;6][6;7][7;8][8;9] # The path.
[0;10][10;11][10;12][10;13][10;14][10;15][10;16][10;17][10;18][10;19][10;20]
" > graphs/test.in

cargo run -r -- --job ilp -i graphs/test.in -o programs/ilp/test.ilp
cargo run -r -- --job lp -i graphs/test.in -o programs/lp/test.lp
cargo run -r -- --job dot -i graphs/test.in -o dot/test.gv

dot -T png -O dot/test.gv
gurobi_cl ResultFile="programs/sol-ilp/test.sol" programs/ilp/test.ilp
gurobi_cl ResultFile="programs/sol-lp/test.sol" programs/lp/test.lp

cargo run -r -- --job dot-cut -i graphs/test.in -o dot/ilp-cut.gv -s programs/sol-ilp/test.sol
cargo run -r -- --job dot-flow -i graphs/test.in -o dot/ilp-flow.gv -s programs/sol-ilp/test.sol

dot -T png -O dot/ilp-cut.gv
dot -T png -O dot/ilp-flow.gv

cargo run -r -- --job dot-cut -i graphs/test.in -o dot/lp-cut.gv -s programs/sol-lp/test.sol
cargo run -r -- --job dot-flow -i graphs/test.in -o dot/lp-flow.gv -s programs/sol-lp/test.sol

dot -T png -O dot/lp-cut.gv
dot -T png -O dot/lp-flow.gv
