use rectree2svg::{ArenaTree,Options,newick2tree,knuth_layout,check_contour_postorder,
                  shift_mod_xy,set_middle_postorder,draw_tree,lca};
use std::fs;
fn main() {
    let mut tree: ArenaTree<String> = ArenaTree::default();
    let mut options: Options = Options::new();
    println!("Reading newick file examples/newick.A4.txt...");
    let contents = fs::read_to_string("examples/newick.A4.txt")
                .expect("Something went wrong reading the newick file");
                println!("Create a first node which will be the root...");
                let root = tree.new_node("Root".to_string());
                println!("Build the tree from the file contents...");
                newick2tree(contents, &mut tree, root, &mut 0);
                let j = tree.get_index("J".to_string());
                println!("Index of leaf J is {}",j);
                let l = tree.get_index("L".to_string());
                println!("Index of leaf L is {}",l);
                let n = tree.get_index("N".to_string());
                println!("Index of leaf N is {}",n);
                let lca_jl = lca(&mut tree,j,l);
                println!("Index of lca betwen J and L is {}",lca_jl);
                tree.arena[lca_jl].name = "LCA of J and L".to_string();
                let lca_jn = lca(&mut tree,j,n);
                println!("Index of lca betwen J and N is {}",lca_jn);
                tree.arena[lca_jn].name = "LCA of J and N".to_string();
                knuth_layout(&mut tree,root, &mut 1);
                check_contour_postorder(&mut tree, root);
                shift_mod_xy(&mut tree, root, &mut 0.0, &mut 0.0);
                set_middle_postorder(&mut tree, root);
                // Display internal nodes
                options.gene_internal = true ;
                draw_tree(&mut tree,"lca.svg".to_string(),&options);
                println!("Please open output file 'lca.svg' with your browser");
                println!("OK.");
}
