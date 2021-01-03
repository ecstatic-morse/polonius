# Control-flow graph

#### `cfg_edge`

Indicates that an edge exists between `source` and `target`

	.decl cfg_edge(sourceNode: Node, targetNode: Node)
	.input cfg_edge

#### `cfg_node`

Enumerates all nodes (note that this approach implies that a single node
graph is essentially not a thing).

	.decl cfg_node(p: Node)

	cfg_node(p) :- cfg_edge(p, _).
	cfg_node(p) :- cfg_edge(_, p).

