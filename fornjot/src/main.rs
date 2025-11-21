fn main() -> fj::Result {
    let mut fj = fj::Instance::new();
    let args = fj::Args::parse();

    // Hard-coded dimensions: 40x40x15mm box
    let model = cuboid::model([40.0, 40.0, 15.0], &mut fj.core);
    fj.process_model_args(&model, args)?;

    Ok(())
}
