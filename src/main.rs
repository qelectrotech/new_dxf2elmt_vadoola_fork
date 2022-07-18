extern crate dxf;
extern crate simple_xml_builder;

use std::fs::File;
use dxf::Drawing;
use dxf::entities::*;
use simple_xml_builder::*;
use std::time::*;
use std::env;

fn main() -> dxf::DxfResult<()>{
    // Start recording time
    let now = Instant::now();
    
    // Collect file name argument
    let args: Vec<String> = env::args().collect();
    let file_name: &str = &format!("{}",args[1]);

    // Load dxf file
    let drawing = Drawing::load_file(file_name)?;
    println!("{} loaded...", file_name);
    
    // Intialize counts
    let mut circle_count = 0;
    let mut line_count = 0;
    let mut arc_count = 0;
    //let mut spline_count = 0; **TODO**
    let mut text_count = 0;
    let mut ellipse_count = 0;
    let mut polyline_count = 0;
    //let mut lwpolyline_count = 0; **TODO**
    //let mut solid_count = 0; **TODO**
    let mut other_count = 0;
    let mut _temp = 0.0;

    // Create output file for .elmt
    let mut out_file = File::create(format!("{}.elmt", &file_name[0..file_name.len() - 4])).unwrap();
    println!("{}.elmt was created... \nNow converting {}...", &file_name[0..file_name.len() - 4], file_name);
    
    // Definition defintion ;)
    let mut definition = XMLElement::new("definition");
    definition.add_attribute("height", 10);
    definition.add_attribute("width", 10);
    definition.add_attribute("hotspot_x", 0);
    definition.add_attribute("hotspot_y", 0);
    definition.add_attribute("version", "0.80");
    definition.add_attribute("link_type", "simple");
    definition.add_attribute("type", "element");

    // Define names
    let mut names = XMLElement::new("names");
    let mut name = XMLElement::new("name");
    name.add_attribute("lang", "en");
    name.add_text(format!("{}", &file_name[0..file_name.len() - 4]));
    names.add_child(name);
    definition.add_child(names);

    // Define information
    let mut information = XMLElement::new("informations");
    information.add_text("Created using dxf2elmt!");
    definition.add_child(information);

    // Start description
    let mut description = XMLElement::new("description");

    // Loop through all entities, appending to xml file
    for e in drawing.entities() {
        match e.specific {
            EntityType::Circle(ref circle) => {
                let mut circle_xml = XMLElement::new("ellipse");
                circle_xml.add_attribute("x", circle.center.x - circle.radius);
                circle_xml.add_attribute("y", -circle.center.y - circle.radius);
                circle_xml.add_attribute("height", circle.radius * 2.0);
                circle_xml.add_attribute("width", circle.radius * 2.0);
                circle_xml.add_attribute("antialias", "false");
                
                if circle.thickness > 0.5{
                    circle_xml.add_attribute("style", "line-style:normal;line-weight:normal;filling:none;color:black");
                }else{
                    circle_xml.add_attribute("style", "line-style:normal;line-weight:thin;filling:none;color:black");
                }
                description.add_child(circle_xml);
                
                circle_count += 1;
            },
            EntityType::Line(ref line) => {
                let mut line_xml = XMLElement::new("line");
                line_xml.add_attribute("x1", line.p1.x);
                line_xml.add_attribute("y1", -line.p1.y);
                line_xml.add_attribute("length1", 1.5);
                line_xml.add_attribute("end1", "none");
                line_xml.add_attribute("x2", line.p2.x);
                line_xml.add_attribute("y2", -line.p2.y);
                line_xml.add_attribute("length2", 1.5);
                line_xml.add_attribute("end2", "none");
                line_xml.add_attribute("antialias", "false");
                
                if line.thickness > 0.5{
                    line_xml.add_attribute("style", "line-style:normal;line-weight:normal;filling:none;color:black");
                }else{
                    line_xml.add_attribute("style", "line-style:normal;line-weight:thin;filling:none;color:black");
                }
                description.add_child(line_xml);

                line_count += 1;
            },
            EntityType::Arc(ref arc)=> {
                let mut arc_xml = XMLElement::new("arc");
                arc_xml.add_attribute("x", arc.center.x - arc.radius);
                arc_xml.add_attribute("y", -arc.center.y - arc.radius);
                arc_xml.add_attribute("width", arc.radius*2.0);
                arc_xml.add_attribute("height", arc.radius*2.0);
                
                if arc.start_angle < 0.0{
                    arc_xml.add_attribute("start", -arc.start_angle);
                }else{
                    arc_xml.add_attribute("start", arc.start_angle);
                }

                if arc.start_angle > arc.end_angle{
                    _temp = (360.0 - arc.start_angle) + arc.end_angle;
                }else{
                    _temp = arc.end_angle - arc.start_angle;
                }
                if _temp < 0.0{
                    arc_xml.add_attribute("angle", -_temp);
                }else{
                    arc_xml.add_attribute("angle", _temp);
                }
                
                arc_xml.add_attribute("antialias", "false");
                if arc.thickness > 0.1{
                    arc_xml.add_attribute("style", "line-style:normal;line-weight:normal;filling:none;color:black");
                }else{
                    arc_xml.add_attribute("style", "line-style:normal;line-weight:thin;filling:none;color:black");
                }
                description.add_child(arc_xml);
                arc_count += 1;
            }
            // **TODO**
            // EntityType::Spline(ref spline)=> {
            //     spline_count += 1;
            // }
            EntityType::Text(ref text)=> {
                let mut text_xml = XMLElement::new("text");
                text_xml.add_attribute("x", text.location.x);
                text_xml.add_attribute("y", -text.location.y);
                text_xml.add_attribute("rotation", 0);
                text_xml.add_attribute("text", &text.value[..]);
                text_xml.add_attribute("font", format!("American Typewriter,{},-1,5,0,0,0,0,0,0,normal", text.text_height.ceil()));
                text_xml.add_attribute("antialias", "false");
                if text.thickness > 0.5{
                    text_xml.add_attribute("style", "line-style:normal;line-weight:normal;filling:none;color:black");
                }else{
                    text_xml.add_attribute("style", "line-style:normal;line-weight:thin;filling:none;color:black");
                }
                description.add_child(text_xml);
                text_count += 1;
            }
            EntityType::Ellipse(ref ellipse)=> {
                let mut ellipse_xml = XMLElement::new("ellipse");
                ellipse_xml.add_attribute("x", ellipse.center.x - ellipse.major_axis.x);
                ellipse_xml.add_attribute("y", -ellipse.center.y - ellipse.major_axis.x * ellipse.minor_axis_ratio);
                ellipse_xml.add_attribute("height", ellipse.major_axis.x * 2.0);
                ellipse_xml.add_attribute("width", ellipse.major_axis.x * 2.0 * ellipse.minor_axis_ratio);
                ellipse_xml.add_attribute("antialias", "false");
                ellipse_xml.add_attribute("style", "line-style:normal;line-weight:thin;filling:none;color:black");
                
                description.add_child(ellipse_xml);
                
                ellipse_count += 1;
            }
            EntityType::Polyline(ref polyline)=> {
                let mut polyline_xml = XMLElement::new("polygon");
                
                let mut j: usize = 0;
                for _i in &polyline.__vertices_and_handles{
                    polyline_xml.add_attribute(format!("x{}", (j+1)), polyline.__vertices_and_handles[j].0.location.x);
                    polyline_xml.add_attribute(format!("y{}", (j+1)), -polyline.__vertices_and_handles[j].0.location.y);
                    j += 1;
                }

                polyline_xml.add_attribute("antialias", "false");
                if polyline.thickness > 0.1{
                    polyline_xml.add_attribute("style", "line-style:normal;line-weight:normal;filling:none;color:black");
                }else{
                    polyline_xml.add_attribute("style", "line-style:normal;line-weight:thin;filling:none;color:black");
                }

                description.add_child(polyline_xml);
                
                polyline_count += 1;
            }
            
            // **TODO**
            // EntityType::LwPolyline(ref lwpolyline)=> {
            //     lwpolyline_count += 1;
            // }
            // EntityType::Solid(ref solid)=> {
            //     solid_count += 1;
            // }
            
            _ => {              
                other_count += 1;
            },
        }
    }

    // Write to output file
    definition.add_child(description);
    definition.write(&mut out_file)?;
    
    println!("Conversion complete!\n");

    // Print stats
    println!("STATS");
    println!("~~~~~~~~~~~~~~~");
    println!("Circles: {}", circle_count);
    println!("Lines: {}", line_count);
    println!("Arcs: {}", arc_count);
    //println!("Splines: {}", spline_count);
    println!("Texts: {}", text_count);
    println!("Ellipses: {}", ellipse_count);
    println!("Polylines: {}", polyline_count);
    //println!("LwPolylines: {}", lwpolyline_count);
    //println!("Solids: {}", solid_count);
    println!("Currently Unsupported: {}", other_count);

    println!("\nTime Elapsed: {} ms",now.elapsed().as_millis());

    Ok(())
}
