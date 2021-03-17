//! # tree2svg
//!
//! `tree2svg` is a collection of utilities to draw phylogentic trees in svg format.

// Pour la doc et pour facilier l'usage par les utilisateurs du module:
mod arena;
pub use self::arena::Event;
pub use self::arena::Noeud;
pub use self::arena::ArenaTree;
pub use self::arena::taxo2tree;
pub use self::arena::xml2tree;
pub use self::arena::find_first_clade;
pub use self::arena::find_sptree;
pub use self::arena::find_rgtree;
pub use self::arena::knuth_layout;
pub use self::arena::postorder;
pub use self::arena::cladogramme;
pub use self::arena::check_contour_postorder;
pub use self::arena::push_right;
pub use self::arena::get_contour_left;
pub use self::arena::get_contour_right;
pub use self::arena::shift_mod_xy;
pub use self::arena::set_middle_postorder;

mod drawing;
pub use self::drawing::draw_tree;
pub use self::drawing::draw_sptree_gntree;
pub use self::drawing::get_carre;
pub use self::drawing::get_chemin_carre;
pub use self::drawing::get_chemin_sp;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn  check_val() {
        let mut tree: ArenaTree<String> = ArenaTree::default();
        let test = String::from("Test");
        let copy = test.clone();
        let index = tree.new_node(test);
        let mut node = &mut tree.arena[index];
        let val = node.get_val();
        assert_eq!(*val,copy);
    }

    #[test]
    fn  check_event() {
        let mut tree: ArenaTree<String> = ArenaTree::default();
        let test = String::from("Test");
        let copy = test.clone();
        let index = tree.new_node(test);
        let mut node = &mut tree.arena[index];
        node.set_event(Event::Undef);
        let event = node.get_event();
        assert_eq!(*event,Event::Undef);
    }
    #[test]
    fn  check_coords_noref() {
        let mut tree: ArenaTree<String> = ArenaTree::default();
        let test = String::from("Test");
        let copy = test.clone();
        let index = tree.new_node(test);
        let mut node = &mut tree.arena[index];
        node.set_x_noref(10.0);
        node.set_y_noref(20.0);
        let (x,y) = node.get_coords();
        assert_eq!((*x,*y),(10.0,20.0));
    }
    #[test]
    fn  check_coords() {
        let mut tree: ArenaTree<String> = ArenaTree::default();
        let test = String::from("Test");
        let copy = test.clone();
        let index = tree.new_node(test);
        let mut node = &mut tree.arena[index];
        node.set_x(&10.0);
        node.set_y(&20.0);
        let coords = node.get_coords();

        assert_eq!(coords,(&10.0,&20.0));
    }
    #[test]
    fn  check_x_noref() {
        let mut tree: ArenaTree<String> = ArenaTree::default();
        let test = String::from("Test");
        let copy = test.clone();
        let index = tree.new_node(test);
        let mut node = &mut tree.arena[index];
        node.set_x_noref(10.0);
        let x = node.get_x();
        assert_eq!(*x, 10.0);
    }
    #[test]
    fn  check_x() {
        let mut tree: ArenaTree<String> = ArenaTree::default();
        let test = String::from("Test");
        let copy = test.clone();
        let index = tree.new_node(test);
        let mut node = &mut tree.arena[index];
        node.set_x(&10.0);
        let x = node.get_x();
        assert_eq!(*x, 10.0);
    }




}
