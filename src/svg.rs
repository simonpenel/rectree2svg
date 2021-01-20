
use crate::arena::ArenaTree;
use draw::*;

pub fn drw_tree (tree: &mut ArenaTree<String>) {
println!("DRAW {:?}",tree);

}

pub fn drw_sq () {
println!("Drawing");


let mut canvas = Canvas::new(100, 100);

// create a new drawing
let mut rect = Drawing::new()
// give it a shape
.with_shape(Shape::Rectangle {
    width: 50,
    height: 50,
})
// move it around
.with_xy(25.0, 25.0)
// give it a cool style
.with_style(Style::stroked(5, Color::black()));

// add it to the canvas
canvas.display_list.add(rect);

// save the canvas as an svg
render::save(
&canvas,
"basic_end_to_end.svg",
SvgRenderer::new(),
)
.expect("Failed to save");

    }
