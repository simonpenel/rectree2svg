
use crate::arena::ArenaTree;
// use crate::arena::Noeud;
use svg::Document;
use svg::node::element::Path;
use svg::node::element::Style;
use svg::node::Text;
use svg::node::element::Element;
use svg::node::element::path::Data;
use svg::Node;

pub fn draw_tree (tree: &mut ArenaTree<String>, name: String) {
    let  mut document = Document::new()
    .set("viewBox", (0, 0, 1000, 1000));
    let style = Style::new(".vert { font: italic 12px serif; fill: green; }");
    document.append(style);
    for  index in &tree.arena {
         let parent =  match index.parent {
             Some(p) => {
                 // println!("SVG Parent ={:?}",p);
                 let n = &tree.arena[p];
                 // println!("SVG Chemin de {:?}  {:?}  a  {:?}   {:?}  ",index.x,index.y, n.x,n.y);
                 let chemin = get_chemin(index.x,index.y,n.x,n.y);
                let chemin = get_chemin_carre(index.x,index.y,n.x,n.y);
                 document.append(chemin);
                 0
                },
             None => {
                 // println!("SVG Pas de Parent");
                 -1},
         };
         // println!("SVG Parent={:?}",parent);
         let carre = get_carre(index.x,index.y,3.0);
         document.append(carre);
         let mut element = Element::new("text");
         element.assign("x", index.x);
         element.assign("y", index.y);
         element.assign("class", "vert");
         let txt  = Text::new(&index.name);
         let txt  = Text::new(&index.x.to_string());
         let string = &index.x.to_string();
         let string2 = string.to_owned()+&"_".to_string();
         let string2 = string2.to_owned()+&index.idx.to_string();
         // let txt  = Text::new(string.to_owned()+string2);
         let txt  = Text::new(string2);
         element.append(txt);
         document.append(element);
     }
     svg::save(name, &document).unwrap();
}

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


pub fn get_chemin (x1: f32, y1:f32,x2: f32, y2:f32) -> Path {
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
