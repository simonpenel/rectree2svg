use rectree2svg::{ArenaTree,Options,newick2tree,knuth_layout,check_contour_postorder,
                  shift_mod_xy,set_middle_postorder,draw_tree,real_length};
use std::fs;
fn main() {
    let mut tree: ArenaTree<String> = ArenaTree::default();
    let options: Options = Options::new();
    println!("Reading newick file examples/newick.txt...");
    let contents = fs::read_to_string("examples/newick.txt")
                .expect("Something went wrong reading the newick file");
    println!("Create a first node which will be the root...");
    let root = tree.new_node("Root".to_string());
    println!("Build the tree from the file contents...");
    newick2tree(contents, &mut tree, root, &mut 0);
    println!("Calculate initial x y positions...");
    knuth_layout(&mut tree,root, &mut 1);
    println!("Calculate xmod values to avoid conflicts...");
    check_contour_postorder(&mut tree, root);
    println!("Move nodes along x according to  xmod...");
    shift_mod_xy(&mut tree, root, &mut 0.0, &mut 0.0);
    println!("Set parent horizontal position between its children...");
    set_middle_postorder(&mut tree, root);
    println!("Rotate -90 and draw cladogram tree...");
    draw_tree(&mut tree,"read_newick-clado.svg".to_string(),&options);
    println!("Please open output file 'read_newick-clado.svg' with your browser");
    real_length(&mut tree, root, &mut 0.0, & options);
    println!("Rotate -90 and draw real length tree...");
    draw_tree(&mut tree,"read_newick-real_length.svg".to_string(),&options);
    println!("Please open output file 'read_newick-real_length.svg' with your browser");
    println!("OK.");
}
