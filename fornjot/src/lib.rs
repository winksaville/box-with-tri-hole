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

/// Create a regular polygon hole (clockwise winding) circumscribed by a circle
/// with the given radius. The polygon has one vertex pointing up (at -90°).
#[inline]
fn poly_hole(radius: Scalar, segments: usize, core: &mut fj::core::Core) -> Cycle {
    assert!(segments >= 3, "Polygon must have at least 3 segments");

    let angle_step = std::f64::consts::TAU / segments as f64;
    let r = radius.into_f64();

    // Iterate in reverse for clockwise winding (hole)
    // Offset by -90° so first vertex points up
    let points: Vec<[f64; 2]> = (0..segments)
        .rev()
        .map(|i| {
            let angle = i as f64 * angle_step - std::f64::consts::FRAC_PI_2;
            [r * angle.cos(), r * angle.sin()]
        })
        .collect();

    Cycle::polygon(points, core.layers.topology.surfaces.space_2d(), core)
}

pub fn model(
    box_size: impl Into<Vector<3>>,
    hole_radius: f64,
    hole_segments: usize,
    core: &mut fj::core::Core,
) -> Solid {
    let [x, y, z] = box_size.into().components;

    let bottom_surface = core.layers.topology.surfaces.xy_plane();
    let sweep_path = Vector::from([Scalar::ZERO, Scalar::ZERO, z]);

    // Create the rectangle exterior cycle (counter-clockwise)
    let rect_cycle = rectangle_cycle(x, y, core).insert(core);

    // Create the polygon hole cycle (clockwise for interior)
    let hole_cycle = poly_hole(Scalar::from(hole_radius), hole_segments, core).insert(core);

    // Create a single Region with the rectangle as exterior and polygon as interior hole
    let region_with_hole = Region::new(rect_cycle, [hole_cycle]);

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
    fn poly_hole_winding_is_clockwise() {
        let mut core = Core::new();
        let radius = Scalar::from(7.5);

        // Test with triangle (3 segments)
        let cycle = poly_hole(radius, 3, &mut core);
        let surface = core.layers.topology.surfaces.space_2d();

        let winding = cycle.winding(&core.layers.geometry, &surface);
        assert_eq!(
            winding,
            Winding::Cw,
            "Polygon hole cycle should be clockwise"
        );
    }
}
