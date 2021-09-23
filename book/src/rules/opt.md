# Loans (optimized)

#### `loan_issued_at`

Indicates that the given loan `Loan` was "issued" at the given node `Node`,
creating a reference with the origin `Origin`.

	.decl loan_issued_at(origin: Origin, loan: Loan, node: Node)
	.input loan_issued_at

#### `loan_killed_at`

Indicates that the path borrowed by the loan `Loan` has changed in some way
that the loan no longer needs to be tracked. (in particular, mutations to the
path that was borrowed no longer invalidate the loan.)

	.decl loan_killed_at(loan: Loan, node: Node)
	.input loan_killed_at

#### `loan_invalidated_at`

Indicates that the loan is "invalidated" by some action tha takes place at the
given node; if any origin that references this loan is live, that is an error

	.decl loan_invalidated_at(loan: Loan, node: Node)
	.input loan_invalidated_at

#### `subset_base`

Indicates that `O1 <= O2` -- i.e., the set of loans in O1 are a subset of those
in O2.

	.decl subset_base(origin1: Origin, origin2: Origin, n: Node)
	.input subset_base

#### `placeholder`

Declares a "placeholder origin" and loan. These are the named lifetimes that
appear on function declarations and the like (e.g., the `'a` in `fn
foo<'a>(...)`).

	.decl placeholder(o: Origin, l: Loan)
	.input placeholder

#### `placeholder_origin`

	.decl placeholder_origin(o: Origin)
	placeholder_origin(o) :- placeholder(o, _).

#### `known_placeholder_subset`

Declares a known subset relation between two placeholder origins. For example,
`fn foo<'a, 'b: 'a>()` would have a relation to `'b: 'a`. This is not
transitive.

	.decl known_placeholder_subset(origin1: Origin, origin2: Origin)
	.input known_placeholder_subset

#### `live_to_dying_regions`

The origins `origin1` and `origin2` are "live to dead"
on the edge `point1 -> point2` if:

- In `point1`, `origin1` <= `origin2`
- In `point2`, `origin1` is live but `origin2` is dead.

In that case, `point2` would like to add all the
live things reachable from `origin2` to `origin1`.

	.decl live_to_dying_regions(origin1: Origin, origin2: Origin, point1: Node, point2: Node)

	live_to_dying_regions(origin1, origin2, point1, point2) :-
	    subset(origin1, origin2, point1),
	    cfg_edge(point1, point2),
	    origin_live_on_entry(origin1, point2),
	    !origin_live_on_entry(origin2, point2).

#### `dying_region_requires`

The `origin` requires `loan`, but the `origin` goes dead
along the edge `point1 -> point2`.

	.decl dying_region_requires(origin: Origin, point1: Node, point2: Node, loan: Loan)

	dying_region_requires(origin, point1, point2, loan) :-
	    requires(origin, loan, point1),
	    !loan_killed_at(loan, point1),
	    cfg_edge(point1, point2),
	    !origin_live_on_entry(origin, point2).

#### `dying_can_reach_origins`

Contains dead origins where we are interested
in computing the transitive closure of things they
can reach.

	.decl dying_can_reach_origins(origin: Origin, point1: Node, point2: Node)

	dying_can_reach_origins(origin2, point1, point2) :-
	    live_to_dying_regions(_, origin2, point1, point2).

#### `dying_can_reach`

Indicates that `origin1`, which is dead
in `point2`, can reach `origin2` in `point1`.

This is effectively the transitive subset
relation, but we try to limit it to origins
that are dying on the edge `point1 -> point2`.

	.decl dying_can_reach(origin1: Origin, origin2: Origin, point1: Node, point2: Node)

	dying_can_reach(origin1, origin2, point1, point2) :-
	    dying_can_reach_origins(origin1, point1, point2),
	    subset(origin1, origin2, point1).

This is the "transitive closure" rule, but
note that we only apply it with the
"intermediate" `origin2` is dead at `point2`.

	dying_can_reach(origin1, origin3, point1, point2) :-
	    dying_can_reach(origin1, origin2, point1, point2),
	    !origin_live_on_entry(origin2, point2),
	    subset(origin2, origin3, point1).

#### `dying_can_reach_live`

Indicates that, along the edge `point1 -> point2`, the dead (in `point2`)
`origin1` can reach the live (in `point2`) `origin2` via a subset
relation. This is a subset of the full `dying_can_reach`
relation where we filter down to those cases where `origin2` is
live in `point2`.

	.decl dying_can_reach_live(origin1: Origin, origin2: Origin, point1: Node, point2: Node)

	dying_can_reach_live(origin1, origin2, point1, point2) :-
	    dying_can_reach(origin1, origin2, point1, point2),
	    origin_live_on_entry(origin2, point2).

#### `dead_borrow_region_can_reach_root`

Indicates a "borrow region" `origin` at `point` which is not live on
entry to `point`.

	.decl dead_borrow_region_can_reach_root(origin: Origin, point: Node, loan: Loan)

	dead_borrow_region_can_reach_root(origin, point, loan) :-
	    loan_issued_at(origin, loan, point),
	    !origin_live_on_entry(origin, point).

#### `dead_borrow_region_can_reach_dead`

	.decl dead_borrow_region_can_reach_dead(origin: Origin, point: Node, loan: Loan)

	dead_borrow_region_can_reach_dead(origin, point, loan) :-
	    dead_borrow_region_can_reach_root(origin, point, loan).

	dead_borrow_region_can_reach_dead(origin2, point, loan) :-
	    dead_borrow_region_can_reach_dead(origin1, point, loan),
	    subset(origin1, origin2, point),
	    !origin_live_on_entry(origin2, point).

#### `subset`

	.decl subset(origin1: Origin, origin2: Origin, point: Node)
	.output subset

	subset(origin1, origin2, point) :- subset_base(origin1, origin2, point).

Carry `origin1 <= origin2` from `point1` into `point2` if both `origin1` and
`origin2` are live in `point2`.

	subset(origin1, origin2, point2) :-
	    subset(origin1, origin2, point1),
	    cfg_edge(point1, point2),
	    origin_live_on_entry(origin1, point2),
	    origin_live_on_entry(origin2, point2).

	subset(origin1, origin3, point2) :-
	    live_to_dying_regions(origin1, origin2, point1, point2),
	    dying_can_reach_live(origin2, origin3, point1, point2).

#### `requires`

	.decl requires(origin: Origin, loan: Loan, point: Node)

Communicate a `origin1 requires loan` relation across
an edge `point1 -> point2` where `origin1` is dead in `point2`; in
that case, for each origin `origin2` live in `point2`
where `origin1 <= origin2` in `point1`, we add `origin2 requires loan`
to `point2`.

	requires(origin2, loan, point2) :-
	    dying_region_requires(origin1, point1, point2, loan),
	    dying_can_reach_live(origin1, origin2, point1, point2).

	requires(origin, loan, point2) :-
	    requires(origin, loan, point1),
	    !loan_killed_at(loan, point1),
	    cfg_edge(point1, point2),
	    origin_live_on_entry(origin, point2).

#### `borrow_live_at`

	.decl borrow_live_at(loan: Loan, point: Node)

	borrow_live_at(loan, point) :-
	  requires(origin, loan, point),
	  origin_live_on_entry(origin, point).

	borrow_live_at(loan, point) :-
	  dead_borrow_region_can_reach_dead(origin1, point, loan),
	  subset(origin1, origin2, point),
	  origin_live_on_entry(origin2, point).

#### `errors`

	.decl errors(loan: Loan, point: Node)
	.output errors

	errors(loan, point) :-
	  loan_invalidated_at(loan, point),
	  borrow_live_at(loan, point).

#### `subset_placeholder`

All subset relationships whose left-hand side is a placeholder origin.

	.decl subset_placeholder(origin1: Origin, origin2: Origin, point: Node)

	subset_placeholder(Origin1, Origin2, Point) :-
	    subset(Origin1, Origin2, Point),
	    placeholder_origin(Origin1).

We compute the transitive closure of the placeholder origins, so we
maintain the invariant from the rule above that `Origin1` is a placeholder origin.

	subset_placeholder(Origin1, Origin3, Point) :-
	    subset_placeholder(Origin1, Origin2, Point),
	    subset(Origin2, Origin3, Point).

#### `subset_errors`

	.decl subset_errors(origin1: Origin, origin2: Origin, point: Node)
	.output subset_errors

	subset_errors(Origin1, Origin2, Point) :-
	    Origin1 != Origin2,
	    subset_placeholder(Origin1, Origin2, Point),
	    placeholder_origin(Origin2),
	    !known_placeholder_subset(Origin1, Origin2).

