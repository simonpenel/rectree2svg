use taxonomy::Taxonomy;
use log::{info};
pub const BLOCK: f32 = 60.0;
pub const PIPEBLOCK: f32 = BLOCK / 4.0;
// Structures
// ==========

/// Structure Noeud.
///
#[derive(Debug)]
pub struct Noeud<T>
where
    T: PartialEq
{
    pub idx: usize,             // index dans la structure
    val: T,                     // valeur unique dans la structure
    pub name: String,           // nom du noeud ou de la feuille
    pub parent: Option<usize>,  // index du parent
    pub children: Vec<usize>,   // indexes des enfants
    pub x: f32,                 // coordonnee x (avant rotation 90 svg)
    pub xmod: f32,              // decalage x a ajouter a x
    pub y: f32,                 // coordonnee y (avant rotation 90 svg)
    pub ymod: f32,              // decalage y a ajouter a y (pour les arbres reconcilies)
    pub l: f32,                 // longueur de branche lue dans le fichier
    pub e: Event,               // evenement (dans le cas d'arbre de gene) Duplication, Loss, etc.
    pub location: String,       // SpeciesLocaton associe evenement (dans le cas d'arbre de gene)
    pub width: f32,             // largeur du tuyeau (dans le cas d'arbre d'espece)
    pub height: f32,            // hauteur du tuyeau (dans le cas d'arbre d'espece)
    pub nbg: usize,             // nombre de noeud  d'arbre de genes associcés à ce noeud  (dans le cas d'arbre d'espece)
    pub nodes: Vec<(usize,usize)>,      // gene nodes associes (dans le cas d'arbre d'espece)
    pub is_a_transfert: bool,    // the node come from a tarnsfert, i.e. he is a transferBack and
                                // his parent is a BranchingOut . Since more than 1 event is
                                // associated to the node in xml (as transferBack+leaf) and only one is
                                // associated to the node in the structure ( here leaf), this is useful
                                // for drawing the transfer.
}

impl<T> Noeud<T>
where
    T: PartialEq
{
    pub fn new(idx: usize, val: T) -> Self {
        Self {
            idx,
            val,
            name: String::from("Undefined"),
            parent: None,
            children: vec![],
            x: 0.0,
            xmod: 0.0,
            y: 0.0,
            ymod: 0.0,
            l: 0.0,
            e: Event::Undef,
            location: String::from("Undefined"),
            width: PIPEBLOCK ,
            height: PIPEBLOCK ,
            nbg: 0,
            nodes: vec![],
            is_a_transfert:false,
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
    pub fn set_ymod_noref(&mut self, ymod: f32)
    {
        self.ymod = ymod;
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

    //  Get index from name
    pub fn get_index(&mut self, name: String) -> usize {
        for node in &self.arena {
    //        match node.parent {
    //            None => return node.idx,
    //            Some (t) => 0,
            if node.name == name {
                return node.idx
             }

            }
        panic!("Unable to find {} in the tree",name);
    }

    //A AMELIORER : RENVOYER UN RESULTS
    /// Get the index of the root.
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

    /// Check if the node is a leaf.
    pub fn is_leaf(&self, idx: usize) -> bool {
        match self.arena[idx].children.len() {
        0 => true,
        _ => false,
        }
    }
    /// Check if the node is a left node.
    #[allow(dead_code)]
    pub fn is_left(&self, idx: usize) -> bool {
        match self.arena[idx].parent {
        Some(p) => {
            let children = &self.arena[p].children;
            idx == children[0]
        },
        None => false,
        }
    }
    /// Check if the node is the root.
    #[allow(dead_code)]
    pub fn is_root(&self, idx: usize) -> bool {
        match self.arena[idx].parent {
        Some(_p) => false,
        None => true,
        }
    }


    /// Get the depth of the tree.
    pub fn depth(&self, idx: usize) -> usize {
        match self.arena[idx].parent {
            Some(id) => 1 + self.depth(id),
            None => 0,
        }
    }

    /// Get the largest x value of a tree.
    pub fn get_largest_x(&mut self) -> f32 {
        let mut max = 0.0;
        for node in &self.arena {
            if node.x + node.width / 2.0  > max {
                max = node.x + node.width / 2.0 ;
                }
            }
        max
    }

    /// Get the largest y value of a tree.
    pub fn get_largest_y(&mut self) -> f32 {
        let mut max = 0.0;
        for node in &self.arena {
            if node.y  + node.height / 2.0  > max {
                max = node.y  + node.height / 2.0  ;
             }
            }
        max
    }

    /// Get the smallest x value of a tree.
    pub fn get_smallest_x(&mut self) -> f32 {
        let mut min = 1000000.0;
        for node in &self.arena {
            if node.x - node.width / 2.0  < min {
                min = node.x  - node.width / 2.0 ;
             }
            }
        min
    }

    /// Get the smallest y value of a tree.
    pub fn get_smallest_y(&mut self) -> f32 {
        let mut min = 1000000.0;
        for node in &self.arena {
            if node.y   - node.height / 2.0  < min {
                min = node.y  - node.height / 2.0;
             }
            }
        min
    }
}
/// enum of the possible events in a gene tree
#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Event {
    Speciation,
    Duplication,
    Loss,
    BranchingOut,
    TransferBack,
    BifurcationOut,
    Leaf,
    ObsoleteSpeciationLoss,
    Undef,
}
/// There  is no Default pour enum, we define one.
impl Default for Event {
    fn default() -> Self { Event::Undef }
}

//  Structure Options
#[derive(Debug)]
pub struct Options{
    pub gene_internal :bool,
    pub species_internal :bool,
    pub scale :f32,
}
impl Options {
    pub fn new() -> Self {
        Self {
            gene_internal:false,
            species_internal:false,
            scale:1.0,
        }
    }
}

// Fonctions
// =========

/// Fill an ArenaTree structure with the contents of a parentheses  tree.
pub fn newick2tree(arbre:String, tree : &mut ArenaTree<String>, index:usize, num: &mut usize) {
    let (left,right,trail) = find_left_right(arbre);
    println!("LEFT = {} RIGHT = {} TRAIL = {}",left,right,trail);
    match trail.find(':') {
        Some(j) => {
            println!("Name of father          : {}",trail[0..j].to_string());
            // println!("Length of father branch : {}",trail[j+1..].to_string().parse::<f32>().unwrap());
            tree.arena[index].l = trail[j+1..].to_string().parse::<f32>().unwrap();
            tree.arena[index].name = trail[0..j].to_string();

        },
        None => {},
    };

    match  left.find(',') {
        Some(_i)=> {
            *num = *num + 1;
            //  Nouveau noeud
            let name = "NODE_".to_string()+&num.to_string();
            let new_index = tree.new_node(name.to_string());
            // tree.arena[new_index].name = int_name.to_string();
            tree.arena[index].children.push(new_index);
            tree.arena[new_index].parent = Some(index);
            newick2tree(left, tree,new_index,num);
        }
            ,
        None => {
            println!("{} is a leaf",left);
            *num = *num + 1;
            let name = "NODE_".to_string()+&num.to_string();
            let left_index = tree.new_node(name);
            tree.arena[index].children.push(left_index);
            tree.arena[left_index].parent = Some(index);
            match  left.find(':') {
                Some(i)=> {
                    tree.arena[left_index].name = left[0..i].to_string();
                    tree.arena[left_index].l = left[i+1..].to_string().parse::<f32>().unwrap();
                },
                None => {
                    tree.arena[left_index].name = left;
                },
            }
        },
    };
    match  right.find(',') {
        Some(_i)=> {
            *num = *num + 1;
            let name = "NODE_".to_string()+&num.to_string();
            let new_index = tree.new_node(name.to_string());
            tree.arena[index].children.push(new_index);
            tree.arena[new_index].parent = Some(index);
            newick2tree(right, tree,new_index,num);
        },
        None => {
            println!("{} is a leaf",right);
            *num = *num + 1;
            let name = "NODE_".to_string()+&num.to_string();
            let right_index = tree.new_node(name);
            tree.arena[index].children.push(right_index);
            tree.arena[right_index].parent = Some(index);
            match  right.find(':') {
                Some(i)=> {
                    println!("Name is : {}",right[0..i].to_string());
                    tree.arena[right_index].name = right[0..i].to_string();
                    tree.arena[right_index].l = right[i+1..].to_string().parse::<f32>().unwrap();
                },
                None => {
                    tree.arena[right_index].name = right;
                },
            }
        },
    };
}

/// Split a parenthesed tree into  left and right  parenthsed trees and trailing  string
pub fn find_left_right(arbre:String)-> (String,String,String){
    let mut len = arbre.len() - 1;
    if &arbre[len..] == "\n" {
        len -= 1;
    }
    assert_eq!(&arbre[..1],"(");
    let mut left:std::string::String = String::new();
    let mut right:std::string::String = String::new();
    let mut num_par = 0;
    let mut i = 0;
    for char in arbre.chars() {
        i += 1;
         match char {
             '(' => num_par += 1,
             ')' => num_par -= 1,
             ',' => {
                 if num_par == 1 {
                     break
                 }
             },
             _ => {}
         };
    }
    left = (&arbre[1..i-1]).to_string();
    right = (&arbre[i..len]).to_string();

    let trail =  match right.rfind(')'){
        Some(k) =>  right[k+1..].to_string(),
        None => "".to_string(),
    };
    let right_clean =  match right.rfind(')'){
        Some(k) =>  right[..k].to_string(),
        None => right[..].to_string(),
    };
    (left,right_clean,trail)
}

/// Fill an ArenaTree structure with the contents of a GeneralTaxonomy structure.
pub fn taxo2tree(t: &taxonomy::GeneralTaxonomy, n: usize, tree: &mut ArenaTree<String>) {
    let children = &t.children(n).expect("Pas de fils");
    let name = t.from_internal_id(n).expect("Pas de nom");
    let parent = t.parent(n).expect("Pas de parent");
    let parent_name = match parent {
        None => "root",
        Some ((id, _dist)) => t.from_internal_id(id).expect("Pas de nom")
    };
    let parent_dist = match parent {
        None => -1.0,
        Some ((_id, dist)) => {
            dist
        },
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
    tree.arena[name].l = parent_dist;
    tree.arena[name].name = initial_name.to_string();
    for child in children {
        taxo2tree(& t,*child,  tree);
    }
}

/// Fill an ArenaTree structure with the contents of a roxmltre::Node structure
pub fn xml2tree(node: roxmltree::Node, parent: usize, mut numero : &mut usize, mut  tree: &mut ArenaTree<String>) {
    // Note : speciationLoss", "speciationOutLoss", "speciationOut" sont obsolètes (vieux fomat recphyloxml)
    // On essaie de les traiter correctement
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
            let mut event_num = 0; // Le nb d'evenements dans balise eventsRec
            let mut sploss_num = 0; // pour le format obsolete
            let mut current_sploss_name = parent;
            for evenement in child.children() {
                if evenement.has_tag_name("speciationLoss"){
                    info!("[xml2tree] Find obsolete tag speciationLoss (# {})",sploss_num);
                    info!("[xml2tree] Index of current father (current_sploss_name)  {}",current_sploss_name);
                    // panic!("Warning: taxon 'speciationOutLoss' is obsolete");
                    // event_num += 1;
                    sploss_num += 1;
                    if sploss_num == 1 {
                        current_sploss_name = match tree.arena[parent].parent {
                        Some(p) => p,
                        None => panic!("No parent! Unable to process obsolete format"),
                    };
                    }
                    // increment le numero
                    *numero += 1;
                    // Nouveau nom
                    let sploss_name = "ADDED_SPECLOSS".to_owned()+&numero.to_string();
                    info!("[xml2tree] Create new node sploss {}",sploss_name);
                    //  index de ce nouveau nom
                    let sploss_name = tree.new_node(sploss_name.to_string());
                    info!("[xml2tree] New node sploss : {:?}",tree.arena[sploss_name]);
                    info!("[xml2tree] Adding sploss {} to current_sploss {}",sploss_name,current_sploss_name);
                    info!("[xml2tree] Node current_sploss was {:?}",tree.arena[current_sploss_name]);
                    tree.arena[current_sploss_name].children.push(sploss_name);
                    tree.arena[sploss_name].parent = Some(current_sploss_name);
                    info!("[xml2tree] Node current_sploss is now {:?}",tree.arena[current_sploss_name]);
                    info!("[xml2tree] Node sploss is now {:?}",tree.arena[sploss_name]);

                    // Nouveau nom
                    let loss_name = "ADDED_LOSS".to_owned()+&numero.to_string();
                    info!("[xml2tree] Create new node loss {}",loss_name);
                    //  index de ce nouveau nom
                    let loss_name = tree.new_node(loss_name.to_string());
                    info!("[xml2tree] New node loss : {:?}",tree.arena[loss_name]);

                    info!("[xml2tree] Adding loss {} to sploss {}",loss_name,sploss_name);
                    info!("[xml2tree] Node sploss was {:?}",tree.arena[sploss_name]);

                    tree.arena[sploss_name].children.push(loss_name);
                    tree.arena[loss_name].parent = Some(sploss_name);
                    info!("[xml2tree] Node sploss is now {:?}",tree.arena[sploss_name]);
                    info!("[xml2tree] Node loss is now {:?}",tree.arena[loss_name]);
                    info!("[xml2tree] Setting event of loss node: Loss");
                    tree.arena[loss_name].set_event(Event::Loss);
                    tree.arena[loss_name].name = "Added Loss".to_string();
                    info!("[xml2tree] Node loss is now {:?}",tree.arena[loss_name]);
                    info!("[xml2tree] event Nb {} = {:?}",event_num,evenement);
                    info!("[xml2tree] Setting event of sploss node: ObsoleteSpeciationLoss ");
                    tree.arena[sploss_name].set_event(Event::ObsoleteSpeciationLoss);
                    tree.arena[sploss_name].name = "SpecOutLoss".to_string();
                    info!("[xml2tree] Node sploss is now {:?}",tree.arena[sploss_name]);
                    info!("[xml2tree] Check current_sploss {:?}",tree.arena[current_sploss_name]);
                    assert!(evenement.has_attribute("speciesLocation"));
                    assert_eq!(evenement.attributes()[0].name(),"speciesLocation");
                    let location = evenement.attributes()[0].value();
                    info!("[xml2tree] Location is {}",location);
                    info!("[xml2tree] Set Location of sploss_name:");
                    tree.arena[sploss_name].location = location.to_string();
                    info!("[xml2tree] Node sploss_name is now {:?}:",tree.arena[sploss_name]);
                    // BIDON, JUSTE POUR VOIR
                    // tree.arena[loss_name].location = location.to_string();
                    current_sploss_name = sploss_name;
                    info!("[xml2tree] New index of current father (current_sploss_name)  {}",current_sploss_name);
                }
                if evenement.has_tag_name("speciationOutLoss"){
                    panic!("Warning: taxon 'speciationOutLoss' is obsolete");
                }
                if evenement.has_tag_name("speciationOut"){
                    panic!("Warning: taxon 'speciationOut' is obsolete");
                }
                    if evenement.has_tag_name("loss"){
                        event_num += 1;
                        info!("xml2tree: event Nb {} = {:?}",event_num,evenement);
                        tree.arena[parent].set_event(Event::Loss);
                        info!("xml2tree: setting event of {:?} : {:?}",tree.arena[parent].name,tree.arena[parent].e);
                        assert!(evenement.has_attribute("speciesLocation"));
                        assert_eq!(evenement.attributes()[0].name(),"speciesLocation");
                        let location = evenement.attributes()[0].value();
                        tree.arena[parent].location = location.to_string();
                    }
                    if evenement.has_tag_name("leaf"){
                        event_num += 1;
                        info!("xml2tree: event Nb {} = {:?}",event_num,evenement);
                        // TODO
                        // C'est une feuille mais pas forcement  le premier evenement:
                        // <eventsRec>
                        //   <leaf speciesLocation="5"></leaf>
                        // </eventsRec>
                        //  mais dans les autres cas
                        // <eventsRec>
                        //   <transferBack destinationSpecies="4"></transferBack>
                        //   <leaf speciesLocation="4"></leaf>
                        // </eventsRec>
                        //  On va ecraser l'info  transferBack, mais celle-ci a ete stockée dans
                        //  le champs is_a_transfert
                        tree.arena[parent].set_event(Event::Leaf);
                        info!("xml2tree: setting event of {:?} : {:?}",tree.arena[parent].name,tree.arena[parent].e);
                        info!("Attributes of {:?} are {:?}",evenement,evenement.attributes());
                        let nb_att = evenement.attributes().len();
                        info!("Number of attributes  = {}",nb_att);
                        assert!(evenement.has_attribute("speciesLocation"));
                        if nb_att == 1 {
                            assert_eq!(evenement.attributes()[0].name(),"speciesLocation");
                            let location = evenement.attributes()[0].value();
                            tree.arena[parent].location = location.to_string();
                        }
                        else {
                            // TODO tres sale verifier : on a les deux possibilites:
                            // <leaf speciesLocation="Lentisphaerae" geneName="Lentisphaerae"></leaf>
                            // <leaf geneName="a2_a" speciesLocation="a"></leaf>
                            assert!(nb_att == 2);
                            assert!(evenement.has_attribute("geneName"));
                            assert!(evenement.has_attribute("speciesLocation"));
                            if evenement.attributes()[0].name() == "geneName" {
                                assert_eq!(evenement.attributes()[1].name(),"speciesLocation");
                                let name = evenement.attributes()[0].value();
                                tree.arena[parent].name = name.to_string();
                                info!("set name {:?}",tree.arena[parent]);
                                let location = evenement.attributes()[1].value();
                                tree.arena[parent].location = location.to_string();
                            }
                            if evenement.attributes()[1].name() == "geneName" {
                                assert_eq!(evenement.attributes()[0].name(),"speciesLocation");
                                let name = evenement.attributes()[1].value();
                                tree.arena[parent].name = name.to_string();
                                info!("set name {:?}",tree.arena[parent]);
                                let location = evenement.attributes()[0].value();
                                tree.arena[parent].location = location.to_string();
                            }
                        }
                    }
                    if evenement.has_tag_name("speciation"){
                        event_num += 1;
                        info!("xml2tree: event Nb {} = {:?}",event_num,evenement);
                        // TODO
                        // Traitement speciationloss:
                        if sploss_num > 0 {
                            panic!("Old format not supported yet");
                        }

                        tree.arena[parent].set_event(Event::Speciation);
                        info!("xml2tree: setting event of {:?} : {:?}",tree.arena[parent].name,tree.arena[parent].e);
                        info!("Attributes of {:?} are {:?}",evenement,evenement.attributes());
                        assert!(evenement.has_attribute("speciesLocation"));
                        assert_eq!(evenement.attributes()[0].name(),"speciesLocation");
                        let location = evenement.attributes()[0].value();
                        info!("xml2tree: set location = {}",location);
                        tree.arena[parent].location = location.to_string();
                    }
                    if evenement.has_tag_name("duplication"){
                        event_num += 1;
                        info!("xml2tree: event Nb {} = {:?}",event_num,evenement);
                        tree.arena[parent].set_event(Event::Duplication);
                        info!("xml2tree: setting event of {:?} : {:?}",tree.arena[parent].name,tree.arena[parent].e);
                        info!("Attributes of {:?} are {:?}",evenement,evenement.attributes());
                        assert!(evenement.has_attribute("speciesLocation"));
                        assert_eq!(evenement.attributes()[0].name(),"speciesLocation");
                        let location = evenement.attributes()[0].value();
                        info!("xml2tree: set location = {}",location);
                        tree.arena[parent].location = location.to_string();

                    }
                    if evenement.has_tag_name("branchingOut"){
                        event_num += 1;
                        info!("xml2tree: event Nb {} = {:?}",event_num,evenement);
                        tree.arena[parent].set_event(Event::BranchingOut);
                        info!("xml2tree: setting event of {:?} : {:?}",tree.arena[parent].name,tree.arena[parent].e);
                        info!("Attributes of {:?} are {:?}",evenement,evenement.attributes());
                        assert!(evenement.has_attribute("speciesLocation"));
                        assert_eq!(evenement.attributes()[0].name(),"speciesLocation");
                        let location = evenement.attributes()[0].value();
                        info!("xml2tree: set location = {}",location);
                        tree.arena[parent].location = location.to_string();
                    }
                    // TODO
                    // a verifier
                    if evenement.has_tag_name("transferBack"){
                        // Ici on plusieurs evenements
                        // Par exemple
                        // <eventsRec>
                        // <transferBack destinationSpecies="5"></transferBack>
                        // <branchingOut speciesLocation="5"></branchingOut>
                        // </eventsRec>
                        // ou
                        // <eventsRec>
                        // <transferBack destinationSpecies="10"></transferBack>
                        // <speciation speciesLocation="10"></speciation>
                        // </eventsRec>
                        // Le destinationSpecies est donc l'emplacement ou doit etre
                        // le noeud représentant l'arivee du transfert
                        // le point de depart du transfer etant le pere de ce noeud
                        event_num += 1;
                        info!("xml2tree: event Nb {} = {:?}",event_num,evenement);
                        tree.arena[parent].set_event(Event::TransferBack); // a priori ce ne sera pas conserve
                        info!("xml2tree: setting event of {:?} : {:?}",tree.arena[parent].name,tree.arena[parent].e);
                        info!("Attributes of {:?} are {:?}",evenement,evenement.attributes());
                        assert!(evenement.has_attribute("destinationSpecies"));
                        assert_eq!(evenement.attributes()[0].name(),"destinationSpecies");
                        let location = evenement.attributes()[0].value();
                        info!("xml2tree: set destinationSpecies = {}",location);
                        tree.arena[parent].location = location.to_string();
                        tree.arena[parent].is_a_transfert = true;
                    }
                    // TODO
                    if evenement.has_tag_name("bifurcationOut"){
                        event_num += 1;
                        info!("xml2tree: event Nb {} = {:?}",event_num,evenement);
                        tree.arena[parent].set_event(Event::BifurcationOut);
                        info!("Attributes of {:?} are {:?}",evenement,evenement.attributes());
                        let grandparent =  tree.arena[parent].parent;
                        match grandparent {
                            Some(p)     => {
                                let location =  &tree.arena[p].location;
                                info!("xml2tree: set location according to its father = {}",location);
                                tree.arena[parent].location = location.to_string();},
                            None        => panic!("BifurcationOut node as no parent : {:?}",tree.arena[parent]),
                        };
                        tree.arena[parent].is_a_transfert = true;
                        //  A verifier
                        // Meme espece que son pere
                        // assert!(evenement.has_attribute("destinationSpecies"));
                        // assert_eq!(evenement.attributes()[0].name(),"destinationSpecies");
                        // let location = evenement.attributes()[0].value();
                        // tree.arena[parent].location = location.to_string();
                    }
                }
        info!("xml2tree:Event closed");
        }
    }
}

// chec
pub fn check_for_obsolete( gene_tree:&mut ArenaTree<String>, species_tree:&mut ArenaTree<String>) {
    info!("[check_for_obsolete] Initial gene tree {:?}",&gene_tree);
    let mut obs = Vec::new();
    for  index in &mut gene_tree.arena {
        if index.e == Event::ObsoleteSpeciationLoss {
            info!("[check_for_obsolete] Find ObsoleteSpeciationLoss (OSL): {:?}",index);
            let parent = match index.parent {
                Some(p) => p,
                None => panic!("Error when processing obsolete format"),
            };
            info!("[check_for_obsolete] Parent of OSL: {:?}",parent);
            obs.push(parent);
            // let  children =  &gene_tree.arena[parent].children;
        }
    }
    info!("[check_for_obsolete] Find {} OSL in the tree",obs.len());
    for no in obs {
        info!("[check_for_obsolete] Processing parent: {:?}",no);
        let children = &gene_tree.arena[no].children;
        info!("[check_for_obsolete] Number of children: {:?}",children.len());
        assert_eq!(children.len(),3);
        let mut to_move_node = 0;    //Le Neoud de duplication a deplacer sous le noeud specaitonLoss
        let mut specloss_node = 0;
        // assert_eq!(gene_tree.arena[children[0]].e, Event::ObsoleteSpeciationLoss);
        for child in children {
            info!("[check_for_obsolete] Fils {:?}: {:?} {:?} {:?}",*child,&gene_tree.arena[*child].val, &gene_tree.arena[*child].e, &gene_tree.arena[*child].location);
            if &gene_tree.arena[*child].e == &Event::Duplication {
                info!("[check_for_obsolete] Find the Duplication node to move:  {:?}",child);
                to_move_node = *child;
            }
            if &gene_tree.arena[*child].e == &Event::ObsoleteSpeciationLoss {
                info!("[check_for_obsolete] Find the 'destination' OSL  node  :  {:?}",child);
                specloss_node = *child;
            }
        }
        let loss_node=gene_tree.arena[specloss_node].children[0];
        info!("[check_for_obsolete] Adding  {:?} to {:?}",to_move_node,specloss_node);
        gene_tree.arena[specloss_node].children.push(to_move_node);
        info!("[check_for_obsolete] Set parent of {:?} as {:?}",to_move_node,specloss_node);
        gene_tree.arena[to_move_node].parent =  Some(specloss_node);
        info!("[check_for_obsolete] Remove   {:?} from  {:?}",specloss_node,no);
        gene_tree.arena[no].children.retain(|&x| x !=  to_move_node);
        info!("[check_for_obsolete] Chidren of the OSL {:?}: {:?}",specloss_node,gene_tree.arena[specloss_node].children);
        info!("[check_for_obsolete] Details of the OSL {:?}: {:?}: {:?}",specloss_node,gene_tree.arena[specloss_node].val,gene_tree.arena[specloss_node].location);
        let specloss_species = &gene_tree.arena[specloss_node].location;
        info!("[check_for_obsolete] Species of the OSL = {}",specloss_species);
        let to_move_species = &gene_tree.arena[to_move_node].location;
        info!("[check_for_obsolete] Species of the node moved under OSL = {}",to_move_species);
        let species_node = species_tree.get_index(specloss_species.to_string());// index de  specloss_species dans l'arbre d'espece
        info!("[check_for_obsolete] Node of the OSL in the species tree : {:?}",species_tree.arena[species_node]);
        let species_node_left =  &species_tree.arena[species_node].children[0];
        let species_node_right =  &species_tree.arena[species_node].children[1];
        let species_left =  &species_tree.arena[*species_node_left].name;
        let species_right =  &species_tree.arena[*species_node_right].name;
        info!("[check_for_obsolete] Species under the OSL = {}, {}",species_left,species_right);
        let loss_species = match to_move_species ==  species_left {
            true => species_right,
            false => species_left,
        };
        // TO DO : VERIFIER  L'AUTRE PARS ECUROTE
        info!("[check_for_obsolete] Species of the loss is thus {}",loss_species);
        gene_tree.arena[loss_node].location = loss_species.to_string();

    //     for child in  &mut gene_tree.arena[no].children {
    //     println!("CHECK {:?}", &mut gene_tree.arena[*child]);
    // }
    }
        info!("[check_for_obsolete] Final gene tree {:?}",&gene_tree);
}

/// Set the coordinates of the gene tree according to species tree coordinates
pub fn map_gene_trees(sp_tree: &mut ArenaTree<String>, gene_trees:&mut std::vec::Vec<ArenaTree<String>>) {
    let  nb_gntree =  gene_trees.len(); // Nombre d'arbres de gene
    info!("map_gene_trees: {} gene trees to be processed",nb_gntree);
    let mut idx_rcgen = 0;  // Boucle sur les arbres de genes
    loop {
        info!("map_gene_trees: => Processing Gene Tree {}",idx_rcgen);
        for  index in &mut gene_trees[idx_rcgen].arena {
            let mut mapped = false;
            // println!("MAP node {:?} event {:?} location {:?}",index.idx, index.e,index.location);
            for spindex in  &mut sp_tree.arena {
                if  index.location == spindex.name {
                    mapped = true;
                    let x = spindex.x;
                    index.x = x;
                    let y = spindex.y;
                    index.y = y;
                    info!("map_tree: [{}] Gene node {:?} mapped to  species node {:?}",idx_rcgen,index,spindex);
                }
            }
            if !mapped {
                panic!("Unable to map Node {:?}",index);
            }
        }
        // Passe à l'arbre de gènes suivant
        idx_rcgen += 1;
        if idx_rcgen == nb_gntree {
            break;
        }
    } //Fin de la boucle sur les arbres de gènes
}

/// Determine the number of gene nodes associated to a species node
pub fn map_species_trees(sp_tree: &mut ArenaTree<String>, gene_trees: &mut std::vec::Vec<ArenaTree<String>>) {
    let  nb_gntree =  gene_trees.len(); // Nombre d'arbres de gene
    info!("map_species_trees: {} gene trees to be processed",nb_gntree);
    let mut idx_rcgen = 0;  // Boucle sur les arbres de genes
    loop {
        info!("map_species_trees: => Processing Gene Tree {}",idx_rcgen);
        // Boucle sur les noeuds de l'arbre de gene idx_rcgen
        for  index in &mut gene_trees[idx_rcgen].arena {
            let mut mapped = false;
            // Boucle sur les noeuds de l'arbre d'espèce
            for spindex in  &mut sp_tree.arena {
                if  index.location == spindex.name {
                    mapped = true;
                    // Incremente le nb de noeuds de gene associé au noeud d'espece
                    let mut nbg = spindex.nbg;
                    nbg = nbg + 1 ;
                    spindex.nbg = nbg;
                    // Ajoute le tuple (indexe de l'arbre de  gene, index du noeud de gene ) associé
                    spindex.nodes.push((idx_rcgen,index.idx));
                    // spindex.nodes.insert(0,(idx_rcgen,index.idx));
                    info!("map_tree: Gene node {:?} mapped to  species node {:?}",index,spindex);
                }
            }
            if !mapped {
                panic!("Unable to map Node {:?}",index);
            }
        }
        // Passe à l'arbre de gènes suivant
        idx_rcgen += 1;
        if idx_rcgen == nb_gntree {
            break;
        }
    } //Fin de la boucle sur les arbres de gènes
}

/// Shift the gene nodes in a given species node to avoid superposition.
pub fn bilan_mappings(sp_tree: &mut ArenaTree<String>, gene_trees: &mut std::vec::Vec<ArenaTree<String>>, index: usize) {
    info!("BILAN MAPPING : Species Node {}",sp_tree.arena[index].name);
        let ratio = 1.0 ; // permet de rglere l'ecrtement entre les noeid de genes dans l'arbre d'espece
        let  mut shift = 0.0;
        let  mut shift_x = sp_tree.arena[index].nbg as f32 -1.0 ;
        let incr = 1.0;
        // TODO classer selon le Y du pere pour eviter les croisement
        // boucle sur m'espeve
        for (index_node, node)  in &sp_tree.arena[index].nodes {
            info!(">>> {:?} {:?}",gene_trees[*index_node].arena[*node].name,gene_trees[*index_node].arena[*node].e);
            // println!("DEBUG {}/{}",shift,&sp_tree.arena[index].nbg);
            let bool_left = sp_tree.is_left(index);
            match  gene_trees[*index_node].arena[*node].e {
                Event::Duplication => {
                    let x = gene_trees[*index_node].arena[*node].x;
                    let x = match bool_left {
                        true   => x + PIPEBLOCK*shift / ratio,
                        false  => x + PIPEBLOCK*shift_x / ratio,
                    };
                    gene_trees[*index_node].arena[*node].set_x_noref(x);
                    let y = gene_trees[*index_node].arena[*node].y;
                    let y = y + PIPEBLOCK*shift / ratio;
                    gene_trees[*index_node].arena[*node].set_y_noref(y);
                    shift = shift + incr;
                    shift_x = shift_x - incr;

                },
                Event::Speciation => {
                    let x = gene_trees[*index_node].arena[*node].x;
                    let x = match bool_left {
                        true   => x + PIPEBLOCK*shift / ratio,
                        false  => x + PIPEBLOCK*shift_x / ratio,
                    };
                    gene_trees[*index_node].arena[*node].set_x_noref(x);
                    let y = gene_trees[*index_node].arena[*node].y;
                    let y = y + PIPEBLOCK*shift / ratio;
                    gene_trees[*index_node].arena[*node].set_y_noref(y);
                    shift = shift + incr;
                    shift_x = shift_x - incr;
                },
                Event::TransferBack => {
                    let x = gene_trees[*index_node].arena[*node].x;
                    let x = match bool_left {
                        true   => x + PIPEBLOCK*shift / ratio,
                        false  => x + PIPEBLOCK*shift_x / ratio,
                    };
                    gene_trees[*index_node].arena[*node].set_x_noref(x);
                    let y = gene_trees[*index_node].arena[*node].y;
                    let y = y + PIPEBLOCK*shift / ratio;
                    gene_trees[*index_node].arena[*node].set_y_noref(y);
                    shift = shift + incr;
                    shift_x = shift_x - incr;
                },
                Event::BranchingOut => {
                    let x = gene_trees[*index_node].arena[*node].x;
                    let x = match bool_left {
                        true   => x + PIPEBLOCK*shift / ratio,
                        false  => x + PIPEBLOCK*shift_x / ratio,
                    };
                    gene_trees[*index_node].arena[*node].set_x_noref(x);
                    let y = gene_trees[*index_node].arena[*node].y;
                    let y = y + PIPEBLOCK*shift / ratio;
                    gene_trees[*index_node].arena[*node].set_y_noref(y);
                    shift = shift + incr;
                    shift_x = shift_x - incr;
                },
                Event::Leaf => {
                    let x = gene_trees[*index_node].arena[*node].x;
                    let x = match bool_left {
                        true   => x + PIPEBLOCK*shift / ratio,
                        false  => x + PIPEBLOCK*shift_x / ratio,
                    };
                    gene_trees[*index_node].arena[*node].set_x_noref(x);
                    let y = gene_trees[*index_node].arena[*node].y;
                    let y = y + PIPEBLOCK*shift;
                    gene_trees[*index_node].arena[*node].set_y_noref(y);
                    shift = shift + incr;
                    shift_x = shift_x - incr;
                },
                Event::Loss => {
                    let x = gene_trees[*index_node].arena[*node].x;
                    let x = match bool_left {
                        true   => x + PIPEBLOCK*shift / ratio,
                        false  => x + PIPEBLOCK*shift_x / ratio,
                    };
                    gene_trees[*index_node].arena[*node].set_x_noref(x);
                    // let parent = gene_trees[*index_node].arena[*node].parent;
                    // let y = match parent {
                    //     None =>  {
                    //         panic!("Loss node with no parent");
                    //     },
                    //     Some(p) => {
                    //         gene_trees[*index_node].arena[p].y + PIPEBLOCK *1.2
                    //     },
                    // };
                    // // let y = y + PIPEBLOCK*shift / ratio;
                    // // essai
                    let y = gene_trees[*index_node].arena[*node].y;
                    let y = y + PIPEBLOCK*shift / ratio;
                    // fin essai

                    gene_trees[*index_node].arena[*node].set_y_noref(y);
                    shift = shift + incr;
                    shift_x = shift_x - incr;
                },
                Event::BifurcationOut => {
                    let x = gene_trees[*index_node].arena[*node].x;
                    let x = match bool_left {
                        true   => x + PIPEBLOCK*shift / ratio,
                        false  => x + PIPEBLOCK*shift_x / ratio,
                    };
                    gene_trees[*index_node].arena[*node].set_x_noref(x);
                    let y = gene_trees[*index_node].arena[*node].y;
                    let y = y + PIPEBLOCK*shift / ratio;
                    gene_trees[*index_node].arena[*node].set_y_noref(y);
                    shift = shift + incr;
                    shift_x = shift_x - incr;
                },
                _=> {},
            }
        }
    let children =  &mut  sp_tree.arena[index].children;
    if children.len() > 0 {
        let son_left = children[0];
        let son_right = children[1];
         bilan_mappings( sp_tree, gene_trees,son_left);
         bilan_mappings( sp_tree, gene_trees,son_right);
         // bilan_mapping(&mut sp_tree, &mut gene_tree,children[1]);
    }
}
/// Shift again the previously shifted gene nodes in case of a duplication node or a leaf
pub fn move_dupli_mappings(sp_tree: &mut ArenaTree<String>, gene_trees: &mut std::vec::Vec<ArenaTree<String>>, index: usize) {
    for (index_node, node) in &sp_tree.arena[index].nodes {
        info!(">>> {:?} {:?}",gene_trees[*index_node].arena[*node].name,gene_trees[*index_node].arena[*node].e);
        match  gene_trees[*index_node].arena[*node].e {
            Event::Duplication => {
                let dupli_children =  &mut  gene_trees[*index_node].arena[*node].children;
                let dupli_son_left = dupli_children[0];
                let x = gene_trees[*index_node].arena[dupli_son_left].x;
                gene_trees[*index_node].arena[*node].set_x_noref(x);
            },
            // Il faut deplacer aussi les feuilles pour compenser: on les mets au meme niveau
            Event::Leaf => {
                let y = sp_tree.arena[index].y + sp_tree.arena[index].height / 2.0 ;
                gene_trees[*index_node].arena[*node].set_y_noref(y);
                gene_trees[*index_node].arena[*node].set_ymod_noref(0.0);
            }
            _=> {},
        }
    }
    let children =  &mut  sp_tree.arena[index].children;
    if children.len() > 0 {
        let son_left = children[0];
        let son_right = children[1];
        move_dupli_mappings( sp_tree, gene_trees,son_left);
        move_dupli_mappings( sp_tree, gene_trees,son_right);
    }
}
/// Center the gene nodes into specie snode
pub fn center_gene_nodes(sp_tree: &mut ArenaTree<String>, gene_trees: &mut std::vec::Vec<ArenaTree<String>>, index: usize) {
    let left_sp = sp_tree.arena[index].x - sp_tree.arena[index].width / 2.0  ;
    let right_sp = sp_tree.arena[index].x + sp_tree.arena[index].width / 2.0  ;
    let up_sp = sp_tree.arena[index].y + sp_tree.arena[index].ymod - sp_tree.arena[index].height / 2.0  ;
    let down_sp = sp_tree.arena[index].y + sp_tree.arena[index].ymod + sp_tree.arena[index].height / 2.0  ;
    let mut left_gene = -100000000.0;
    let mut right_gene = 100000000.0;
    let mut down_gene = -100000000.0;
    let mut up_gene = 100000000.0;
    for (index_node, node) in &sp_tree.arena[index].nodes {
        info!(">>> {:?} {:?}",gene_trees[*index_node].arena[*node].name,gene_trees[*index_node].arena[*node].e);
        if  gene_trees[*index_node].arena[*node].x    > left_gene {
            left_gene =  gene_trees[*index_node].arena[*node].x  ;
        }
        if gene_trees[*index_node].arena[*node].x    < right_gene {
            right_gene =  gene_trees[*index_node].arena[*node].x  ;
        }

        if  gene_trees[*index_node].arena[*node].ymod > 0.0 {
            panic!("Unexpected ymod value");
        }
        // match gene_trees[*index_node].arena[*node].e {


            // Event::Undef => {
            //     // Seulement dans le cas ou le seul node de gene associé au neod d'espece
            //     // est ce loss
            //     if sp_tree.arena[index].nbg == 1 {
            //         if gene_trees[*index_node].arena[*node].y    > down_gene {
            //                 down_gene =  gene_trees[*index_node].arena[*node].y  ;
            //         }
            //         if  gene_trees[*index_node].arena[*node].y    < up_gene {
            //                 up_gene =  gene_trees[*index_node].arena[*node].y  ;
            //         }
            //
            //     }
            //
            // },
            // _ => {
                if gene_trees[*index_node].arena[*node].y    > down_gene {
                        down_gene =  gene_trees[*index_node].arena[*node].y  ;
                }
                if  gene_trees[*index_node].arena[*node].y    < up_gene {
                        up_gene =  gene_trees[*index_node].arena[*node].y  ;
                }
            // },
        // }
    }
    let middle_sp = (left_sp + right_sp) / 2.0;
    let middle_gn = (left_gene  + right_gene)  / 2.0;
    let shift = middle_gn  - middle_sp;

    let y_middle_sp = (up_sp + down_sp) / 2.0;
    let y_middle_gn = (up_gene  + down_gene)  / 2.0;
    let y_shift = y_middle_gn  - y_middle_sp;
    for (index_node, node) in &sp_tree.arena[index].nodes {
        let x = gene_trees[*index_node].arena[*node].x;
        let x = x - shift ;
        gene_trees[*index_node].arena[*node].set_x_noref(x);
        let y = gene_trees[*index_node].arena[*node].y;
        let y = y - y_shift ;
        gene_trees[*index_node].arena[*node].set_y_noref(y);
    }

    let children =  &mut  sp_tree.arena[index].children;
    if children.len() > 0 {
        let son_left = children[0];
        let son_right = children[1];
        center_gene_nodes( sp_tree, gene_trees,son_left);
        center_gene_nodes( sp_tree, gene_trees,son_right);
    }
}

/// Set the width of the species tree pipe.
pub fn set_species_width(sp_tree: &mut ArenaTree<String>) {
    for spindex in  &mut sp_tree.arena {
        let  nbg = spindex.nbg;
        if nbg > 0 {
            spindex.width =  nbg as f32 * PIPEBLOCK;
            spindex.height = nbg as f32 * PIPEBLOCK;
        }
        else {
            spindex.width =   PIPEBLOCK;
            spindex.height =  PIPEBLOCK;
        }
    }
}

/// Get the id of the first "clade" tag.
#[allow(dead_code)]
pub fn find_first_clade(  doc: &mut roxmltree::Document) -> Result < roxmltree::NodeId, usize> {
    let descendants = doc.root().descendants();
    // Search for the first occurnce of clade tag
    for  node in descendants {
        if node.has_tag_name("clade"){
            // return Ok(node.id().get())
            return Ok(node.id())
        }
    }
    Err(0)
}

/// Get the id of the first "spTree" tag.
pub fn find_sptree( doc: &mut roxmltree::Document) -> Result < roxmltree::NodeId, usize> {
    let descendants = doc.root().descendants();
    // Search for the first occurnce of clade spTree
    for  node in descendants {
        if node.has_tag_name("spTree"){
            // return Ok(node.id().get())
            return Ok(node.id())
        }
    }
    Err(0)
}

/// Get the list of ids of all the "regGeneTree" tag in a xml document.
pub fn find_rgtrees( doc: &mut roxmltree::Document) -> Result < Vec<roxmltree::NodeId>, usize> {
    let descendants = doc.root().descendants();
    let mut gene_nodes:std::vec::Vec<roxmltree::NodeId> = Vec::new();
    // Search for the first occurnce of clade spTree
    for  node in descendants {
        if node.has_tag_name("recGeneTree"){
            // return Ok(node.id().get())
            // return Ok(node.id())
            gene_nodes.push(node.id());
        }
    }
    info!("find_sptrees: Number of gene trees in xml = {}",gene_nodes.len());
    match gene_nodes.len() > 0 {
        true => return Ok(gene_nodes),
        false => Err(0),
    }
}




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

/// Transforms the tree into real branch  length representation
pub fn real_length( tree: &mut ArenaTree<String>, index: usize, dist: &mut f32, options: &Options) {
    let  dist_father = tree.arena[index].l;
    let mut dist = *dist + dist_father;
    tree.arena[index].set_y_noref(dist * BLOCK * options.scale + BLOCK);
    let children  = &mut  tree.arena[index].children;
    if children.len() > 1 {
        let son_left = children[0];
        let son_right = children[1];
        real_length( tree, son_left, &mut dist, options);
        real_length( tree, son_right, &mut dist, options);
    }

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
pub fn shift_mod_xy( tree: &mut ArenaTree<String>, index: usize, xmod: &mut f32, ymod: &mut f32) {
    info!("shift_mod_xy: shifting {:?} xmod={} ymod={}",tree.arena[index],xmod,ymod);
    let x_father = tree.arena[index].x;
    let  xmod_father = tree.arena[index].xmod;
    let mut xmod = *xmod + xmod_father;
    tree.arena[index].set_x_noref(x_father+xmod);
    // tree.arena[index].set_xmod_noref(xmod);inutile
    let y_father = tree.arena[index].y;
    let  ymod_father = tree.arena[index].ymod;
    let mut ymod = *ymod + ymod_father;
    tree.arena[index].set_y_noref(y_father+ymod);
    // tree.arena[index].set_ymod_noref(ymod);inutile
    let children  = &mut  tree.arena[index].children;
    if children.len() > 2 {
        panic!("L'arbre doit être binaire")
    }
    if children.len() > 1 {
        let son_left = children[0];
        let son_right = children[1];
        shift_mod_xy( tree, son_left, &mut xmod, &mut ymod);
        shift_mod_xy( tree, son_right, &mut xmod, &mut ymod);
    }

}

#[allow(dead_code)]
/// Traverse the tree using post-order traversal
pub fn  postorder(tree: &mut ArenaTree<String>){
    let root = tree.get_root();
    explore_postorder(tree,root);
}

#[allow(dead_code)]
/// Traverse the tree using post-order traversal starting from a given node
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

/// Solve the conflicts between a parent and its children
pub fn  check_vertical_contour_postorder(tree: &mut ArenaTree<String>,index:usize, ymod: f32) {
    let children  = &mut  tree.arena[index].children;
    if children.len() > 0 {
        let left = children[0];
        let right = children[1];
        info!("check_vertical_contour_postorder: Father = {} (ymod = {} ) , Left = {}, Right = {}",tree.arena[index].name,tree.arena[index].ymod,tree.arena[left].name,tree.arena[right].name);
        push_down(tree,index, left,right);
        check_vertical_contour_postorder(tree,left,tree.arena[left].ymod + 0.0 *  ymod);
        check_vertical_contour_postorder(tree,right,tree.arena[right].ymod + 0.0 * ymod);
    }
}
/// Check for conficts between parents and children and shift down nodes in order to solve
/// detected conflicts ( for species pipe tree only).
pub fn push_down (tree: &mut ArenaTree<String>, parent: usize, left: usize, right: usize) {
    let node_parent_down_pos = node_ypos(tree,parent,1);
    let node_left_up_pos = node_ypos(tree,left,-1);
    let node_right_up_pos = node_ypos(tree,right,-1);
    if (node_left_up_pos <=  node_parent_down_pos) || (node_right_up_pos <=  node_parent_down_pos) {
        let shift_left = node_parent_down_pos - node_left_up_pos ;
        let shift_right = node_parent_down_pos - node_right_up_pos ;
        let mut shift_down = match shift_left > shift_right {
            true => shift_left,
            false => shift_right,
        };
        if shift_down <= PIPEBLOCK {
            shift_down = PIPEBLOCK;

        }
        // TODO configurable
        let shift_down = shift_down + 4.0 * PIPEBLOCK;
        info!("CONFLIT AT SPEC NODE {}: parent y = {} ymod = {} down = {} left up = {} right up = {} => shift = {}",tree.arena[parent].name,tree.arena[parent].y,tree.arena[parent].ymod,node_parent_down_pos,node_left_up_pos,node_right_up_pos,shift_down);
        info!("SHIFTING Y {} + 1xPIPEBLOCK = {}",shift_down,shift_down + 1.0 * PIPEBLOCK);
        info!("Initial left : y = {}, ymod = {}",tree.arena[left].y,tree.arena[left].ymod);
        let y = tree.arena[left].y;
        let y = y + shift_down ;
        tree.arena[left].set_y_noref(y);
        let y = tree.arena[right].y;
        let y = y +shift_down ;
        tree.arena[right].set_y_noref(y);
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
/// Get the leftest  or rightest x value of a node
pub fn node_xpos(tree: &mut ArenaTree<String>, index: usize, xmod: f32, operator : i32) -> f32 {
    tree.arena[index].x + tree.arena[index].xmod + operator as f32 * tree.arena[index].nbg as f32 /2.0  *PIPEBLOCK + xmod
    // TODO quand la valeur ng est 0, ca peut etre un noeud  d'arbre de gene mais aussi un noeud
    // d'arbre d'espece sans noeud abre de gene associé et du coup la position extreme n'est pas exacte... Verfier su
    // il y a des conflits ( un noud d'arbre d'espexe sans gene a la meme epaissuer qu'un noeud d'arbre d'espece avec 1 gene)
}
/// Get the upper or lower y value of a node
pub fn node_ypos(tree: &mut ArenaTree<String>, index: usize,  operator : i32) -> f32 {
    tree.arena[index].y + tree.arena[index].ymod + operator as f32 * tree.arena[index].nbg as f32 /2.0  *PIPEBLOCK
    // tree.arena[index].y + tree.arena[index].ymod + operator as f32 * tree.arena[index].nbg as f32 /2.0  *PIPEBLOCK + ymod
    // TODO quand la valeur ng est 0, ca peut etre un noeud  d'arbre de gene mais aussi un noeud
    // d'arbre d'espece sans noeud abre de gene associé et du coup la position extreme n'est pas exacte... Verfier su
    // il y a des conflits ( un noud d'arbre d'espexe sans gene a la meme epaissuer qu'un noeud d'arbre d'espece avec 1 gene)
}
/// Get the left 'contour' of a sub tree
pub fn  get_contour_left(tree: &mut ArenaTree<String>,index:usize,depth:usize,contour_left: &mut Vec<f32>,parent_xmod: f32)  {
    info!("get_contour_left: process node {:?}",tree.arena[index]);
    let local_depth = tree.depth(index)-depth; // Profondeur du noeud pa rapport a noeud de depart
    let node_left_pos = node_xpos(tree,index,parent_xmod,-1);
    if contour_left.len() <= local_depth {
        if tree.arena[index].xmod < 0.0 {
            panic!("error: negative xmod");
        }
        contour_left.push(node_left_pos);
        info!("get_contour_left: increment contour is now {:?}",contour_left);
    }
    if tree.arena[index].xmod < 0.0 {
        panic!("erreur: negative  xmod");
    }
    if node_left_pos <= contour_left[local_depth] {
        contour_left[local_depth] = node_left_pos;
        info!("get_contour_left: contour is now {:?}",contour_left);
    }
    let children  = &mut  tree.arena[index].children;
    if children.len() > 0 {
        let left = children[0];
        get_contour_left(tree,left,depth,contour_left,tree.arena[index].xmod + parent_xmod );
    }
}

/// Get the right 'contour' of a sub tree
pub fn  get_contour_right(tree: &mut ArenaTree<String>,index:usize,depth:usize,contour_right: &mut Vec<f32>,parent_xmod: f32)  {
    info!("get_contour_right: process node {:?}",tree.arena[index]);
    let local_depth = tree.depth(index)-depth; // Profondeur du noeud pa rapport a noeud de depart
    let node_right_pos = node_xpos(tree,index,parent_xmod,1);
    if contour_right.len() <= local_depth {
        if tree.arena[index].xmod < 0.0 {
            panic!("erreur: negative xmod");
        }
        contour_right.push(node_right_pos);
            info!("get_contour_right: increment contour is now {:?}",contour_right);
    }
    if tree.arena[index].xmod < 0.0 {
        panic!("erreur: negative xmod");
    }
    if node_right_pos >= contour_right[local_depth] {
        contour_right[local_depth] = node_right_pos ;
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
    let mut right_co_of_left_tr  = vec![tree.arena[left_tree].x+tree.arena[left_tree].xmod + tree.arena[left_tree].nbg as f32 *PIPEBLOCK]; //contour droit de l'arbre de gauche
    let depth_left_tr  = tree.depth(left_tree);
    get_contour_right(tree,left_tree,depth_left_tr,&mut right_co_of_left_tr,0.0);
    info!("push_right: right contour of {} = {:?}",left_tree,right_co_of_left_tr);
    let mut left_co_of_right_tr  = vec![tree.arena[right_tree].x+tree.arena[right_tree].xmod - tree.arena[right_tree].nbg as f32 *PIPEBLOCK]; //contour droit de l'arbre de gauche
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

/// Set the x of the father between its children
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
