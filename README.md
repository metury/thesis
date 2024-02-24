# Master thesis

My master thesis from Charles University in Prague.

## Shortly written options:

### 1) $\min k$ connected cut

V tomhle případu chceme hledat minimální řez $M \subseteq E(G)$ takový, že se graf rozpadne na části $A \dot{\cup} B$ kde $|A| = k$ a zároveň $A$ je spojité.

### 2) Spanning tree congestion

Zde se řeší problém, kde v grafu $G$ se spanning tree $T$ definujeme operátor $\text{cong}(T,e)$ pro $\forall e \in T$ takový, že se počítá "kolik cest tu hranu může nahradit".

Jinak řečeno měříme velikost řezu indukovaného danou hranou $e \in T$.

Poté se díváme na $\text{cong}(T)$ což je maximum přes všechny $e \in T$. Tento problém se snažíme minimalizovat přes možné $T$.

### 3) $L$-cut

Řešení $L$-aproximace (nejspíše) pomocí PSD programování. To jest se nadefinují $s$ a $t$ vektory a pak vějíř mezi nimi.