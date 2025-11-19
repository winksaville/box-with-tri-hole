# box-with-tri-hole

This use [fornjot](https://fornjot.app/), b-rep CAD kernel
written in Rust.

The src/lib.rs and  src/main.rs are exactly from fornjot/models/cuboid
and Cargo.toml is the same excpect for using the "git" dependency
of the latest commit as of today which is from yesterday.

## Usage

```
$ cargo run -- --help
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/cuboid --help`
Standardized CLI for Fornjot models

This is completely optional, as models are just Rust code and don't need any kind of CLI interface. It is useful, however, to provide a standardized interface for viewing and exporting models, and is used for Fornjot's example models and the testing infrastructure they are part of.

You might not want to use this struct directly. [`Instance::process_model`] provides a more high-level and convenient interface.

[`Instance::process_model`]: crate::Instance::process_model

Usage: cuboid [OPTIONS]

Options:
      --x <X>
          Size of the cuboid along the x-axis
          
          [default: 3.0]

      --y <Y>
          Size of the cuboid along the y-axis
          
          [default: 2.0]

      --z <Z>
          Size of the cuboid along the z-axis
          
          [default: 1.0]

  -e, --export <PATH>
          Export model to this path

  -t, --tolerance <TOLERANCE>
          How much the export can deviate from the original model

  -i, --ignore-validation
          Ignore validation errors

  -h, --help
          Print help (see a summary with '-h')
```

## Example

```
wink@3900x 25-11-19T19:33:45.307Z:~/data/3D-Graphics-CAD-CAM/3D-models/box-with-tri-hole/fornjot (main)
$ cargo run -- -e box-with-tri-hole.stl --x 40 --y 40 --z 15
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/cuboid -e box-with-tri-hole.stl --x 40 --y 40 --z 15`
wink@3900x 25-11-19T19:35:25.131Z:~/data/3D-Graphics-CAD-CAM/3D-models/box-with-tri-hole/fornjot (main)
$ f3d box-with-tri-hole.stl --output box-with-tri-hole.stl.png
wink@3900x 25-11-19T19:35:42.275Z:~/data/3D-Graphics-CAD-CAM/3D-models/box-with-tri-hole/fornjot (main)
```
![box-with-tri-hole](./box-with-tri-hole.stl.png)


## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.