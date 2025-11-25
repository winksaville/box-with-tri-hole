use csgrs::traits::CSG;
//use nalgebra::Vector3;

type Mesh = csgrs::mesh::Mesh<()>;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let width = 40.; // width of the box
    let length = 40.; // length of the box
    let height = 15.; // height of the box
    let diameter = 15.; // diameter of circle for the hole
    let segments = 3; // A 3 segment circle is a triangle

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

    let name: String = "box-with-tri-hole".into();
    let shape: Vec<u8> = cuboid.to_stl_ascii(&name).into();
    let file_name: String = name + ".stl";
    println!("Writing file: {}", file_name);
    std::fs::write("box-with-tri-hole.stl", shape).unwrap();

    Ok(())
}
