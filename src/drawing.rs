use std::cmp;
use log::{info};
use crate::arena::ArenaTree;
use crate::arena::Event;
use svg::Document;
use svg::node::element::Path;
use svg::node::element::Circle;
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
         let  event = &index.e;
         match event {
              Event::Leaf        =>  document.append(get_carre(index.x,index.y,3.0,"red".to_string())),
              Event::Duplication =>  document.append(get_carre(index.x,index.y,5.0,"blue".to_string())),
              Event::Loss =>        {
                                        let mut cross = get_cross(index.x,index.y,3.0,"blue".to_string());
                                        cross.assign("transform","rotate(45 ".to_owned()+&index.x.to_string()+" "+&index.y.to_string()+")");
                                        document.append(cross);
                                    },
              _                  =>  document.append(get_circle(index.x,index.y,2.0,"blue".to_string())),
         };
         // document.append(symbole);
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
pub fn get_carre (x: f32, y:f32, s:f32, c:String) -> Path {
    let data = Data::new()
    .move_to((x*1.0 -s*0.5 , y*1.0 -s*0.5))
    .line_by((0, s))
    .line_by((s, 0))
    .line_by((0, -s))
    .close();

    let fill = c.clone();
    let path = Path::new()
    .set("fill", fill)
    .set("stroke", c)
    .set("stroke-width", 3)
    .set("d", data);

    path
}
/// Draw a circle  of size s at x,y
pub fn get_circle (x: f32, y:f32, r:f32, c:String) -> Circle {
    let fill = c.clone();
    let circle = Circle::new()
    .set("cx", x)
    .set("cy", y)
    .set("r", r)
    .set("fill", fill)
    .set("stroke", c)
    .set("stroke-width", 1);

    circle

}

/// Draw a cross  of size s at x,y
pub fn get_cross (x: f32, y:f32, s:f32, c:String) -> Path {
    let data = Data::new()
    .move_to((x*1.0 , y*1.0 -s*2.0))
    .line_by((0, s*4.0))
    .move_to((x*1.0 -s*2.0, y*1.0 ))
    .line_by((s*4.0, 0));

    let fill = c.clone();
    let path = Path::new()
    .set("fill", fill)
    .set("stroke", c)
    .set("stroke-width", s*1.0)
    .set("d", data);
    // .assign("transform","rotate(90");

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
