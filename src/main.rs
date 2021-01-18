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

    println!("ARENA :{:?}",tree);

    tree.arena[hello].children.push(world);
    tree.arena[world].parent = Some(hello);

    println!("ARENA :{:?}",tree);

    let mut tree_svg: ArenaTree<NoeudSVG> = ArenaTree::default();
    let mut noeud_svg_1 = NoeudSVG {
            identifiant: 0,
            name : String::from("Racine"),
            x: 12.1,
            y: 23.5,
            e: Event::Undef,
        };
    let mut noeud_svg: NoeudSVG = Default::default();
    noeud_svg.check();
    println!("NOEUD :{:?}",noeud_svg);

    noeud_svg_1.check();
    noeud_svg_1.check();

}
