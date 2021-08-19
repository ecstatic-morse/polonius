# Loans

## Inputs

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

	.decl placeholder(o: Origin, loan: Loan)
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

## Relations

#### `loan_live_on_entry`

	.decl loan_live_on_entry(loan: Loan, node: Node)

	loan_live_on_entry(loan, node) :-
	  origin_contains_loan_on_entry(origin, loan, node),
	  (origin_live_on_entry(origin, node); placeholder_origin(origin)).

#### `subset`

	.decl subset(origin1: Origin, origin2: Origin, node: Node)

	subset(origin1, origin2, node) :-
	  subset_base(origin1, origin2, node).

	subset(origin1, origin3, node) :-
	  subset(origin1, origin2, node),
	  subset(origin2, origin3, node).

	subset(origin1, origin2, targetNode) :-
	  subset(origin1, origin2, sourceNode),
	  cfg_edge(sourceNode, targetNode),
	  (origin_live_on_entry(origin1, targetNode); placeholder_origin(origin1)),
	  (origin_live_on_entry(origin2, targetNode); placeholder_origin(origin2)).

#### `origin_contains_loan_on_entry`

Formerly `requires`

	.decl origin_contains_loan_on_entry(origin: Origin, loan: Loan, node: Node)

	origin_contains_loan_on_entry(origin, loan, node) :-
	  loan_issued_at(origin, loan, node).

	origin_contains_loan_on_entry(origin2, loan, node) :-
	  origin_contains_loan_on_entry(origin1, loan, node),
	  subset(origin1, origin2, node).

	origin_contains_loan_on_entry(origin, loan, targetNode) :-
	  origin_contains_loan_on_entry(origin, loan, sourceNode),
	  !loan_killed_at(loan, sourceNode),
	  cfg_edge(sourceNode, targetNode),
	  (origin_live_on_entry(origin, targetNode); placeholder_origin(origin)).

#### `loan_live_at`

Formerly `borrow_live_at`

	.decl loan_live_at(loan: Loan, node: Node)

	loan_live_at(loan, node) :-
	  origin_contains_loan_on_entry(origin, loan, node),
	  origin_live_on_entry(origin, node).

## Error reporting

#### `errors`

	.decl errors(l: Loan, n: Node)
	.output errors

	errors(loan, node) :-
	   loan_invalidated_at(loan, node),
	   loan_live_at(loan, node).

#### `subset_errors`

	.decl subset_errors(origin1: Origin, origin2: Origin, node: Node)
	.output subset_errors

	subset_errors(origin1, origin2, node) :-
	  subset(origin1, origin2, node),
	  placeholder_origin(origin1),
	  placeholder_origin(origin2),
	  !known_placeholder_subset(origin1, origin2).

