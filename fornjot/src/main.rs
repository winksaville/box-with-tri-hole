use fj::core::algorithms::triangulate::Triangulate;
use fj_interop::Tolerance;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut fj = fj::Instance::new();
    let width = 40.; // width of the box
    let length = 40.; // length of the box
    let height = 5.; // height of the box
    let diameter = 15.; // diameter of circle for the hole

    let hole_radius = diameter / 2.;

    let segments_string = std::env::args().nth(1).unwrap_or("3".to_string());
    let segments: usize = match segments_string.parse() {
        Ok(v) => v,
        _ => {
            eprintln!("Expected segments to be a number");
            return Ok(());
        }
    };
    if segments < 3 {
        eprintln!("Segments for the hole must be >= 3");
        return Ok(());
    }

    // Create model of the box with hole
    let model = box_with_hole::model([width, length, height], hole_radius, segments, &mut fj.core);

    // Triangulate the model
    let tolerance = Tolerance::from_scalar(0.01)?;
    let tri_mesh = (&model, tolerance).triangulate(&mut fj.core);

    // Export to STL
    let file_name = format!("box-with-hole_segments-{segments}.stl");
    println!("Writing file: {}", file_name);
    fj_export::export(tri_mesh.all_triangles(), &file_name)?;

    Ok(())
}
