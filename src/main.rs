/// name = "rectree2svg"
/// version = "0.5.0"
/// authors = ["Simon Penel <simon.penel@univ-lyon1.fr>"]
/// release = "04/04/2021"
/// license = "CECILL-2.1"
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
use crate::arena::Event;
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
    println!("{} -g gene-parasite -f parasite-host [-b][-h][-i][-I][-l factor][-o output file][-p][-r ratio][-s][-v]",programe_name);
    println!("    -b : open svg in browser");
    println!("    -h : help");
    println!("    -i : display internal gene nodes");
    println!("    -I : display internal species nodes");
    println!("    -l factor : use branch length, using the given factor (default 1.0)");
    println!("    -o outputfile : set name of output file");
    println!("    -p : build a phylogram");
    println!("    -r ratio : set the ratio between width of species and gene tree.");
    println!("               Default 1.0, you usualy do not need to change it. ");

    println!("    -s : drawing species tree only");
    println!("    -v : verbose");
    println!("");
    println!("    Note on -b option : you must set a browser as default application for opening svg file");
    println!("");
    println!("Input format is recPhyloXML.");
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
    let mut opts = getopt::Parser::new(&args, "f:g:l:o:bhiIsr:pv");
    let mut infile_gene_para = String::new();
    let mut infile_para_host = String::new();
    let mut outfile_gene_para = String::from("gene_para.svg");
    let mut outfile_para = String::from("para.svg");
    let mut outfile_para_host = String::from("para_host.svg");
    let mut outfile_host = String::from("host.svg");
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
                Opt('r', Some(string)) => {
                    options.ratio = string.parse().unwrap();
                    },
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
                    infile_para_host = string.clone();
                    nb_args += 1;
                    },
                Opt('g', Some(string)) => {
                    infile_gene_para = string.clone();
                    nb_args += 1;
                        },
                Opt('o', Some(string)) => {},
                Opt('h', None) => display_help(args[0].to_string()),
                _ => unreachable!(),
            }
        }
    }
    if args.len() == 1 {
         display_help(args[0].to_string());
    }
    if nb_args != 2 {
         display_help(args[0].to_string());
    }
    //  Determination du format
    //  ------------------------
    // let filename1 = &infile1.clone();
    // info!("Input filename 1 is {}",filename1);
    // let filename2 = &infile2.clone();
    // info!("Input filename 2 is {}",filename2);

            // On cree une structure Arena pour l'arbre d'espece
            // et un vecteur de  structures Arena pour le(s) arbres de gènes
            // -------------------------------------------------------------
            // Creation de la structure ArenaTree pour l'arbre d'espece
            // --------------------------------------------------------
            let mut tree_para_pipe: ArenaTree<String> = ArenaTree::default();
            let contents = fs::read_to_string(infile_gene_para)
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
                    let name = tree_para_pipe.new_node(name.to_string());
                    // Appelle xlm2tree sur la racine
                    xml2tree(node, name, &mut index, &mut tree_para_pipe);
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
                check_for_obsolete(&mut gene_tree, &mut tree_para_pipe);
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
            let  root = tree_para_pipe.get_root();
            knuth_layout(&mut tree_para_pipe,root, &mut 1);
            // --------------------
            // Option : Cladogramme
            // --------------------
            if clado_flag {
                cladogramme(&mut tree_para_pipe);
            }
            // ---------------------------------------------------------
            // 2eme étape :  mapping des genes sur l'espèce pour
            // connaître le nombre de noeuds d'arbre de gènes associés à
            // chaque noeud de l'arbre d'espèces
            // ---------------------------------------------------------
            map_species_trees(&mut tree_para_pipe,&mut gene_trees);
            info!("Species tree after mapping : {:?}",tree_para_pipe);
            // ---------------------------------------------------------
            // 3eme étape : Vérifie les conflits dans l'arbre d'espèces
            // au niveau horizontal -> valeurs xmod
            // ---------------------------------------------------------
             check_contour_postorder(&mut tree_para_pipe, root);
            // ---------------------------------------------------------
            // 4eme étape : Décale toutes les valeurs de x en fonction
            // de xmod dans l'abre d'espèces
            // ---------------------------------------------------------
            shift_mod_xy(&mut tree_para_pipe, root, &mut 0.0, &mut 0.0);
            // ---------------------------------------------------------
            // 5eme étape : Place le parent entre les enfants dans
            // l'arbre d'espèces
            // ---------------------------------------------------------
            set_middle_postorder(&mut tree_para_pipe, root);
            // ---------------------------------------------------------
            // 6ème etape : Fixe l'épaisseur de l'arbre d'espèces
            // ---------------------------------------------------------
            set_species_width(&mut tree_para_pipe);
            // ---------------------------------------------------------
            // 7ème étape :  Vérifie les conflits verticaux dans
            // l'arbre d'espèces
            // ---------------------------------------------------------
            check_vertical_contour_postorder(&mut tree_para_pipe, root, 0.0);
            // ---------------------------------------------------------
            // 8ème étape :  mapping des noeuds de genes sur les noeuds
            // d'espèce pour initialiser les coordonées des noeuds des
            // arbres de gènes
            // ---------------------------------------------------------
            map_gene_trees(&mut tree_para_pipe,&mut gene_trees);
            // ---------------------------------------------------------
            // 9ème etape : décale les noeuds de gene associés à un
            // noeud d'especes pour éviter qu'ils soit superposés
            // ---------------------------------------------------------
            bilan_mappings(&mut tree_para_pipe, &mut gene_trees,root, & options);
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
            center_gene_nodes(&mut tree_para_pipe,&mut gene_trees,root);
            // ---------------------------------------------------------
            // 12eme etape traite spécifiquement les duplications et les feuilles
            // ---------------------------------------------------------
            move_dupli_mappings(&mut tree_para_pipe, &mut gene_trees,root);
            // ---------------------------------------------------------
            // Fin: Ecriture du fichier svg
            // ---------------------------------------------------------
            // println!("Output filename is {}",outfile);
            let path = env::current_dir().expect("Unable to get current dir");
            let url_file = format!("file:///{}/{}", path.display(),outfile_gene_para);
            let url_spfile = format!("file:///{}/{}", path.display(),outfile_para);

           drawing::draw_tree(&mut tree_para_pipe, outfile_para, & options);

           drawing::draw_sptree_gntrees(&mut tree_para_pipe,&mut gene_trees, outfile_gene_para,
                    & options);


            // EXIT
            // On s'arrete la, le reste du programme concerne les autres formats
            if open_browser {
                if webbrowser::open_browser(Browser::Default,&url_file).is_ok() {
                    info!("Browser OK");
                }
                if webbrowser::open_browser(Browser::Default,&url_spfile).is_ok() {
                    info!("Browser OK");
                }
            }

            // On cree une structure Arena pour l'arbre d'espece
            // et un vecteur de  structures Arena pour le(s) arbres de gènes
            // -------------------------------------------------------------
            // Creation de la structure ArenaTree pour l'arbre d'espece
            // --------------------------------------------------------
            let mut tree_host_pipe: ArenaTree<String> = ArenaTree::default();
            let contents = fs::read_to_string(infile_para_host)
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
                    let name = tree_host_pipe.new_node(name.to_string());
                    // Appelle xlm2tree sur la racine
                    xml2tree(node, name, &mut index, &mut tree_host_pipe);
                    // on s'arrête la
                    break;
                }
            }

            // Creation du vecteur de structure ArenaTree pour les genes
            // ---------------------------------------------------------
            let mut para_trees:std::vec::Vec<ArenaTree<String>> = Vec::new();
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
                check_for_obsolete(&mut gene_tree, &mut tree_host_pipe);
                // Ajoute l'arbre de gene
                para_trees.push(gene_tree);
            }
            let  nb_paratree =  para_trees.len().clone();
            println!("Number of gene trees : {}",nb_paratree);
            info!("List of gene trees : {:?}",para_trees);
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
            let  root = tree_host_pipe.get_root();
            knuth_layout(&mut tree_host_pipe,root, &mut 1);
            // --------------------
            // Option : Cladogramme
            // --------------------
            if clado_flag {
                cladogramme(&mut tree_host_pipe);
            }
            // ---------------------------------------------------------
            // 2eme étape :  mapping des genes sur l'espèce pour
            // connaître le nombre de noeuds d'arbre de gènes associés à
            // chaque noeud de l'arbre d'espèces
            // ---------------------------------------------------------
            map_species_trees(&mut tree_host_pipe,&mut para_trees);
            info!("Species tree after mapping : {:?}",tree_host_pipe);
            // ---------------------------------------------------------
            // 3eme étape : Vérifie les conflits dans l'arbre d'espèces
            // au niveau horizontal -> valeurs xmod
            // ---------------------------------------------------------
             check_contour_postorder(&mut tree_host_pipe, root);
            // ---------------------------------------------------------
            // 4eme étape : Décale toutes les valeurs de x en fonction
            // de xmod dans l'abre d'espèces
            // ---------------------------------------------------------
            shift_mod_xy(&mut tree_host_pipe, root, &mut 0.0, &mut 0.0);
            // ---------------------------------------------------------
            // 5eme étape : Place le parent entre les enfants dans
            // l'arbre d'espèces
            // ---------------------------------------------------------
            set_middle_postorder(&mut tree_host_pipe, root);
            // ---------------------------------------------------------
            // 6ème etape : Fixe l'épaisseur de l'arbre d'espèces
            // ---------------------------------------------------------
            set_species_width(&mut tree_host_pipe);
            // ---------------------------------------------------------
            // 7ème étape :  Vérifie les conflits verticaux dans
            // l'arbre d'espèces
            // ---------------------------------------------------------
            check_vertical_contour_postorder(&mut tree_host_pipe, root, 0.0);
            // ---------------------------------------------------------
            // 8ème étape :  mapping des noeuds de genes sur les noeuds
            // d'espèce pour initialiser les coordonées des noeuds des
            // arbres de gènes
            // ---------------------------------------------------------
            map_gene_trees(&mut tree_host_pipe,&mut para_trees);
            // ---------------------------------------------------------
            // 9ème etape : décale les noeuds de gene associés à un
            // noeud d'especes pour éviter qu'ils soit superposés
            // ---------------------------------------------------------
            bilan_mappings(&mut tree_host_pipe, &mut para_trees,root, & options);
            // ---------------------------------------------------------
            // 10ème étape : recalcule les coordonnées svg de tous les
            // arbres de gènes
            // ---------------------------------------------------------
            let  nb_paratree =  para_trees.len(); // Nombre d'arbres de gene
            info!("map_species_trees: {} para trees to be processed",nb_paratree);
            let mut idx_rcgen = 0;  // Boucle sur les arbres de genes
            loop {
                let  groot = para_trees[idx_rcgen].get_root();
                shift_mod_xy(&mut para_trees[idx_rcgen], groot, &mut 0.0, &mut 0.0);
                idx_rcgen += 1;
                if idx_rcgen == nb_paratree {
                    break;

                }
            }
            // ---------------------------------------------------------
            // 11eme etape : centre les noeuds de genes dans le noeud de l'espece
            // ---------------------------------------------------------
            center_gene_nodes(&mut tree_host_pipe,&mut para_trees,root);
            // ---------------------------------------------------------
            // 12eme etape traite spécifiquement les duplications et les feuilles
            // ---------------------------------------------------------
            move_dupli_mappings(&mut tree_host_pipe, &mut para_trees,root);
            // ---------------------------------------------------------
            // Fin: Ecriture du fichier svg
            // ---------------------------------------------------------
            // println!("Output filename is {}",outfile);
            let path = env::current_dir().expect("Unable to get current dir");
            let url_file = format!("file:///{}/{}", path.display(),outfile_para_host);
            let url_spfile = format!("file:///{}/{}", path.display(),outfile_host);

           drawing::draw_tree(&mut tree_host_pipe, outfile_host, & options);

           drawing::draw_sptree_gntrees(&mut tree_host_pipe,&mut para_trees, outfile_para_host,
                    & options);

            if open_browser {
                        if webbrowser::open_browser(Browser::Default,&url_file).is_ok() {
                            info!("Browser OK");
                        }
                        if webbrowser::open_browser(Browser::Default,&url_spfile).is_ok() {
                            info!("Browser OK");
                        }
                    }
            println!("Parasite trees as a 'gene tree' : {:?}",para_trees);
            println!("Parasite tree as a 'species tree' : {:?}",tree_para_pipe);
            println!("Map gene to species");
            println!("====================");
            map_parasite_g2s(&mut tree_para_pipe, &mut para_trees[0]);
            println!("Map species to gene");
            println!("====================");
            map_parasite_s2g(&mut tree_para_pipe, &mut para_trees[0]);
            //
            // let mut gene_trees_3:std::vec::Vec<ArenaTree<String>> = Vec::new();
            // gene_trees_3.push(sp_tree_2);
            // for index in &mut sp_tree_2.arena {
            //     // println!("SPECIS {:?}",index);
            //
            //      sp_tree_2.arena[index.idx].location = sp_tree_2.arena[index.idx].name;
            // }
            //
            // gene_trees_3.push(sp_tree_2);
            // map_species_trees(&mut sp_tree_1,&mut gene_trees_3);

}
pub fn map_parasite_g2s_obs(para_as_species: &mut ArenaTree<String>,para_as_gene: &mut ArenaTree<String>,) {
    // let  nb =  para_as_species.len(); // Nombre d'arbres de gene
    // info!("[map_gene_trees] {} gene trees to be processed",nb_gntree);
    // let mut idx_rcgen = 0;  // Boucle sur les arbres de genes
    // loop {
    //     info!("[map_gene_trees] => Processing Gene Tree {}",idx_rcgen);
    //     for  index in &mut gene_trees[idx_rcgen].arena {
}

pub fn map_parasite_g2s(para_as_species: &mut ArenaTree<String>,para_as_gene: &mut ArenaTree<String>,) {
    for index in  &mut para_as_species.arena {
        // println!("Mapping {:?}",index.name);
        let name = &index.name;
        let i = para_as_gene.get_index(name.to_string());
        match i {
            Ok(i) => {
                let e = &para_as_gene.arena[i].e;
                println!("Mapping of {} OK, event is {:?} (transfert: {})",name,e,&para_as_gene.arena[i].is_a_transfert);
                index.e=Event::Duplication;
                index.e= match para_as_gene.arena[i].e{
                    Event::Duplication => Event::Duplication,
                    Event::BranchingOut => Event::BranchingOut,
                    _ =>  Event::Undef,
                };

            },
            Err(_err) => {
                println!("Unable to map {}",name);
            },
        }
    }
}

pub fn map_parasite_s2g(para_as_species: &mut ArenaTree<String>,para_as_gene: &mut ArenaTree<String>,) {
    for index in &para_as_gene.arena {
        // println!("Mapping {:?}",index.name);
        let name = &index.name;
        let i = para_as_species.get_index(name.to_string());
        match i {
            Ok(i) => {
                println!("Mapping of {} OK",name);
            },
            Err(_err) => {
                println!("Unable to map {}",name);
            },
        }
    }
}
