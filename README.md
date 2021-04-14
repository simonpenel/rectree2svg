# rectree2svg

[![rectree2svg at crates.io](https://img.shields.io/crates/v/rectree2svg.svg)](https://crates.io/crates/rectree2svg)
[![rectree2svg at docs.rs](https://docs.rs/rectree2svg/badge.svg)](https://docs.rs/rectree2svg)
[![rectree2svg at docs.rs](https://github.com/simonpenel/rectree2svg/actions/workflows/rust.yml/badge.svg)](https://github.com/simonpenel/rectree2svg/actions/workflows/rust.yml)
[![rectree2svg at docs.rs](https://github.com/simonpenel/rectree2svg/actions/workflows/example.yml/badge.svg)](https://github.com/simonpenel/rectree2svg/actions/workflows/example.yml)


Read, build, displays as svg and manipulates phylogenetic trees.

Trees must be rooted.

Build a svg representation of a phylogenetic reconciled (or not) tree with events (loss, duplication, speciation, transfer).

Read a recphyloxml file:  create a svg representation of the  gene tree(s) and the associated species tree.

Read a newick or phyloxml file: create a svg representation of the gene tree only .

You may use the rectree2svg binary to draw trees to as well as the rust rectree2svg funtions to build, manipulate or draw phylogenetic trees (see "Using the code")

Keywords:  phylogeny, reconciled trees, phylogenetic trees

# Formats:

phyloXML, recPhyloXML, rooted newick ( NHX balises will not be considered ).

# Output examples

multiple genes reconciliation recphyloxml:

https://raw.githubusercontent.com/simonpenel/rectree2svg/6414f14e57131a590558711b9981aca76decbcbe/tree2svg.example.svg

single gene reconciliation in recphyloxml:

https://raw.githubusercontent.com/simonpenel/rectree2svg/c75848cc0bbcf32aafb619eafdd1237f980abf8a/FAM000696_reconciliated_recphyloxml.svg

the same gene reconciliation in phyloxml:

https://raw.githubusercontent.com/simonpenel/rectree2svg/e784162ea8cb4926d77e5748e74fddae036818b2/FAM000696_reconciliated_xml.svg

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

    rectree2svg -f input file [-b][-c config file][-g #][-h][-i][-I][-l factor][-o output file][-p][-r ratio][-s][-v]

    -b : open svg in browser
    -c configfile: use a configuration file
    -g <n> : display the gene #n in phyloxml style (no species tree)
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

You will find several input file examples in newick_examples, recphylo_examples and xml_examples directories.


# Configuration file:

You may configure some of the features of the svg with the -c option.

The default values are the values of the "config_default.txt" file.

Modify the default values and save it into  "my_config.txt" then type:

```
rectree2svg -f recphylo_examples/FAM000600_reconciliated_big.recphylo -c my_config.txt -b

```


# Using the API:

https://crates.io/crates/rectree2svg

The API functions and methods are tagged as "API" in the Rust package documentation (https://docs.rs/rectree2svg)

 Semantic Versioning Specification applies only to "API" tagged functions and methods.

You may find code  examples in the "examples" directory.

Simple Rust example: read a newick.txt file and creates the svg
```
    use rectree2svg::{ArenaTree,Options,Config,newick2tree,knuth_layout,check_contour_postorder,
                  shift_mod_xy,set_middle_postorder,draw_tree};
    use std::fs;

    fn main() {
        let mut tree: ArenaTree<String> = ArenaTree::default();
        let options: Options = Options::new();
        let config: Config = Config::new();
        let contents = fs::read_to_string("newick.txt")
                .expect("Something went wrong reading the newick file");
        let root = tree.new_node("Root".to_string());
        newick2tree(contents, &mut tree, root, &mut 0);
        knuth_layout(&mut tree,root, &mut 1);
        check_contour_postorder(&mut tree, root);
        shift_mod_xy(&mut tree, root, &mut 0.0, &mut 0.0);
        set_middle_postorder(&mut tree, root);
        draw_tree(&mut tree,"my_svg.svg".to_string(),&options,&config);
    }
```

Some newick examples are available here : https://github.com/simonpenel/rectree2svg/tree/master/newick_examples

Simple Rust example: build a gene tree with a duplication and creates the svg
```
    use rectree2svg::{ArenaTree,Options,Config,Event,knuth_layout,check_contour_postorder,
                  cladogramme,shift_mod_xy,set_middle_postorder,draw_tree};

    fn main() {
    let mut tree: ArenaTree<String> = ArenaTree::default();
    let mut options: Options = Options::new();
    let config: Config = Config::new();
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

    draw_tree(&mut tree,"my_svg.svg".to_string(),&options,&config);
    }
```
# Code Examples

You may try the codes in the 'examples' directory:

    cargo run --example read_newick

    cargo run --example build_tree

    cargo run --example lca

    cargo run --example modify_tree


# Source documentation

See Rust documentation : https://docs.rs/rectree2svg/

# recPhyloXML documentation

See http://phylariane.univ-lyon1.fr/recphyloxml/

recPhyloXML paper: https://www.ncbi.nlm.nih.gov/pmc/articles/PMC6198865/

# phyloXML documentation

See: http://www.phyloxml.org/

phyloXML paper: https://www.ncbi.nlm.nih.gov/pmc/articles/PMC2774328/

# Under development:
- Possible problem with the obsolete version of recPhyloXML format (speciationLoss is supported, speciationOutLoss and speciationOut are not supported yet)
- Allow 2/3 reconciliation levels (host/species/gene)

# Tree drawing algorithms and structures

"Arena" Tree structure  is inspired by the code proposed [here](https://dev.to/deciduously/no-more-tears-no-more-knots-arena-allocated-trees-in-rust-44k6)

Tree drawing algorithms are well explained [here](https://llimllib.github.io/pymag-trees/)  and [here](https://rachel53461.wordpress.com/2014/04/20/algorithm-for-drawing-trees/)

# Licence
CECILL: https://choosealicense.com/licenses/cecill-2.1/
