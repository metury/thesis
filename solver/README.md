# Connected cut solver

There is a solver (more precisely a parser) for Connected cut problem, where we have to choose $k$ vertices so they induce a connected subgraph and the edges defined by such subset is minimized. Also a source vertex is predefined, that is a vertex $s$ which has to be in the solution.

## Automation with script `automate.sh`

There is a `bash` script which automates the whole process. Note that `gurobi_cl` must be present and also `dot`, `pandoc` and `rust`. Then it it enough to just run `./automate.sh`.

## Rust parser

One can also run just the parser. There are multiple purposes.

1. LP generation
2. ILP generation
3. Running APX program
4. Create DOT picture.
5. Create DOT from flow result.
6. Create DOT from cut result.
7. ENHance the linear program.

You may run `cargo run --release -- --help` to see details.
