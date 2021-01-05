#!/bin/sh

set -eu

cd ${1:-.}

cp borrow_region.facts loan_issued_at.facts
cp killed.facts loan_killed_at.facts

awk '{ print $2"\t"$1 }' invalidates.facts >loan_invalidated_at.facts

awk '{ print $2"\t"$1 }' child_path.facts >parent_path.facts

cp known_subset.facts known_placeholder_subset.facts
cp outlives.facts subset_base.facts
