use taxonomy::Taxonomy;

// Taken from https://dev.to/deciduously/no-more-tears-no-more-knots-arena-allocated-trees-in-rust-44k6
#[derive(Debug)]
pub struct Node<T>
where
    T: PartialEq
{
    idx: usize,
    val: T,
    pub parent: Option<usize>,
    pub children: Vec<usize>,
    pub x: f32,
    pub e: Event,
}

#[derive(Debug, Default)]
pub struct ArenaTree<T>
where
    T: PartialEq
{
    pub arena: Vec<Node<T>>,
}

impl<T> Node<T>
where
    T: PartialEq
{
    pub fn new(idx: usize, val: T) -> Self {
        Self {
            idx,
            val,
            parent: None,
            children: vec![],
            x: 0.0,
            e: Event::Undef,
        }
    }
    pub fn get_val(&mut self) -> &T {
        &self.val
    }
    pub fn get_valx(&mut self) -> &f32 {
        &self.x
    }
    pub fn set_valx(&mut self, x: &f32)
    {
        self.x = *x;
    }

    pub fn set_valxnoref(&mut self, x: f32)
    {
        self.x = x;
    }


    pub fn get_vale(&mut self) -> &Event {
        &self.e
    }
    pub fn set_vale(&mut self, e: Event)
    {
        self.e = e;
    }



}

impl<T> ArenaTree<T>
where
    T: PartialEq
{
    pub fn node(&mut self, val: T) -> usize {
        //first see if it exists
        for node in &self.arena {
            if node.val == val {
                return node.idx;
            }
        }
        // Otherwise, add new node
        let idx = self.arena.len();
        self.arena.push(Node::new(idx, val));
        idx
    }
    pub fn new_node(&mut self, val: T) -> Result<usize, &'static str> {
        println!("TEST IF EXISTST ");
        //first see if it exists
        for node in &self.arena {
            if node.val == val {
                    println!("OUI");
                    panic!("Le noued existe dèja");
                // return Err("Ce noeud existe déjà")
            }
        }
        // Otherwise, add new node
        let idx = self.arena.len();
        self.arena.push(Node::new(idx, val));
        Ok(idx)
    }
    // fn insert(&mut self, orbit: &str) {
    //     // Init nodes
    //     let split = orbit.split(')').collect::<Vec<&str>>();
    //     let inner = self.node(split[0]);
    //     let outer = self.node(split[1]);
    //     // set orbit
    //     match self.arena[outer].parent {
    //         Some(_) => panic!("Attempt to overwrite existing orbit"),
    //         None => self.arena[outer].parent = Some(inner),
    //     }
    //     // set parents
    //     self.arena[inner].children.push(outer);
    // }

}

// enum des evenements possibles
#[derive(Debug, PartialEq)]
pub enum Event {
    Speciation,
    Duplication,
    Loss,
    Transfer,
    Undef,
}
// Pas de trait Default pour enum, donc
impl Default for Event {
    fn default() -> Self { Event::Undef }
}

// NoeudSVG
#[derive(Debug, Default,PartialEq)]
pub struct NoeudSVG
{
    pub identifiant:usize,
    pub name: String,
    pub x: f32 ,
    pub y: f32,
    pub e: Event,
}

impl NoeudSVG {
    pub fn check (&mut self) {
        println!("Name =        {}",self.name);
        println!("Identifiant = {}",self.identifiant);
        println!("Coordinates = {} {}",self.x,self.y);
        println!("Event =       {:?}",self.e);
    }

    pub fn get_event (& self) -> &Event  {
        &self.e
    }

    pub fn set_x (&mut  self, x: f32)  {
        self.x = x;
    }
    pub fn get_x (& self) -> f32 {
        self.x
    }



}

pub fn taxo2tree(t: &taxonomy::GeneralTaxonomy, n: usize, tree: &mut ArenaTree<String>) {

    let children = &t.children(n).expect("Pas de fils");
    let name = t.from_internal_id(n).expect("Pas de nom");
    let parent = t.parent(n).expect("Pas de parent");
    let parent_name = match parent {
        None => "root",
        Some ((id, _dist)) => t.from_internal_id(id).expect("Pas de nom")
    };
    let parent_index = match parent {
        None => 0,
        Some ((id, _dist)) => id
    };
    // let name = "N".to_owned()+&it.to_string()+"_"+name;
    let name = "N".to_owned()+&n.to_string()+"_"+name;
     let parent_name = "N".to_owned()+&parent_index.to_string()+"_"+parent_name;
     let name = tree.node(name.to_string()); // TRES DANGEREUX VERIFIER REDOND (CF NEW_NODE)
     let parent = tree.node(parent_name.to_string());
     tree.arena[parent].children.push(name);
     tree.arena[name].parent = Some(parent);


     for child in children {
        taxo2tree(& t,*child,  tree);
        }
}
