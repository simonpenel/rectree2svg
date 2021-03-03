use taxonomy::Taxonomy;
use log::{info};
const BLOCK: f32 = 30.0;

// Structures
// ==========

/// Structure Noeud.
///
#[derive(Debug)]
pub struct Noeud<T>
where
    T: PartialEq
{
    pub idx: usize,
    val: T,
    pub name: String,
    pub parent: Option<usize>,
    pub children: Vec<usize>,
    pub x: f32,
    pub xmod: f32,
    pub y: f32,
    pub e: Event,
    pub width: f32,
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
            xmod: 0.0,
            y: 0.0,
            e: Event::Undef,
            width: BLOCK / 4.0,
        }
    }
    #[allow(dead_code)]
    pub fn get_val(&mut self) -> &T {
        &self.val
    }
    #[allow(dead_code)]
    pub fn get_index(&mut self) -> &usize {
        &self.idx
    }
    #[allow(dead_code)]
    pub fn get_x(&mut self) -> &f32 {
        &self.x
    }
    #[allow(dead_code)]
    pub fn get_y(&mut self) -> &f32 {
        &self.y
    }
    #[allow(dead_code)]
    pub fn get_coords(&mut self) -> ( &f32, &f32) {
        (&self.x,&self.y)
    }
    #[allow(dead_code)]
    pub fn set_x(&mut self, x: &f32)
    {
        self.x = *x;
    }

    pub fn set_x_noref(&mut self, x: f32)
    {
        self.x = x;
    }

    pub fn set_xmod_noref(&mut self, xmod: f32)
    {
        self.xmod = xmod;
    }
    #[allow(dead_code)]
    pub fn set_y(&mut self, y: &f32)
    {
        self.y = *y;
    }

    pub fn set_y_noref(&mut self, y: f32)
    {
        self.y = y;
    }
    #[allow(dead_code)]
    pub fn get_event(&mut self) -> &Event {
        &self.e
    }
    #[allow(dead_code)]
    pub fn set_event(&mut self, e: Event)
    {
        self.e = e;
    }
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
impl<T> ArenaTree<T>
where
    T: PartialEq
{
    /// Add a node and send its new index. If the
    /// node already exists, send its index.
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

    /// Add a node and send its new index. If the
    /// node already exists, send a panic alert.
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
    /// Send the index of the root
    #[allow(unreachable_code)]
    pub fn get_root(&mut self) -> usize {
        for node in &self.arena {
    //        match node.parent {
    //            None => return node.idx,
    //            Some (t) => 0,
            if node.parent == None {
                return node.idx
             }

            }
        panic!("Unable to get root of the tree");
        0
    }

    /// Send the depth of the tree
    pub fn depth(&self, idx: usize) -> usize {
        match self.arena[idx].parent {
            Some(id) => 1 + self.depth(id),
            None => 0,
        }
    }
    pub fn get_largest_x(&mut self) -> f32 {
        let mut max = 0.0;
        for node in &self.arena {
            if node.x > max {
                max = node.x;
             }
            }
        max
    }
    pub fn get_largest_y(&mut self) -> f32 {
        let mut max = 0.0;
        for node in &self.arena {
            if node.y > max {
                max = node.y;
             }
            }
        max
    }
    #[allow(dead_code)]
    pub fn rotate(&mut self)  {
        let root = self.get_root();
        let x_0 = self.arena[root].x;
        let y_0 = self.arena[root].y;
        for  node in &mut self.arena {
            let x = node.x;
            let y = node.y;
            node.x = y + -y_0 + x_0;
            node.y = -x + x_0 + y_0;
        }
    }
}

/// enum of the possible events in a gene tree
#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Event {
    Speciation,
    Duplication,
    Loss,
    Transfer,
    Leaf,
    Undef,
}

/// There  is no Default pour enum, we define one.
impl Default for Event {
    fn default() -> Self { Event::Undef }
}

// Fonctions
// =========

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
/// Fill an ArenaTree structure with the contents of a roxmltre::Node structure
pub fn xml2tree(node: roxmltree::Node, parent: usize, mut numero : &mut usize, mut  tree: &mut ArenaTree<String>) {
        // je cherche les fils
        let children = node.children();
         for child in children {
            if child.has_tag_name("clade"){
                    // increment le numero
                    *numero += 1;
                    // Nouveau nom
                    let name = "N".to_owned()+&numero.to_string();
                    //  index de ce nouveau nom
                    let name = tree.new_node(name.to_string());
                    //Ajoute ce noeud au parent
                    tree.arena[parent].children.push(name);
                    // Attribue un parent a ce noeud
                    tree.arena[name].parent = Some(parent);
                    // Explore le reste de l'arbre a partir de ce noeud
                    xml2tree(child, name, &mut numero, &mut tree);

            }
            // Attribue le nom defini dans le tag id
            if child.has_tag_name("id"){
                let nom = child.first_child().unwrap().text();
                match nom {
                    Some(text) => tree.arena[parent].name = text.to_string(),
                    None    => tree.arena[parent].name = "Unkwnown".to_string(),
                };
            }
            // Attribue le nom defini dans le tag name
            if child.has_tag_name("name"){
                let nom = child.first_child().unwrap().text();
                match nom {
                    Some(text) => tree.arena[parent].name = text.to_string(),
                    None    => tree.arena[parent].name = "Unkwnown".to_string(),
                };
            }
            // Attribue l evenement
            if child.has_tag_name("eventsRec"){
                info!("xml2tree:Event detected");
                for evenement in child.children() {
                        if evenement.has_tag_name("loss"){
                            info!("xml2tree:{:?}",evenement);
                            tree.arena[parent].set_event(Event::Loss);
                        }
                        if evenement.has_tag_name("leaf"){
                            info!("xml2tree:{:?}",evenement);
                            tree.arena[parent].set_event(Event::Leaf);
                        }
                        if evenement.has_tag_name("speciation"){
                            info!("xml2tree:{:?}",evenement);
                            tree.arena[parent].set_event(Event::Speciation);
                        }
                        if evenement.has_tag_name("duplication"){
                            info!("xml2tree:{:?}",evenement);
                            tree.arena[parent].set_event(Event::Duplication);
                        }



                // match nom {
                //     Some(text) => tree.arena[parent].name = text.to_string(),
                //     None    => tree.arena[parent].name = "Unkwnown".to_string(),
                // };
            }
                info!("xml2tree:Event closed");
                // println!("Event = {:?}",evenement);
            }



    }
}


// Renvoie le NodeId du premier tag "clade"
pub fn find_first_clade( mut doc: &mut roxmltree::Document) -> Result < roxmltree::NodeId, usize> {
    let mut descendants = doc.root().descendants();
    // Search for the first occurnce of clade tag
    for  node in descendants {
        if node.has_tag_name("clade"){
            // return Ok(node.id().get())
            return Ok(node.id())
        }
    }
    Err(0)
}
// Renvoie le NodeId du premier tag "spTree"
pub fn find_spTree( mut doc: &mut roxmltree::Document) -> Result < roxmltree::NodeId, usize> {
    let mut descendants = doc.root().descendants();
    // Search for the first occurnce of clade spTree
    for  node in descendants {
        if node.has_tag_name("spTree"){
            // return Ok(node.id().get())
            return Ok(node.id())
        }
    }
    Err(0)
}

//
// pub fn find_first_tag( mut doc: &mut roxmltree::Document, tag: String) -> Result < roxmltree::NodeId, usize> {
// let mut descendants = doc.root().descendants();
// // Search for the first occurnce of clade tag
// for  node in descendants {
//     if node.has_tag_name(tag){
//         // return Ok(node.id().get())
//         return Ok(node.id())
//     }
// }
// Err(0)
// }



/// Set x and y of nodes :  left son x is 0;  right son x is 1; y is depth
pub fn  knuth_layout(tree: &mut ArenaTree<String>,index: usize,depth: &mut usize){
    tree.arena[index].set_y_noref(BLOCK* (*depth as f32));
    let children  = &mut  tree.arena[index].children;
    if children.len() > 2 {
        panic!("L'arbre doit être binaire")
    }
    if children.len() > 0 {
        let son_left = children[0];
        let son_right = children[1];
        tree.arena[son_left].set_x_noref(0.0);
        tree.arena[son_right].set_x_noref(BLOCK);
        knuth_layout(tree,son_left,&mut(*depth+1));
        knuth_layout(tree,son_right,&mut(*depth+1));
    }
}

/// Transforms the tree into cladogram
pub fn cladogramme( tree: &mut ArenaTree<String>) {
    let root = tree.get_root();
    let mut  max_depth = get_maxdepth(tree,root,&mut 0);
    set_leaves_to_bottom(tree,root,&mut max_depth);
}

/// Get the depth of the tree
pub fn get_maxdepth( tree: &mut ArenaTree<String>, index:usize, max :&mut usize) -> usize {
    let children  = &mut  tree.arena[index].children;
    if children.len() > 0 {
        let son_left = children[0];
        let son_right = children[1];
        if  tree.depth(son_left) > *max {
            *max =  tree.depth(son_left);
        }
        if  tree.depth(son_right) > *max {
            *max =  tree.depth(son_right);
        }
         get_maxdepth(tree,son_left,max);
         get_maxdepth(tree,son_right,max);
    }
    *max
}

/// Set the y values of the leaves of the node index to  max value
pub fn set_leaves_to_bottom( tree: &mut ArenaTree<String>, index: usize, max:&mut  usize) {
    let children  = &mut  tree.arena[index].children;
    if children.len() > 0 {
        let son_left = children[0];
        let son_right = children[1];
        set_leaves_to_bottom(tree,son_left,max);
        set_leaves_to_bottom(tree,son_right,max);
    }
    else {
        match tree.arena[index].e {
            Event::Loss => tree.arena[index].set_y_noref(BLOCK* (*max as f32 )),
            _ => tree.arena[index].set_y_noref(BLOCK* (*max as f32 + 1.0)),
        };
        // tree.arena[index].set_y_noref(BLOCK* (*max as f32 + 1.0));
    }
}

/// Shift the  x values  of a node and its children according to the cumulated xmod values
pub fn shift_mod_x( tree: &mut ArenaTree<String>, index: usize, xmod: &mut f32) {
    info!("shift_mod_x: shifting {:?} xmod={}",tree.arena[index],xmod);
    let x_father = tree.arena[index].x;
    let  xmod_father = tree.arena[index].xmod;
    let mut xmod = *xmod + xmod_father;
    tree.arena[index].set_x_noref(x_father+xmod);
    tree.arena[index].set_xmod_noref(xmod);
    let children  = &mut  tree.arena[index].children;
    if children.len() > 2 {
        panic!("L'arbre doit être binaire")
    }
    if children.len() > 0 {
        let son_left = children[0];
        let son_right = children[1];
        shift_mod_x( tree, son_left, &mut xmod);
        shift_mod_x( tree, son_right, &mut xmod);
    }
}

#[allow(dead_code)]
/// Traverse the tree using post-order traversal
pub fn  postorder(tree: &mut ArenaTree<String>){
    let root = tree.get_root();
    explore_postorder(tree,root);
}

#[allow(dead_code)]
/// Traverse the tree using post-order traversal starting from a given node  defined by its index
pub fn  explore_postorder(tree: &mut ArenaTree<String>,index:usize) {
    let children  = &mut  tree.arena[index].children;
    if children.len() > 0 {
        let left = children[0];
        let right = children[1];
        explore_postorder(tree,left);
        explore_postorder(tree,right);
        println!("POST-ORDER TRAVERSAL : INTERNAL NODE  {:?} / DEPTH = {}",tree.arena[index],tree.depth(index));
    }
    else{
        println!("POST-ORDER TRAVERSAL : LEAF           {:?} / DEPTH = {}",tree.arena[index],tree.depth(index));
    }
}

/// Solve the conflicts between the left subtree and the right subtree
pub fn  check_contour_postorder(tree: &mut ArenaTree<String>,index:usize) {
    let children  = &mut  tree.arena[index].children;
    if children.len() > 0 {
        let left = children[0];
        let right = children[1];
        check_contour_postorder(tree,left);
        check_contour_postorder(tree,right);
        push_right(tree,left,right);
    }
    else{
    }
}

/// Get the left 'contout' of a sub tree
pub fn  get_contour_left(tree: &mut ArenaTree<String>,index:usize,depth:usize,contour_left: &mut Vec<f32>,parent_xmod: f32)  {
    info!("get_contour_left: process node {:?}",tree.arena[index]);
    let local_depth = tree.depth(index)-depth; // Profondeur du noeud pa rapport a noeud de depart
    if contour_left.len() <= local_depth {
        if tree.arena[index].xmod < 0.0 {
            panic!("error: negative xmod");
        }
        contour_left.push(tree.arena[index].x+tree.arena[index].xmod+parent_xmod);
        info!("get_contour_left: increment contour is now {:?}",contour_left);
    }
    if tree.arena[index].xmod < 0.0 {
        panic!("erreur: negative  xmod");
    }
    info!("get_contour_left: compare  {} + {} + {} infeq ctl at depth {} : {} ",tree.arena[index].x, tree.arena[index].xmod,parent_xmod,local_depth, contour_left[local_depth] );
    if tree.arena[index].x + tree.arena[index].xmod + parent_xmod <= contour_left[local_depth] {
        contour_left[local_depth] = tree.arena[index].x + tree.arena[index].xmod + parent_xmod;
        info!("get_contour_left: contour is now {:?}",contour_left);
    }
    let children  = &mut  tree.arena[index].children;
    if children.len() > 0 {
        let left = children[0];
        get_contour_left(tree,left,depth,contour_left,tree.arena[index].xmod + parent_xmod );
    }
}

/// Get the right 'contout' of a sub tree
pub fn  get_contour_right(tree: &mut ArenaTree<String>,index:usize,depth:usize,contour_right: &mut Vec<f32>,parent_xmod: f32)  {
    info!("get_contour_right: process node {:?}",tree.arena[index]);
    let local_depth = tree.depth(index)-depth; // Profondeur du noeud pa rapport a noeud de depart
    if contour_right.len() <= local_depth {
        if tree.arena[index].xmod < 0.0 {
            panic!("erreur: negative xmod");
        }
        contour_right.push(tree.arena[index].x+tree.arena[index].xmod+parent_xmod);
            info!("get_contour_right: increment contour is now {:?}",contour_right);
    }
    if tree.arena[index].xmod < 0.0 {
        panic!("erreur: negative xmod");
    }
    info!("get_contour_right: compare  {} + {} + {} infeq ctl at depth {} : {} ",tree.arena[index].x, tree.arena[index].xmod,parent_xmod,local_depth, contour_right[local_depth] );
    if tree.arena[index].x +  tree.arena[index].xmod + parent_xmod  >= contour_right[local_depth] {
        contour_right[local_depth] = tree.arena[index].x +  tree.arena[index].xmod + parent_xmod ;
            info!("get_contour_right: contour is now {:?}",contour_right);
    }
    let children  = &mut  tree.arena[index].children;
    if children.len() > 0 {
        let right = children[1];
        get_contour_right(tree,right,depth,contour_right,tree.arena[index].xmod + parent_xmod );
    }
}

/// Check for conficts between subtrees and shift conflicting right-hand subtrees to the right
/// in order to solve detected  conflicts.
pub fn  push_right(tree: &mut ArenaTree<String>,left_tree:usize,right_tree:usize) -> f32 {
    info!("push_right: compare right contour of {} and left contour of {}",left_tree, right_tree);
    let mut right_co_of_left_tr  = vec![tree.arena[left_tree].x+tree.arena[left_tree].xmod]; //contour droit de l'arbre de gauche
    let depth_left_tr  = tree.depth(left_tree);
    get_contour_right(tree,left_tree,depth_left_tr,&mut right_co_of_left_tr,0.0);
    info!("push_right: right contour of {} = {:?}",left_tree,right_co_of_left_tr);
    let mut left_co_of_right_tr  = vec![tree.arena[right_tree].x+tree.arena[right_tree].xmod]; //contour droit de l'arbre de gauche
    let depth_right_tr  = tree.depth(right_tree);
    get_contour_left(tree,right_tree,depth_right_tr,&mut left_co_of_right_tr,0.0);
    info!("push_right: left contour of {} = {:?}",right_tree,left_co_of_right_tr);
    // Si on   a pas le meme longeur de contour on complete le plus petit
    // en remplissant ce qui manque avec la derniere valeur, pour eviter
    // qu'un sous arbre vosin se place sous une feuille
    let right_len = right_co_of_left_tr.len();
    let left_len = left_co_of_right_tr.len();
    if left_len > right_len {
        let last_val =  right_co_of_left_tr[right_len-1];
        let last_vals =  vec![last_val;left_len-right_len];
        right_co_of_left_tr.extend(last_vals.iter().copied());
        info!("push_right: complete right contour with last value {}", last_val);
    }
    if left_len < right_len {
        let last_val =  left_co_of_right_tr[left_len-1];
        let last_vals =  vec![last_val;right_len - left_len];
        left_co_of_right_tr.extend(last_vals.iter().copied());
        info!("push_right: complete left contour with last value {}", last_val);
    }
    info!("push_right: comparing  right cont. of left tree: {:?}",right_co_of_left_tr);
    info!("push_right: with left cont. of right tree:       {:?} ",left_co_of_right_tr);

    let iter = left_co_of_right_tr.iter().zip(right_co_of_left_tr).map(|(x, y )| (x-y));
    let shift = iter.min_by(|x, y| (*x as i64) .cmp(&(*y as i64 )));
    info!("push_right: distance max  = {:?}",shift);
    match shift {
        Some(val) => {
            info!("push_right: distance max  = {:?}",shift);
            if val <= 0.0 {// bidouilel
                info!("push_right: ================CONFLIT==========");
                info!("push_right: Modify node {:?}",tree.arena[right_tree]);
                let x_mod =  tree.arena[right_tree].xmod;
                info!("push_right: initial x_mod = {}",x_mod);
                let x_mod =  x_mod -1.0 *val + BLOCK ;//bidouille
                info!("push_right: new x_mod = {}",x_mod);
                tree.arena[right_tree].set_xmod_noref(x_mod);
                info!("push_right: updated node {:?}",tree.arena[right_tree]);
                info!("push_right: ================CONFLIT==========");
            }
        },
        None => {}
    }
    0.0
}

/// Set the x of the father between its chlidren
pub fn  set_middle_postorder(tree: &mut ArenaTree<String>,index:usize) {
    let children  = &mut  tree.arena[index].children;
    if children.len() > 0 {
        let left = children[0];
        let right = children[1];
        set_middle_postorder(tree,left);
        set_middle_postorder(tree,right);
        info!("set_middle_postorder: node {:?}",index);
        let x_left = tree.arena[left].x;
        let x_right = tree.arena[right].x;
        let x = tree.arena[index].x;
        let x_middle = ( x_right + x_left ) / 2.0 ;
        info!("set_middle_postorder: x father set from {} to {}",x,x_middle);
        tree.arena[index].set_x_noref(x_middle);
        let x_mod =  tree.arena[right].xmod;
        let x_mod =  x_mod + x_middle - x;
        tree.arena[index].set_xmod_noref(x_mod);

    }
}
