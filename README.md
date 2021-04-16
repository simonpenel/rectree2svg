# rectree2svg

[![rectree2svg at crates.io](https://img.shields.io/crates/v/rectree2svg.svg)](https://crates.io/crates/rectree2svg)



Build a svg representation of a phylogenetic reconciled (or not) tree with events (loss, duplication, speciation, transfer).

Read a recphyloxml file:  create a svg representation of the  gene tree(s) and the associated species tree.

Read a newick (rooted tree only) or phyloxml file: create a svg representation of the gene tree only .


Keywords:  phylogeny, reconciled trees, phylogenetic trees

# Formats:

phyloXML, recPhyloXML, rooted newick ( NHX balises will not be considered ).

# Output examples

multiple genes reconciliation recphyloxml:

https://raw.githubusercontent.com/simonpenel/rectree2svg/9244f3136961f909fd7b33818f0a220e3f32c880/tree2svg.example.svg


single gene reconciliation in recphyloxml:

https://raw.githubusercontent.com/simonpenel/rectree2svg/9244f3136961f909fd7b33818f0a220e3f32c880/FAM000696_reconciliated_recphyloxml.svg

the same gene reconciliation in phyloxml:

https://raw.githubusercontent.com/simonpenel/rectree2svg/9244f3136961f909fd7b33818f0a220e3f32c880/FAM000696_reconciliated_xml.svg


# Install:

rectree2svg is written in Rust. The code is managed using Cargo and published on crates.io.

Install cargo:

    curl https://sh.rustup.rs -sSf | sh

or for Windows see  https://doc.rust-lang.org/cargo/getting-started/installation.html

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
    -L : display as landscape
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


rectree2svg use the light_phylogeny crate:

https://crates.io/crates/light_phylogeny


# Source documentation

See Rust documentation : https://docs.rs/rectree2svg/

# recPhyloXML documentation

See http://phylariane.univ-lyon1.fr/recphyloxml/

recPhyloXML paper: https://www.ncbi.nlm.nih.gov/pmc/articles/PMC6198865/

# phyloXML documentation

See: http://www.phyloxml.org/

phyloXML paper: https://www.ncbi.nlm.nih.gov/pmc/articles/PMC2774328/

# Under development:

- Allow 2/3 reconciliation levels (host/species/gene)



# Licence
CECILL: https://choosealicense.com/licenses/cecill-2.1/
