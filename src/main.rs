use std::fs::File;
use taxonomy::formats::newick;
use taxonomy::Taxonomy;
mod arena;
use crate::arena::ArenaTree;
use crate::arena::taxo2tree;
use crate::arena::set_tree_coords;
mod svg;

fn main() {
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
    println!("Arbre  Taxo: {:?}",taxo);
    let racine: &str = taxo.root();
    let racine_tid = taxo.to_internal_id(racine).expect("Pas de racine");
    taxo2tree(& taxo,racine_tid,&mut tree);
    println!("Arbre Arena: {:?}",tree);
    // let longueur = tree.arena.len();
    // let mut count = 0usize;
    //  loop {
    //      tree.arena[count].set_x_noref(10.0* (count as f32));
    //      tree.arena[count].set_y_noref(15.0* (count as f32));
    //     count += 1;
    //
    //     if count == longueur {
    //         break;
    //     }
    // }

        // tree.set_coords();

    set_tree_coords(&mut tree);
    // let coucou = tree.node(String::from("World"));
    // tree.arena[coucou].set_x(&1.2);
    // tree.arena[0].set_x_noref(1.2);
    // tree.arena[0].set_y_noref(1.2);
    //
    // tree.arena[1].set_x_noref(12.0);
    // tree.arena[1].set_y_noref(12.0);
    //
    // tree.arena[2].set_x_noref(24.0);
    // tree.arena[2].set_y_noref(12.0);
    //
    // tree.arena[3].set_x_noref(12.0);
    // tree.arena[3].set_y_noref(24.0);


    // let mut i = 1;
    // tree.arena[i].set_x_noref(1.2);
    // i = i + 1;
    // println!("INDEX 2 {:?}",tree.arena[2]);
    // for mut index in &tree.arena {
    //     index.set_x_noref(10.0);
    //     println!("INDEX {:?}",index.x);
    //
    // }
    svg::draw_tree(&mut tree);




    // let mut tree: ArenaTree<String> = ArenaTree::default();
    // let hello = tree.node("Hello".into());
    // println!("Index de Hello = {}",hello);
    // let world = tree.node("World".into());
    // println!("Index de World = {}",world);
    // let coucou = tree.node(String::from("Coucou"));
    // println!("Index de Coucou = {}",coucou);
    // let coucou = tree.node(String::from("World"));
    // println!("Index de World = {}",coucou);
    // let coucou = tree.node(String::from("World"));
    // println!("Index de World = {:?}",coucou);
    // let coucou = tree.node(String::from("World"));
    // println!("Index de Coucou = {}",coucou);
    //
    // println!("Noeud associé l'index {} = {:?}",coucou ,tree.arena[coucou]);
    //     println!("X associé l'index {} = {:?}",coucou ,tree.arena[coucou].x);
    //     println!("Val associé l'index {} = {:?}",coucou ,tree.arena[coucou].get_x());
    //     println!("Val X associé l'index {} = {:?}",coucou ,tree.arena[coucou].get_x());
    //     let n = 1.666666;
    //
    //     tree.arena[coucou].set_x(&1.2);
    //     tree.arena[coucou].set_x(&n);
    //     tree.arena[coucou].set_x_noref(3.23111);
    //
    //     // tree.arena[coucou].set_event(Event::Duplication);
    //     let n=122222;
    //     println!("Val event associé l'index {} = {:?}",coucou ,tree.arena[coucou].get_event());

    println!("ARENA :{:?}",tree);

}
