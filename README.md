# rectree2svg
Build a svg phylogenetic reconciled tree with Rust

Read a recphyloxml file:  create a svg representation of the  gene trees and species tree with events (loss, duplication, speciation, transfer).

Read a newick or phyloxml file:  and create a svg representation of the tree.

Currently under development.

# Under development:
- Accept old recPhyloXML format
- Improve gene transfer representation
- Allow 2/3 reconciliation levels (host/species/gene)
- Passing from O(n^2) to O(n) complexity (not really necessary :  huge trees will no be visible in svg)

# Instructions:
- cargo build --release
- target/release/rectree2svg -h
