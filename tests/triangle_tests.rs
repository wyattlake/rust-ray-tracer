#[cfg(test)]

mod tests {
    use rust_ray_tracer::objects::triangle::Triangle;
    use rust_ray_tracer::objects::object::Object;
    use rust_ray_tracer::core::vector::Vec4;
    use rust_ray_tracer::ray_tracing::ray::Ray;
    use rust_ray_tracer::materials::material::Material;

    #[test]
    //Tests creating a triangle
    fn creating_a_triangle() {
        let triangle = Triangle::new(Vec4(0.0, 1.0, 0.0, 1.0), Vec4(-1.0, 0.0, 0.0, 1.0), Vec4(1.0, 0.0, 0.0, 1.0), Material::default());
        assert_eq!(triangle.e1, Vec4(-1.0, -1.0, 0.0, 0.0));
        assert_eq!(triangle.e2, Vec4(1.0, -1.0, 0.0, 0.0));
        assert_eq!(triangle.normal, Vec4(0.0, 0.0, -1.0, 0.0));
    }

    #[test]
    //Tests finding the normal on a triangle
    fn triangle_normals() {
        let triangle = Triangle::default();
        assert_eq!(triangle.normal(&Vec4(0.0, 0.5, 0.0, 1.0), None, None), triangle.normal);
    }

    #[test]
    //Tests intersecting a ray parallel to a triangle
    fn parallel_ray_intersection() {
        let triangle = Triangle::default();
        let ray = Ray::new((0.0, -1.0, -2.0), (0.0, 1.0, 0.0));
        let intersections = triangle.intersect(&ray);
        assert_eq!(intersections, None);
    }

    #[test]
    //Tests a miss along the p1-p3 edge
    fn p1_p3_ray_miss() {
        let triangle = Triangle::default();
        let ray = Ray::new((1.0, 1.0, -2.0), (0.0, 0.0, 1.0));
        let intersections = triangle.intersect(&ray);
        assert_eq!(intersections, None);
    }

    #[test]
    //Tests a miss along the p1-p2 edge
    fn p1_p2_ray_miss() {
        let triangle = Triangle::default();
        let ray = Ray::new((-1.0, 1.0, -2.0), (0.0, 0.0, 1.0));
        let intersections = triangle.intersect(&ray);
        assert_eq!(intersections, None);
    }

    #[test]
    //Tests a miss along the p2-p3 edge
    fn p2_p3_ray_miss() {
        let triangle = Triangle::default();
        let ray = Ray::new((0.0, -1.0, -2.0), (0.0, 0.0, 1.0));
        let intersections = triangle.intersect(&ray);
        assert_eq!(intersections, None);
    }

    #[test]
    //Tests a ray hitting a triangle
    fn ray_hits_triangle() {
        let triangle = Triangle::default();
        let ray = Ray::new((0.0, 0.5, -2.0), (0.0, 0.0, 1.0));
        let intersections = triangle.intersect(&ray).unwrap();
        assert_eq!(&intersections.len(), &1);
        assert_eq!(&intersections[0].t, &2.0);
    }
}