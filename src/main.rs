use std::fs::File;
use taxonomy::formats::newick;
use taxonomy::Taxonomy;
mod arena;
use crate::arena::ArenaTree;
use crate::arena::taxo2tree;
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
    svg::drw_tree(&mut tree);
}
