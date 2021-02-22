use std::fs::File;
use taxonomy::formats::newick;
use taxonomy::Taxonomy;
mod arena;
use crate::arena::ArenaTree;
use crate::arena::taxo2tree;
use crate::arena::shift_initial_x;
use crate::arena::knuth_layout;
use crate::arena::postorder;
use crate::arena::set_x_postorder;
use crate::arena::set_middle_postorder;
use crate::arena::get_contour_left;
use crate::arena::get_contour_right;
use crate::arena::push_right;
mod drawing;
// use clap_verbosity_flag::Verbosity;
// use structopt::StructOpt;

fn main() {
    // Builder::new()
    //     .parse(&env::var("MY_APP_LOG").unwrap_or_default())
    //     .init();

    // log::info!("informational message");
    // log::warn!("warning message");
    // log::error!("this is an error {}", "message");

    let mut tree: ArenaTree<String> = ArenaTree::default();
    let mut f = File::open("newick.txt").expect("Impossible d'ouvrir le fichier");
    let taxo = newick::load_newick(&mut f);
    let taxo = match taxo {
        Ok(taxo) => {
            println!("Le fichier est correct");
            taxo},
        Err(error) => {
                panic!("Probleme lors de la lecture du fichier : {:?}", error);
            }
    };
    // println!("Arbre  Taxo: {:?}",taxo);
    let racine: &str = taxo.root();
    let racine_tid = taxo.to_internal_id(racine).expect("Pas de racine");
    let children = taxo.children(racine_tid).expect("Pas de fils");
    for child in children {
        taxo2tree(& taxo, child,  &mut tree);
    }
    //taxo2tree(& taxo,racine_tid,&mut tree);
    // println!("Arbre Arena: {:?}",tree);
    // set_tree_coords(&mut tree);
    let  root = tree.get_root();
    // find_leftest(&mut tree,root);
    // postorder(&mut tree);


    // set root coordinates
    // tree.arena[root].set_x_noref(150.0);
    // tree.arena[root].set_y_noref(150.0);
    // preset_child_coords(&mut tree, root);
    // set_initial_x(&mut tree, root);
    // set_initial_y(&mut tree);
    // shift_initial_x(&mut tree, root);
    knuth_layout(&mut tree,root, &mut 1);
    let mut x_coords  = vec![0; tree.arena.len()];
    set_x_postorder(&mut tree, root, &mut x_coords );
    shift_initial_x(&mut tree, root);

    let mut index_noeud = root;
    let mut depth_noeud  = tree.depth(index_noeud);
    let mut contour_right  = vec![tree.arena[index_noeud].x];

    let mut contour_left  = vec![tree.arena[index_noeud].x];
    println!("CONTOUR LEFT = {:?}",contour_left);
    get_contour_left(&mut tree,index_noeud,depth_noeud,&mut contour_left);
    println!("CONTOUR LEFT = {:?}",contour_left);

    println!("CONTOUR RIGHT = {:?}",contour_right);
    get_contour_right(&mut tree,index_noeud,depth_noeud,&mut contour_right);
    println!("CONTOUR RIGHT = {:?}",contour_right);

    let mut index_noeud = 4;
    let mut depth_noeud  = tree.depth(index_noeud);
    let mut contour_right  = vec![tree.arena[index_noeud].x];

    println!("CONTOUR RIGHT = {:?}",contour_right);
    get_contour_right(&mut tree,index_noeud,depth_noeud,&mut contour_right);
    println!("CONTOUR RIGHT = {:?}",contour_right);
    // set_middle_postorder(&mut tree, root); ( ne marche pas)
    // postorder(&mut tree);
    push_right(&mut tree,3,4);
    drawing::draw_tree(&mut tree);
    println!("ARENA :{:?}",tree);

}
