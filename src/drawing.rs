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
             Event::Leaf        =>  document.append(get_carre(index.x,index.y,2.0,"red".to_string())),
             Event::Duplication =>  document.append(get_carre(index.x,index.y,2.0,"blue".to_string())),
             Event::Loss =>        {
                                        let mut cross = get_cross(index.x,index.y,2.0,"blue".to_string());
                                        cross.assign("transform","rotate(45 ".to_owned()+&index.x.to_string()+" "+&index.y.to_string()+")");
                                        document.append(cross);
                                    },
              _                  =>  document.append(get_circle(index.x,index.y,2.0,"blue".to_string())),
         };
         // document.append(symbole);
         // match event {
         //     Event::Leaf => {
                 let mut element = Element::new("text");
                 element.assign("x", index.x-5.0);
                 element.assign("y", index.y+10.0);
                 element.assign("class", "vert");
                 let txt  = Text::new(&index.name);
                 element.append(txt);
                 element.assign("transform","rotate(90 ".to_owned()+&(index.x - 5.0).to_string()+","+&(index.y + 10.0).to_string()+")");
                 document.append(element);
         //     },
         //     _           =>  {},
         // }
     }
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

/// Draw a svg species tree
pub fn draw_sptree (tree: &mut ArenaTree<String>, name: String) {
    let largest_x = tree.get_largest_x() + 200.0 ;
    let largest_y = tree.get_largest_y() + 200.0 ;
    let  mut document = Document::new()
    .set("viewBox", (-100, -100, largest_x,largest_y));
    let style = Style::new(".vert { font: italic 12px serif; fill: green; }");
    document.append(style);
    for  index in &tree.arena {
         let _parent =  match index.parent {
             Some(p) => {
                 let n = &tree.arena[p];
                 let chemin = get_chemin_sp(index.x,index.y,index.width,n.x,n.y);
                 document.append(chemin);
                 0
                },
             None => {
                 -1},
         };
         let mut element = Element::new("text");
         element.assign("x", index.x-5.0);
         element.assign("y", index.y+10.0);
         element.assign("class", "vert");
         let txt  = Text::new(&index.name);
         element.append(txt);
         element.assign("transform","rotate(90 ".to_owned()+&index.x.to_string()+","+&index.y.to_string()+")");
         document.append(element);
     }
     let smallest = cmp::min(largest_x as i32, largest_y as i32);
     let mut transfo: String = "rotate(-90)   translate( -".to_owned();
     transfo.push_str(&(smallest/2).to_string());
     transfo.push_str(" -");
     transfo.push_str(&(smallest/2).to_string());
     transfo.push_str(")");
     info!("drawsp_tree: svg transform = {}",transfo);
     document.assign("transform",transfo);
     svg::save(name, &document).unwrap();
}

/// Draw a svg species tree
pub fn draw_sptree_gntree (sp_tree: &mut ArenaTree<String>, gene_tree: &mut ArenaTree<String>, name: String) {
    let largest_x = sp_tree.get_largest_x() + 200.0 ;
    let largest_y = sp_tree.get_largest_y() + 200.0 ;
    let  mut document = Document::new()
    .set("viewBox", (-100, -100, largest_x,largest_y));
    let style = Style::new(".vert { font:  8px serif; fill: green; }");
    document.append(style);
    let style = Style::new(".jaune { font: italic 8px serif; fill: orange; }");
    document.append(style);
    for  index in &sp_tree.arena {
         let _parent =  match index.parent {
             Some(p) => {
                 let n = &sp_tree.arena[p];
                 let chemin = get_chemin_sp(index.x,index.y,index.width,n.x,n.y);
                 document.append(chemin);
                 0
                },
             None => {
                 -1},
         };
         let mut element = Element::new("text");
         element.assign("x", index.x+15.0);
         element.assign("y", index.y+20.0);
         element.assign("class", "jaune");
         let txt  = Text::new(&index.name);
         element.append(txt);
         element.assign("transform","rotate(90 ".to_owned()+&index.x.to_string()+","+&index.y.to_string()+")");
         document.append(element);
     }
     for  index in &gene_tree.arena {
          let _parent =  match index.parent {
              Some(p) => {
                  let n = &gene_tree.arena[p];
                  // La forme du chemin depend de l'evenement
                  let chemin = match index.e {
                      Event::TransferBack   => get_chemin_transfer(index.x,index.y,n.x,n.y),
                      _                     => get_chemin_carre(index.x,index.y,n.x,n.y),
                  };
                  document.append(chemin);
                  0
                 },
              None => {
                  -1},
          };
          let  event = &index.e;
          match event {
               Event::Leaf        =>  document.append(get_carre(index.x,index.y,1.0,"red".to_string())),
               Event::Duplication =>  document.append(get_carre(index.x,index.y,2.0,"blue".to_string())),
               Event::Loss =>        {
                   let mut cross = get_cross(index.x,index.y,2.0,"blue".to_string());
                   cross.assign("transform","rotate(45 ".to_owned()+&index.x.to_string()+" "+&index.y.to_string()+")");
                   document.append(cross);
                },
                Event::TransferBack => {
                    let _parent =  match index.parent {
                        Some(p) => {
                            // Attention, ici on place le symbole a l'emplacement du pere du noeud.
                            // Rappel sur le xml des ev.transferBack : il est toujours suivi d'un
                            // autre evenement:
                            // <eventsRec>
                            //   <transferBack destinationSpecies="5"></transferBack>
                            //   <branchingOut speciesLocation="5"></branchingOut>
                            // </eventsRec>
                            //
                            // <eventsRec>
                            //   <transferBack destinationSpecies="10"></transferBack>
                            //   <speciation speciesLocation="10"></speciation>
                            // </eventsRec>
                            let n = &gene_tree.arena[p];
                            let mut diamond = get_carre(n.x,n.y,4.0,"green".to_string());
                            diamond.assign("transform","rotate(45 ".to_owned()+&n.x.to_string()+" "+&n.y.to_string()+")");
                            document.append(diamond);},
                        None =>{panic!("The TransferBack Node as no father {:?}",index)},
                    };
                },
                Event::BranchingOut  =>  document.append(get_carre(index.x,index.y,1.0,"pink".to_string())),
                Event::BifurcationOut  =>  document.append(get_carre(index.x,index.y,5.0,"yellow".to_string())),
               _                  =>  document.append(get_circle(index.x,index.y,2.0,"blue".to_string())),
          };
          let mut element = Element::new("text");
          element.assign("x", index.x-5.0);
          element.assign("y", index.y+10.0);
          element.assign("class", "vert");
          let txt  = Text::new(&index.name);
          element.append(txt);
          element.assign("transform","rotate(90 ".to_owned()+&index.x.to_string()+","+&index.y.to_string()+")");
          document.append(element);
      }
     let smallest = cmp::min(largest_x as i32, largest_y as i32);
     let mut transfo: String = "rotate(-90)   translate( -".to_owned();
     transfo.push_str(&(smallest/2).to_string());
     transfo.push_str(" -");
     transfo.push_str(&(smallest/2).to_string());
     transfo.push_str(")");
     info!("drawsp_tree: svg transform = {}",transfo);
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
    .set("stroke-width", 1)
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
    .set("stroke-width", 1)
    .set("d", data);

    path
}
/// Draw a transfer path between x1,y1 ad x2,y2
pub fn get_chemin_transfer (x1: f32, y1:f32,x2: f32, y2:f32) -> Path {
    // Courbure de la courbe de Bezier
    // let bez_y = BLOCK; TO DO
    let bez_y = 20.0;
    // Point de controle de la courbe de Bezier
    let controle_x = (x1 + x2) / 2.0 ;
    let controle_y = (y1 + y2) / 2.0 - bez_y ;
    let data = Data::new()
    .move_to((x1, y1))
    .line_to((x2, y2));
    let data = "M".to_owned()+&x1.to_string()+" "+&y1.to_string()+" Q "+&controle_x.to_string()+" "+&controle_y.to_string()+" "+&x2.to_string()+" "+&y2.to_string();
    let path = Path::new()
    .set("fill", "none")
    .set("stroke", "pink")
    .set("stroke-width", 0.5)
    .set("stroke-dasharray","2, 1")
    .set("d",data);

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
    .set("stroke-width", 1)
    .set("d", data);

    path
}
/// Draw a square pipe path between x1,y1 ad x2,y2
pub fn get_chemin_sp (x1: f32, y1:f32, width:f32, x2: f32, y2:f32) -> Path {
    if x1 < x2 {
        let data = Data::new()
        .move_to((x1 - width, y1 - width))
        .line_to((x1 - width, y2 - width))
        .line_to((x2 - width, y2 - width))
        .move_to((x1 + width, y1 - width))
        .line_to((x1 + width, y2 + width))
        .line_to((x2, y2 + width));

        let path = Path::new()
        .set("fill", "none")
        .set("stroke", "pink")
        .set("stroke-width", 1)
        .set("d", data);

        path
    }
    else {
        let data = Data::new()
        .move_to((x1 + width, y1 - width))
        .line_to((x1 + width, y2 - width))
        .line_to((x2 + width, y2 - width))
        .move_to((x1 - width, y1 - width))
        .line_to((x1 - width, y2 + width))
        .line_to((x2, y2 + width));

        let path = Path::new()
        .set("fill", "none")
        .set("stroke", "pink")
        .set("stroke-width", 1)
        .set("d", data);

        path

    }
}
