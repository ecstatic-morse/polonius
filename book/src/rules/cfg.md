# Control-flow graph

## Nodes

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

## Basic Blocks

#### `bb_edge`

	.decl bb_edge(src: Block, targ: Block)
	.input bb_edge

#### `node_is_loc`

	.decl node_is_loc(node: Node, loc: Location)
	.input node_is_loc

#### `precedes_in_block`

	.decl precedes_in_block(loc1: Location, loc2: Location) inline

	precedes_in_block([bb, stmt1, mid1], [bb, stmt2, mid2]) :-
	    stmt2 > stmt1; (stmt1 = stmt2, mid2 = "Mid", mid1 = "Start").

#### `succeeds_in_block`

	.decl succeeds_in_block(loc1: Location, loc2: Location) inline

	succeeds_in_block([bb, stmt1, mid1], [bb, stmt2, mid2]) :-
	    stmt2 > stmt1; (stmt1 = stmt2, mid1 = "Mid", mid2 = "Start").

