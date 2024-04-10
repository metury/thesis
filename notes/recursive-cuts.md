# Recursive cuts on STC

The plan is to assign weights to edges so that we may either iteratively create spanning tree by lowest current weight or just in one go, which is probably way worse.

## Min cut

For every edge $e = \{s,t\}$ we run min cut based on the $s-t$ flow (note that we will use the property of connected vertices of the polyhedron). Then we choose the edge with the smallest weight and add it to the tree. After that we split the graph based on the smallest cut. Next we recurse on both graphs separately.

### Complexity

We run $m$ times min cut, which is polynomial. Then choose smallest one, which can be done in $O(1)$ when adding edges to heap. Then recurse, which is the hardest part. Note that we will add $n-1$ edges to the spanning tree, when we are considering connected graphs, otherwise even less. So that if the recursion tree would be a path then still the depth will be polynomial and on every level we will compute min cut for in total at most $m$ edges, thus this is still polynomial.

## Max cut

Otherwise pretty much the same process only we do not look at min cuts but rather max cuts. That is we try to avoid the heviest edges, not the lightest ones as previously.

### Complexity

For this the complexity is pretty much the same except the fact, that max connected cut is not well known and perhpas that may not even exist.

## Combination

The final piece of puzzle is to use both cuts to determine which edges to take. Note that when taking an edge which has small enough difference then the edge is light and in worse case still light, therefore it is pretty much close to optimal. On the other hand what if the difference is LARGE. Then it means that in the graph there are two cuts where one is large and the second one small.


*Thinking out loud: What if we took all the edges in the LARGE cut, but they may form a cycle. Also if we chop the graph by the small cut, then the big cut may split into smaller ones, but in worst case will remain in one component. This may call for induction.*

* * *

# Further notes on max cut

The basic idea of the algorithm is to iterate through the vertices and look to which $s$ or $t$ are connected. If only one add it to the component. If none, jsut skip it. If both put it to the vertex which has more neighbours. If the size of neighbours is same choose arbitrarily.

**Definition:** Connected $s-t$ cut for $G = (V,E)$ connected graph and $s \neq t \in V(G)$ is a subset $W \subseteq V(G)$ for which the properties hold:

1. $s \in W$
2. $t \notin W$
3. $G[W]$ is connected
4. $G[V \setminus W]$ is also connected

Then the max $s-t$ cut is max of $E(W, V \setminus W)$ over all connected $s-t$ cuts.

## Relation to STC

My first claim would be that minimizing max $s-t$ connected cut over all edges (which means $\{s,t\} = e \in E(G)$ ) is the same as STC.
