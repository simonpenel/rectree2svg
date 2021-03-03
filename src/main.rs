// Temps de calcul en O(n^2) correct pour des arbres inferieur à  1500 feuilles
// Au dela le calcul devient long et l'affichage illisible.
use std::fs;
use std::fs::File;
use std::env;
use std::process;
use getopt::Opt;
use taxonomy::formats::newick;
// use taxonomy::formats::phyloxml;
use taxonomy::Taxonomy;
mod arena;
use crate::arena::ArenaTree;
use crate::arena::taxo2tree;
use crate::arena::xml2tree;
use crate::arena::find_first_clade;
use crate::arena::find_spTree;
use crate::arena::knuth_layout;
use crate::arena::set_middle_postorder;
use crate::arena::shift_mod_x;
use crate::arena::check_contour_postorder;
use crate::arena::cladogramme;
mod drawing;
use log::{info};


// Message d'erreur
// ----------------
fn display_help(programe_name:String) {
    const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
    const NAME: Option<&'static str> = option_env!("CARGO_PKG_NAME");
    const DESCRIPTION: Option<&'static str> = option_env!("CARGO_PKG_DESCRIPTION");
// ...

    println!("{} v{}", NAME.unwrap_or("unknown"),VERSION.unwrap_or("unknown"));
    println!("{}", DESCRIPTION.unwrap_or("unknown"));
    println!("Usage:");
    println!("{} -f input file [-o output file][-h][-p][-v]",programe_name);
    println!("    -p : build a phylogram");
    println!("    -h : help");
    println!("    -v : verbose");
    process::exit(1);
}

#[derive(Debug)]
enum  Format {
    Newick,
    Phyloxml,
    Recphyloxml,
}

fn main()  {
    // Gestion des arguments et des options
    // ------------------------------------
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
         display_help(args[0].to_string());
    }
    let mut opts = getopt::Parser::new(&args, "f:o:hvp");
    let mut infile = String::new();
    let mut outfile = String::from("tree2svg.svg");
    let mut clado_flag = true;
    let mut verbose = false;
    let mut nb_args = 0;
    loop {
        match opts.next().transpose().expect("Unknown option") {
            None => break,
            Some(opt) => match opt {
                Opt('p', None) => clado_flag = false,
                Opt('v', None) => {
                    verbose = true;
                    env::set_var("RUST_LOG", "info");
                    env_logger::init();
                    info!("Verbosity set to Info");
                    },
                Opt('f', Some(string)) => {
                    infile = string.clone();
                    nb_args += 1;
                    },
                Opt('o', Some(string)) => outfile = string.clone(),
                Opt('h', None) => display_help(args[0].to_string()),
                _ => unreachable!(),
            }
        }
    }
    if args.len() == 1 {
         display_help(args[0].to_string());
    }
    if nb_args != 1 {
         display_help(args[0].to_string());
    }

    //  Determination du format
    //  ------------------------
    let filename = &infile.clone();
    info!("Input filename is {}",filename);
    let dot = filename.rfind('.');
    let format = match dot {
        None => Format::Newick,
        Some(dot) => {
            let suffix = &filename[dot..];
            info!("File suffix is {:?}",suffix);
            match suffix {
                ".xml" => Format::Phyloxml,
                ".phyloxml" => Format::Phyloxml,
                ".recphyloxml" => Format::Recphyloxml,
                ".recphylo" => Format::Recphyloxml,
                _ => Format::Newick,
            }
            },
    };
    info!("Format = {:?}",format);


    // Ouverture  du fichier
    // ----------------------
    let  f = File::open(filename);
    let  mut file = match f {
            Ok(file) => {
                info!("File exists");
                file},
            Err(error) => {
                eprintln!("Unable to read {}",filename);
                eprintln!("Error: {}", error);
                process::exit(1);
            }
    };

    // Creation de la structure ArenaTree
    // ---------------------------------
    let mut tree: ArenaTree<String> = ArenaTree::default();

    // Charge l'arbre selon le format de fichier
    //  ----------------------------------------

    match format {
        // Phymxoml
        Format::Phyloxml => {
            let contents = fs::read_to_string(filename)
                .expect("Something went wrong reading the phyloxml file");
            let doc = roxmltree::Document::parse(&contents).unwrap();
            let descendants = doc.root().descendants();
            // Search for the first occurnce of clade tag
            for node in descendants {
                if node.has_tag_name("clade"){
                    // node est la racine
                    let mut index  = &mut 0;
                    // Nom de la racine
                    let name = "N".to_owned()+&index.to_string();
                    // Cree le nouveau noeud et recupere son index
                    let name = tree.new_node(name.to_string());
                    // Appelle xlm2tree sur la racine
                    xml2tree(node, name, &mut index, &mut tree);
                    // on s'arrête la
                    break;
                }
            }
        },
        // Newick
        Format::Newick => {
            // Stocke l'arbre dans une structure GeneralTaxonomy
            let taxo = newick::load_newick(&mut file);
            let taxo = match taxo {
                Ok(taxo) => {
                    info!("File is ok");
                    taxo},
                Err(error) => {
                    panic!("Something went wrong reading the newick file : {:?}", error);
                }
            };
            info!("taxonomy : {:?}",taxo);
            // Stocke l'arbre dans une structure ArenaTree
            let racine: &str = taxo.root();
            let racine_tid = taxo.to_internal_id(racine).expect("Pas de racine");
            let children = taxo.children(racine_tid).expect("Pas de fils");
            for child in children {
                taxo2tree(& taxo, child,  &mut tree);
            }
        },
        // Recphyloxml
        Format::Recphyloxml => {
            // Attention dance cas, on cree une structure Arena pour l'arbre d'espece
            // et un vecteur de  structures Arena poru le(s) arbres de gènes

            // Creation de la structure ArenaTree pour l'espece
            // ------------------------------------------------
            let mut sptree: ArenaTree<String> = ArenaTree::default();

            // let mut gntrees: Vec<ArenaTree<String>> = ArenaTree::default();

            println!("This format is not yet supported");
            let contents = fs::read_to_string(filename)
                .expect("Something went wrong reading the recphyloxml file");
            let doc = &mut roxmltree::Document::parse(&contents).unwrap();
            // Recupere le NodeId associe au premiere element aavce un tag spTree
            let spnode = find_spTree(doc).expect("No clade spTree has been found in xml");
            // Recupere le Node associe grace ai NodeId
            let spnode = doc.get_node(spnode).expect("Unable to get the Node associated to this nodeId");
            println!("spTree Id : {:?}",spnode);

            let descendants = spnode.descendants();
            // Search for the first occurnce of clade tag
            for node in descendants {
                if node.has_tag_name("clade"){
                    // node est la racine
                    let mut index  = &mut 0;
                    // Nom de la racine
                    let name = "N".to_owned()+&index.to_string();
                    // Cree le nouveau noeud et recupere son index
                    let name = sptree.new_node(name.to_string());
                    // Appelle xlm2tree sur la racine
                    xml2tree(node, name, &mut index, &mut sptree);
                    // on s'arrête la
                    break;
                }
            }

            // Calcule les coordondees de l'arbre d'espece

            // 1ere etape : profondeur => Y, left => X= 0, right X=1
            // ======================================================
            let  root = sptree.get_root();
            knuth_layout(&mut sptree,root, &mut 1);
            if verbose {
                drawing::draw_tree(&mut sptree,"verbose-knuth.svg".to_string());
            }

            // Cladogramme
            // ===========
            cladogramme(&mut sptree);

            // 2eme etape : Verifie les contours
            // ==================================
             check_contour_postorder(&mut sptree, root);

             // 3eme etape : Decale toutes les valeurs de x en fonction de xmod
            // ===============================================================
            shift_mod_x(&mut sptree, root, &mut 0.0);
            if verbose {
                drawing::draw_tree(&mut sptree,"verbose-shifted.svg".to_string());
            }
            // 4eme etape : Place le parent entre les enfants
            // ==============================================
            set_middle_postorder(&mut sptree, root);

            info!("Output filename is {}",outfile);
            drawing::draw_tree(&mut sptree,outfile);

            // On s'arrete la, lereste du programme concerne les autres formats
            process::exit(0);
        },
    }
    info!("Tree : {:?}",tree);
    // Calcul des coordonees x y
    // =========================

    // 1ere etape : profondeur => Y, left => X= 0, right X=1
    // ======================================================
    let  root = tree.get_root();
    knuth_layout(&mut tree,root, &mut 1);
    if verbose {
        drawing::draw_tree(&mut tree,"verbose-knuth.svg".to_string());
    }

    // Option : Cladogramme
    // ====================
    if clado_flag {
        cladogramme(&mut tree);
    }

    // 2eme etape : Verifie les contours
    // ==================================
     check_contour_postorder(&mut tree, root);

    // 3eme etape : Decale toutes les valeurs de x en fonction de xmod
    // ===============================================================
    shift_mod_x(&mut tree, root, &mut 0.0);
    if verbose {
        drawing::draw_tree(&mut tree,"verbose-shifted.svg".to_string());
    }
    // 4eme etape : Place le parent entre les enfants
    // ==============================================
    set_middle_postorder(&mut tree, root);

    info!("Output filename is {}",outfile);
    drawing::draw_tree(&mut tree,outfile);

}
