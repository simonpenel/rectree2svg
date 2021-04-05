# rectree2svg

Read, build, displays as svg and manipulates phylogenetic trees.

Trees must be rooted.

Build a svg representation of a phylogenetic reconciled (or not) tree

Read a recphyloxml file:  create a svg representation of the  gene trees and species tree with events (loss, duplication, speciation, transfer).

Read a newick or phyloxml file: create a svg representation of the tree.

You may use the rectree2svg binary as well as the rust rectree2svg funtions to build, manipulate or draw phylogenetic trees (see "Using the code")

Keywords:  phylogeny, reconciled trees, phylogenetic trees

Formats: newick, phyloXML, recPhyloXML (nhx will be available on 7th April)

# Output example

https://raw.githubusercontent.com/simonpenel/rectree2svg/6414f14e57131a590558711b9981aca76decbcbe/tree2svg.example.svg

# Install:

rectree2svg is written in Rust. The code is managed using Cargo and published on crates.io. 
Once Cargo is installed just open a terminal and type:

    cargo install rectree2svg

You may as well install from the sources. Clone or download  the sources here https://github.com/simonpenel/rectree2svg and type:

    cargo build --release
    target/release/rectree2svg -h

# Run the binary:
Read a newick, phyloxml or recPhyloXML file and create a svg.

Format is guessed according to filename (default is newick)

Usage:

    rectree2svg -f input file [-b][-h][-i][-I][-l factor][-o output file][-p][-r ratio][-s][-v]

    -b : open svg in browser
    -h : help
    -i : display internal gene nodes
    -I : display internal species nodes
    -l factor: use branch length, using the given factor
    -o outputfile : set name of output file
    -p : build a phylogram   
    -r ratio : set the ratio between width of species and gene tree.
               Default 1.0, you usualy do not need to change it. 
    -s : drawing species tree only
    -v : verbose

`Input format is guessed according to the file name extension:`

    - .xml         => phyloxml
    - .phyloxml    => phyloXML
    - .recphyloxml => recPhyloXML
    - .recPhyloXML => recPhyloXML
    - .recphylo    => recPhyloXML
    - any other    => newick

# Using the code:

https://crates.io/crates/rectree2svg

Simple Rust example: read a newick.txt file and creates the svg

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

Some newick examples are available here : https://github.com/simonpenel/rectree2svg/tree/master/newick_examples

Simple Rust example: build a gene tree with a duplication and creates the svg

    use rectree2svg::{ArenaTree,Options,Event,knuth_layout,check_contour_postorder,
                  cladogramme,shift_mod_xy,set_middle_postorder,draw_tree};

    fn main() {
    let mut tree: ArenaTree<String> = ArenaTree::default();
    let mut options: Options = Options::new();

    // Create a new node root
    let root = tree.new_node("root".to_string());
    // Create  new nodes
    let a1 = tree.new_node("a1".to_string());
    let a2 = tree.new_node("a2".to_string());
    let a = tree.new_node("a".to_string());
    let b = tree.new_node("b".to_string());
    let c = tree.new_node("c".to_string());
    let d = tree.new_node("d".to_string());

    // Set names
    tree.arena[root].name = "MyRoot".to_string();
    tree.arena[a].name = "Gene A".to_string();
    tree.arena[a1].name = "Gene A1".to_string();
    tree.arena[a2].name = "Gene A2".to_string();
    tree.arena[b].name = "Gene B".to_string();
    tree.arena[c].name = "Gene C".to_string();
    tree.arena[d].name = "Gene D".to_string();

    // Set hierarchy
    //  a1 and a2 are children of a
    tree.arena[a1].parent = Some(a);
    tree.arena[a2].parent = Some(a);
    tree.arena[a].children.push(a1);
    tree.arena[a].children.push(a2);
    // a and b are children of c
    tree.arena[a].parent = Some(c);
    tree.arena[b].parent = Some(c);
    tree.arena[c].children.push(a);
    tree.arena[c].children.push(b);
    // c and d are children of root
    tree.arena[c].parent = Some(root);
    tree.arena[d].parent = Some(root);
    tree.arena[root].children.push(c);
    tree.arena[root].children.push(d);

    // set duplication
    tree.arena[a].e = Event::Duplication;

    knuth_layout(&mut tree,root, &mut 1);
    // Display cladogram
    cladogramme(&mut tree);
    check_contour_postorder(&mut tree, root);
    shift_mod_xy(&mut tree, root, &mut 0.0, &mut 0.0);
    set_middle_postorder(&mut tree, root);

    // Display internal nodes
    options.gene_internal = true ;

    draw_tree(&mut tree,"my_svg.svg".to_string(),&options);
    }

# Code Examples

You may try the codes in the 'examples' directory:

    cargo run --example read_newick

    cargo run --example build_tree 
    
    cargo run --example lca
    
    cargo run --example modify_tree 

    

# Source documentation

See Rust documentation : https://docs.rs/rectree2svg/

# RecPhyloXML documentation

See http://phylariane.univ-lyon1.fr/recphyloxml/

# Under development:
- Possible problem with the obsolete version of recPhyloXML format (speciationLoss is supported, speciationOutLoss and speciationOut are not supported yet)
- Allow 2/3 reconciliation levels (host/species/gene)

# Tree drawing algorithms and structures

"Arena" Tree structure  is inspired by the code proposed [here](https://dev.to/deciduously/no-more-tears-no-more-knots-arena-allocated-trees-in-rust-44k6)

Tree drawing algorithms are well explained [here](https://llimllib.github.io/pymag-trees/)  and [here](https://rachel53461.wordpress.com/2014/04/20/algorithm-for-drawing-trees/)

# Licence
CECILL: https://choosealicense.com/licenses/cecill-2.1/
