/// name = "rectree2svg"
/// version = "0.1.0"
/// authors = ["Simon Penel <simon.penel@univ-lyon1.fr>"]
/// edition = "2021"
///
/// Usage:
/// Draw phylogenetic trees in a svg file.
/// Draw multiple reconciled gene trees with the associated species tree.
/// Draw simple gene or species tree too.
/// Read a newick, phyloxml or recPhyloXML file.
/// Format is guessed according to filename (default is newick).

use std::fs;
use std::env;
use std::process;
use getopt::Opt;
use webbrowser::{Browser};
mod arena;
use crate::arena::Options;
use crate::arena::ArenaTree;
use crate::arena::newick2tree;
use crate::arena::xml2tree;
use crate::arena::check_for_obsolete;
use crate::arena::map_gene_trees;
use crate::arena::map_species_trees;
use crate::arena::bilan_mappings;
use crate::arena::move_dupli_mappings;
use crate::arena::center_gene_nodes;
use crate::arena::set_species_width;
use crate::arena::find_sptree;
use crate::arena::find_rgtrees;
use crate::arena::knuth_layout;
use crate::arena::set_middle_postorder;
use crate::arena::shift_mod_xy;
use crate::arena::check_contour_postorder;
use crate::arena::check_vertical_contour_postorder;
use crate::arena::cladogramme;
use crate::arena::real_length;
mod drawing;
use log::{info};

// Message d'erreur
// ----------------
fn display_help(programe_name:String) {
    const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
    const NAME: Option<&'static str> = option_env!("CARGO_PKG_NAME");
    const DESCRIPTION: Option<&'static str> = option_env!("CARGO_PKG_DESCRIPTION");
    println!("{} v{}", NAME.unwrap_or("unknown"),VERSION.unwrap_or("unknown"));
    println!("{}", DESCRIPTION.unwrap_or("unknown"));
    println!("Usage:");
    println!("{} -f input file [-b][-h][-i][-I][-l factor][-o output file][-p][-s][-v]",programe_name);
    println!("    -b : open svg in browser");
    println!("    -h : help");
    println!("    -i : display internal gene nodes");
    println!("    -I : display internal species nodes");
    println!("    -l factor: use branch length, using the given factor");
    println!("    -o outputfile : set name of output file");
    println!("    -p : build a phylogram");
    println!("    -s : drawing species tree only");
    println!("    -v : verbose");
    println!("");
    println!("    Note on -b option : you must set a browser as default application for opening svg file");
    println!("");
    println!("Input format is guessed according to the file name extension:");

    println!(".xml         => phyloxml");
    println!(".phyloxml    => phyloXML");
    println!(".recphyloxml => recPhyloXML");
    println!(".recPhyloXML => recPhyloXML");
    println!(".recphylo    => recPhyloXML");
    println!("any other    => newick");
    process::exit(1);
}
/// enum of the possible input file Formats
#[derive(Debug)]
pub enum  Format {
    Newick,
    Phyloxml,
    Recphyloxml,
}

fn main()  {
    // Initialise les options
    // Some of the program options needed several functions
    let mut options: Options = Options::new();
    // Gestion des arguments et des options
    // ------------------------------------
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
         display_help(args[0].to_string());
    }
    let mut opts = getopt::Parser::new(&args, "f:l:o:bhiIspv");
    let mut infile = String::new();
    let mut outfile = String::from("tree2svg.svg");
    let mut clado_flag = true;
    let mut species_only_flag = false;
    let mut open_browser = false;
    let mut real_length_flag = false;
    let mut verbose = false;
    let mut nb_args = 0;
    loop {
        match opts.next().transpose().expect("Unknown option") {
            None => break,
            Some(opt) => match opt {
                Opt('i', None) => options.gene_internal = true,
                Opt('I', None) => options.species_internal = true,
                Opt('b', None) => open_browser = true,
                Opt('p', None) => clado_flag = false,
                Opt('s', None) => species_only_flag = true,
                Opt('l', Some(string)) => {
                    real_length_flag = true;
                    options.scale = string.parse().unwrap();
                    },
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
                ".recPhyloXML" => Format::Recphyloxml,
                ".recphylo" => Format::Recphyloxml,
                _ => Format::Newick,
            }
            },
    };
    println!("Assume that format is {:?}",format);
    // Creation d'une structure ArenaTree (pour phyloxml et newick)
    // -----------------------------------------------------------
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
            // Cherche la premiere occurence du clade tag
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
                let contents = fs::read_to_string(filename)
                .expect("Something went wrong reading the newick file");
                let root = tree.new_node("Root".to_string());
                newick2tree(contents, &mut tree, root, &mut 0);
        },
        // Recphyloxml
        Format::Recphyloxml => {
            // On cree une structure Arena pour l'arbre d'espece
            // et un vecteur de  structures Arena pour le(s) arbres de gènes
            // -------------------------------------------------------------
            // Creation de la structure ArenaTree pour l'arbre d'espece
            // --------------------------------------------------------
            let mut sp_tree: ArenaTree<String> = ArenaTree::default();
            println!("The handling of this format is still under development");
            let contents = fs::read_to_string(filename)
                .expect("Something went wrong reading the recphyloxml file");
            let doc = &mut roxmltree::Document::parse(&contents).unwrap();
            // Recupere le NodeId associe au premiere element aavce un tag spTree
            let spnode = find_sptree(doc).expect("No clade spTree has been found in xml");
            // Recupere le Node associe grace ai NodeId
            let spnode = doc.get_node(spnode).expect("Unable to get the Node associated to this nodeId");
            info!("spTree Id: {:?}",spnode);
            let descendants = spnode.descendants();
            // Cherche la premiere occurence du  clade tag
            for node in descendants {
                if node.has_tag_name("clade"){
                    // node est la racine
                    let mut index  = &mut 0;
                    // Nom de la racine
                    let name = "N".to_owned()+&index.to_string();
                    // Cree le nouveau noeud et recupere son index
                    let name = sp_tree.new_node(name.to_string());
                    // Appelle xlm2tree sur la racine
                    xml2tree(node, name, &mut index, &mut sp_tree);
                    // on s'arrête la
                    break;
                }
            }
            if verbose {
                info!("Drawing tree species verbose-species.svg");
                drawing::draw_tree(&mut sp_tree,"verbose-species.svg".to_string(),&options);
            }
            // Creation du vecteur de structure ArenaTree pour les genes
            // ---------------------------------------------------------
            let mut gene_trees:std::vec::Vec<ArenaTree<String>> = Vec::new();
            // Recupere la liste des noeuds associés à la balise  recGeneTree
            let rgnodes = find_rgtrees(doc).expect("No clade recGeneTree has been found in xml");
            for rgnode in rgnodes {
                let mut gene_tree: ArenaTree<String> = ArenaTree::default();
                info!("Search recGeneTree node {:?}",rgnode);
                let rgnode = doc.get_node(rgnode).expect("Unable to get the Node associated to this nodeId");
                info!("Associated recGeneTree  : {:?}",rgnode);
                // Analyse le gene tree
                let descendants = rgnode.descendants();
                // Cherche la premiere occurenvce du clade tag
                for node in descendants {
                    if node.has_tag_name("clade"){
                        // node est la racine
                        let mut index  = &mut 0;
                        // Nom de la racine
                        let name = "N".to_owned()+&index.to_string();
                        // Cree le nouveau noeud et recupere son index
                        let name = gene_tree.new_node(name.to_string());
                        // Appelle xlm2tree sur la racine
                        xml2tree(node, name, &mut index, &mut gene_tree);
                        // on s'arrête la
                        break;
                    }
                }
                // Traitement des balises obsoletes potentielles (ancien format recPhyloXML)
                check_for_obsolete(&mut gene_tree, &mut sp_tree);
                // Ajoute l'arbre de gene
                gene_trees.push(gene_tree);
            }
            let  nb_gntree =  gene_trees.len().clone();
            println!("Number of gene trees : {}",nb_gntree);
            info!("List of gene trees : {:?}",gene_trees);
            // -----------------------
            // Traitement en 12 etapes
            // -----------------------
            // Au depart l'arbre est orienté du haut vers le bas (i.e. selon Y)
            // Le svg sera tourné de -90 a la fin.
            //
            //----------------------------------------------------------
            // 1ere étape :initialisation des x,y de l'arbre d'espèces :
            // profondeur => Y, left => X= 0, right X=1
            // ---------------------------------------------------------
            let  root = sp_tree.get_root();
            knuth_layout(&mut sp_tree,root, &mut 1);
            // --------------------
            // Option : Cladogramme
            // --------------------
            if clado_flag {
                cladogramme(&mut sp_tree);
            }
            // ---------------------------------------------------------
            // 2eme étape :  mapping des genes sur l'espèce pour
            // connaître le nombre de noeuds d'arbre de gènes associés à
            // chaque noeud de l'arbre d'espèces
            // ---------------------------------------------------------
            map_species_trees(&mut sp_tree,&mut gene_trees);
            info!("Species tree after mapping : {:?}",sp_tree);
            // ---------------------------------------------------------
            // 3eme étape : Vérifie les conflits dans l'arbre d'espèces
            // au niveau horizontal -> valeurs xmod
            // ---------------------------------------------------------
             check_contour_postorder(&mut sp_tree, root);
            // ---------------------------------------------------------
            // 4eme étape : Décale toutes les valeurs de x en fonction
            // de xmod dans l'abre d'espèces
            // ---------------------------------------------------------
            shift_mod_xy(&mut sp_tree, root, &mut 0.0, &mut 0.0);
            // ---------------------------------------------------------
            // 5eme étape : Place le parent entre les enfants dans
            // l'arbre d'espèces
            // ---------------------------------------------------------
            set_middle_postorder(&mut sp_tree, root);
            // ---------------------------------------------------------
            // 6ème etape : Fixe l'épaisseur de l'arbre d'espèces
            // ---------------------------------------------------------
            set_species_width(&mut sp_tree);
            // ---------------------------------------------------------
            // 7ème étape :  Vérifie les conflits verticaux dans
            // l'arbre d'espèces
            // ---------------------------------------------------------
            check_vertical_contour_postorder(&mut sp_tree, root, 0.0);
            // ---------------------------------------------------------
            // 8ème étape :  mapping des noeuds de genes sur les noeuds
            // d'espèce pour initialiser les coordonées des noeuds des
            // arbres de gènes
            // ---------------------------------------------------------
            map_gene_trees(&mut sp_tree,&mut gene_trees);
            // ---------------------------------------------------------
            // 9ème etape : décale les noeuds de gene associés à un
            // noeud d'especes pour éviter qu'ils soit superposés
            // ---------------------------------------------------------
            bilan_mappings(&mut sp_tree, &mut gene_trees,root);
            // ---------------------------------------------------------
            // 10ème étape : recalcule les coordonnées svg de tous les
            // arbres de gènes
            // ---------------------------------------------------------
            let  nb_gntree =  gene_trees.len(); // Nombre d'arbres de gene
            info!("map_species_trees: {} gene trees to be processed",nb_gntree);
            let mut idx_rcgen = 0;  // Boucle sur les arbres de genes
            loop {
                let  groot = gene_trees[idx_rcgen].get_root();
                shift_mod_xy(&mut gene_trees[idx_rcgen], groot, &mut 0.0, &mut 0.0);
                idx_rcgen += 1;
                if idx_rcgen == nb_gntree {
                    break;
                }
            }
            // ---------------------------------------------------------
            // 11eme etape : centre les noeuds de genes dans le noeud de l'espece
            // ---------------------------------------------------------
            center_gene_nodes(&mut sp_tree,&mut gene_trees,root);
            // ---------------------------------------------------------
            // 12eme etape traite spécifiquement les duplications et les feuilles
            // ---------------------------------------------------------
            move_dupli_mappings(&mut sp_tree, &mut gene_trees,root);
            // ---------------------------------------------------------
            // Fin: Ecriture du fichier svg
            // ---------------------------------------------------------
            println!("Output filename is {}",outfile);
            let path = env::current_dir().expect("Unable to get current dir");
            let url_file = format!("file:///{}/{}", path.display(),outfile);
            match species_only_flag {
                true => {
                    if options.species_internal {
                         options.gene_internal = true;}
                         drawing::draw_tree(&mut sp_tree, outfile,&options);

                },
                false => drawing::draw_sptree_gntrees(&mut sp_tree,&mut gene_trees, outfile,&options),
            };

            // EXIT
            // On s'arrete la, le reste du programme concerne les autres formats
            if open_browser {
                if webbrowser::open_browser(Browser::Default,&url_file).is_ok() {
                    info!("Browser OK");
                }
            }

            process::exit(0);
        },
    }
    info!("Tree : {:?}",tree);
    // -----------------------
    // Traitement en 4 étapes
    // -----------------------
    // Au départ l'arbre est orienté du haut vers le bas (i.e. selon Y)
    // Le svg sera tourné de -90 a la fin.
    //
    //----------------------------------------------------------
    // 1ère étape :initialisation des x,y de l'arbre :
    // profondeur => Y, left => X= 0, right X=1
    // ---------------------------------------------------------
    let  root = tree.get_root();
    knuth_layout(&mut tree,root, &mut 1);

    // ---------------------------------------------------------
    // Option : Cladogramme
    // ---------------------------------------------------------
    if clado_flag {
        cladogramme(&mut tree);
    }
    // ---------------------------------------------------------
    // 2ème étape : Vérifie les contours
    // ---------------------------------------------------------
     check_contour_postorder(&mut tree, root);
    // ---------------------------------------------------------
    // 3eme etape : Decale toutes les valeurs de x en fonction
    // de xmod
    // ---------------------------------------------------------
    shift_mod_xy(&mut tree, root, &mut 0.0, &mut 0.0);

    // ---------------------------------------------------------
    // 4ème étape : Place le parent entre les enfants
    // ---------------------------------------------------------
    set_middle_postorder(&mut tree, root);
    // ---------------------------------------------------------
    // Option : real_length
    // ---------------------------------------------------------
    if real_length_flag {
        real_length(&mut tree, root, &mut 0.0, & options);
    }
    // ---------------------------------------------------------
    // Fin: Ecriture du fichier svg
    // ---------------------------------------------------------
    println!("Output filename is {}",outfile);

    let path = env::current_dir().expect("Unable to get current dir");
    let url_file = format!("file:///{}/{}", path.display(),outfile);
    drawing::draw_tree(&mut tree,outfile,&options);
    // EXIT
    // On s'arrete la, le reste du programme concerne les autres formats
    if open_browser {
        if webbrowser::open_browser(Browser::Default,&url_file).is_ok() {
            info!("Browser OK");
        }
    }
}
