
use crate::arena::ArenaTree;
use svg::Document;
use svg::node::Comment;
use svg::node::element::Path;
use svg::node::element::Rectangle;
use svg::node::Text;
use svg::node::element::Element;
use svg::node::element::path::Data;
use svg::Node;

pub fn draw_tree (tree: &mut ArenaTree<String>) {
    let  mut document = Document::new()
    .set("viewBox", (0, 0, 700, 700));

    // let rectangle2 = Rectangle::new()
    //     .set("v", "12")
    //     .set("y", "54")
    //     .set("width", "200")
    //     .set("height", "35");

    for  index in &tree.arena {
         println!("SVG {:?}",index.x);
         let carre = get_carre(index.x,index.y,3.0);
         document.append(carre);
         let mut element = Element::new("text");
         element.assign("x", index.x*3.0);
         element.assign("y", index.y*2.0);
         element.assign("colour", "green");
         let mut txt  = Comment::new("lol");
         let mut txt  = Text::new(&index.name);
         //txt.set("x","10");
         //element.append(Text::new("ARF"));
         element.append(txt);

         document.append(element);
     }
     svg::save("image.svg", &document).unwrap();
}

pub fn get_carre (x: f32, y:f32,s:f32) -> Path {
    let data = Data::new()
    .move_to((x*3.0, y*2.0))
    .line_by((0, s))
    .line_by((s, 0))
    .line_by((0, -s))
    .close();

    let path = Path::new()
    .set("fill", "none")
    .set("stroke", "black")
    .set("stroke-width", 3)
    .set("d", data);

    path
}
