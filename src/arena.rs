use taxonomy::Taxonomy;

/// Structure Noeud.
///
#[derive(Debug)]
pub struct Noeud<T>
where
    T: PartialEq
{
    idx: usize,
    val: T,
    pub name: String,
    pub parent: Option<usize>,
    pub children: Vec<usize>,
    pub x: f32,
    pub y: f32,
    pub e: Event,
}
/// Structure ArenaTree.
///
/// Taken from https://dev.to/deciduously/no-more-tears-no-more-knots-arena-allocated-trees-in-rust-44k6
#[derive(Debug, Default)]
pub struct ArenaTree<T>
where
    T: PartialEq
{
    pub arena: Vec<Noeud<T>>,
}

impl<T> Noeud<T>
where
    T: PartialEq
{
    pub fn new(idx: usize, val: T) -> Self {
        Self {
            idx,
            val,
            name :String::from("Undefined"),
            parent: None,
            children: vec![],
            x: 0.0,
            y: 0.0,
            e: Event::Undef,
        }
    }
    pub fn get_val(&mut self) -> &T {
        &self.val
    }
    pub fn get_x(&mut self) -> &f32 {
        &self.x
    }
    pub fn get_y(&mut self) -> &f32 {
        &self.y
    }
    pub fn get_coords(&mut self) -> ( &f32, &f32) {
        (&self.x,&self.y)
    }

    pub fn set_x(&mut self, x: &f32)
    {
        self.x = *x;
    }
    pub fn set_x_noref(&mut self, x: f32)
    {
        self.x = x;
    }
    pub fn set_y(&mut self, y: &f32)
    {
        self.y = *y;
    }
    pub fn set_y_noref(&mut self, y: f32)
    {
        self.y = y;
    }
    pub fn get_event(&mut self) -> &Event {
        &self.e
    }
    pub fn set_event(&mut self, e: Event)
    {
        self.e = e;
    }
}

/// Arena structure taken from https://dev.to/deciduously/no-more-tears-no-more-knots-arena-allocated-trees-in-rust-44k6
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
        self.arena.push(Noeud::new(idx, val));
        idx
    }
    pub fn new_node(&mut self, val: T) -> usize {
        //first see if it exists
        for node in &self.arena {
            if node.val == val {
                    panic!("Le noeud existe dèja");
            }
        }
        // Otherwise, add new node
        let idx = self.arena.len();
        self.arena.push(Noeud::new(idx, val));
        // Ok(idx)
        idx
    }
    //A AMELIORER : RENVOYER UN RESULTS
    pub fn get_root(&mut self) -> usize {
        //first see if it exists
        for node in &self.arena {
    //        match node.parent {
    //            None => return node.idx,
    //            Some (t) => 0,
            if node.parent == None {
                return node.idx
             }

            }
        0
    }
    pub fn depth(&self, idx: usize) -> usize {
        match self.arena[idx].parent {
            Some(id) => 1 + self.depth(id),
            None => 0,
        }
    }
    pub fn set_coords(&mut self)  {
        let i = 10.0;

    }

}

/// enum of the possible events in a gene tree
#[derive(Debug, PartialEq)]
pub enum Event {
    Speciation,
    Duplication,
    Loss,
    Transfer,
    Undef,
}
/// There  is no Default pour enum, we define one.
impl Default for Event {
    fn default() -> Self { Event::Undef }
}

/// Fill an ArenaTree structure with the contents of a GeneralTaxonomy structure
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
    println!("N = {} Name={:?}  Parent Name={:?} Parent Index={}",n,name,parent_name,parent_index);
    let initial_name = name.clone();
    let initial_parent_name = parent_name.clone();
    let name = "N".to_owned()+&n.to_string()+"_"+name;
    let parent_name = "N".to_owned()+&parent_index.to_string()+"_"+parent_name;
    let name = tree.new_node(name.to_string());

    let parent = tree.node(parent_name.to_string());
    tree.arena[parent].name = initial_parent_name.to_string();
    tree.arena[parent].children.push(name);
    tree.arena[name].parent = Some(parent);

    tree.arena[name].name = initial_name.to_string();
    for child in children {
        taxo2tree(& t,*child,  tree);
    }
}


pub fn set_tree_coords( tree: &mut ArenaTree<String>) {
let longueur = tree.arena.len();
let mut count = 0usize;
 loop {
     // tree.arena[count].set_x_noref(10.0* (count as f32));
      tree.arena[count].set_y_noref(15.0);
     tree.arena[count].set_y_noref(15.0* (count as f32)+30.0);
     tree.arena[count].set_x_noref(30.0* (count as f32));
    count += 1;

    if count == longueur {
        break;
    }
}
}
// pub fn preset_child_coords( tree: &mut ArenaTree<String>, index: usize) {
//     let x_father = tree.arena[index].x;
//     let y_father = tree.arena[index].y;
//     println!("Coords {} ",x_father);
//     let children  = &mut  tree.arena[index].children;
//     if (children.len() > 0) {
//         let mut left = -1;
//         let son_left = children[0];
//         let son_right = children[1];
//         tree.arena[son_left].set_x_noref(x_father - 90.0);
//         tree.arena[son_right].set_x_noref(x_father + 90.0);
//         if (tree.arena[son_right].x <= tree.arena[son_left].x+90.0) {
//             tree.arena[son_left].set_x_noref(x_father - 120.0);
//             tree.arena[son_right].set_x_noref(x_father + 120.0);
//
//         }
//         tree.arena[son_left].set_y_noref(y_father + 30.0);
//         tree.arena[son_right].set_y_noref(y_father + 30.0);
//         preset_child_coords( tree, son_left);
//         preset_child_coords( tree, son_right);
//     }
// }
// pub fn set_initial_y( tree: &mut ArenaTree<String>) {
//     let longueur = tree.arena.len();
//     let mut count = 0usize;
//     loop {
//         let  h = tree.depth(count);
//         println!("Hauteur du noud {} = {}", count,h);
//         tree.arena[count].set_y_noref(30.0* (h as f32));
//         count += 1;
//
//         if count == longueur {
//             break;
//         }
//     }
// }
// pub fn set_initial_x( tree: &mut ArenaTree<String>, index: usize) {
//     let initial_x_width = 150.0;
//     let children  = &mut  tree.arena[index].children;
//     if (children.len() > 2) {
//         panic!("L'arbre doit être binaire")
//     }
//     if (children.len() > 0) {
//         let son_left = children[0];
//         let son_right = children[1];
//         tree.arena[son_left].set_x_noref(0.0);
//         tree.arena[son_right].set_x_noref(1.0*initial_x_width);
//         set_initial_x( tree, son_left);
//         set_initial_x( tree, son_right);
//     }
// }
pub fn shift_initial_x( tree: &mut ArenaTree<String>, index: usize) {
    let x_father = tree.arena[index].x;
    println!("Adding {} ",x_father);
    let children  = &mut  tree.arena[index].children;
    println!("Chlidren {} ",children.len());
    if (children.len() > 2) {
        panic!("L'arbre doit être binaire")
    }
    if (children.len() > 0) {
        let son_left = children[0];
        let son_right = children[1];
        let x_left = tree.arena[son_left].x;
        tree.arena[son_left].set_x_noref(x_left + x_father);
        let x_right = tree.arena[son_right].x;
        tree.arena[son_right].set_x_noref(x_right + x_father);
        shift_initial_x( tree, son_left);
        shift_initial_x( tree, son_right);
    }
}
pub fn  pseudo_knuth_layout(tree: &mut ArenaTree<String>,index: usize){
    let longueur = tree.arena.len();
    let mut count = 0usize;
    loop {
        let  h = tree.depth(count);
        println!("Hauteur du noud {} = {}", count,h);
        tree.arena[count].set_y_noref(30.0* (h as f32));
        tree.arena[count].set_x_noref(30.0* (count as f32));
        count += 1;

        if count == longueur {
            break;
        }
    }
}

pub fn  knuth_layout(tree: &mut ArenaTree<String>,index: usize,depth: &mut usize){
    println!("Knuth before processing {:?}",tree.arena[index]);
    tree.arena[index].set_y_noref(30.0* (*depth as f32));
    println!("Knuth after processing {:?}",tree.arena[index]);
    let children  = &mut  tree.arena[index].children;
    if (children.len() > 2) {
        panic!("L'arbre doit être binaire")
    }
    // *depth += 1;
    if (children.len() > 0) {
        let son_left = children[0];
        let son_right = children[1];
        tree.arena[son_left].set_x_noref(0.0);
        tree.arena[son_right].set_x_noref(60.0);
        knuth_layout(tree,son_left,&mut(*depth+1));
        knuth_layout(tree,son_right,&mut(*depth+1));
    }
}
pub fn  set_father_between(tree: &mut ArenaTree<String>){

}
pub fn  postorder(tree: &mut ArenaTree<String>){
    let mut root = tree.get_root();
    explore_postorder(tree,root);
    let mut leftest = find_leftest(tree,root);
    println!("THE  LEFTEST IS  {:?}",tree.arena[leftest]);

    // find_leftest(tree,root);
}

pub fn  explore_postorder(tree: &mut ArenaTree<String>,index:usize) {
    let children  = &mut  tree.arena[index].children;
    if (children.len() > 0) {
        let left = children[0];
        let right = children[1];
        explore_postorder(tree,left);
        explore_postorder(tree,right);
        // println!("POST-ORDER INT {:?} : DEPTH = {}",tree.arena[index],tree.depth(index));
    }
    else{
        // println!("POST-ORDER LEAF {:?} : DEPTH = {}",tree.arena[index],tree.depth(index));
    }
}

pub fn  set_x_postorder(tree: &mut ArenaTree<String>,index:usize, x_coords: &mut Vec<usize>) {
    let children  = &mut  tree.arena[index].children;
    if (children.len() > 0) {
        let left = children[0];
        let right = children[1];

        set_x_postorder(tree,left,x_coords);
        set_x_postorder(tree,right,x_coords);
        x_coords[tree.depth(index)] += 1;
        println!("POST-ORDER INT {:?} : DEPTH = {} => {}",tree.arena[index],tree.depth(index),x_coords[tree.depth(index)]);
        let posx = x_coords[tree.depth(index)];
        println!("POST-ORDER INT {:?} : DEPTH = {} => {}",tree.arena[index],tree.depth(index),posx);
        tree.arena[index].set_x_noref((posx as f32) * 30.0);
    }
    else{
        x_coords[tree.depth(index)] += 1;
        let posx = x_coords[tree.depth(index)];
        println!("POST-ORDER LEAF {:?} : DEPTH = {} => {}",tree.arena[index],tree.depth(index),x_coords[tree.depth(index)]);
        tree.arena[index].set_x_noref((posx as f32) * 30.0);
    }
}

pub fn  set_middle_postorder(tree: &mut ArenaTree<String>,index:usize) {
    let children  = &mut  tree.arena[index].children;
    if (children.len() > 0) {
        let left = children[0];
        let right = children[1];
        let x_left = tree.arena[left].x;
        let x_right = tree.arena[right].x;
        let x_middle = ( x_right + x_left ) / 2.0 ;
        // println!("POST-ORDER MODIF FATHER {:?} X = {}",tree.arena[index],tree.arena[index].x);
        // println!("POST-ORDER MODIF FATHER  X LEFT = {} X RIGHT ={}",x_left, x_right);
        tree.arena[index].set_x_noref(x_middle);
        // println!("POST-ORDER MODIF FATHER {:?} NEW X = {}",tree.arena[index],tree.arena[index].x);

        set_middle_postorder(tree,left);
        set_middle_postorder(tree,right);
        // tree.arena[index].set_x_noref(x_middle);
        println!("POST-ORDER INT {:?}",tree.arena[index]);

    }
    else{
        println!("POST-ORDER LEAF {:?}",tree.arena[index]);
    }
}


pub fn  find_leftest(tree: &mut ArenaTree<String>,index:usize) -> usize {
    println!("FIND LEFTEST: {:?}",tree.arena[index]);
    let children  = &mut  tree.arena[index].children;
    if (children.len() > 0) {
        let left = children[0];
        find_leftest(tree,left)
    }
    else {
        index
    }
}
