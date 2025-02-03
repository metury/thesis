For each edge we would like to force that if we have a cut edge we want small flow, that is:

$$
k - d(s,u) - (k - d(s,u)) \cdot x_{v,u} \geq f_{v,u}
$$


*Conjecture:* For a tree and optimal cut $S$, where we have a leaf $v$ such that $v \in S$ then running the algorithm from $v$ will output the optimal value.

There are only few types of vertices selected by the algorithm. Either they are just 1 or there is a cluster of multiple adjacent vertices with the same value. In the latter it means that this point is somewhat still crucial but we do not have enough capacity. In the former it is either "must select" or just not so bad choice at a first glance.
