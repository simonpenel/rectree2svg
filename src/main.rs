// Temps de calcul en O(n^2) correct pour des arbres inferieur à  1500 feuilles
// Au dela le calcul devient long et l'affichage illisible.
use std::fs;
use std::fs::File;
use std::env;
use std::process;
use getopt::Opt;
use taxonomy::formats::newick;
use taxonomy::Taxonomy;
use webbrowser::{Browser};
mod arena;
use crate::arena::Options;
use crate::arena::ArenaTree;
use crate::arena::taxo2tree;
use crate::arena::xml2tree;
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
    println!("{} -f input file [-o output file][-b][-h][-i][-p][-l][-v]",programe_name);
    println!("    -b : open svg in browser");
    println!("    -p : build a phylogram");
    println!("    -i : display internal gene nodes ");
    println!("    -l : use branch length");
    println!("    -h : help");
    println!("    -v : verbose");
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
    // Initialise Options
    let mut options: Options = Options::new();
    // Gestion des arguments et des options
    // ------------------------------------
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
         display_help(args[0].to_string());
    }
    let mut opts = getopt::Parser::new(&args, "f:o:bhivpl");
    let mut infile = String::new();
    let mut outfile = String::from("tree2svg.svg");
    let mut clado_flag = true;
    let mut open_browser = false;
    let mut real_length_flag = false;
    let mut verbose = false;
    let mut nb_args = 0;
    loop {
        match opts.next().transpose().expect("Unknown option") {
            None => break,
            Some(opt) => match opt {
                Opt('i', None) => options.gene_internal = true,
                Opt('b', None) => open_browser = true,
                Opt('p', None) => clado_flag = false,
                Opt('l', None) => real_length_flag = true,
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
            // Search for the first occurnce of clade tag
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
                // Search for the first gene trees
                let descendants = rgnode.descendants();
                // Search for the first occurnce of clade tag
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
                gene_trees.push(gene_tree);
            }
            let  nb_gntree =  gene_trees.len().clone();
            println!("Number of gene trees : {}",nb_gntree);
            info!("List of gene trees : {:?}",gene_trees);
            // -----------------------
            // Traitement en 10 etapes
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
            if verbose {
                drawing::draw_tree(&mut sp_tree,"verbose-knuth.svg".to_string());
            }
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
            drawing::draw_sptree_gntrees(&mut sp_tree,&mut gene_trees, outfile,options);
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
    if verbose {
        drawing::draw_tree(&mut tree,"verbose-knuth.svg".to_string());
    }
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
    if verbose {
        drawing::draw_tree(&mut tree,"verbose-shifted.svg".to_string());
    }
    // ---------------------------------------------------------
    // 4ème étape : Place le parent entre les enfants
    // ---------------------------------------------------------
    set_middle_postorder(&mut tree, root);
    // ---------------------------------------------------------
    // Option : real_length
    // ---------------------------------------------------------
    if real_length_flag {
        real_length(&mut tree, root, &mut 0.0);
    }
    // ---------------------------------------------------------
    // Fin: Ecriture du fichier svg
    // ---------------------------------------------------------
    println!("Output filename is {}",outfile);
    drawing::draw_tree(&mut tree,outfile);
}
