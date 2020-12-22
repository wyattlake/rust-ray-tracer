#[cfg(test)]

mod tests {
    use rust_ray_tracer::objects::cylinder::Cylinder;
    use rust_ray_tracer::objects::object::*;
    use rust_ray_tracer::core::matrix::Matrix4x4;
    use rust_ray_tracer::core::vector::Vec4;
    use rust_ray_tracer::materials::material::*;
    use rust_ray_tracer::ray_tracing::ray::Ray;

    #[test]
    //Tests if rays miss a cylinder
    fn rays_miss_cylinder() {
        let cylinder = Cylinder::default();

        let ray1 = Ray::new((1.0, 0.0, 0.0), (0.0, 1.0, 0.0));
        let ray2 = Ray::new((0.0, 0.0, 0.0), (0.0, 1.0, 0.0));
        let ray3 = Ray::new((0.0, 0.0, -5.0), (1.0, 1.0, 1.0));

        assert_eq!(cylinder.intersect(&ray1), None);
        assert_eq!(cylinder.intersect(&ray2), None);
        assert_eq!(cylinder.intersect(&ray3), None);
    }

    #[test]
    //Tests if rays hit a cylinder
    fn rays_hit_cylinder() {
        let cylinder = Cylinder::default();

        let ray1 = Ray::new((1.0, 0.0, -5.0), (0.0, 0.0, 1.0));

        assert_eq!(cylinder.intersect(&ray1).unwrap()[0].t, 5.0);
        assert_eq!(cylinder.intersect(&ray1).unwrap()[1].t, 5.0);
        
        let ray2 = Ray::new((0.0, 0.0, -5.0), (0.0, 0.0, 1.0));

        assert_eq!(cylinder.intersect(&ray2).unwrap()[0].t, 4.0);
        assert_eq!(cylinder.intersect(&ray2).unwrap()[1].t, 6.0);

        // let ray3 = Ray::new((0.5, 0.0, -5.0), (0.1, 1.0, 1.0));

        // assert_eq!(cylinder.intersect(&ray3).unwrap()[0].t, 6.80798);
        // assert_eq!(cylinder.intersect(&ray3).unwrap()[1].t, 7.08872);
    }

    #[test]
    //Tests cylinder normals
    fn cylinder_normal() {
        let cylinder = Cylinder::default();
        
        assert_eq!(cylinder.normal(&Vec4(1.0, 0.0, 0.0, 1.0)), Vec4(1.0, 0.0, 0.0, 0.0));
        assert_eq!(cylinder.normal(&Vec4(0.0, 5.0, -1.0, 1.0)), Vec4(0.0, 0.0, -1.0, 0.0));
        assert_eq!(cylinder.normal(&Vec4(0.0, -2.0, 1.0, 1.0)), Vec4(0.0, 0.0, 1.0, 0.0));
        assert_eq!(cylinder.normal(&Vec4(-1.0, -1.0, 0.0, 1.0)), Vec4(-1.0, 0.0, 0.0, 0.0));
    }

    #[test]
    //Tests truncated cylinder intersections
    fn truncated_cylinder_intersections() {
        let cylinder = Cylinder::new(Matrix4x4::identity(), Material::default(), 1.0, 2.0, false);

        assert_eq!(cylinder.intersect(&Ray::new((0.0, 1.5, 0.0), (0.1, 1.0, 0.0))), None);
        assert_eq!(cylinder.intersect(&Ray::new((0.0, 3.0, -5.0), (0.0, 0.0, 1.0))), None);
        assert_eq!(cylinder.intersect(&Ray::new((0.0, 0.0, -5.0), (0.0, 0.0, 1.0))), None);
        assert_eq!(cylinder.intersect(&Ray::new((0.0, 2.0, -5.0), (0.0, 0.0, 1.0))), None);
        assert_eq!(cylinder.intersect(&Ray::new((0.0, 1.0, -5.0), (0.0, 0.0, 1.0))), None);
        assert_eq!(cylinder.intersect(&Ray::new((0.0, 1.5, -2.0), (0.0, 0.0, 1.0))).unwrap().len(), 2);
    }

    #[test]
    //Tests cylinder cap intersection
    fn cylinder_cap_intersections() {
        let cylinder = Cylinder::new(Matrix4x4::identity(), Material::default(), 1.0, 2.0, true);

        assert_eq!(cylinder.intersect(&Ray::new((0.0, 3.0, 0.0), (0.0, -1.0, 0.0))).unwrap().len(), 2);
        assert_eq!(cylinder.intersect(&Ray::new((0.0, 3.0, -2.0), (0.0, -1.0, 2.0))).unwrap().len(), 2);
        assert_eq!(cylinder.intersect(&Ray::new((0.0, 4.0, -2.0), (0.0, -1.0, 1.0))).unwrap().len(), 2);
        assert_eq!(cylinder.intersect(&Ray::new((0.0, 0.0, -2.0), (0.0, 1.0, 2.0))).unwrap().len(), 2);
        assert_eq!(cylinder.intersect(&Ray::new((0.0, -1.0, -2.0), (0.0, 1.0, 1.0))).unwrap().len(), 2);
    }
}