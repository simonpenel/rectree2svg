use taxonomy::Taxonomy;
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
        self.children.push(noeud);
        Ok(())
    }
    pub fn get_children(& self) -> &Vec<Noeud<'a>> {
        self.children
    }
}

pub fn explore_arbre_ref(  n: & Noeud) {
    println!("Explore le noeud {:?}",n);
    let children = n.get_children();
    let v1_iter = children.iter();
    for (i, c) in v1_iter.enumerate() {
        println!("->Fils numero {}: {:?}",i, c);
        explore_arbre_ref(& c);
    }
}

// Fonction liée à la structure GeneralTaxonomy de taxonomy
pub fn deroule_taxo(t: &taxonomy::GeneralTaxonomy, n: usize) {
    let children = &t.children(n).expect("Pas de fils");
    let name = t.from_internal_id(n).expect("Pas de nom");
    let parent = t.parent(n);
    println!("Deroule Taxonomy : parent de {:?} =  {:?}",n,parent);
    println!("Deroule Taconomy : fils de {:?} ({}) =  {:?}",n,name,children);

    for child in children {
        println!("fils  {:?}",child);
        deroule_taxo(& t,*child);
        }
}
