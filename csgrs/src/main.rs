use csgrs::traits::CSG;
//use nalgebra::Vector3;

type Mesh = csgrs::mesh::Mesh<()>;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let width = 40.; // width of the box
    let length = 40.; // length of the box
    let height = 5.; // height of the box
    let diameter = 15.; // diameter of circle for the hole
    let segments_string = std::env::args().nth(1).unwrap_or("3".to_string());
    let segments: f64 = match segments_string.parse() {
        Ok(v) => v,
        _ => {
            println!("Expected segments to be a number");
            return Ok(());
        }
    };
    if segments < 3.0 {
        println!("Segements for the hole and must be >= 3");
        return Ok(());
    }

    // Simple box centered on the origin
    let mut cuboid =
        Mesh::cuboid(width, length, height, None).translate(-width / 2., -length / 2., 0.);

    // Create a hole in the cuboid if diameter is greater than zero
    if diameter > 0.0 {
        let hole_radius = diameter / 2.0;

        // A "cylinder" represented by polygon of segments
        let hole = Mesh::cylinder(hole_radius, height, segments as usize, None);

        cuboid = cuboid.difference(&hole);
    }

    let vertices = cuboid.vertices().len();
    let name: String = format!("box-with-hole_segments-{segments}_vertices-{vertices}").into();
    let shape: Vec<u8> = cuboid.to_stl_ascii(&name).into();
    let file_name: String = name + ".stl";
    println!("Writing file: {}", file_name);
    std::fs::write(file_name, shape).unwrap();

    Ok(())
}
