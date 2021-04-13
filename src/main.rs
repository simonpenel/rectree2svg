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
use crate::arena::Config;
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
use crate::arena::reset_pos;
use crate::arena::set_middle_postorder;
use crate::arena::shift_mod_xy;
use crate::arena::summary;
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
    // Initialise la config
    let mut config: Config = Config::new();
    // Charge la config par deuakt si elle existe
    let fconf = "config_default.txt";
     if fs::metadata(fconf).is_ok() {
         set_config(fconf.to_string(), &mut config);

     }
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
    let mut outfile_para_rec = String::from("para_rec.svg");
    let mut outfile_para_host = String::from("para_host.svg");
    let mut outfile_host = String::from("host.svg");
    let mut nb_args = 0;
    loop {
        match opts.next().transpose().expect("Unknown option") {
            None => break,
            Some(opt) => match opt {
                Opt('i', None) => options.gene_internal = true,
                Opt('I', None) => options.species_internal = true,
                Opt('b', None) => options.open_browser = true,
                Opt('r', Some(string)) => {
                    options.ratio = string.parse().unwrap();
                    },
                Opt('p', None) => options.clado_flag = false,
                Opt('s', None) => options.species_only_flag = true,
                Opt('l', Some(string)) => {
                    options.real_length_flag = true;
                    options.scale = string.parse().unwrap();
                    },
                Opt('v', None) => {
                    options.verbose = true;
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
            let mut index  = &mut 0;                // Nom de la racine
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
    recphyloxml_processing(&mut tree_para_pipe,&mut  gene_trees, &mut options, &config,outfile_gene_para);
    reset_pos(&mut tree_para_pipe);
    phyloxml_processing(&mut tree_para_pipe, &mut options, &config,outfile_para);

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

    recphyloxml_processing(&mut tree_host_pipe,&mut  para_trees, &mut options, &config,outfile_para_host);
    reset_pos(&mut tree_host_pipe);
    phyloxml_processing(&mut tree_host_pipe, &mut options, &config,outfile_host);
    reset_pos(&mut para_trees[0]);
    phyloxml_processing(&mut para_trees[0], &mut options, &config,outfile_para_rec);
    println!("Parasite trees as a 'gene tree' : {:?}",para_trees);
    println!("Parasite tree as a 'species tree' : {:?}",tree_para_pipe);
    println!("Map parasite as 'gene' to parasite as 'species'");
    println!("==============================================");
    map_parasite_g2s(&mut tree_para_pipe, &mut para_trees[0]);
    println!("Map parasite as 'species' to paraiste as 'gene'");
    println!("==============================================");
    map_parasite_s2g(&mut tree_para_pipe, &mut para_trees[0]);
    println!("MAP AGAIN!");
    println!("Map parasite as 'gene' to parasite as 'species'");
    println!("==============================================");
    map_parasite_g2s(&mut tree_para_pipe, &mut para_trees[0]);
         summary(&mut tree_para_pipe);

    reset_pos(&mut tree_para_pipe);
    reset_pos(&mut gene_trees[0]);
    recphyloxml_processing_nomap(&mut tree_para_pipe, &mut gene_trees, &mut options, &config,"test_mapped.svg".to_string());
    // drawing::draw_tree(&mut tree_para_pipe, "test_mapped.svg".to_string(), &options, &config);
    // reset_pos(&mut tree_para_pipe);
    // reset_pos(&mut gene_trees[0]);
    // recphyloxml_processing(&mut tree_para_pipe,&mut  gene_trees, &mut options, &config,"test.svg".to_string());
     // reset_pos(&mut tree_host_pipe);


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
                // index.e=Event::Duplication;
                // index.e = match &para_as_gene.arena[i].e{
                index.is_a_transfert = para_as_gene.arena[i].is_a_transfert;
                index.e = match  e {
                    &Event::Duplication => Event::Duplication,
                    &Event::BranchingOut => Event::BranchingOut,
                    &Event::Speciation => Event::Speciation,
                    &Event::Loss => Event::Loss,
                    &Event::Leaf => Event::Leaf,
                    _ => {println!("Event {:?} not selected",e);
                         Event::Undef},
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
                println!("Unable to map {} {:?}",name,&index.e);
                let p = &index.parent;
                let p = match p {
                    Some(p) => p,
                    None => panic!("[map_parasite_s2g] Error: node as not parent"),
                };
                let parent_name = para_as_gene.arena[*p].name.clone();
                println!("   => parent in the 'gene' tree is {}({})",p,parent_name);
                let j = para_as_species.get_index(parent_name.to_string());
                let j = match j {
                    Ok(j) => j,
                    Err(_e) => panic!("Unable to find parent in 'species' tree"),
                };
                println!("   => Mapping of {} OK",parent_name);
                match index.e {
                    Event::Loss => {
                        println!("   => missing node is a Loss, I add it to parent");
                        let new_loss = para_as_species.new_node(name.to_string());
                        para_as_species.arena[new_loss].name = name.to_string();
                        para_as_species.arena[new_loss].parent = Some(j);
                        para_as_species.arena[j].children.push(new_loss);
                    },
                    _ => {
                        println!("   => missing node is a not Loss, I insert it between  parent and chidren");
                        let new_node = para_as_species.new_node(name.to_string());
                        para_as_species.arena[new_node].name = name.to_string();
                        let children = &para_as_species.arena[j].children;
                        // println!("   => children are {:?}",&children);
                        let left = children[0];
                        let right = children[1];
                        para_as_species.arena[left].parent = Some(new_node);
                        para_as_species.arena[right].parent = Some(new_node);
                        para_as_species.arena[new_node].children.push(left);
                        para_as_species.arena[new_node].children.push(right);
                        para_as_species.arena[j].children.retain(|&x| x !=  left);
                        para_as_species.arena[j].children.retain(|&x| x !=  right);
                        para_as_species.arena[j].children.push(new_node);
                        para_as_species.arena[new_node].parent = Some(j);
                            // para_as_species.arena[new_node].children.push(child);

                        // Je suppose qu'on a 2 nouds, mais meme si le loss a ete rajoté on s'en
                        // fout je consiere que les 2 premiers
                        //Il faut remaper les nouveua
                        let gnodes = &para_as_species.arena[j].nodes;
                        para_as_species.arena[new_node].nodes = gnodes.to_vec();



                    },
                };


            },
        }
    }
}

fn phyloxml_processing(mut tree: &mut ArenaTree<String>, options: &Options, config: &Config,
     outfile: String ) {
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
    if options.clado_flag {
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
    if options.real_length_flag {
        real_length(&mut tree, root, &mut 0.0, & options);
    }
    // ---------------------------------------------------------
    // Fin: Ecriture du fichier svg
    // ---------------------------------------------------------
    println!("Output filename is {}",outfile);

    let path = env::current_dir().expect("Unable to get current dir");
    let url_file = format!("file:///{}/{}", path.display(),outfile);
    drawing::draw_tree(&mut tree, outfile, & options, & config);
    // EXIT
    // On s'arrete la, le reste du programme concerne les autres formats
    if options.open_browser {
        if webbrowser::open_browser(Browser::Default,&url_file).is_ok() {
            info!("Browser OK");
        }
    }

}
fn recphyloxml_processing(mut sp_tree: &mut ArenaTree<String>,
    mut gene_trees:&mut std::vec::Vec<ArenaTree<String>>,
    mut options: &mut Options, config: &Config, outfile: String ) {
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
if options.clado_flag {
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
bilan_mappings(&mut sp_tree, &mut gene_trees,root, & options);
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
match options.species_only_flag {
    true => {
        if options.species_internal {
             options.gene_internal = true;}
             drawing::draw_tree(&mut sp_tree, outfile, &options,  &config);

    },
    false => drawing::draw_sptree_gntrees(&mut sp_tree, &mut gene_trees, outfile,
        &options, &config),
};
if options.open_browser {
    if webbrowser::open_browser(Browser::Default,&url_file).is_ok() {
        info!("Browser OK");
    }
}
}

fn recphyloxml_processing_nomap(mut sp_tree: &mut ArenaTree<String>,
    mut gene_trees:&mut std::vec::Vec<ArenaTree<String>>,
    mut options: &mut Options, config: &Config, outfile: String ) {
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
if options.clado_flag {
    cladogramme(&mut sp_tree);
}
// ---------------------------------------------------------
// 2eme étape :  mapping des genes sur l'espèce pour
// connaître le nombre de noeuds d'arbre de gènes associés à
// chaque noeud de l'arbre d'espèces
// ---------------------------------------------------------
// map_species_trees(&mut sp_tree,&mut gene_trees);
// info!("Species tree after mapping : {:?}",sp_tree);
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
// map_gene_trees(&mut sp_tree,&mut gene_trees);
// ---------------------------------------------------------
// 9ème etape : décale les noeuds de gene associés à un
// noeud d'especes pour éviter qu'ils soit superposés
// ---------------------------------------------------------
bilan_mappings(&mut sp_tree, &mut gene_trees,root, & options);
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
match options.species_only_flag {
    true => {
        if options.species_internal {
             options.gene_internal = true;}
             drawing::draw_tree(&mut sp_tree, outfile, &options,  &config);

    },
    false => drawing::draw_sptree_gntrees(&mut sp_tree, &mut gene_trees, outfile,
        &options, &config),
};
if options.open_browser {
    if webbrowser::open_browser(Browser::Default,&url_file).is_ok() {
        info!("Browser OK");
    }
}
}

fn set_config(configfile: String, config: &mut Config) {
    let contents = fs::read_to_string(configfile)
                .expect("Something went wrong reading the config file");
    let conf = contents.split('\n');
    for line in conf {
        let test: Vec<&str> = line.split(':').collect();
        if test.len() == 2 {
            match test[0] {
                "species_color" => {
                    info!("[set_config] species_color was {}",config.species_color);
                    config.species_color=test[1].to_string();
                    info!("[set_config] species_color is now {}",config.species_color);
                },
                "single_gene_color" => {
                    info!("[set_config] single_gene_color was {}",config.single_gene_color);
                    config.single_gene_color=test[1].to_string();
                    info!("[set_config] single_gene_color is now {}",config.single_gene_color);
                },
                "species_police_color" => {
                    info!("[set_config] species_police_color was {}",config.species_police_color);
                    config.species_police_color=test[1].to_string();
                    info!("[set_config] species_police_color is now {}",config.species_police_color);
                },
                "species_police_size" => {
                    info!("[set_config] species_police_size was {}",config.species_police_size);
                    config.species_police_size=test[1].to_string();
                    info!("[set_config] species_police_size is now {}",config.species_police_size);
                },
                "gene_police_size" => {
                    info!("[set_config] gene_police_size was {}",config.gene_police_size);
                    config.gene_police_size=test[1].to_string();
                    info!("[set_config] gene_police_size is now {}",config.gene_police_size);
                },
                _ => {},
            }
        }

    }
}
