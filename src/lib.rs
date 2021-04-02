//! # rectree2svg
//!
//! `rectree2svg` Read  phylogenetic trees in newick, phyloxml or recPhyloXML format,
//!  build and manipulate phylogenetic trees, create svg representation of the tree
//!  with several options.

// Pour la doc et pour facilier l'usage par les utilisateurs du module:

mod arena;
pub use self::arena::Options;
pub use self::arena::Event;
pub use self::arena::Noeud;
pub use self::arena::ArenaTree;

pub use self::arena::bilan_mappings;
pub use self::arena::center_gene_nodes;
pub use self::arena::check_contour_postorder;
pub use self::arena::check_for_obsolete;
pub use self::arena::check_vertical_contour_postorder;
pub use self::arena::cladogramme;
pub use self::arena::find_left_right;
pub use self::arena::find_rgtrees;
pub use self::arena::find_sptree;
pub use self::arena::get_contour_left;
pub use self::arena::get_contour_right;
pub use self::arena::get_maxdepth;
pub use self::arena::knuth_layout;
pub use self::arena::lca;
pub use self::arena::map_gene_trees;
pub use self::arena::map_species_trees;
pub use self::arena::move_dupli_mappings;
pub use self::arena::newick2tree;
pub use self::arena::node_xpos;
pub use self::arena::node_ypos;
pub use self::arena::push_down;
pub use self::arena::push_right;
pub use self::arena::real_length;
pub use self::arena::set_leaves_to_bottom;
pub use self::arena::set_middle_postorder;
pub use self::arena::set_species_width;
pub use self::arena::shift_mod_xy;
pub use self::arena::xml2tree;

mod drawing;
pub use self::drawing::close_chemin_sp;
pub use self::drawing::draw_sptree_gntrees;
pub use self::drawing::draw_tree;
pub use self::drawing::get_carre;
pub use self::drawing::get_circle;
pub use self::drawing::get_chemin_carre;
pub use self::drawing::get_chemin_sp;
pub use self::drawing::get_chemin_transfer;
pub use self::drawing::get_cross;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn  check_event() {
        let mut tree: ArenaTree<String> = ArenaTree::default();
        let test = String::from("Test");
        let index = tree.new_node(test);
        let node = &mut tree.arena[index];
        node.set_event(Event::Duplication);
        assert_eq!(node.e,Event::Duplication);
    }
    #[test]
    fn  check_x_noref() {
        let mut tree: ArenaTree<String> = ArenaTree::default();
        let test = String::from("Test");
        let index = tree.new_node(test);
        let node = &mut tree.arena[index];
        node.set_x_noref(10.0);
        let x = node.x;
        assert_eq!(x, 10.0);
    }





}
