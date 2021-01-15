use std::io;
use std::fs::File;
use taxonomy::formats::newick;
use taxonomy::formats::phyloxml;
use taxonomy::Taxonomy;
use draw::*;

/// Un noeud présente  un nom,  un identifiant et une liste de descendants qui peut être vide.
/// La liste de descendants est une référence car on  veut garder la propriété (borower)
/// Elle a le même temps de vie (lifetime) que le noeud père, ainsi que les noeuds de la liste.
/// Elle est mutable car on peut ajouter des descendants.
#[derive(Debug)]
pub struct Noeud <'a> {
    name: String,
    id: usize,
    children: &'a mut Vec<Noeud<'a>>,
}

impl <'a> Noeud <'a>{
    pub fn new(name: String, children: &'a mut Vec<Noeud<'a>>) -> Noeud<'a> {
         Noeud {
             name: name,
             id: 0,
             children: children,
         }
    }
    pub fn add(&mut self, noeud: Noeud<'a>) -> Result<(),  &'static str> {
        println!("Processing  {:?}",self);
        println!("Processing  children {:?}",self.children);
        self.children.push(noeud);
        Ok(())
    }
    pub fn get_children(& self) -> &Vec<Noeud<'a>> {
        self.children
    }
}
// impl Default for Noeud {
//     /// Create a new GeneralTaxonomy with only a root node.
//     fn default() -> Self {
//         let mut noeud = Noeud {
//             id: 0,
//             name : String::from("root"),
//             children:  &Vec::new(),
//         };
//         noeud
//     }
// }

fn explore_arbre_ref(  n: & Noeud) {
    println!("EXPLORE {:?}",n);
    let children = n.get_children();
        println!("DEROULE fils {:?}",children);
        let v1_iter = children.iter();
        for (i, c) in v1_iter.enumerate() {
     println!(" REF fils numero {}: {:?}",i, c);
     explore_arbre_ref(& c);
 }
}

fn main() {
    let mut v: Vec<Noeud> = Vec::new();
    let mut  noeud = Noeud::new(String::from("Ma Racine"),&mut v);

    println!("Noeud : {:?}",noeud);
    let mut v1: Vec<Noeud> = Vec::new();
    let mut  feuille1 = Noeud::new(String::from("Feuille 1"),&mut v1);
    noeud.add(feuille1);
    let mut v2: Vec<Noeud> = Vec::new();
    let mut  feuille2 = Noeud::new(String::from("Feuille 2"),&mut v2);
    noeud.add(feuille2);
    explore_arbre_ref(& noeud);



    let mut v3: Vec<Noeud> = Vec::new();
    v3.push(noeud);
    let mut  noeud2 = Noeud::new(String::from("Ancetre"),&mut v3);
    explore_arbre_ref(& noeud2);

    let mut v4: Vec<Noeud> = Vec::new();
    let mut  noeud3 = Noeud::new(String::from("Bof"),&mut v4);
    noeud2.add(noeud3);
    println!("NOEUD2 =  {:?} ", noeud2 );
    explore_arbre_ref(& noeud2);

    let mut f = File::open("newick.txt").expect("Impossible d'ouvrir le fichier");
    let phyl = newick::load_newick(&mut f);

    let phyl = match phyl {
        Ok(taxo) => {
            println!("Le fichier est correct");
            taxo},
        Err(error) => {
                panic!("Probleme lors de la lecture du fichier : {:?}", error);
                // error
            }
    };
    // println!("Arbre : {:?}",phyl);
    // let racine: &str = phyl.root();   // Bordel de bite
    // println!(">>>Racine : taxid = {}",racine);
    // let racine_tid = phyl.to_internal_id(racine).expect("Pas de racine");
    // deroule_tree(& phyl,racine_tid);
    // display_tree(phyl); // Pas de reference donc t va etre borrwed par la fonction
    // display_tree_ref(&phyl); // Avec reference
}

// fn display_tree(t: taxonomy::GeneralTaxonomy) {
//     println!("Arbre : {:?}",t);
//     let racine: &str = t.root();   // Bordel de bite
//     println!(">>>Racine : taxid = {}",racine);
//     let racine_tid = t.to_internal_id(racine).expect("Pas de racine");
//     println!(">>>Racine : internal id = {:?}",racine_tid);
//     println!(">>>Racine : fils = {:?}",t.children(racine));
//     println!(">>>Racine : fils = {:?}",t.children(racine_tid));
//     let children = t.children(racine_tid).expect("Pas de fils");
//     for child in children {
//         println!("fils  {:?}",child);
//             println!(">>>>>>>> fils = {:?}",t.children(child));
//
//     }
// }
//
// fn deroule_tree(t: &taxonomy::GeneralTaxonomy, n: usize) {
//     let children = &t.children(n).expect("Pas de fils");
//     let name = t.from_internal_id(n).expect("Pas de nom");
//     let parent = t.parent(n);
//     println!("DEROULE parent de {:?} =  {:?}",n,parent);
//     println!("DEROULE fils de {:?} ({}) =  {:?}",n,name,children);
//
//     for child in children {
//         println!("fils  {:?}",child);
//         deroule_tree(& t,*child);
//         }
// }
//
//
// fn display_tree_ref(t: & taxonomy::GeneralTaxonomy) {
//     println!("Arbre : {:?}",t);
//
// }
