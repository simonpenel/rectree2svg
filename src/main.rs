/// name = "rectree2svg"
/// version = "2.0.0"
/// authors = ["Simon Penel <simon.penel@univ-lyon1.fr>"]
/// release = "16/04/2021"
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
use light_phylogeny::*;
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
    println!("{} -f input file [-b][-c config file][-g #][-h][-i][-I][-l factor][-L][-o output file][-p][-r ratio][-s][-v]",programe_name);
    println!("    -b : open svg in browser");
    println!("    -c configfile: use a configuration file");
    println!("    -g <n> : display the gene #n in phyloxml style (no species tree)");
    println!("    -h : help");
    println!("    -i : display internal gene nodes");
    println!("    -I : display internal species nodes");
    println!("    -l factor : use branch length, using the given factor (default 1.0)");
    println!("    -L : display as landscape");
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
    let mut options: Options =  Options::new();
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
    let mut opts = getopt::Parser::new(&args, "c:f:g:l:Lo:bhiIsr:pv");
    let mut infile = String::new();
    let mut outfile = String::from("tree2svg.svg");
    let mut nb_args = 0;
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
                    options.ratio = match string.parse::<f32>(){
                        Ok(valeur) => valeur,
                        Err(_err) => {
                            eprintln!("Error! Please give a numeric value with -r option");
                            process::exit(1);
                        },
                    };
                    },
                Opt('p', None) => options.clado_flag = false,
                Opt('s', None) => options.species_only_flag = true,
                Opt('l', Some(string)) => {
                    options.real_length_flag = true;
                    options.scale = match string.parse::<f32>(){
                        Ok(valeur) => valeur,
                        Err(_err) => {
                            eprintln!("Error! Please give a numeric value with -l option");
                            process::exit(1);
                        },
                    };
                    },
                Opt('L', None) => options.rotate = false,
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
    // get the url
    let path = env::current_dir().expect("Unable to get current dir");
    let url_file = format!("file:///{}/{}", path.display(),outfile.clone());

    // Creation d'une structure ArenaTree (pour phyloxml et newick)
    // -----------------------------------------------------------
    let mut tree: ArenaTree<String> = ArenaTree::default();
    // Charge l'arbre selon le format de fichier
    //  ----------------------------------------
    match format {
        // Phymxoml
        Format::Phyloxml => {
            read_phyloxml(filename.to_string(), &mut tree);
            phyloxml_processing(&mut tree, &options, &config, outfile);
        },
        // Newick
        Format::Newick => {
            read_newick(filename.to_string(), &mut tree);
            phyloxml_processing(&mut tree, &options, &config, outfile);
        },
        // Recphyloxml
        Format::Recphyloxml => {
            // On cree une structure Arena pour l'arbre d'espece
            // et un vecteur de  structures Arena pour le(s) arbres de gènes
            // -------------------------------------------------------------
            // Creation de la structure ArenaTree pour l'arbre d'espece
            // --------------------------------------------------------
            let mut sp_tree: ArenaTree<String> = ArenaTree::default();
            // Creation du vecteur de structure ArenaTree pour les genes
            // ---------------------------------------------------------
            let mut gene_trees:std::vec::Vec<ArenaTree<String>> = Vec::new();
            // Empty additional transfers
            let transfers = vec![];
            read_recphyloxml(filename.to_string(), &mut sp_tree, &mut gene_trees);
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
                phyloxml_processing(&mut tree, &options, &config, outfile);
            }
            else {
                recphyloxml_processing(&mut sp_tree,&mut  gene_trees, &mut options, &config,
                    true, &transfers, outfile);
            }
        },
    }
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
                "species_opacity" => {
                    info!("[set_config] species_opacity was {}",config.species_opacity);
                    config.species_opacity=test[1].to_string();
                    info!("[set_config] species_opacity is now {}",config.species_opacity);
                },
                "single_gene_color" => {
                    info!("[set_config] single_gene_color was {}",config.single_gene_color);
                    config.single_gene_color=test[1].to_string();
                    info!("[set_config] single_gene_color is now {}",config.single_gene_color);
                },
                "gene_opacity" => {
                    info!("[set_config] gene_opacity was {}",config.gene_opacity);
                    config.gene_opacity=test[1].to_string();
                    info!("[set_config] gene_opacity is now {}",config.gene_opacity);
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
                "bezier" => {
                    info!("[set_config] bezier was {}",config.bezier);
                    config.bezier=test[1].to_string();
                    info!("[set_config] bezier is now {}",config.bezier);
                },
                _ => {},
            }
        }

    }
}
