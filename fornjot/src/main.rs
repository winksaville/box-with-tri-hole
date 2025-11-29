use std::path::PathBuf;

fn main() -> fj::Result {
    let mut fj = fj::Instance::new();

    // Create the box with triangular hole model (40x40x15mm as per spec)
    let model = box_with_tri_hole::model([40.0, 40.0, 5.0], &mut fj.core);

    // Create fs::Args::parse and save to "box-with-tri-hole.stl"
    let mut args = fj::Args::parse();
    args.export = Some(PathBuf::from("box-with-tri-hole.stl"));

    // Output the model
    fj.process_model_args(&model, args)?;

    Ok(())
}
