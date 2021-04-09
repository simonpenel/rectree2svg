/// name = "rectree2svg"
/// version = "0.8.0"
/// authors = ["Simon Penel <simon.penel@univ-lyon1.fr>"]
/// release = "09/04/2021"
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
use crate::arena::ArenaTree;
use crate::arena::Config;
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
    println!("{} -f input file [-b][-c config file][-g #][-h][-i][-I][-l factor][-o output file][-p][-r ratio][-s][-v]",programe_name);
    println!("    -b : open svg in browser");
    println!("    -c configfile: use a configuration file");
    println!("    -g <n> : display the gene #n in phyloxml style (no species tree)");
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
    println!("Input format is guessed according to the file name extension:");

    println!(".xml         => phyloxml");
    println!(".phyloxml    => phyloXML");
    println!(".recphyloxml => recPhyloXML");
    println!(".recPhyloXML => recPhyloXML");
    println!(".recphylo    => recPhyloXML");
    println!("any other    => newick");
    println!("");
    println!("About recPhyloXML format: http://phylariane.univ-lyon1.fr/recphyloxml/");
    println!("recPhyloXML paper: https://www.ncbi.nlm.nih.gov/pmc/articles/PMC6198865/");
    println!("About phyloXML format: http://www.phyloxml.org/");
    println!("phyloXML paper: https://www.ncbi.nlm.nih.gov/pmc/articles/PMC2774328/");


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
    let mut options: Options = Options::new();
    // Initialise la config
    let mut config: Config = Config::new();
    // Gestion des arguments et des options
    // ------------------------------------
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
         display_help(args[0].to_string());
    }
    let mut opts = getopt::Parser::new(&args, "c:f:g:l:o:bhiIsr:pv");
    let mut infile = String::new();
    let mut outfile = String::from("tree2svg.svg");
    // let mut clado_flag = true;
    // let mut species_only_flag = false;
    // let mut open_browser = false;
    // let mut real_length_flag = false;
    // let mut verbose = false;
    let mut nb_args = 0;
    // let mut disp_gene = 0;
    loop {
        match opts.next().transpose().expect("Unknown option") {
            None => break,
            Some(opt) => match opt {
                Opt('g', Some(string)) => {
                    options.disp_gene = string.parse().unwrap();
                    },
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
                Opt('c', Some(string)) => {
                        set_config(string, &mut config);
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
            phyloxml_processing(&mut tree, options, config, outfile);
        },
        // Newick
        Format::Newick => {
            println!("=================================================================");
            println!("WARNING: 06 April 2021: NHX balises if any will not be considered");
            println!("=================================================================");
            let contents = fs::read_to_string(filename)
                .expect("Something went wrong reading the newick file");
            let root = tree.new_node("Root".to_string());
            newick2tree(contents, &mut tree, root, &mut 0);
            phyloxml_processing(&mut tree, options, config, outfile);
        },
        // Recphyloxml
        Format::Recphyloxml => {
            // On cree une structure Arena pour l'arbre d'espece
            // et un vecteur de  structures Arena pour le(s) arbres de gènes
            // -------------------------------------------------------------
            // Creation de la structure ArenaTree pour l'arbre d'espece
            // --------------------------------------------------------
            let mut sp_tree: ArenaTree<String> = ArenaTree::default();
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
            if options.verbose {
                info!("Drawing tree species verbose-species.svg");
                drawing::draw_tree(&mut sp_tree, "verbose-species.svg".to_string(), &options, &config);
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

            if options.disp_gene  > 0 {
                // On traite l'arbre de gene comme un arbre au format phylxoml
                if options.disp_gene > nb_gntree {
                    println!("There are only {} genes in the file, unable to display gene #{}",
                    nb_gntree,options.disp_gene);
                    process::exit(1);
                }
                let  mut tree = &mut gene_trees[options.disp_gene-1];
                phyloxml_processing(&mut tree, options, config, outfile);
            }
            else {
                recphyloxml_processing(&mut sp_tree,&mut  gene_trees, options, config, outfile);
            }
        },
    }
}
fn phyloxml_processing(mut tree: &mut ArenaTree<String>, options:Options, config:Config,
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
    mut options:Options, config:Config, outfile: String ) {
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
