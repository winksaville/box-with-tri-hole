use fj::{
    core::{
        operations::{
            build::{BuildRegion, BuildSketch},
            sweep::SweepSketch,
            update::UpdateSketch,
        },
        topology::{Region, Sketch, Solid},
    },
    math::{Scalar, Vector},
};

#[allow(dead_code)]
#[inline]
fn rectangle_region(x: Scalar, y: Scalar, core: &mut fj::core::Core) -> Region {
    Region::polygon(
        [
            [-x / 2., -y / 2.],
            [x / 2., -y / 2.],
            [x / 2., y / 2.],
            [-x / 2., y / 2.],
        ],
        core.layers.topology.surfaces.space_2d(),
        core,
    )
}

pub fn model(size: impl Into<Vector<3>>, core: &mut fj::core::Core) -> Solid {
    let [x, y, z] = size.into().components;

    let bottom_surface = core.layers.topology.surfaces.xy_plane();
    let sweep_path = Vector::from([Scalar::ZERO, Scalar::ZERO, -z]);

    Sketch::empty(&core.layers.topology)
        .add_regions(
            [rectangle_region(x, y, core)],
            core,
        )
        .sweep_sketch(bottom_surface, sweep_path, core)
}

#[cfg(test)]
mod tests {
    use super::*;
    use fj::core::{
        validation::{ValidationConfig, ValidationError},
        Core,
    };
    use fj::math::Winding;

    /// Trait to validate objects and return the first error if any
    trait ValidateExt {
        fn validate_and_return_first_error(
            &self,
            geometry: &fj::core::geometry::Geometry,
        ) -> Result<(), ValidationError>;
    }

    impl ValidateExt for Region {
        fn validate_and_return_first_error(
            &self,
            geometry: &fj::core::geometry::Geometry,
        ) -> Result<(), ValidationError> {
            use fj::core::validate::Validate;
            let config = ValidationConfig::default();
            let mut errors = Vec::new();
            self.validate(&config, &mut errors, geometry);
            if let Some(err) = errors.into_iter().next() {
                Err(err)
            } else {
                Ok(())
            }
        }
    }

    #[test]
    fn rectangle_region_basic_structure() {
        let mut core = Core::new();
        let x = Scalar::from(4.0);
        let y = Scalar::from(3.0);

        let region = rectangle_region(x, y, &mut core);

        // Should have an exterior cycle
        let exterior = region.exterior();

        // Exterior cycle should have 4 half-edges (one for each side of rectangle)
        assert_eq!(
            exterior.half_edges().len(),
            4,
            "Rectangle should have 4 edges"
        );

        // Should have no interior cycles (no holes)
        assert_eq!(
            region.interiors().len(),
            0,
            "Rectangle should have no interior cycles"
        );
    }

    #[test]
    fn rectangle_region_validation_passes() {
        let mut core = Core::new();
        let x = Scalar::from(10.0);
        let y = Scalar::from(5.0);

        let region = rectangle_region(x, y, &mut core);

        // Region should pass validation
        region
            .validate_and_return_first_error(&core.layers.geometry)
            .expect("Rectangle region should be valid");
    }

    #[test]
    fn rectangle_region_winding_is_counterclockwise() {
        let mut core = Core::new();
        let x = Scalar::from(6.0);
        let y = Scalar::from(4.0);

        let region = rectangle_region(x, y, &mut core);
        let surface = core.layers.topology.surfaces.space_2d();

        // Exterior cycle should have counter-clockwise winding
        let winding = region.exterior().winding(&core.layers.geometry, &surface);
        assert_eq!(
            winding,
            Winding::Ccw,
            "Exterior cycle should be counter-clockwise"
        );
    }

    #[test]
    fn rectangle_region_different_aspect_ratios() {
        let mut core = Core::new();

        // Test square
        let square = rectangle_region(Scalar::from(5.0), Scalar::from(5.0), &mut core);
        assert_eq!(square.exterior().half_edges().len(), 4);
        square
            .validate_and_return_first_error(&core.layers.geometry)
            .expect("Square should be valid");

        // Test wide rectangle
        let wide = rectangle_region(Scalar::from(10.0), Scalar::from(2.0), &mut core);
        assert_eq!(wide.exterior().half_edges().len(), 4);
        wide.validate_and_return_first_error(&core.layers.geometry)
            .expect("Wide rectangle should be valid");

        // Test tall rectangle
        let tall = rectangle_region(Scalar::from(2.0), Scalar::from(10.0), &mut core);
        assert_eq!(tall.exterior().half_edges().len(), 4);
        tall.validate_and_return_first_error(&core.layers.geometry)
            .expect("Tall rectangle should be valid");
    }

    #[test]
    fn rectangle_region_small_dimensions() {
        let mut core = Core::new();

        // Test very small rectangle
        let small = rectangle_region(Scalar::from(0.1), Scalar::from(0.1), &mut core);
        assert_eq!(small.exterior().half_edges().len(), 4);
        small
            .validate_and_return_first_error(&core.layers.geometry)
            .expect("Small rectangle should be valid");
    }

    #[test]
    fn rectangle_region_vertex_positions() {
        let mut core = Core::new();
        let x = Scalar::from(4.0);
        let y = Scalar::from(2.0);

        let region = rectangle_region(x, y, &mut core);
        let surface = core.layers.topology.surfaces.space_2d();

        // Collect all vertices by iterating through half-edges
        let half_edges: Vec<_> = region.exterior().half_edges().iter().collect();

        // Should have 4 half-edges, each with a start vertex
        assert_eq!(half_edges.len(), 4, "Should have 4 half-edges");

        // Get positions of all vertices in surface coordinates
        let mut positions = Vec::new();
        for half_edge in &half_edges {
            let vertex = half_edge.start_vertex();
            let curve = half_edge.curve();

            // Get vertex position on the curve
            if let Some(vertex_geom) = core.layers.geometry.of_vertex(vertex) {
                if let Some(pos_on_curve) = vertex_geom.local_on(curve) {
                    // Convert from curve coords to surface coords
                    if let Some(curve_geom) = core.layers.geometry.of_curve(curve) {
                        if let Some(local_curve) = curve_geom.local_on(&surface) {
                            let pos_surface = local_curve.path.point_from_path_coords(pos_on_curve.position);
                            positions.push(pos_surface);
                        }
                    }
                }
            }
        }

        assert_eq!(positions.len(), 4, "Should have 4 vertex positions");

        // Expected corners (in counter-clockwise order from bottom-left)
        let expected_x = x / 2.0;
        let expected_y = y / 2.0;
        let tolerance = 1e-10;

        // Verify we have all four corners (order may vary, so we check for presence)
        let expected_corners = vec![
            [-expected_x.into_f64(), -expected_y.into_f64()],
            [expected_x.into_f64(), -expected_y.into_f64()],
            [expected_x.into_f64(), expected_y.into_f64()],
            [-expected_x.into_f64(), expected_y.into_f64()],
        ];

        for expected in &expected_corners {
            let found = positions.iter().any(|pos| {
                let dx = (pos.coords.components[0].into_f64() - expected[0]).abs();
                let dy = (pos.coords.components[1].into_f64() - expected[1]).abs();
                dx < tolerance && dy < tolerance
            });
            assert!(
                found,
                "Expected corner {:?} not found in positions {:?}",
                expected,
                positions.iter().map(|p| [p.coords.components[0].into_f64(), p.coords.components[1].into_f64()]).collect::<Vec<_>>()
            );
        }
    }
}
