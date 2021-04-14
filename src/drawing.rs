use log::{info};
use crate::arena::Options;
use crate::arena::ArenaTree;
use crate::arena::Config;
use crate::arena::Event;
use crate::arena::BLOCK;
use crate::arena::PIPEBLOCK;
use svg::Document;
use svg::node::element::Path;
use svg::node::element::Circle;
use svg::node::element::Style;
use svg::node::Text;
use svg::node::element::Element;
use svg::node::element::path::Data;
use svg::Node;
use random_color::{Color,RandomColor,Luminosity};

const GTHICKNESS: usize = 3; // Epaisseur trait gene_
const STHICKNESS: usize = 6; // Epaisseur trait species
const SQUARESIZE: f32 = 6.0; // taille carre dupli

/// Draw a svg simple tree
pub fn draw_tree (tree: &mut ArenaTree<String>, name: String, options: &Options,
    config: &Config) {
    info!("[draw_tree] Drawing tree...");
    let gene_color = & config.single_gene_color;
    let largest_x = tree.get_largest_x() * 1.0 + 0.0 ;
    let largest_y = tree.get_largest_y() * 1.0 + 0.0 ;
    let smallest_x = tree.get_smallest_x() * 1.0 + 0.0 ;
    let smallest_y = tree.get_smallest_y() * 1.0 + 0.0 ;
    let width_svg = largest_x - smallest_x + 0.0;
    let width_svg = width_svg * 1.0;
    let height_svg = largest_y - smallest_y + 2.0 * BLOCK;  // Ajout car les fuielles de l'espece
                                                            // sont allongées lors de la creation
                                                            // du  path . A ameliorer
    let height_svg = height_svg * 1.0;
    let x_viewbox = smallest_x - 0.0 ;
    let y_viewbox = smallest_y - 0.0;
    let  mut document = Document::new()
            .set("width",height_svg + BLOCK )
            .set("height",width_svg + BLOCK )
            .set("viewBox", (x_viewbox,y_viewbox,height_svg + 2.0 * BLOCK ,width_svg + 2.0 * BLOCK ));

    // let style = Style::new(".vert { font: italic 12px serif; fill: green; }");
    let style = Style::new(".gene { font:  ".to_owned()
        + &config.gene_police_size.to_string()+"px serif; fill:"
        + &gene_color.to_string() + "; }" );
    document.append(style);
    let mut g = Element::new("g");
    for  index in &tree.arena {
        let _parent =  match index.parent {
            Some(p) => {
                let n = &tree.arena[p];
                let chemin = match index.is_a_transfert {
                true => {get_chemin_carre(index.x,index.y,n.x,n.y,gene_color.to_string(),
                         config.gene_opacity.to_string(),true)},
                false => {get_chemin_carre(index.x,index.y,n.x,n.y,gene_color.to_string(),
                         config.gene_opacity.to_string(),false)},
                };
                g.append(chemin);
                0
            },
            None => {-1},
        };
        let  event = &index.e;
        match event {
            Event::Leaf        =>  g.append(get_carre(index.x,index.y,2.0,"red".to_string())),
            Event::Duplication =>  g.append(get_carre(index.x,index.y,SQUARESIZE,gene_color.to_string())),
            Event::Loss =>  {
                let mut cross = get_cross(index.x,index.y,4.0,gene_color.to_string());
                cross.assign("transform","rotate(45 ".to_owned()+&index.x.to_string()
                + " "+&index.y.to_string()+")");
                g.append(cross);
            },
            Event::BranchingOut =>  {
                let mut diamond = get_carre(index.x,index.y,12.0,"orange".to_string());
                diamond.assign("transform","rotate(45 ".to_owned() + &index.x.to_string()
                     + " " + &index.y.to_string() + ")" );
                g.append(diamond);
            },

            _   =>  g.append(get_circle(index.x,index.y,3.0,gene_color.to_string())),
        };
        match index.is_a_transfert {
            true => { g.append(get_triangle(index.x,index.y - 6.0,12.0,"yellow".to_string())) },
            false => {},
        };

        let mut element = Element::new("text");
        element.assign("x", index.x-5.0);
        element.assign("y", index.y+10.0);
        element.assign("class", "gene");
        let txt  = Text::new(&index.name);
        element.append(txt);
        element.assign("transform","rotate(90 ".to_owned()+&(index.x - 5.0).to_string()
        + ","+&(index.y + 10.0).to_string()+")");
        match tree.is_leaf(index.idx) {
            true => g.append(element),
            _   =>  match options.gene_internal {
                        true =>  g.append(element),
                        false => {},
                    },
        };
    }
    let mut transfo: String = "translate(  ".to_owned();
    transfo.push_str(&( x_viewbox).to_string());
    transfo.push_str(" ");
    transfo.push_str(&((width_svg  + y_viewbox)).to_string());
    transfo.push_str(") rotate(-90 0 0 ) ");
    g.assign("transform",transfo);
    document.append(g);
    svg::save(name, &document).unwrap();
}

/// Draw a svg pipe species tree and  several gene trees inside it
pub fn draw_sptree_gntrees (
    sp_tree: &mut ArenaTree<String>,
    gene_trees:&mut std::vec::Vec<ArenaTree<String>>,
    name: String, options: &Options, config: &Config
    ) {
    let largest_x = sp_tree.get_largest_x() * 1.0 + 0.0 ;
    let largest_y = sp_tree.get_largest_y() * 1.0 + 0.0 ;
    let smallest_x = sp_tree.get_smallest_x() * 1.0 + 0.0 ;
    let smallest_y = sp_tree.get_smallest_y() * 1.0 + 0.0 ;
    let width_svg = largest_x - smallest_x+ 1.0 * BLOCK;
    let width_svg = width_svg * 1.0;
    let height_svg = largest_y - smallest_y + 2.0 * BLOCK;  // Ajout car les fuielles de l'espece
                                                            // sont allongées lors de la creation
                                                            // du  path . A ameliorer
    let height_svg = height_svg * 1.0;
    let x_viewbox = smallest_x - 0.0 ;
    let y_viewbox = smallest_y - 0.0;
    let  mut document = Document::new()
            .set("width",height_svg  )
            .set("height",width_svg  )
            .set("viewBox", (x_viewbox,y_viewbox,height_svg + 2.0 *BLOCK ,width_svg + 2.0 *BLOCK ));

    // let style = Style::new(".vert { font:  12px serif; fill: green; }");
    // document.append(style);
    let style = Style::new(".species { font: italic ".to_owned()
        + &config.species_police_size.to_string()+"px serif; fill: "
        + &config.species_police_color.to_string()+"; }");
    document.append(style);
    let mut g = Element::new("g");
    // Dessine l'arbre d'espece
    for index in &sp_tree.arena {
        // Dessine le tuyeau
        match index.parent {
            Some(p) => {
                let n = &sp_tree.arena[p];
                let chemin = get_chemin_sp(index.x, index.y,
                                           index.width/2.0, index.height/2.0,
                                           n.x, n.y,
                                           n.width/2.0, n.height/2.0,
                                           config.species_color.to_string(),
                                           config.species_opacity.to_string());
                g.append(chemin);
                if sp_tree.is_leaf(index.idx) {
                    let chemin = close_chemin_sp(index.x, index.y,
                                                 index.width/2.0, index.height/2.0,
                                                 config.species_color.to_string(),
                                                 config.species_opacity.to_string());
                    g.append(chemin);
                }
            },
            None => {},
        };
        let mut element = Element::new("text");
        // Affiche le texte associe au noeud
        match sp_tree.is_leaf(index.idx) {
            true => {
                element.assign("x", index.x-15.0);
                element.assign("y", index.y - index.width /2.0 - 10.0);
                element.assign("class", "species");
                let txt  = Text::new(&index.name);
                element.append(txt);
                element.assign("transform","rotate(90 ".to_owned() + &index.x.to_string()
                + "," + &index.y.to_string() + ")" );
                g.append(element);
            },
            false => {
                match options.species_internal {
                    true => {
                        element.assign("x", index.x+15.0);
                        element.assign("y", index.y+20.0);
                        element.assign("class", "species");
                        let txt  = Text::new(&index.name);
                        element.append(txt);
                        element.assign("transform","rotate(90 ".to_owned()+&index.x.to_string()
                        + "," + &index.y.to_string() + ")" );
                        g.append(element);
                    },
                    false => {},
                };
            },
        };
     }
     let  nb_gntree =  gene_trees.len(); // Nombre d'arbres de gene
     let mut idx_rcgen = 0;
     // Boucle sur les arbres de genes
     loop {
         let base_couleur = match &idx_rcgen % 6 {
             5 => Color::Orange,
             0 => Color::Blue,
             1 => Color::Purple,
             2 => Color::Green,
             3 => Color::Red,
             4 => Color::Yellow,
             _ => Color::Monochrome, // Jamais
         };
        let gene_color = RandomColor::new()
            .hue(base_couleur)
            .luminosity(Luminosity::Bright) // Optional
            .alpha(1.0) // Optional
            .to_rgb_string(); //
        let style = Style::new(".gene_".to_owned()+&idx_rcgen.to_string()
            + " { font: "+ &config.gene_police_size.to_string()+"px serif; fill:" + &gene_color.to_string() + "; }" );
        document.append(style);
        for  index in &gene_trees[idx_rcgen].arena {
            // Dessine le chemin du noeud a son pere
            match index.parent {
                 Some(p) => {
                     let n = &gene_trees[idx_rcgen].arena[p];
                     // La forme du chemin depend de l'evenement
                     let chemin = match index.is_a_transfert {
                        true => {
                            // Verifie que le parent est bien un branchingout
                            match n.e {
                                Event::BranchingOut => get_chemin_transfer(index.x,index.y,
                                                                           n.x,n.y,
                                                                           gene_color.to_string(),
                                                                           true
                                                                           ),
                                Event::BifurcationOut => get_chemin_transfer(index.x,index.y,
                                                                            n.x,n.y,
                                                                            gene_color.to_string(),
                                                                            true
                                                                            ),
                                _ => panic!("Wrong recPhyloXML feature.
                                The father node should be BranchingOut or
                                BifurcationOut, but I found a {:?}\n{:?}",n.e,n),
                            }
                        },
                        false => get_chemin_carre(index.x,index.y,n.x,n.y ,gene_color.to_string(),
                                    config.gene_opacity.to_string(),false),
                     };
                     g.append(chemin);
                 },
                 None => {
                    // C'est la racine
                    let triangle=get_triangle(index.x,index.y-SQUARESIZE,SQUARESIZE,gene_color.to_string());
                    g.append(triangle);
                 },
             };
             // Dessine le symbole associe au noeud
             let  event = &index.e;
             match event {
                 Event::Leaf        =>  g.append(get_carre(index.x,index.y,1.0,"red".to_string())),
                 Event::Duplication =>  g.append(get_carre(index.x,index.y,SQUARESIZE,
                                                 gene_color.to_string())),
                 Event::Loss => {
                     let mut cross = get_cross(index.x,index.y,2.0,gene_color.to_string());
                     cross.assign("transform","rotate(45 ".to_owned() + &index.x.to_string()
                     + " " + &index.y.to_string() + ")" );
                     g.append(cross);
                 },
                // Normalement il ny' a pas d event transferBack : comme il est toujour associé
                // a un autre event,c'est ce dernier qui est stocké dans le champs "e"
                // Par contre le champs "is_a_transfert" indique si le noeud prvient d'un transfer

                Event::BranchingOut  =>  {
                    let mut diamond = get_carre(index.x,index.y,4.0,gene_color.to_string());
                    diamond.assign("transform","rotate(45 ".to_owned() + &index.x.to_string()
                    + " " + &index.y.to_string() + ")" );
                    g.append(diamond);
                    },
                // Est ce que BifurcationOut existe encore ???
                Event::BifurcationOut  =>  g.append(get_carre(index.x,index.y,5.0,
                                                    "yellow".to_string())),
                _ =>  g.append(get_circle(index.x,index.y,3.0,gene_color.to_string())),
            };
            // Affiche le texte associe au noeud
            match event {
                Event::Leaf        => {
                    let mut element = Element::new("text");
                    element.assign("x", index.x+10.0);
                    element.assign("y", index.y+0.0);
                    element.assign("class", "gene_".to_owned()+&idx_rcgen.to_string());
                    let txt  = Text::new(&index.name);
                    element.append(txt);
                    element.assign("transform","rotate(90 ".to_owned() + &index.x.to_string()
                    + "," + &index.y.to_string() + ")" );
                    g.append(element);
                    },
                _ => {
                    match options.gene_internal {
                        true =>  {
                            let mut element = Element::new("text");
                            element.assign("x", index.x+10.0);
                            element.assign("y", index.y+0.0);
                            element.assign("class", "gene_".to_owned() + &idx_rcgen.to_string());
                            let txt  = Text::new(&index.name);
                            element.append(txt);
                            element.assign("transform","rotate(90 ".to_owned()
                            + &index.x.to_string() + "," + &index.y.to_string()+")");
                            g.append(element);
                        },
                        false => {},
                    }
                },
            }
      }
      idx_rcgen += 1;
      if idx_rcgen == nb_gntree {
          break;
      }
  }
  // g.append(get_cadre(x_viewbox,y_viewbox,width_svg,height_svg,"red".to_string()));
  let mut transfo: String = "translate(  ".to_owned();
  transfo.push_str(&( x_viewbox).to_string());
  transfo.push_str(" ");
  transfo.push_str(&((width_svg  + y_viewbox)).to_string());
  transfo.push_str(") rotate(-90 0 0 ) ");
  g.assign("transform",transfo);
  document.append(g);
  svg::save(name, &document).unwrap();
}

#[allow(dead_code)]
/// Draw a frame
pub fn get_cadre (x: f32, y:f32,w:f32,h:f32, c:String) -> Path {
    let data = Data::new()
    .move_to((x , y))
    .line_by((w, 0.0 ))
    .line_by((0.0, h))
    .line_by((-w, 0.0))
    .close();
    let path = Path::new()
     .set("fill", "none")
    .set("stroke", c)
    .set("stroke-width", 3)
    .set("d", data);
    path
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

/// Draw a triangle  of size s at x,y
pub fn get_triangle (x: f32, y:f32, s:f32, c:String) -> Path {
    let data = Data::new()
    .move_to((x*1.0, y*1.0))
    .line_by((-s, -s))
    .line_by((2.0 * s, 0))
    // .line_by((0, -s))
    .close();
    let fill = c.clone();
    let path = Path::new()
    .set("fill", fill)
    .set("stroke", c)
    .set("stroke-width", 1)
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
    path
}

#[allow(dead_code)]
/// Draw a semisquare path between x1,y1 ad x2,y2
// pub fn get_chemin_semisquare (x1: f32, y1:f32,x2: f32, y2:f32) -> Path {
//     let data = Data::new()
//     .move_to((x1*1.0, y1*1.0))
//     .line_to((x1*1.0, (y1+y2)*1.0/2.0))
//     .line_to((x2*1.0, (y1+y2)*1.0/2.0))
//     .line_to((x2*1.0, y2*1.0));
//     let path = Path::new()
//     .set("fill", "none")
//     .set("stroke", "blue")
//     .set("stroke-width", GTHICKNESS)
//     .set("d", data);
//     path
// }

/// Draw a square path between x1,y1 ad x2,y2
pub fn get_chemin_carre (x1: f32, y1:f32,x2: f32, y2:f32, c:String, o:String, stroke:bool)-> Path {
    let data = Data::new()
    .move_to((x1*1.0, y1*1.0))
    .line_to((x1*1.0, y2*1.0))
    .line_to((x2*1.0, y2*1.0));
    let path = Path::new()
    .set("fill", "none")
    .set("stroke", c)
    .set("opacity", o)
    .set("stroke-width", GTHICKNESS);
    let path = match stroke {
        true => path.set("stroke-dasharray","1, 1"),
        false => path,
    };
    let path  = path.set("d", data);
    path
}

/// Draw a transfer path between x1,y1 ad x2,y2
pub fn get_chemin_transfer (x1: f32, y1:f32,x2: f32, y2:f32, c:String, stroke:bool) -> Path {
    // Arrivee du point: un peu avant pour dessiner la fleche
    let initial_y1 = y1 ;
    let y1 = y1 - PIPEBLOCK;
    // Courbure de la courbe de Bezier
    let bez_y = BLOCK;
    // let bez_y = 20.0;
    // Point de controle de la courbe de Bezier
    let controle_x = (x1 + x2) / 2.0 ;
    let controle_y = (y1 + y2) / 2.0 - bez_y ;
    // ligne horizontale
    let data = "M".to_owned() + &x1.to_string() +" " + &initial_y1.to_string()
             +" L "+ &x1.to_string() + " " + &y1.to_string();
    // fleche
    let data = data.to_owned()+ "M" + &x1.to_string() + " "
              + &(initial_y1- PIPEBLOCK / 4.0).to_string() + "L "
              + &(x1 - PIPEBLOCK / 4.0 ).to_string() + " "
              + &(initial_y1 - PIPEBLOCK / 2.0 ).to_string();
    let data = data.to_owned()+ "M"+&x1.to_string() + " "
               + &(initial_y1- PIPEBLOCK / 4.0).to_string() + "L "
               + &(x1 + PIPEBLOCK / 4.0 ).to_string() + " "
               + &(initial_y1 - PIPEBLOCK / 2.0 ).to_string();
    // bezier
    let data = data.to_owned() + "M" + &x1.to_string() + " " + &y1.to_string()
               + " Q " + &controle_x.to_string() + " " + &controle_y.to_string()
               + " " + &x2.to_string() + " " + &y2.to_string();
    let path = Path::new()
    .set("fill", "none")
    .set("stroke", c)
    .set("stroke-width", GTHICKNESS);
    let path = match stroke {
        true => path.set("stroke-dasharray","1, 1"),
        false => path,
    };
    let path  = path.set("d", data);
    path
}

#[allow(dead_code)]
/// Draw a direct path between x1,y1 ad x2,y2
// pub fn get_chemin_simple (x1: f32, y1:f32,x2: f32, y2:f32) -> Path {
//     let data = Data::new()
//     .move_to((x1*1.0, y1*1.0))
//     .line_to((x2*1.0, y2*1.0));
//     let path = Path::new()
//     .set("fill", "none")
//     .set("stroke", "blue")
//     .set("stroke-width", 1)
//     .set("d", data);
//     path
// }

/// Draw a square pipe path between x1,y1 ad x2,y2
pub fn get_chemin_sp (x1: f32, y1:f32, width1:f32, height1:f32, x2: f32, y2:f32,
                      width2:f32, height2:f32, c:String, o:String ) -> Path {
    if x1 < x2 {
        let data = Data::new()
        .move_to((x1 - width1, y1 - height1 + (STHICKNESS / 2)  as f32))
        .line_to((x1 - width1, y2 - height2))
        .line_to((x2 - width2, y2 - height2))
        .move_to((x1 + width1, y1 - height1 + (STHICKNESS / 2)  as f32 ))
        .line_to((x1 + width1, y2 + height2))
        .line_to((x2, y2 + height2));
        let path = Path::new()
        .set("fill", "none")
        .set("stroke", c)
        .set("opacity", o)
        .set("stroke-width", STHICKNESS)
        .set("d", data);
        path
    }
    else {
        let data = Data::new()
        .move_to((x1 + width1, y1 - height1 + (STHICKNESS / 2)  as f32 ))
        .line_to((x1 + width1, y2 - height2))
        .line_to((x2 + width2, y2 - height2))
        .move_to((x1 - width1, y1 - height1 + (STHICKNESS / 2)  as f32))
        .line_to((x1 - width1, y2 + height2))
        .line_to((x2, y2 + height2));
        let path = Path::new()
        .set("fill", "none")
        .set("stroke", c)
        .set("opacity", o)
        .set("stroke-width", STHICKNESS)
        .set("d", data);
        path
    }
}

/// Finish  the drawing of species tree at the leaves level.
pub fn close_chemin_sp (x1: f32, y1:f32, width1:f32, height1:f32, c:String, o:String ) -> Path {
        let data = Data::new()
        .move_to((x1 - width1, y1 - height1))
        .line_to((x1 - width1, y1 + 2.0 * height1))
        .line_to((x1 + width1, y1 + 2.0 * height1))
        .line_to((x1 + width1, y1 - height1));
        let path = Path::new()
        .set("fill", "none")
        .set("stroke", c)
        .set("opacity", o)
        .set("stroke-width", STHICKNESS)
        .set("d", data);
        path
}
