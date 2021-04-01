# rectree2svg
Build a svg representation of a phylogenetic reconciled (or not) tree

Keywords:  phylogeny, reconciled trees, phylogenetic trees

Read a recphyloxml file:  create a svg representation of the  gene trees and species tree with events (loss, duplication, speciation, transfer).

Read a newick or phyloxml file: create a svg representation of the tree.

Currently under development.

# Output example
https://raw.githubusercontent.com/simonpenel/rectree2svg/6414f14e57131a590558711b9981aca76decbcbe/tree2svg.example.svg

# Under development:
- Possible problem with obsolete recPhyloXML format (speciationLoss is supported , speciationOutLoss and speciationOut are not supported yet)
- Allow 2/3 reconciliation levels (host/species/gene)

# Instructions:
- cargo build --release
- target/release/rectree2svg -h

# Help:
Read a newick, phyloxml or recPhyloXML file and create a svg.

Format is guessed according to filename (default is newick)

Usage:

target/release/rectree2svg -f input file [-b][-h][-i][-I][-l factor][-o output file][-p][-s][-v]

    -b : open svg in browser
    -h : help
    -i : display internal gene nodes
    -I : display internal species nodes
    -l factor: use branch length, using the given factor
    -o outputfile : set name of output file
    -p : build a phylogram
    -s : drawing species tree only
    -v : verbose

`Input format is guessed according to the file name extension:`

    - .xml         => phyloxml
    - .phyloxml    => phyloXML
    - .recphyloxml => recPhyloXML
    - .recPhyloXML => recPhyloXML
    - .recphylo    => recPhyloXML
    - any other    => newick

# Source documentation

See Rust documentation : https://docs.rs/rectree2svg/

# RecPhyloXML documentation

See http://phylariane.univ-lyon1.fr/recphyloxml/

# Tree drawing algorithms and sructures

"Arena" Tree structure  is inspired by the code proposed [here](https://dev.to/deciduously/no-more-tears-no-more-knots-arena-allocated-trees-in-rust-44k6)

Tree drawing algorithms are well explained [here](https://llimllib.github.io/pymag-trees/)  and [here](https://rachel53461.wordpress.com/2014/04/20/algorithm-for-drawing-trees/)
