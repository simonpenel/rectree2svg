# tree2svg
Build a svg phylogenetic tree with Rust

Read a newick or phyloxml file and create a svg representation of the tree.
When reading  phyloxml file, display events as loss, dulplication and speciation.

Currently under development.

# Next steps
- Reading recphyloxml
- Drawing reconciliation (2 /3 levels)
- Passing from O(n^2) to O(n) complexity (not really necessary :  huge trees will no be visible in svg) 

# Instructions:
- cargo build
- target/debug/tree2svg  -h
