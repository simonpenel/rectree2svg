use std::fs::File;
use taxonomy::formats::newick;
use taxonomy::Taxonomy;
mod arena;
use crate::arena::ArenaTree;
use crate::arena::taxo2tree;
use crate::arena::knuth_layout;
// use crate::arena::postorder;
use crate::arena::set_middle_postorder;
// use crate::arena::get_contour_left;
// use crate::arena::get_contour_right;
// use crate::arena::push_right;
use crate::arena::shift_mod_x;
use crate::arena::check_contour_postorder;
use crate::arena::cladogramme;
mod drawing;
use log::{info};

fn main() {
    env_logger::init();
    info!("Verbosity set to Info");
    let mut tree: ArenaTree<String> = ArenaTree::default();
    let mut f = File::open("newick.txt").expect("Impossible d'ouvrir le fichier");
    let taxo = newick::load_newick(&mut f);
    let taxo = match taxo {
        Ok(taxo) => {
            info!("File is ok");
            taxo},
        Err(error) => {
                panic!("Probleme lors de la lecture du fichier : {:?}", error);
            }
    };

    let racine: &str = taxo.root();
    let racine_tid = taxo.to_internal_id(racine).expect("Pas de racine");
    let children = taxo.children(racine_tid).expect("Pas de fils");
    for child in children {
        taxo2tree(& taxo, child,  &mut tree);
    }

    let  root = tree.get_root();

    // 1ere etape : profondeur => Y, left => X= 0, right X=1
    // ======================================================
    knuth_layout(&mut tree,root, &mut 1);
    drawing::draw_tree(&mut tree,"knuth.svg".to_string());

    // Cladogramme
    // ===========
    cladogramme(&mut tree);

    // Veifie les contours
    // ===================
     check_contour_postorder(&mut tree, root);

    // Decale toutes les valeurs de x en finction de xmod
    // ===================================================
    shift_mod_x(&mut tree, root, &mut 0.0);
    drawing::draw_tree(&mut tree,"shifted.svg".to_string());

    // Place le parent entre les enfants
    // =================================
    set_middle_postorder(&mut tree, root);

    // tree.rotate();
    drawing::draw_tree(&mut tree,"smiddle.svg".to_string());

}
