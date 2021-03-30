# rectree2svg
Build a svg phylogenetic reconciled tree with Rust

Read a recphyloxml file:  create a svg representation of the  gene trees and species tree with events (loss, duplication, speciation, transfer).

Read a newick or phyloxml file:  and create a svg representation of the tree.

Currently under development.

# Under development:
- Accepting old recPhyloXML format
- Allow 2/3 reconciliation levels (host/species/gene)

# Instructions:
- cargo build --release
- target/release/rectree2svg -h
