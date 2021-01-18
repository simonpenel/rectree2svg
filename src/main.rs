use std::fs::File;
use taxonomy::formats::newick;
use taxonomy::Taxonomy;
use tree2svg::*;

fn main() {
    // Test l'Arena
    let mut tree: ArenaTree<String> = ArenaTree::default();
    let hello = tree.node("Hello".into());
    println!("Index de Hello = {}",hello);
    let world = tree.node("World".into());
    println!("Index de World = {}",world);
    let coucou = tree.node(String::from("Coucou"));
    println!("Index de Coucou = {}",coucou);
    let coucou = tree.node(String::from("World"));
    println!("Index de World = {}",coucou);
        let coucou = tree.new_node(String::from("World"));
        println!("Index de World = {:?}",coucou);

    println!("ARENA :{:?}",tree);

    tree.arena[hello].children.push(world);
    tree.arena[world].parent = Some(hello);

    println!("ARENA :{:?}",tree);

    // let mut tree_svg: ArenaTree<NoeudSVG> = ArenaTree::default();
    // let mut noeud_svg_1 = NoeudSVG {
    //         identifiant: 0,
    //         name : String::from("Racine"),
    //         x: 12.1,
    //         y: 23.5,
    //         e: Event::Undef,
    //     };
    // let  racine_svg: NoeudSVG = Default::default();
    // let  racine_svg = NoeudSVG {
    //         identifiant: 1,
    //         name : String::from("Racine"),
    //         x: 12.1,
    //         y: 23.5,
    //         e: Event::Speciation,
    //     };
    //
    // let racine = tree_svg.node(racine_svg);
    //
    // let  feuille_svg: NoeudSVG = Default::default();
    // let feuille = tree_svg.node(feuille_svg);
    //
    // tree_svg.arena[racine].children.push(feuille);
    // tree_svg.arena[feuille].parent = Some(racine);
    //
    // println!("TREE SVG:{:?}",tree_svg);
}
