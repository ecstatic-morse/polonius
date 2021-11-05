#!/bin/sh

#set -eu

cd ${1:-.}

cp borrow_region.facts loan_issued_at.facts
cp killed.facts loan_killed_at.facts

awk -v OFS='\t' '{ print $2, $1 }' invalidates.facts >loan_invalidated_at.facts

awk -v OFS='\t' '{ print $2, $1 }' child_path.facts >parent_path.facts

cp known_subset.facts known_placeholder_subset.facts
cp outlives.facts subset_base.facts

# Record the basic block and statement for all CFG nodes. `Start` nodes are
# even, `Mid` nodes are odd.
gawk -v OFS='\t' '
{
    for(i=1; i<3; i++) {
        match($i, /(Start|Mid)\(bb([0-9]+)\[([0-9]+)\]\)/, m);
        print $i, "\"bb"m[2]"\"", (m[1] == "Start") ? int(m[3])*2 : int(m[3])*2+1
    }
}' cfg_edge.facts | sort -u -o node_loc.facts

# Record the outgoing edges for each basic block
gawk -v OFS='\t' '
match($2, /Start\(bb([0-9]+)\[0\]/, dest) {
    match($1, /bb([0-9]+)/, src);
    print "\"bb"src[1]"\"", "\"bb"dest[1]"\""
}' cfg_edge.facts > bb_edge.facts
