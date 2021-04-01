# rectree2svg
Build a svg representation of a phylogenetic reconciled (or not) tree

Keywords:  phylogeny, reconciled trees, phylogenetic trees

Read a recphyloxml file:  create a svg representation of the  gene trees and species tree with events (loss, duplication, speciation, transfer).

Read a newick or phyloxml file: create a svg representation of the tree.

You may use the rectree2svg binary as well as the rust rectree2svg funtions (see "Using the code")

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

# Using the code

Simple Rust example: read a newick.txt file and create the svg

    use rectree2svg::{ArenaTree,Options,newick2tree,knuth_layout,check_contour_postorder,
                  shift_mod_xy,set_middle_postorder,draw_tree};
    use std::fs;
    
    fn main() {
        let mut tree: ArenaTree<String> = ArenaTree::default();
        let options: Options = Options::new();
        let contents = fs::read_to_string("newick.txt")
                .expect("Something went wrong reading the newick file");
        let root = tree.new_node("Root".to_string());
        newick2tree(contents, &mut tree, root, &mut 0);
        knuth_layout(&mut tree,root, &mut 1);
        check_contour_postorder(&mut tree, root);
        shift_mod_xy(&mut tree, root, &mut 0.0, &mut 0.0);
        set_middle_postorder(&mut tree, root);
        draw_tree(&mut tree,"my_svg.svg".to_string(),&options);
    }

# Source documentation

See Rust documentation : https://docs.rs/rectree2svg/

# RecPhyloXML documentation

See http://phylariane.univ-lyon1.fr/recphyloxml/

# Tree drawing algorithms and structures

"Arena" Tree structure  is inspired by the code proposed [here](https://dev.to/deciduously/no-more-tears-no-more-knots-arena-allocated-trees-in-rust-44k6)

Tree drawing algorithms are well explained [here](https://llimllib.github.io/pymag-trees/)  and [here](https://rachel53461.wordpress.com/2014/04/20/algorithm-for-drawing-trees/)
