# Move Paths

## Inputs

#### `path_is_var`

Relates the path to the variable it begins with;
so `path_begins_with_var(a.b, a)` is true, and so
forth.

	.decl path_is_var(path: Path, var: Var)
	.input path_is_var

#### `child_path`

True if Child is a direct "subpath" of Parent.
e.g. `child(a.b, a)` would be true, but not
`child(a, a.b.c)`.

	.decl child_path(parent: Path, child: Path)
	.input child_path

#### `path_assigned_at_base`

Indicates that `Path` is assigned a value
at the point `Node`.

Important: This includes a tuple for each
argument to the function, indicating that it is
initialized on entry.

	.decl path_assigned_at_base(parent: Path, node: Node)
	.input path_assigned_at_base

#### `path_moved_at_base`

Indicates that the value in `Path` is moved
at the point `Node`.

Important: This includes a tuple for each
local variable in the MIR, indicating that it is
"moved" (uninitialized) on entry.

	.decl path_moved_at_base(path: Path, node: Node)
	.input path_moved_at_base

#### `path_accessed_at_base`

Indicates that the value in `Path` is accessed
at the point `Node`.

	.decl path_accessed_at_base(path: Path, node: Node)
	.input path_accessed_at_base

## Relations

#### `parent_path`

The reverse of `child_path`

FIXME: generate this fact directly.

	.decl parent_path(parent: Path, child: Path)

	parent_path(parent, child) :- child_path(child, parent).

#### `ancestor_path`

Computes the transitive closure over paths

	.decl ancestor_path(ancestor: Path, descendant: Path)

	ancestor_path(parent, child) :-
	  parent_path(parent, child).

	ancestor_path(parent, grandchild) :-
	  ancestor_path(parent, child),
	  parent_path(child, grandchild).

#### `path_assigned_at`

	.decl path_assigned_at(path: Path, node: Node)

	path_assigned_at(path, node) :-
	  path_assigned_at_base(path, node).

If you initialize the path `a`, you also initialize `a.b`

	path_assigned_at(childPath, node) :-
	  path_assigned_at(path, node),
	  ancestor_path(path, childPath).

#### `path_moved_at`

	.decl path_moved_at(path: Path, node: Node)

	path_moved_at(path, node) :-
	  path_moved_at_base(path, node).

If you move the path `a`, you also move `a.b`

	path_moved_at(childPath, node) :-
	  path_moved_at(path, node),
	  ancestor_path(path, childPath).

#### `path_accessed_at`

	.decl path_accessed_at(path: Path, node: Node)

	path_accessed_at(path, node) :-
	  path_accessed_at_base(path, node).

If you access the path `a`, you also access `a.b`

	path_accessed_at(childPath, node) :-
	  path_accessed_at(path, node),
	  ancestor_path(path, childPath).

#### `path_begins_with_var`

True if `var` is the base of `path` (e.g. `path_begins_with_var(a.b.c, a)`)

	.decl path_begins_with_var(path: Path, var: Var)

	path_begins_with_var(path, var) :-
	    path_is_var(path, var).

	path_begins_with_var(path, var) :-
	    path_is_var(root, var),
	    ancestor_path(root, path).

