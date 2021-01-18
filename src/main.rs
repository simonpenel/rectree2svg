use std::fs::File;
use taxonomy::formats::newick;
use taxonomy::Taxonomy;
use tree2svg::*;

fn main() {
    // Cree le noeud Ma Racine
    // let mut v: Vec<Noeud> = Vec::new();
    // let mut  noeud = Noeud::new(String::from("Ma Racine"),&mut v);

    // Cree le noeud "Feuille 1"
    // let mut v1: Vec<Noeud> = Vec::new();
    // let feuille1 = Noeud::new(String::from("Feuille 1"),&mut v1);

    // Cree le noeud "Feuille 2"
    // let mut v2: Vec<Noeud> = Vec::new();
    // let feuille2 = Noeud::new(String::from("Feuille 2"),&mut v2);

    //  Ajoute "Feuille 1" et "Feuille 2" à "Ma Racine"
    // noeud.add(feuille1).expect("Impossible d'ajouter le noeud");
    // noeud.add(feuille2).expect("Impossible d'ajouter le noeud");

    // Cree le noeud "Ancetre" parent de "Ma Racine"
    // let mut v3: Vec<Noeud> = Vec::new();
    // v3.push(noeud);
    // let mut  noeud2 = Noeud::new(String::from("Ancetre"),&mut v3);

    // Cree le noeud "Bof"
    // let mut v4: Vec<Noeud> = Vec::new();
    // let noeud3 = Noeud::new(String::from("Bof"),&mut v4);

    //  Ajoute "Bof" et "Feuille 2" à "Ancetre"
    // noeud2.add(noeud3).expect("Impossible d'ajouter le noeud");

    //  Explore l'arbre
    // explore_arbre_ref(& noeud2);


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
            e: String::from("Duplication")
        };

    noeud_svg_1.check();
    noeud_svg_1.check();
    // Teste le module de lecture de Taxonomie
    // let mut f = File::open("newick.txt").expect("Impossible d'ouvrir le fichier");
    // let phyl = newick::load_newick(&mut f);
    // let phyl = match phyl {
        // Ok(taxo) => {
            // println!("Le fichier est correct");
            // taxo},
        // Err(error) => {
                // panic!("Probleme lors de la lecture du fichier : {:?}", error);
            // }
    // };
    // println!("Arbre : {:?}",phyl);
    // let racine: &str = phyl.root();
    // let racine_tid = phyl.to_internal_id(racine).expect("Pas de racine");
    // Explore la taxonomie
     // deroule_taxo(& phyl,racine_tid);
}
