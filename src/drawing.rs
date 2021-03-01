use std::cmp;
use log::{info};
use crate::arena::ArenaTree;
use svg::Document;
use svg::node::element::Path;
use svg::node::element::Style;
use svg::node::Text;
use svg::node::element::Element;
use svg::node::element::path::Data;
use svg::Node;

/// Draw a svg tree
pub fn draw_tree (tree: &mut ArenaTree<String>, name: String) {
    let largest_x = tree.get_largest_x() + 200.0 ;
    let largest_y = tree.get_largest_y() + 200.0 ;
    // if largest_x < 700.0 {
    //     largest_x = 700.0;
    // }
    // if largest_y < 700.0 {
    //     largest_y = 700.0;
    // }
    // let root = tree.get_root();
    // let x_0 = tree.arena[root].x;
    // let y_0 = tree.arena[root].y;
    let  mut document = Document::new()
    .set("viewBox", (-100, -100, largest_x,largest_y));
    let style = Style::new(".vert { font: italic 12px serif; fill: green; }");
    document.append(style);
    for  index in &tree.arena {
         let _parent =  match index.parent {
             Some(p) => {
                 let n = &tree.arena[p];
                 let chemin = get_chemin_carre(index.x,index.y,n.x,n.y);
                 document.append(chemin);
                 0
                },
             None => {
                 -1},
         };
         let carre = get_carre(index.x,index.y,3.0);
         document.append(carre);
         let mut element = Element::new("text");
         element.assign("x", index.x-5.0);
         element.assign("y", index.y+10.0);
         element.assign("class", "vert");
         let txt  = Text::new(&index.name);
         element.append(txt);
         element.assign("transform","rotate(90 ".to_owned()+&index.x.to_string()+","+&index.y.to_string()+")");
         document.append(element);
     }
     // let largest = cmp::max(largest_x as i32, largest_y as i32);
     let smallest = cmp::min(largest_x as i32, largest_y as i32);
     let mut transfo: String = "rotate(-90)   translate( -".to_owned();
     transfo.push_str(&(smallest/2).to_string());
     transfo.push_str(" -");
     transfo.push_str(&(smallest/2).to_string());
     transfo.push_str(")");
     info!("draw_tree: svg transform = {}",transfo);
     document.assign("transform",transfo);

     svg::save(name, &document).unwrap();
}

/// Draw a square  of size s at x,y
pub fn get_carre (x: f32, y:f32,s:f32) -> Path {
    let data = Data::new()
    .move_to((x*1.0 -s*0.5 , y*1.0 -s*0.5))
    .line_by((0, s))
    .line_by((s, 0))
    .line_by((0, -s))
    .close();

    let path = Path::new()
    .set("fill", "none")
    .set("stroke", "red")
    .set("stroke-width", 3)
    .set("d", data);

    path
}

#[allow(dead_code)]
/// Draw a semisquare path between x1,y1 ad x2,y2
pub fn get_chemin_semisquare (x1: f32, y1:f32,x2: f32, y2:f32) -> Path {
    let data = Data::new()
    .move_to((x1*1.0, y1*1.0))
    .line_to((x1*1.0, (y1+y2)*1.0/2.0))
    .line_to((x2*1.0, (y1+y2)*1.0/2.0))
    .line_to((x2*1.0, y2*1.0));

    let path = Path::new()
    .set("fill", "none")
    .set("stroke", "blue")
    .set("stroke-width", 3)
    .set("d", data);

    path
}
/// Draw a square path between x1,y1 ad x2,y2
pub fn get_chemin_carre (x1: f32, y1:f32,x2: f32, y2:f32) -> Path {
    let data = Data::new()
    .move_to((x1*1.0, y1*1.0))
    .line_to((x1*1.0, y2*1.0))
    .line_to((x2*1.0, y2*1.0));

    let path = Path::new()
    .set("fill", "none")
    .set("stroke", "blue")
    .set("stroke-width", 3)
    .set("d", data);

    path
}

#[allow(dead_code)]
/// Draw a direct path between x1,y1 ad x2,y2
pub fn get_chemin_simple (x1: f32, y1:f32,x2: f32, y2:f32) -> Path {
    let data = Data::new()
    .move_to((x1*1.0, y1*1.0))
    .line_to((x2*1.0, y2*1.0));

    let path = Path::new()
    .set("fill", "none")
    .set("stroke", "blue")
    .set("stroke-width", 3)
    .set("d", data);

    path
}
