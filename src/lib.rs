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
        }
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
        //first see if it exists
        for node in &self.arena {
            if node.val == val {
                return Err("Ce noeud existe déjà")
            }
        }
        // Otherwise, add new node
        let idx = self.arena.len();
        self.arena.push(Node::new(idx, val));
        Ok(idx)
    }
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

    pub fn get_event (&mut self) -> &Event  {
        &self.e
    }


}
