#[cfg(test)]

mod tests {
    use rust_ray_tracer::objects::smooth_triangle::SmoothTriangle;
    use rust_ray_tracer::objects::object::Object;
    use rust_ray_tracer::core::vector::Vec4;
    use rust_ray_tracer::ray_tracing::ray::Ray;

    #[test]
    //Tests intersecting a ray with a smooth triangle
    fn smooth_triangle_intersection() {
        let triangle = SmoothTriangle::default();
        let ray = Ray::new((-0.2, 0.3, -2.0), (0.0, 0.0, 1.0));
        let intersections = triangle.intersect(&ray).unwrap();
        assert_eq!(intersections.len(), 1);
        assert_eq!(intersections[0].u.unwrap(), 0.45);
        assert_eq!(intersections[0].v.unwrap(), 0.25);
    }

    #[test]
    //Tests finding the normal on a triangle
    fn triangle_normals() {
        let triangle = SmoothTriangle::default();
        let ray = Ray::new((-0.2, 0.3, -2.0), (0.0, 0.0, 1.0));
        let intersections = triangle.intersect(&ray).unwrap(); 
        assert_eq!(intersections[0].normal.round(), Vec4(-0.5547, 0.83205, 0.0, 0.0).round());
    }
}