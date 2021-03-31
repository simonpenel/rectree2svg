# rectree2svg
Build a svg phylogenetic reconciled tree with Rust

Keywords:  phylogeny, reconciled trees, phylogenetic trees

Read a recphyloxml file:  create a svg representation of the  gene trees and species tree with events (loss, duplication, speciation, transfer).

Read a newick or phyloxml file: create a svg representation of the tree.

Currently under development.

# Output example 
https://raw.githubusercontent.com/simonpenel/rectree2svg/6414f14e57131a590558711b9981aca76decbcbe/tree2svg.example.svg

# Under development:
- Accepting old recPhyloXML format
- Allow 2/3 reconciliation levels (host/species/gene)

# Instructions:
- cargo build --release
- target/release/rectree2svg -h

# Help:
Read a newick, phyloxml or recPhyloXML file and create a svg.

Format is guessed according to filename (default is newick)

Usage:

target/debug/rectree2svg -f input file [-b][-h][-i][-I][-l factor][-o output file][-p][-s][-v]

    -b : open svg in browser
    -h : help
    -i : display internal gene nodes
    -I : display internal species nodes
    -l factor: use branch length, using the given factor
    -o outputfile : set name of output file
    -p : build a phylogram
    -s : drawing species tree only
    -v : verbose
