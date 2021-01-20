//! # tree2svg
//!
//! `tree2svg` is a collection of utilities to draw phylogentic trees in svg format.

// Pour la doc et pour facilier l'usage par les utilisateurs du module:
mod arena;
pub use self::arena::Event;
pub use self::arena::Node;
pub use self::arena::ArenaTree;
pub use self::arena::taxo2tree;
// pub use self::arena::get_val;
