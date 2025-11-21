# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This repository contains multiple implementations of a 40x40x15mm box with a triangular hole (circumscribed by a 7.5mm radius circle):

- **FreeCAD/**: Design created with FreeCAD 1.0.2, includes .FCStd source file, STL exports, and pre-sliced gcode
- **fornjot/**: Parametric Rust implementation using the Fornjot b-rep CAD kernel

**IMPORTANT**: The Fornjot implementation is currently incomplete. The code in `fornjot/src/lib.rs` is the unmodified fornjot/models/cuboid example and does not yet include the triangular hole feature. Only the basic cuboid geometry is implemented.

## Fornjot (Rust) Implementation

The fornjot implementation is a command-line parametric model written in Rust. It's based on the fornjot/models/cuboid example but intended to be customized for this project.

### Building and Running

```bash
cd fornjot
cargo run -- [OPTIONS]
```

### Command-line Options

- `--x <X>`: Size along x-axis (default: 3.0)
- `--y <Y>`: Size along y-axis (default: 2.0)
- `--z <Z>`: Size along z-axis (default: 1.0)
- `-e, --export <PATH>`: Export model to specified path (supports .stl, .3mf formats)
- `-t, --tolerance <TOLERANCE>`: How much the export can deviate from original
- `-i, --ignore-validation`: Ignore validation errors

### Example Export

```bash
cd fornjot
cargo run -- -e box-with-tri-hole.stl --x 40 --y 40 --z 15
```

To visualize the exported STL, you can use f3d:
```bash
f3d box-with-tri-hole.stl --output box-with-tri-hole.stl.png
```

### Architecture

The Fornjot implementation consists of:

- **src/lib.rs**: Contains the `model()` function that constructs the 3D geometry using Fornjot's API. Creates a rectangular sketch and sweeps it along the z-axis to form a solid. Currently implements only a basic cuboid - the triangular hole feature needs to be added.
- **src/main.rs**: CLI entry point using clap for argument parsing. Parses parameters and passes them to the model function through Fornjot's `Instance` and `process_model_args()`.
- **Cargo.toml**: Uses Fornjot from git repository (revision 9c51e6f8beba413818753cf7acb2f4810f080167) rather than crates.io to access latest features. Package name is currently "cuboid" from the original example.

The model is built using Fornjot's operation-based API: creating sketches with regions, then sweeping them to produce solids. To add the triangular hole, you'll need to use boolean operations or create the hole as part of the sketch regions.

## FreeCAD Implementation

The FreeCAD version includes:
- `box-with-tri-hole.FCStd`: Source design file (requires FreeCAD 1.0.2+)
- Pre-generated STL files for 3D printing
- Pre-sliced gcode for Prusa MK3S with 0.2mm layer height
- `.3mf` project file for PrusaSlicer

The triangular hole is circumscribed by a circle with a 7.5mm radius.

No special commands needed - open `.FCStd` files directly in FreeCAD.

## Project Structure

This is a multi-tool CAD comparison project showing the same design implemented in different systems. Each subdirectory is self-contained with its own README explaining that implementation's specifics.
