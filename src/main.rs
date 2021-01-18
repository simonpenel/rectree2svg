use std::fs::File;
use taxonomy::formats::newick;
use taxonomy::Taxonomy;
use tree2svg::*;

fn main() {
    // // Test l'Arena
    // let mut tree: ArenaTree<String> = ArenaTree::default();
    // let hello = tree.node("Hello".into());
    // println!("Index de Hello = {}",hello);
    // let world = tree.node("World".into());
    // println!("Index de World = {}",world);
    // let coucou = tree.node(String::from("Coucou"));
    // println!("Index de Coucou = {}",coucou);
    // let coucou = tree.node(String::from("World"));
    // println!("Index de World = {}",coucou);
    // let coucou = tree.new_node(String::from("World"));
    // println!("Index de World = {:?}",coucou);
    // let coucou = tree.node(String::from("World"));
    // println!("Index de Coucou = {}",coucou);
    //
    // println!("Noeud associé l'index {} = {:?}",coucou ,tree.arena[coucou]);
    // println!("ARENA :{:?}",tree);
    //
    // tree.arena[hello].children.push(world);
    // tree.arena[world].parent = Some(hello);
    //
    // println!("ARENA :{:?}",tree);

    let mut tree_svg: ArenaTree<NoeudSVG> = ArenaTree::default();
    let mut noeud1 = NoeudSVG {
            identifiant: 0,
            name : String::from("Racine"),
            x: 12.1,
            y: 23.5,
            e: Event::Undef,
        };
    let  racine: NoeudSVG = Default::default();
    let  feuille: NoeudSVG = Default::default();
    let mut tree_svg: ArenaTree<NoeudSVG> = ArenaTree::default();
    let racine_idx = tree_svg.node(racine);
    let feuille_idx = tree_svg.node(feuille);

    tree_svg.arena[racine_idx].children.push(feuille_idx);
    tree_svg.arena[feuille_idx].parent = Some(racine_idx);

    println!("TREE SVG:{:?}",tree_svg);

    let  racine = NoeudSVG {
            identifiant: 0,
            name : String::from("Racine"),
            x: 0.0,
            y: 0.0,
            e: Event::Undef,
        };
    let  feuille = NoeudSVG {
                identifiant: 0,
                name : String::from("Feuille"),
                x: 0.0,
                y: 0.0,
                e: Event::Undef,
            };

    let mut tree_svg: ArenaTree<NoeudSVG> = ArenaTree::default();
    let racine_idx = tree_svg.node(racine);
    let feuille_idx = tree_svg.node(feuille);

    tree_svg.arena[racine_idx].children.push(feuille_idx);
    tree_svg.arena[feuille_idx].parent = Some(racine_idx);

    println!("TREE SVG:{:?}",tree_svg);


}
