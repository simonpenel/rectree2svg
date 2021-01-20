
use crate::arena::ArenaTree;
use draw::*;

pub fn draw_tree (tree: &mut ArenaTree<String>) {
let mut canvas = Canvas::new(500, 500);

for  index in &tree.arena {
         println!("SVG {:?}",index.x);
         println!("SVG {:?}",index.name);
         let rec = get_sq(index.x,index.y,2);
// add it to the canvas
canvas.display_list.add(rec);

}
// let rec = get_sq(12.0,74.0,25);
// // add it to the canvas
// canvas.display_list.add(rec);
//
// let rec = get_sq(52.0,24.0,15);
// // add it to the canvas
// canvas.display_list.add(rec);

// save the canvas as an svg
render::save(
&canvas,
"basic_end_to_end.svg",
SvgRenderer::new(),
)
.expect("Failed to save");



}

pub fn draw_tree1 (tree: &mut ArenaTree<String>) {
println!("DRAW {:?}",tree);
// draw_sq(12.0,74.0,25);

}
pub fn get_sq (x: f32, y:f32,s:u32) -> Drawing {
    let mut rect = Drawing::new()
    // give it a shape
    .with_shape(Shape::Rectangle {
        width: s,
        height: s,
    })
    // move it around
    .with_xy(x, y)
    // give it a cool style
    .with_style(Style::stroked(5, Color::black()));
    rect

}

pub fn draw_sq (x: f32, y:f32,s:u32) {
println!("Drawing");


let mut canvas = Canvas::new(100, 100);

// create a new drawing
let mut rect = Drawing::new()
// give it a shape
.with_shape(Shape::Rectangle {
    width: s,
    height: s,
})
// move it around
.with_xy(x, y)
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
