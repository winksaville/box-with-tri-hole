use fj::{
    core::{
        operations::{
            build::{BuildCycle, BuildSketch},
            insert::Insert,
            sweep::SweepSketch,
            update::UpdateSketch,
        },
        topology::{Cycle, Region, Sketch, Solid},
    },
    math::{Scalar, Vector},
};

#[allow(dead_code)]
#[inline]
fn rectangle_cycle(x: Scalar, y: Scalar, core: &mut fj::core::Core) -> Cycle {
    // Counter-clockwise winding for exterior (solid material)
    Cycle::polygon(
        [
            [-x / 2., -y / 2.], // bottom-left
            [x / 2., -y / 2.],  // bottom-right
            [x / 2., y / 2.],   // top-right
            [-x / 2., y / 2.],  // top-left
        ],
        core.layers.topology.surfaces.space_2d(),
        core,
    )
}

#[inline]
fn equilateral_tri_cycle(base: Scalar, core: &mut fj::core::Core) -> Cycle {
    // For an equilateral triangle, calculate the vertices
    // Center the triangle at the origin
    // The base parameter defines the side length
    // The height parameter is for positioning (y-offset from center)

    // For an equilateral triangle with side length 'base':
    // - Height from base to apex: base * sqrt(3) / 2
    // - Distance from center to vertex: base / sqrt(3)

    let half_base = base / 2.;
    let tri_height = base * Scalar::from(3.0f64.sqrt()) / 2.;
    let centroid_to_base = tri_height / 3.;
    let centroid_to_apex = tri_height * 2. / 3.;
    println!(
        "eth  base={base}, half_base={half_base}, tri_height={tri_height}, centroid_to_base={centroid_to_base}, centroid_to_apex={centroid_to_apex}"
    );

    // Define vertices in CLOCKWISE order (since this is a hole)
    // Starting from top apex, going clockwise
    let apex = [Scalar::ZERO, centroid_to_apex];
    let bottom_right = [half_base, -centroid_to_base];
    let bottom_left = [-half_base, -centroid_to_base];

    println!("eth  apex={apex:?}, bottom_right={bottom_right:?}, bottom_left={bottom_left:?}");
    Cycle::polygon(
        [apex, bottom_right, bottom_left],
        core.layers.topology.surfaces.space_2d(),
        core,
    )
}

pub fn model(size: impl Into<Vector<3>>, core: &mut fj::core::Core) -> Solid {
    let [x, y, z] = size.into().components;

    // For now hard code size  of hole
    let tri_base = Scalar::from(15.);

    let bottom_surface = core.layers.topology.surfaces.xy_plane();
    let sweep_path = Vector::from([Scalar::ZERO, Scalar::ZERO, z]);

    // Create the rectangle exterior cycle (counter-clockwise)
    let rect_cycle = rectangle_cycle(x, y, core).insert(core);

    // Create the triangular hole cycle (clockwise for interior)
    let tri_cycle = equilateral_tri_cycle(tri_base, core).insert(core);

    // Create a single Region with the rectangle as exterior and triangle as interior
    let region_with_hole = Region::new(rect_cycle, [tri_cycle]);

    Sketch::empty(&core.layers.topology)
        .add_regions([region_with_hole], core)
        .sweep_sketch(bottom_surface, sweep_path, core)
}

#[cfg(test)]
mod tests {
    use super::*;
    use fj::{core::Core, math::Winding};

    #[test]
    fn rectangle_cycle_basic_structure() {
        let mut core = Core::new();
        let x = Scalar::from(4.0);
        let y = Scalar::from(3.0);

        let cycle = rectangle_cycle(x, y, &mut core);

        // Cycle should have 4 half-edges (one for each side of rectangle)
        assert_eq!(
            cycle.half_edges().len(),
            4,
            "Rectangle cycle should have 4 edges"
        );
    }

    #[test]
    fn rectangle_cycle_winding_is_counterclockwise() {
        let mut core = Core::new();
        let x = Scalar::from(6.0);
        let y = Scalar::from(4.0);

        let cycle = rectangle_cycle(x, y, &mut core);
        let surface = core.layers.topology.surfaces.space_2d();

        // Cycle should have counter-clockwise winding
        let winding = cycle.winding(&core.layers.geometry, &surface);
        assert_eq!(
            winding,
            Winding::Ccw,
            "Rectangle cycle should be counter-clockwise"
        );
    }

    #[test]
    fn equilateral_tri_cycle_winding_is_clockwise() {
        let mut core = Core::new();
        let base = Scalar::from(15.0);

        let cycle = equilateral_tri_cycle(base, &mut core);
        let surface = core.layers.topology.surfaces.space_2d();

        // Triangle hole cycle should have clockwise winding
        let winding = cycle.winding(&core.layers.geometry, &surface);
        assert_eq!(
            winding,
            Winding::Cw,
            "Triangle hole cycle should be clockwise"
        );
    }
}
